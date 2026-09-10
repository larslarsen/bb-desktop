use std::time::Duration;

use prost::Message;
use tokio::runtime::{Builder, Runtime};
use tokio::time::{sleep, timeout};
use tonic::transport::{Certificate, Channel, ClientTlsConfig, Endpoint};
use zcash_client_backend::proto::compact_formats::CompactBlock;
use zcash_client_backend::proto::service::{
    BlockId, BlockRange, ChainSpec, Empty, TreeState,
    compact_tx_streamer_client::CompactTxStreamerClient,
};
use zcash_protocol::consensus::{BlockHeight, BranchId};

use super::live::{LiveCancellation, LiveEndpoint, LiveSource, SourceMetadata};
use super::{
    MAX_COMPACT_BLOCK_BYTES, MAX_LIVE_BATCH_BLOCKS, MAX_LIVE_BATCH_BYTES, Network, ZecError,
};

const DEADLINE: Duration = Duration::from_secs(15);

pub(crate) struct TonicLiveSource {
    runtime: Option<Runtime>,
    client: Option<CompactTxStreamerClient<Channel>>,
    deadline: Duration,
}

impl TonicLiveSource {
    pub(crate) fn connect(
        endpoint: &LiveEndpoint,
        cancel: &LiveCancellation,
    ) -> Result<Self, ZecError> {
        Self::connect_configured(endpoint, cancel, DEADLINE, None)
    }

    fn connect_configured(
        endpoint: &LiveEndpoint,
        cancel: &LiveCancellation,
        deadline: Duration,
        ca_pem: Option<&[u8]>,
    ) -> Result<Self, ZecError> {
        if deadline.is_zero() {
            return Err(ZecError::schema());
        }
        let tls = match ca_pem {
            Some(bytes) if !bytes.is_empty() => {
                ClientTlsConfig::new().ca_certificate(Certificate::from_pem(bytes))
            }
            Some(_) => return Err(ZecError::schema()),
            None => ClientTlsConfig::new().with_webpki_roots(),
        };
        let configured = Endpoint::from_shared(endpoint.as_str().to_owned())
            .map_err(|_| ZecError::schema())?
            .tls_config(tls)
            .map_err(|_| ZecError::unavailable())?
            .connect_timeout(deadline)
            .timeout(deadline);
        let runtime = Builder::new_current_thread()
            .enable_io()
            .enable_time()
            .build()
            .map_err(|_| ZecError::unavailable())?;
        let channel=runtime.block_on(async { tokio::select! { biased; _=cancelled(cancel)=>Err(ZecError::cancelled()), value=timeout(deadline,configured.connect())=>value.map_err(|_|ZecError::unavailable())?.map_err(|_|ZecError::unavailable()) } });
        let channel = match channel {
            Ok(channel) => channel,
            Err(error) => {
                runtime.shutdown_timeout(Duration::from_millis(250));
                return Err(error);
            }
        };
        Ok(Self {
            runtime: Some(runtime),
            client: Some(
                CompactTxStreamerClient::new(channel)
                    .max_decoding_message_size(MAX_COMPACT_BLOCK_BYTES),
            ),
            deadline,
        })
    }
}

async fn cancelled(cancel: &LiveCancellation) {
    while !cancel.is_cancelled() {
        sleep(Duration::from_millis(20)).await
    }
}
fn rpc_error(cancel: &LiveCancellation) -> ZecError {
    if cancel.is_cancelled() {
        ZecError::cancelled()
    } else {
        ZecError::unavailable()
    }
}
fn status_error(cancel: &LiveCancellation, status: tonic::Status) -> ZecError {
    if matches!(
        status.code(),
        tonic::Code::ResourceExhausted | tonic::Code::OutOfRange
    ) {
        ZecError::limit()
    } else {
        rpc_error(cancel)
    }
}

impl LiveSource for TonicLiveSource {
    fn metadata(
        &mut self,
        _network: Network,
        cancel: &LiveCancellation,
    ) -> Result<SourceMetadata, ZecError> {
        let rt = self.runtime.take().ok_or_else(ZecError::unavailable)?;
        let mut client = self.client.take().ok_or_else(ZecError::unavailable)?;
        let result=rt.block_on(async {tokio::select!{biased;_=cancelled(cancel)=>Err(ZecError::cancelled()),v=timeout(self.deadline,async{let info=client.get_lightd_info(Empty{}).await?.into_inner();let tip=client.get_latest_block(ChainSpec{}).await?.into_inner();Ok::<_,tonic::Status>((info,tip))})=>v.map_err(|_|rpc_error(cancel))?.map_err(|_|rpc_error(cancel))}});
        self.client = Some(client);
        self.runtime = Some(rt);
        let (info, tip) = result?;
        Ok(SourceMetadata { info, tip })
    }
    fn tree_state(
        &mut self,
        height: u32,
        cancel: &LiveCancellation,
    ) -> Result<TreeState, ZecError> {
        let rt = self.runtime.take().ok_or_else(ZecError::unavailable)?;
        let mut client = self.client.take().ok_or_else(ZecError::unavailable)?;
        let result=rt.block_on(async{tokio::select!{biased;_=cancelled(cancel)=>Err(ZecError::cancelled()),v=timeout(self.deadline,client.get_tree_state(BlockId{height:u64::from(height),hash:Vec::new()}))=>v.map_err(|_|rpc_error(cancel))?.map(|response|response.into_inner()).map_err(|_|rpc_error(cancel))}});
        self.client = Some(client);
        self.runtime = Some(rt);
        result
    }
    fn blocks(
        &mut self,
        from: u32,
        through: u32,
        cancel: &LiveCancellation,
    ) -> Result<Vec<CompactBlock>, ZecError> {
        if through < from || through - from >= MAX_LIVE_BATCH_BLOCKS as u32 {
            return Err(ZecError::limit());
        }
        let rt = self.runtime.take().ok_or_else(ZecError::unavailable)?;
        let mut client = self.client.take().ok_or_else(ZecError::unavailable)?;
        let result=rt.block_on(async{tokio::select!{biased;_=cancelled(cancel)=>Err(ZecError::cancelled()),v=timeout(self.deadline,async{let mut stream=client.get_block_range(BlockRange{start:Some(BlockId{height:u64::from(from),hash:Vec::new()}),end:Some(BlockId{height:u64::from(through),hash:Vec::new()}),pool_types:Vec::new()}).await.map_err(|status|status_error(cancel,status))?.into_inner();let mut out=Vec::new();let mut bytes=0usize;while let Some(block)=stream.message().await.map_err(|status|status_error(cancel,status))?{let size=block.encoded_len();if size>MAX_COMPACT_BLOCK_BYTES{return Err(ZecError::limit())}bytes=bytes.checked_add(size).ok_or_else(ZecError::limit)?;if bytes>MAX_LIVE_BATCH_BYTES||out.len()>=MAX_LIVE_BATCH_BLOCKS{return Err(ZecError::limit())}out.push(block)}Ok(out)})=>v.map_err(|_|rpc_error(cancel))?}});
        self.client = Some(client);
        self.runtime = Some(rt);
        result
    }
}

impl Drop for TonicLiveSource {
    fn drop(&mut self) {
        drop(self.client.take());
        if let Some(rt) = self.runtime.take() {
            rt.shutdown_timeout(Duration::from_millis(250));
        }
    }
}

#[derive(Debug)]
pub(crate) struct LiveTransportProbe {
    pub(crate) tip_height: u32,
    pub(crate) block_count: usize,
}

pub(crate) fn probe_live_transport(
    endpoint: &str,
    ca_pem: Option<&[u8]>,
    cancellation: LiveCancellation,
    deadline: Duration,
) -> Result<LiveTransportProbe, ZecError> {
    let endpoint = LiveEndpoint::parse(endpoint)?;
    let mut source =
        TonicLiveSource::connect_configured(&endpoint, &cancellation, deadline, ca_pem)?;
    let metadata = source.metadata(Network::Testnet, &cancellation)?;
    let tip_height =
        u32::try_from(metadata.tip.height).map_err(|_| ZecError::protocol_incompatible())?;
    let expected_branch = format!(
        "{:08x}",
        u32::from(BranchId::for_height(
            &zcash_protocol::consensus::Network::TestNetwork,
            BlockHeight::from_u32(tip_height),
        ))
    );
    if metadata.info.chain_name != "test"
        || metadata.info.block_height != metadata.tip.height
        || metadata.info.estimated_height < metadata.tip.height
        || metadata.info.sapling_activation_height != 280_000
        || metadata.info.consensus_branch_id != expected_branch
        || metadata.tip.hash.len() != 32
        || tip_height == 0
    {
        return Err(ZecError::protocol_incompatible());
    }
    let prior_height = tip_height
        .checked_sub(1)
        .ok_or_else(ZecError::protocol_incompatible)?;
    let tree = source.tree_state(prior_height, &cancellation)?;
    if tree.network != "test" || tree.height != u64::from(prior_height) {
        return Err(ZecError::protocol_incompatible());
    }
    let prior = tree
        .to_chain_state()
        .map_err(|_| ZecError::protocol_incompatible())?;
    let blocks = source.blocks(tip_height, tip_height, &cancellation)?;
    super::live::validate_batch(&blocks, tip_height, tip_height)?;
    let block = blocks.first().ok_or_else(ZecError::protocol_incompatible)?;
    if block.prev_hash.as_slice() != prior.block_hash().0.as_slice()
        || block.hash.as_slice() != metadata.tip.hash.as_slice()
    {
        return Err(ZecError::protocol_incompatible());
    }
    Ok(LiveTransportProbe {
        tip_height,
        block_count: blocks.len(),
    })
}
