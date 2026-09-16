(function paymentInboxModule(global) {
  'use strict';

  var POLL_MS = 5000;

  function make(tag, className, text) {
    var node = document.createElement(tag);
    if (className) node.className = className;
    if (text !== undefined && text !== null) node.textContent = String(text);
    return node;
  }

  function snapshotKey(value) {
    try {
      return JSON.stringify(value);
    } catch (error) {
      return '';
    }
  }

  function isVisible() {
    var doc = global.document;
    if (!doc) return true;
    if (typeof doc.visibilityState === 'string') return doc.visibilityState === 'visible';
    return !doc.hidden;
  }

  function identitiesMatch(paymentPeer, socialPeer) {
    return typeof paymentPeer === 'string' && typeof socialPeer === 'string'
      && paymentPeer.length > 0 && socialPeer.length > 0
      && paymentPeer === socialPeer;
  }

  function bindSnapshot(dto, socialPeerId) {
    if (!dto || dto.v !== 1 || typeof dto.state !== 'string' || dto.state !== 'ready') {
      return { bound: false, notice: 'Payment requests unavailable', requests: [] };
    }
    if (!identitiesMatch(dto.peer_id, socialPeerId)) {
      return { bound: false, notice: 'Payment requests unavailable for this identity', requests: [] };
    }
    return {
      bound: true,
      notice: '',
      requests: Array.isArray(dto.requests) ? dto.requests.slice() : [],
      peer_id: dto.peer_id,
    };
  }

  function previewForRequest(row) {
    return 'Payment request · ' + row.amount_display + ' ' + row.asset + ' · ' + row.status;
  }

  function mergeConversationList(textConversations, requests) {
    var map = Object.create(null);
    function ensure(peerId) {
      if (!Object.prototype.hasOwnProperty.call(map, peerId)) {
        map[peerId] = {
          peerId: peerId,
          timestamp: '',
          lastMessage: '',
          outgoing: false,
          unread: 0,
          textMs: 0,
          paymentMs: 0,
          paymentPreview: null,
        };
      }
      return map[peerId];
    }
    (textConversations || []).forEach(function addText(conv) {
      if (!conv || !conv.peerId) return;
      var row = ensure(conv.peerId);
      row.timestamp = conv.timestamp || row.timestamp;
      row.lastMessage = conv.lastMessage || '';
      row.outgoing = !!conv.outgoing;
      row.unread = conv.unread || 0;
      row.textMs = Date.parse(conv.timestamp) || 0;
    });
    (requests || []).forEach(function addRequest(req) {
      if (!req || !req.payee_peer_id) return;
      var row = ensure(req.payee_peer_id);
      var created = Date.parse(req.created_at) || 0;
      if (!row.paymentPreview || created >= row.paymentMs) {
        row.paymentMs = created > row.paymentMs ? created : row.paymentMs;
        if (!row.paymentMs) row.paymentMs = created;
        row.paymentPreview = req;
      } else if (row.paymentPreview.request_id === req.request_id) {
        row.paymentPreview = req;
      }
    });
    var list = [];
    Object.keys(map).forEach(function pushRow(peerId) {
      var row = map[peerId];
      row.activityMs = row.textMs > row.paymentMs ? row.textMs : row.paymentMs;
      if (row.paymentPreview && row.paymentMs >= row.textMs) {
        row.lastMessage = previewForRequest(row.paymentPreview);
        row.outgoing = false;
        if (!row.timestamp || row.paymentMs >= row.textMs) row.timestamp = row.paymentPreview.created_at;
      }
      list.push(row);
    });
    list.sort(function sortRows(a, b) {
      if (b.activityMs !== a.activityMs) return b.activityMs - a.activityMs;
      if (a.peerId < b.peerId) return -1;
      if (a.peerId > b.peerId) return 1;
      return 0;
    });
    return list;
  }

  function mergeTranscript(messages, requests) {
    var entries = [];
    (messages || []).forEach(function addMessage(message, index) {
      entries.push({
        kind: 'text',
        time: Date.parse(message && message.timestamp) || 0,
        index: index,
        message: message,
      });
    });
    (requests || []).forEach(function addRequest(request) {
      entries.push({
        kind: 'request',
        time: Date.parse(request && request.created_at) || 0,
        request_id: request && request.request_id ? request.request_id : '',
        digest: request && request.digest ? request.digest : '',
        request: request,
      });
    });
    entries.sort(function sortEntries(a, b) {
      if (a.time !== b.time) return a.time - b.time;
      if (a.kind !== b.kind) return a.kind === 'text' ? -1 : 1;
      if (a.kind === 'text') return a.index - b.index;
      if (a.request_id !== b.request_id) return a.request_id < b.request_id ? -1 : (a.request_id > b.request_id ? 1 : 0);
      if (a.digest < b.digest) return -1;
      if (a.digest > b.digest) return 1;
      return 0;
    });
    return entries;
  }

  function collectOpenRequestIds(root) {
    var open = {};
    if (!root) return open;
    var nodes = root.querySelectorAll ? root.querySelectorAll('.paymentRequestDetails') : [];
    Array.prototype.forEach.call(nodes, function eachDetails(details) {
      if (!details.open) return;
      var line = details.firstChild;
      while (line) {
        var text = line.textContent || '';
        if (text.indexOf('Request ID ') === 0) {
          open[text.slice('Request ID '.length)] = true;
          break;
        }
        line = line.nextSibling;
      }
    });
    return open;
  }

  function renderCard(row, extras) {
    extras = extras || {};
    var item = make('article', 'paymentRequest card');
    item.setAttribute('data-request-id', row.request_id || '');
    item.setAttribute('data-peer-id', row.payee_peer_id || '');
    item.setAttribute('data-digest', row.digest || '');
    item.appendChild(make('p', 'paymentRequestKicker', 'Payment request'));
    item.appendChild(make('h3', 'paymentRequestAmount', row.amount_display + ' ' + row.asset));
    item.appendChild(make('p', 'paymentRequestMeta', row.network + ' · ' + row.status));
    item.appendChild(make('p', 'paymentRequestMemo', row.memo || 'No memo'));
    var details = make('details', 'paymentRequestDetails');
    details.appendChild(make('summary', '', 'Request details'));
    details.appendChild(make('p', '', 'Request ID ' + row.request_id));
    details.appendChild(make('p', '', 'Payee ' + row.payee_peer_id));
    details.appendChild(make('p', '', 'Created ' + row.created_at + ' (UTC)'));
    details.appendChild(make('p', '', 'Expires ' + row.expires_at + ' (UTC)'));
    details.appendChild(make('p', '', 'Digest ' + row.digest));
    if (extras.open) details.open = true;
    item.appendChild(details);
    return item;
  }

  function createReader(options) {
    options = options || {};
    var bridge = options.bridge || {};
    var onSnapshot = options.onSnapshot || function noop() {};
    var disposed = false;
    var started = false;
    var generation = 0;
    var inflight = false;
    var pendingFresh = false;
    var timer = null;
    var lastKey = null;
    var doc = global.document;

    function clearTimer() {
      if (timer !== null) {
        global.clearTimeout(timer);
        timer = null;
      }
    }

    function scheduleNext() {
      clearTimer();
      if (disposed || !started || !isVisible()) return;
      timer = global.setTimeout(function onPoll() {
        timer = null;
        read();
      }, POLL_MS);
    }

    function afterSettle(gen) {
      if (disposed || gen !== generation) {
        if (!disposed && started && pendingFresh && isVisible() && !inflight) {
          pendingFresh = false;
          read();
        }
        return;
      }
      if (pendingFresh && started && isVisible()) {
        pendingFresh = false;
        read();
        return;
      }
      scheduleNext();
    }

    function emit(dto) {
      var key = snapshotKey(dto);
      if (key && key === lastKey) return;
      lastKey = key;
      onSnapshot(dto, { changed: true });
    }

    function fail() {
      emit({ v: 1, state: 'unavailable', peer_id: '', requests: [] });
    }

    function read() {
      if (disposed || !started) return;
      if (!isVisible()) return;
      if (inflight) {
        pendingFresh = true;
        return;
      }
      if (!bridge || typeof bridge.get !== 'function') {
        fail();
        scheduleNext();
        return;
      }
      var gen = generation;
      inflight = true;
      pendingFresh = false;
      var startedRead;
      try {
        startedRead = bridge.get();
      } catch (error) {
        inflight = false;
        if (disposed || gen !== generation) return;
        fail();
        afterSettle(gen);
        return;
      }
      Promise.resolve(startedRead).then(function onInbox(dto) {
        inflight = false;
        if (disposed || gen !== generation) {
          afterSettle(gen);
          return;
        }
        emit(dto);
        afterSettle(gen);
      }, function onFail() {
        inflight = false;
        if (disposed || gen !== generation) {
          afterSettle(gen);
          return;
        }
        fail();
        afterSettle(gen);
      });
    }

    function onVisibility() {
      if (disposed || !started) return;
      if (isVisible()) {
        if (inflight) pendingFresh = true;
        else read();
        return;
      }
      clearTimer();
    }

    function start() {
      if (disposed) return;
      started = true;
      if (inflight) pendingFresh = true;
      else read();
    }

    function invalidate() {
      generation += 1;
      pendingFresh = false;
      lastKey = null;
      clearTimer();
    }

    function dispose() {
      disposed = true;
      started = false;
      pendingFresh = false;
      generation += 1;
      inflight = false;
      lastKey = null;
      clearTimer();
      if (doc && typeof doc.removeEventListener === 'function') {
        doc.removeEventListener('visibilitychange', onVisibility);
      }
    }

    if (doc && typeof doc.addEventListener === 'function') {
      doc.addEventListener('visibilitychange', onVisibility);
    }

    return {
      start: start,
      invalidate: invalidate,
      dispose: dispose,
    };
  }

  global.BitBookPaymentInbox = {
    bindSnapshot: bindSnapshot,
    collectOpenRequestIds: collectOpenRequestIds,
    createReader: createReader,
    mergeConversationList: mergeConversationList,
    mergeTranscript: mergeTranscript,
    previewForRequest: previewForRequest,
    renderCard: renderCard,
  };
}(window));
