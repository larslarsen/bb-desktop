use std::collections::HashSet;
use std::fmt;
use std::io::{self, Read, Write};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, SyncSender};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use serde::Serialize;
use serde::de::{DeserializeSeed, Deserializer, Error, MapAccess, SeqAccess, Visitor};
use serde_json::Number;
use sha2::{Digest, Sha256};

const PROTOCOL_NAME: &str = "bitbook-wallet-broker";
const SESSION_DOMAIN: &str = "bitbook-wallet-session-v1\n";
const MAX_FRAME_BODY: u32 = 65_536;
const MAX_JSON_DEPTH: u32 = 128;
const MAX_SEEN_IDS: usize = 4096;
const ACK_DEADLINE: Duration = Duration::from_secs(2);
const HEX: &[u8; 16] = b"0123456789abcdef";
const CLOSED_DIAGNOSTIC: &[u8] = b"broker closed\n";

const REQUEST_KEYS: [&str; 8] = [
    "v",
    "id",
    "seq",
    "kind",
    "method",
    "params",
    "session",
    "expires_ms",
];
const ACK_KEYS: [&str; 4] = ["protocol", "version", "parent_nonce", "parent_pid"];

struct ProtocolClose;

#[derive(Clone, Copy)]
struct SafeError {
    code: &'static str,
    message: &'static str,
    retryable: bool,
}

const ERROR_SCHEMA: SafeError = SafeError {
    code: "SCHEMA",
    message: "Invalid request",
    retryable: false,
};
const ERROR_UNAVAILABLE: SafeError = SafeError {
    code: "UNAVAILABLE",
    message: "Unavailable",
    retryable: true,
};
const ERROR_TIMEOUT: SafeError = SafeError {
    code: "TIMEOUT",
    message: "Timed out",
    retryable: true,
};

enum JsonValue {
    Null,
    Bool,
    Number(Number),
    String(String),
    Array,
    Object(Vec<(String, JsonValue)>),
}

struct ValueSeed {
    depth: u32,
}

struct ValueVisitor {
    depth: u32,
}

impl<'de> DeserializeSeed<'de> for ValueSeed {
    type Value = JsonValue;

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<JsonValue, D::Error> {
        deserializer.deserialize_any(ValueVisitor { depth: self.depth })
    }
}

impl<'de> Visitor<'de> for ValueVisitor {
    type Value = JsonValue;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a JSON value")
    }

    fn visit_bool<E: Error>(self, _: bool) -> Result<JsonValue, E> {
        Ok(JsonValue::Bool)
    }

    fn visit_i64<E: Error>(self, value: i64) -> Result<JsonValue, E> {
        Ok(JsonValue::Number(value.into()))
    }

    fn visit_u64<E: Error>(self, value: u64) -> Result<JsonValue, E> {
        Ok(JsonValue::Number(value.into()))
    }

    fn visit_f64<E: Error>(self, value: f64) -> Result<JsonValue, E> {
        Number::from_f64(value)
            .map(JsonValue::Number)
            .ok_or_else(|| E::custom("non-finite number"))
    }

    fn visit_str<E: Error>(self, value: &str) -> Result<JsonValue, E> {
        Ok(JsonValue::String(value.to_string()))
    }

    fn visit_string<E: Error>(self, value: String) -> Result<JsonValue, E> {
        Ok(JsonValue::String(value))
    }

    fn visit_none<E: Error>(self) -> Result<JsonValue, E> {
        Ok(JsonValue::Null)
    }

    fn visit_unit<E: Error>(self) -> Result<JsonValue, E> {
        Ok(JsonValue::Null)
    }

    fn visit_some<D: Deserializer<'de>>(self, deserializer: D) -> Result<JsonValue, D::Error> {
        ValueSeed { depth: self.depth }.deserialize(deserializer)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<JsonValue, A::Error> {
        if self.depth >= MAX_JSON_DEPTH {
            return Err(A::Error::custom("nesting too deep"));
        }
        while seq
            .next_element_seed(ValueSeed {
                depth: self.depth + 1,
            })?
            .is_some()
        {}
        Ok(JsonValue::Array)
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<JsonValue, A::Error> {
        if self.depth >= MAX_JSON_DEPTH {
            return Err(A::Error::custom("nesting too deep"));
        }
        let mut object = Vec::new();
        let mut seen = HashSet::new();
        while let Some(key) = map.next_key::<String>()? {
            if !seen.insert(key.clone()) {
                return Err(A::Error::custom("duplicate object name"));
            }
            let value = map.next_value_seed(ValueSeed {
                depth: self.depth + 1,
            })?;
            object.push((key, value));
        }
        Ok(JsonValue::Object(object))
    }
}

#[derive(Clone, Copy)]
enum ReadState {
    Header,
    Body { remaining: usize },
}

struct FrameDecoder {
    buf: Vec<u8>,
    state: ReadState,
}

impl FrameDecoder {
    fn new() -> Self {
        Self {
            buf: Vec::with_capacity(4),
            state: ReadState::Header,
        }
    }

    fn push(&mut self, mut input: &[u8], tx: &SyncSender<Inbound>) -> Result<(), ProtocolClose> {
        while !input.is_empty() {
            match self.state {
                ReadState::Header => {
                    let need = 4 - self.buf.len();
                    let take = need.min(input.len());
                    self.buf.extend_from_slice(&input[..take]);
                    input = &input[take..];
                    if self.buf.len() == 4 {
                        let mut header = [0u8; 4];
                        header.copy_from_slice(&self.buf);
                        let length = u32::from_be_bytes(header);
                        self.buf.clear();
                        if length == 0 || length > MAX_FRAME_BODY {
                            let _ = tx.send(Inbound::Protocol);
                            return Err(ProtocolClose);
                        }
                        let remaining = length as usize;
                        self.buf.reserve(remaining);
                        self.state = ReadState::Body { remaining };
                    }
                }
                ReadState::Body { remaining } => {
                    let take = remaining.min(input.len());
                    self.buf.extend_from_slice(&input[..take]);
                    input = &input[take..];
                    let left = remaining - take;
                    if left == 0 {
                        let body = std::mem::take(&mut self.buf);
                        self.state = ReadState::Header;
                        if tx.send(Inbound::Frame(body)).is_err() {
                            return Err(ProtocolClose);
                        }
                    } else {
                        self.state = ReadState::Body { remaining: left };
                    }
                }
            }
        }
        Ok(())
    }

    fn is_idle(&self) -> bool {
        matches!(self.state, ReadState::Header) && self.buf.is_empty()
    }
}

enum Inbound {
    Frame(Vec<u8>),
    CleanEof,
    PartialEof,
    Protocol,
    Io,
}

struct Ack {
    parent_nonce: String,
    parent_pid: String,
}

struct Request {
    id: String,
    method: String,
    params_empty: bool,
    expires_ms: i64,
}

struct Session {
    id: String,
    expected_parent_seq: u64,
    child_seq: u64,
    seen_ids: HashSet<String>,
}

#[derive(Serialize)]
struct Hello<'a> {
    protocol: &'static str,
    min: u8,
    max: u8,
    child_nonce: &'a str,
    child_pid: &'a str,
}

#[derive(Serialize)]
struct Snapshot {
    v: u8,
    broker: &'static str,
    accounts: [&'static str; 0],
}

#[derive(Serialize)]
struct Response<'a> {
    v: u8,
    id: &'a str,
    seq: u64,
    kind: &'static str,
    result: Snapshot,
    session: &'a str,
}

#[derive(Serialize)]
struct ErrorBody {
    code: &'static str,
    message: &'static str,
    retryable: bool,
}

#[derive(Serialize)]
struct ErrorEnvelope<'a> {
    v: u8,
    id: &'a str,
    seq: u64,
    kind: &'static str,
    error: ErrorBody,
    session: &'a str,
}

pub fn run() -> i32 {
    match run_session() {
        Ok(()) => 0,
        Err(ProtocolClose) => {
            let mut stderr = io::stderr();
            let _ = stderr.write_all(CLOSED_DIAGNOSTIC);
            let _ = stderr.flush();
            1
        }
    }
}

fn run_session() -> Result<(), ProtocolClose> {
    let child_pid = std::process::id().to_string();
    let child_nonce = fresh_nonce()?;
    let (tx, rx) = mpsc::sync_channel(1);
    // Dedicated stdin reader so the main thread stays free for a later native
    // event loop. The join handle is dropped; a blocked reader is never joined.
    let reader = thread::Builder::new()
        .name("wallet-broker-stdin".to_string())
        .spawn(move || read_stdin(tx))
        .map_err(|_| ProtocolClose)?;
    drop(reader);

    let mut stdout = io::stdout().lock();
    let hello = Hello {
        protocol: PROTOCOL_NAME,
        min: 1,
        max: 1,
        child_nonce: &child_nonce,
        child_pid: &child_pid,
    };
    write_json(&mut stdout, &hello)?;
    let deadline = Instant::now() + ACK_DEADLINE;
    let ack = wait_ack(&rx, deadline)?;
    let session_id =
        derive_session_id(&ack.parent_pid, &child_pid, &ack.parent_nonce, &child_nonce);
    let mut session = Session {
        id: session_id,
        expected_parent_seq: 1,
        child_seq: 1,
        seen_ids: HashSet::new(),
    };
    loop {
        match recv_inbound(&rx, None)? {
            Inbound::Frame(body) => handle_request(&mut stdout, &mut session, &body)?,
            Inbound::CleanEof => return Ok(()),
            Inbound::PartialEof | Inbound::Protocol | Inbound::Io => {
                return Err(ProtocolClose);
            }
        }
    }
}

fn wait_ack(rx: &Receiver<Inbound>, deadline: Instant) -> Result<Ack, ProtocolClose> {
    match recv_inbound(rx, Some(deadline))? {
        Inbound::Frame(body) => parse_ack(&body),
        Inbound::CleanEof | Inbound::PartialEof | Inbound::Protocol | Inbound::Io => {
            Err(ProtocolClose)
        }
    }
}

fn recv_inbound(
    rx: &Receiver<Inbound>,
    deadline: Option<Instant>,
) -> Result<Inbound, ProtocolClose> {
    match deadline {
        Some(deadline) => {
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() {
                return Err(ProtocolClose);
            }
            match rx.recv_timeout(remaining) {
                Ok(inbound) => Ok(inbound),
                Err(RecvTimeoutError::Timeout) | Err(RecvTimeoutError::Disconnected) => {
                    Err(ProtocolClose)
                }
            }
        }
        None => rx.recv().map_err(|_| ProtocolClose),
    }
}

fn handle_request<W: Write>(
    stdout: &mut W,
    session: &mut Session,
    body: &[u8],
) -> Result<(), ProtocolClose> {
    let request = parse_request(body, &session.id, session.expected_parent_seq)?;
    if session.seen_ids.len() >= MAX_SEEN_IDS && !session.seen_ids.contains(&request.id) {
        return Err(ProtocolClose);
    }
    if !session.seen_ids.insert(request.id.clone()) {
        return Err(ProtocolClose);
    }
    session.expected_parent_seq = session
        .expected_parent_seq
        .checked_add(1)
        .ok_or(ProtocolClose)?;
    if request.expires_ms <= now_ms() {
        return write_error(stdout, session, &request.id, ERROR_TIMEOUT);
    }
    match request.method.as_str() {
        "status.get" | "sync.subscribe" => {
            if request.params_empty {
                write_snapshot(stdout, session, &request.id)
            } else {
                write_error(stdout, session, &request.id, ERROR_SCHEMA)
            }
        }
        "account.list" | "account.lock" | "receiver.fresh" | "intent.begin" | "intent.cancel" => {
            write_error(stdout, session, &request.id, ERROR_UNAVAILABLE)
        }
        _ => write_error(stdout, session, &request.id, ERROR_SCHEMA),
    }
}

fn write_snapshot<W: Write>(
    stdout: &mut W,
    session: &mut Session,
    id: &str,
) -> Result<(), ProtocolClose> {
    let seq = take_child_seq(session)?;
    let envelope = Response {
        v: 1,
        id,
        seq,
        kind: "res",
        result: Snapshot {
            v: 1,
            broker: "degraded",
            accounts: [],
        },
        session: &session.id,
    };
    write_json(stdout, &envelope)
}

fn write_error<W: Write>(
    stdout: &mut W,
    session: &mut Session,
    id: &str,
    error: SafeError,
) -> Result<(), ProtocolClose> {
    let seq = take_child_seq(session)?;
    let envelope = ErrorEnvelope {
        v: 1,
        id,
        seq,
        kind: "error",
        error: ErrorBody {
            code: error.code,
            message: error.message,
            retryable: error.retryable,
        },
        session: &session.id,
    };
    write_json(stdout, &envelope)
}

fn take_child_seq(session: &mut Session) -> Result<u64, ProtocolClose> {
    let seq = session.child_seq;
    session.child_seq = session.child_seq.checked_add(1).ok_or(ProtocolClose)?;
    Ok(seq)
}

fn write_json<W: Write, T: Serialize>(stdout: &mut W, value: &T) -> Result<(), ProtocolClose> {
    let body = serde_json::to_vec(value).map_err(|_| ProtocolClose)?;
    write_frame(stdout, &body)
}

fn write_frame<W: Write>(stdout: &mut W, body: &[u8]) -> Result<(), ProtocolClose> {
    let length = u32::try_from(body.len()).map_err(|_| ProtocolClose)?;
    stdout
        .write_all(&length.to_be_bytes())
        .and_then(|_| stdout.write_all(body))
        .and_then(|_| stdout.flush())
        .map_err(|_| ProtocolClose)
}

fn parse_json_object(body: &[u8]) -> Result<Vec<(String, JsonValue)>, ProtocolClose> {
    let text = std::str::from_utf8(body).map_err(|_| ProtocolClose)?;
    let mut deserializer = serde_json::Deserializer::from_str(text);
    let value = ValueSeed { depth: 0 }
        .deserialize(&mut deserializer)
        .map_err(|_| ProtocolClose)?;
    deserializer.end().map_err(|_| ProtocolClose)?;
    match value {
        JsonValue::Object(fields) => Ok(fields),
        _ => Err(ProtocolClose),
    }
}

fn parse_ack(body: &[u8]) -> Result<Ack, ProtocolClose> {
    let object = parse_json_object(body)?;
    if !exact_keys(&object, &ACK_KEYS) {
        return Err(ProtocolClose);
    }
    let protocol = json_str(field(&object, "protocol")?)?;
    let version = json_u64(field(&object, "version")?)?;
    let parent_nonce = json_str(field(&object, "parent_nonce")?)?;
    let parent_pid = json_str(field(&object, "parent_pid")?)?;
    if protocol != PROTOCOL_NAME
        || version != 1
        || !is_hex_lower(parent_nonce, 32)
        || !is_decimal_pid(parent_pid)
    {
        return Err(ProtocolClose);
    }
    Ok(Ack {
        parent_nonce: parent_nonce.to_string(),
        parent_pid: parent_pid.to_string(),
    })
}

fn parse_request(
    body: &[u8],
    session_id: &str,
    expected_seq: u64,
) -> Result<Request, ProtocolClose> {
    let object = parse_json_object(body)?;
    if !exact_keys(&object, &REQUEST_KEYS) {
        return Err(ProtocolClose);
    }
    let version = json_u64(field(&object, "v")?)?;
    let id = json_str(field(&object, "id")?)?;
    let seq = json_u64(field(&object, "seq")?)?;
    let kind = json_str(field(&object, "kind")?)?;
    let method = json_str(field(&object, "method")?)?;
    let params_empty = json_object_empty(field(&object, "params")?)?;
    let session = json_str(field(&object, "session")?)?;
    let expires_ms = json_i64(field(&object, "expires_ms")?)?;
    if version != 1
        || kind != "req"
        || !is_hex_lower(id, 32)
        || !is_method_name(method)
        || session != session_id
        || seq != expected_seq
    {
        return Err(ProtocolClose);
    }
    Ok(Request {
        id: id.to_string(),
        method: method.to_string(),
        params_empty,
        expires_ms,
    })
}

fn field<'a>(
    object: &'a [(String, JsonValue)],
    name: &str,
) -> Result<&'a JsonValue, ProtocolClose> {
    object
        .iter()
        .find(|(key, _)| key == name)
        .map(|(_, value)| value)
        .ok_or(ProtocolClose)
}

fn exact_keys(object: &[(String, JsonValue)], keys: &[&str]) -> bool {
    object.len() == keys.len()
        && keys
            .iter()
            .all(|key| object.iter().any(|(have, _)| have == key))
}

fn json_str(value: &JsonValue) -> Result<&str, ProtocolClose> {
    match value {
        JsonValue::String(text) => Ok(text),
        _ => Err(ProtocolClose),
    }
}

fn json_object_empty(value: &JsonValue) -> Result<bool, ProtocolClose> {
    match value {
        JsonValue::Object(fields) => Ok(fields.is_empty()),
        _ => Err(ProtocolClose),
    }
}

fn json_u64(value: &JsonValue) -> Result<u64, ProtocolClose> {
    match value {
        JsonValue::Number(number) => number
            .as_u64()
            .or_else(|| number.as_i64().and_then(|int| u64::try_from(int).ok()))
            .ok_or(ProtocolClose),
        _ => Err(ProtocolClose),
    }
}

fn json_i64(value: &JsonValue) -> Result<i64, ProtocolClose> {
    match value {
        JsonValue::Number(number) => number
            .as_i64()
            .or_else(|| number.as_u64().and_then(|int| i64::try_from(int).ok()))
            .ok_or(ProtocolClose),
        _ => Err(ProtocolClose),
    }
}

fn fresh_nonce() -> Result<String, ProtocolClose> {
    let mut bytes = [0u8; 16];
    getrandom::fill(&mut bytes).map_err(|_| ProtocolClose)?;
    Ok(to_hex_lower(&bytes))
}

fn derive_session_id(
    parent_pid: &str,
    child_pid: &str,
    parent_nonce: &str,
    child_nonce: &str,
) -> String {
    let preimage =
        format!("{SESSION_DOMAIN}{parent_pid}\n{child_pid}\n{parent_nonce}\n{child_nonce}");
    to_hex_lower(Sha256::digest(preimage.as_bytes()).as_slice())
}

fn to_hex_lower(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for &byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}

fn is_hex_lower(value: &str, length: usize) -> bool {
    value.len() == length
        && value
            .bytes()
            .all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'))
}

fn is_decimal_pid(value: &str) -> bool {
    let bytes = value.as_bytes();
    match bytes.split_first() {
        Some((first, rest)) => {
            (b'1'..=b'9').contains(first) && rest.iter().all(|byte| byte.is_ascii_digit())
        }
        None => false,
    }
}

fn is_method_name(value: &str) -> bool {
    let mut parts = 0usize;
    for part in value.split('.') {
        let mut chars = part.chars();
        let Some(first) = chars.next() else {
            return false;
        };
        if !first.is_ascii_lowercase() {
            return false;
        }
        if !chars.all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit()) {
            return false;
        }
        parts += 1;
    }
    parts >= 2
}

fn now_ms() -> i64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => i64::try_from(duration.as_millis()).unwrap_or(i64::MAX),
        Err(_) => 0,
    }
}

fn read_stdin(tx: SyncSender<Inbound>) {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut decoder = FrameDecoder::new();
    let mut chunk = [0u8; 8192];
    loop {
        match stdin.read(&mut chunk) {
            Ok(0) => {
                let inbound = if decoder.is_idle() {
                    Inbound::CleanEof
                } else {
                    Inbound::PartialEof
                };
                let _ = tx.send(inbound);
                return;
            }
            Ok(count) => {
                if decoder.push(&chunk[..count], &tx).is_err() {
                    return;
                }
            }
            Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
            Err(_) => {
                let _ = tx.send(Inbound::Io);
                return;
            }
        }
    }
}
