(function startBitBook() {
  'use strict';

  var core = window.BitBookCore;
  var defaultAPI = 'http://127.0.0.1:4002';
  var state = {
    apiURL: localStorage.getItem('bitbook.apiURL') || defaultAPI,
    connected: false,
    config: {},
    profile: {},
    following: [],
    profiles: Object.create(null),
    ownPosts: [],
    remotePosts: {},
    peers: [],
    conversations: [],
    activeChat: '',
    messages: [],
    socket: null,
    socketTimer: null,
    typingTimer: null,
    lastTypingAt: 0,
    sessionGen: 0,
    boundPeerId: '',
    paymentNotice: '',
    paymentRequests: [],
    paymentKey: '',
    conversationKey: '',
    transcriptKey: '',
    historyError: false,
    chatInflight: false,
    chatPending: false,
    chatTimer: null,
    chatAbort: null,
    openAbort: null,
    paymentReader: null,
    disposed: false,
    chatGen: 0,
    chatPassId: 0,
    chatPass: null,
    readReceipt: null,
    readAcknowledged: Object.create(null),
    readRetryPending: null,
  };

  function byID(id) {
    return document.getElementById(id);
  }

  function clear(node) {
    while (node.firstChild) node.removeChild(node.firstChild);
  }

  function make(tag, className, content) {
    var node = document.createElement(tag);
    if (className) node.className = className;
    if (content !== undefined && content !== null) node.textContent = String(content);
    return node;
  }

  function action(label, className, handler) {
    var button = make('button', className, label);
    button.type = 'button';
    button.addEventListener('click', handler);
    return button;
  }

  function emptyState(title, detail) {
    var wrapper = make('div', 'emptyState');
    wrapper.appendChild(make('strong', '', title));
    wrapper.appendChild(make('p', '', detail));
    return wrapper;
  }

  function initials(profile, fallback) {
    var name = core.displayName(profile, fallback).replace(/^@/, '').trim();
    var parts = name.split(/\s+/).filter(Boolean);
    if (!parts.length) return '?';
    if (parts.length === 1) return parts[0].slice(0, 2).toUpperCase();
    return (parts[0][0] + parts[parts.length - 1][0]).toUpperCase();
  }

  function formatTime(value) {
    var date = new Date(value);
    if (isNaN(date.getTime())) return '';
    var elapsed = Date.now() - date.getTime();
    if (elapsed >= 0 && elapsed < 60000) return 'now';
    if (elapsed >= 0 && elapsed < 3600000) return Math.floor(elapsed / 60000) + 'm';
    if (elapsed >= 0 && elapsed < 86400000) return Math.floor(elapsed / 3600000) + 'h';
    return date.toLocaleString([], {
      month: 'short',
      day: 'numeric',
      hour: 'numeric',
      minute: '2-digit',
    });
  }

  function toast(message, kind) {
    var node = make('div', 'toast ' + (kind || ''), message);
    byID('toasts').appendChild(node);
    window.setTimeout(function removeToast() {
      if (node.parentNode) node.parentNode.removeChild(node);
    }, 5000);
  }

  function abortError() {
    var err = new Error('Request timed out.');
    err.name = 'AbortError';
    return err;
  }

  async function api(path, options) {
    var settings = options || {};
    var headers = Object.assign({}, settings.headers || {});
    if (settings.body !== undefined) headers['Content-Type'] = 'application/json';
    var controller = new AbortController();
    var timeout = window.setTimeout(function abortDeadline() { controller.abort(); }, 5000);
    function abortFromOuter() { controller.abort(); }
    if (settings.signal) {
      if (settings.signal.aborted) {
        window.clearTimeout(timeout);
        throw abortError();
      }
      settings.signal.addEventListener('abort', abortFromOuter);
    }
    var response;
    var raw = '';
    try {
      response = await fetch(state.apiURL + path, Object.assign({}, settings, {
        headers: headers,
        signal: controller.signal,
      }));
      raw = await response.text();
    } catch (error) {
      window.clearTimeout(timeout);
      if (settings.signal) settings.signal.removeEventListener('abort', abortFromOuter);
      if (controller.signal.aborted || (settings.signal && settings.signal.aborted)) throw abortError();
      throw new Error('Cannot reach the BitBook daemon at ' + state.apiURL + '.');
    }
    window.clearTimeout(timeout);
    if (settings.signal) settings.signal.removeEventListener('abort', abortFromOuter);
    var body = null;
    if (raw) {
      try {
        body = JSON.parse(raw);
      } catch (error) {
        body = raw;
      }
    }
    if (!response.ok) {
      var reason = body && body.reason ? body.reason : ('Daemon returned HTTP ' + response.status + '.');
      throw new Error(reason);
    }
    return body;
  }

  function setConnection(connected, label) {
    state.connected = connected;
    byID('statusDot').classList.toggle('online', connected);
    byID('connectionText').textContent = label || (connected ? 'Connected' : 'Offline');
    byID('publishPostButton').disabled = !connected;
    byID('sendMessageButton').disabled = !connected || !state.activeChat;
  }

  function profileFor(peerID) {
    if (peerID === state.config.peerID) return state.profile;
    return state.profiles[peerID] || { peerID: peerID };
  }

  async function fetchProfile(peerID) {
    if (!peerID || peerID === state.config.peerID) return state.profile;
    var session = state.sessionGen;
    var url = state.apiURL;
    try {
      var profile = await api('/ob/profile/' + encodeURIComponent(peerID));
      if (state.disposed || state.sessionGen !== session || state.apiURL !== url) return;
      state.profiles[peerID] = profile || { peerID: peerID };
    } catch (error) {
      if (state.disposed || state.sessionGen !== session || state.apiURL !== url) return;
      if (!state.profiles[peerID]) state.profiles[peerID] = { peerID: peerID };
    }
    return state.profiles[peerID];
  }

  async function loadProfile() {
    var session = state.sessionGen;
    var url = state.apiURL;
    var profile = await api('/ob/profile') || {};
    if (state.disposed || state.sessionGen !== session || state.apiURL !== url) return;
    state.profile = profile;
  }

  async function loadFollowing() {
    state.following = await api('/ob/following') || [];
  }

  async function loadOwnPosts() {
    state.ownPosts = await api('/ob/posts') || [];
  }

  async function loadPeers() {
    state.peers = await api('/ob/peers') || [];
  }

  async function loadConversations() {
    var session = state.sessionGen;
    var url = state.apiURL;
    var rows = await api('/ob/chatconversations') || [];
    if (state.disposed || state.sessionGen !== session || state.apiURL !== url) return;
    state.conversations = rows;
  }

  async function hydratePeers() {
    var ids = state.following.slice();
    state.conversations.forEach(function addConversation(conversation) {
      if (ids.indexOf(conversation.peerId) === -1) ids.push(conversation.peerId);
    });
    await Promise.all(ids.map(fetchProfile));
  }

  async function loadRemotePosts() {
    var unavailable = 0;
    await Promise.all(state.following.map(async function fetchPosts(peerID) {
      try {
        state.remotePosts[peerID] = await api('/ob/posts/' + encodeURIComponent(peerID)) || [];
      } catch (error) {
        state.remotePosts[peerID] = [];
        unavailable += 1;
      }
    }));
    return unavailable;
  }

  function renderProfile() {
    var profile = state.profile || {};
    var display = core.displayName(profile, state.config.peerID);
    byID('avatar').textContent = initials(profile, state.config.peerID);
    byID('profileName').textContent = display;
    byID('profileHandle').textContent = profile.handle ? '@' + profile.handle.replace(/^@/, '') : '';
    byID('profileAbout').textContent = profile.about || 'Create a profile to introduce yourself.';
    byID('peerID').textContent = core.shortID(state.config.peerID || 'not connected', 9);
    byID('peerID').title = state.config.peerID || '';
  }

  function personRow(peerID) {
    var profile = profileFor(peerID);
    var row = make('div', 'person');
    row.appendChild(make('div', 'miniAvatar', initials(profile, peerID)));
    var identity = make('button', 'personIdentity');
    identity.type = 'button';
    identity.appendChild(make('strong', '', core.displayName(profile, peerID)));
    identity.appendChild(make('code', '', core.shortID(peerID, 6)));
    identity.title = 'Message ' + peerID;
    identity.addEventListener('click', function openPersonChat() { openChat(peerID); });
    row.appendChild(identity);
    row.appendChild(action('×', 'personRemove', function removeFollow() { unfollow(peerID); }));
    return row;
  }

  function renderFollowing() {
    var list = byID('followingList');
    clear(list);
    byID('followingCount').textContent = state.following.length;
    if (!state.following.length) {
      list.appendChild(make('p', 'muted small', 'Your feed is local until you follow someone.'));
      return;
    }
    state.following.forEach(function addPerson(peerID) { list.appendChild(personRow(peerID)); });
  }

  function decoratedFeed() {
    var result = state.ownPosts.map(function own(post) {
      return Object.assign({}, post, {
        _peerID: state.config.peerID,
        _profile: state.profile,
        _own: true,
      });
    });
    state.following.forEach(function addRemote(peerID) {
      (state.remotePosts[peerID] || []).forEach(function addPost(post) {
        result.push(Object.assign({}, post, {
          _peerID: peerID,
          _profile: profileFor(peerID),
          _own: false,
        }));
      });
    });
    return core.sortFeed(result);
  }

  function postCard(post) {
    var card = make('article', 'post card');
    var head = make('div', 'postHead');
    head.appendChild(make('div', 'miniAvatar postAvatar', initials(post._profile, post._peerID)));
    var author = make('div', 'postAuthor');
    author.appendChild(make('strong', '', core.displayName(post._profile, post._peerID)));
    var meta = make('span', '', core.shortID(post._peerID, 5));
    var time = formatTime(post.timestamp);
    if (time) meta.textContent += ' · ' + time;
    author.appendChild(meta);
    head.appendChild(author);
    if (post._own) {
      head.appendChild(action('Delete', 'textButton danger', function deleteOwnPost() {
        deletePost(post.slug || post.hash);
      }));
    } else {
      head.appendChild(action('Message', 'textButton', function messageAuthor() {
        openChat(post._peerID);
      }));
    }
    card.appendChild(head);
    card.appendChild(make('p', 'postBody', core.postText(post)));
    if (post.hash) {
      var hash = make('code', 'postHash', core.shortID(post.hash, 9));
      hash.title = post.hash;
      card.appendChild(hash);
    }
    return card;
  }

  function renderTimeline(targetID, posts, title, detail) {
    var list = byID(targetID);
    clear(list);
    if (!posts.length) {
      list.appendChild(emptyState(title, detail));
      return;
    }
    posts.forEach(function addPost(post) { list.appendChild(postCard(post)); });
  }

  function renderPosts() {
    renderTimeline(
      'feedList',
      decoratedFeed(),
      'The feed is quiet',
      'Publish the first post or follow another BitBook peer.'
    );
    var own = state.ownPosts.map(function decorate(post) {
      return Object.assign({}, post, {
        _peerID: state.config.peerID,
        _profile: state.profile,
        _own: true,
      });
    });
    renderTimeline('ownPostsList', core.sortFeed(own), 'No posts yet', 'Your signed posts will appear here.');
  }

  function renderNetwork() {
    var list = byID('connectedPeers');
    clear(list);
    byID('connectedCount').textContent = state.peers.length;
    byID('peerCount').textContent = state.peers.length + (state.peers.length === 1 ? ' connected peer' : ' connected peers');
    if (!state.peers.length) {
      list.appendChild(emptyState('No live peers', 'The daemon will discover peers through configured bootstrap nodes.'));
      return;
    }
    state.peers.forEach(function addPeer(peerID) {
      var card = make('div', 'peerCard');
      card.appendChild(make('div', 'miniAvatar', initials(profileFor(peerID), peerID)));
      var copy = make('div', 'peerDetails');
      copy.appendChild(make('strong', '', core.displayName(profileFor(peerID), peerID)));
      var id = make('code', '', core.shortID(peerID, 9));
      id.title = peerID;
      copy.appendChild(id);
      card.appendChild(copy);
      card.appendChild(action('Chat', 'secondary smallButton', function chatPeer() { openChat(peerID); }));
      list.appendChild(card);
    });
  }

  function renderPaymentNotice() {
    var node = byID('paymentNotice');
    if (!node) return;
    node.textContent = state.paymentNotice || '';
    node.classList.toggle('hidden', !state.paymentNotice);
  }

  function renderConversations() {
    var list = byID('conversationList');
    var inbox = window.BitBookPaymentInbox;
    var merged = inbox
      ? inbox.mergeConversationList(state.conversations, state.paymentRequests)
      : (state.conversations || []).slice();
    var key = JSON.stringify(merged.map(function keyRow(row) {
      return [row.peerId, row.lastMessage, row.unread || 0, row.timestamp || '', row.activityMs || 0];
    }));
    if (key === state.conversationKey) return;
    state.conversationKey = key;
    clear(list);
    if (!merged.length) {
      list.appendChild(emptyState('No conversations', 'Start a signed, peer-to-peer chat.'));
      return;
    }
    merged.forEach(function addConversation(conversation) {
      var profile = profileFor(conversation.peerId);
      var row = make('button', 'conversationRow' + (state.activeChat === conversation.peerId ? ' selected' : ''));
      row.type = 'button';
      row.setAttribute('data-peer-id', conversation.peerId);
      row.appendChild(make('div', 'miniAvatar', initials(profile, conversation.peerId)));
      var copy = make('div', 'conversationCopy');
      var title = make('div', 'conversationName');
      title.appendChild(make('strong', '', core.displayName(profile, conversation.peerId)));
      title.appendChild(make('time', '', formatTime(conversation.timestamp)));
      copy.appendChild(title);
      copy.appendChild(make('p', '', (conversation.outgoing ? 'You: ' : '') + (conversation.lastMessage || '')));
      row.appendChild(copy);
      if (conversation.unread) row.appendChild(make('span', 'unread', conversation.unread));
      row.addEventListener('click', function selectConversation() { openChat(conversation.peerId); });
      list.appendChild(row);
    });
  }

  function nearBottom(node) {
    return (node.scrollHeight - node.scrollTop - node.clientHeight) <= 24;
  }

  function messageEntryKey(message, occurrences) {
    var scope = JSON.stringify([state.sessionGen, state.boundPeerId || '', state.activeChat || '']);
    if (message && typeof message.messageId === 'string' && message.messageId) {
      return 't:' + scope + ':id:' + message.messageId;
    }
    var fallback = JSON.stringify([
      message && message.timestamp || '',
      message && message.outgoing ? 1 : 0,
      message && message.message || '',
    ]);
    var occurrence = occurrences[fallback] || 0;
    occurrences[fallback] = occurrence + 1;
    return 't:' + scope + ':fallback:' + fallback + ':' + occurrence;
  }

  function entryKey(entry, occurrences) {
    if (entry.kind === 'request') {
      var req = entry.request || {};
      return 'r:' + (state.boundPeerId || '') + ':' + (req.payee_peer_id || '') + ':' + (req.request_id || '');
    }
    var message = entry.message || {};
    return messageEntryKey(message, occurrences);
  }

  function captureViewport(list) {
    var focused = document.activeElement;
    var focusKey = '';
    var node = focused;
    while (node && node !== list) {
      if (node.getAttribute && node.getAttribute('data-entry-key')) {
        focusKey = node.getAttribute('data-entry-key');
        break;
      }
      node = node.parentNode;
    }
    var listTop = list.getBoundingClientRect ? list.getBoundingClientRect().top : 0;
    var anchorKey = '';
    var anchorOffset = 0;
    Array.prototype.forEach.call(list.childNodes, function findAnchor(child) {
      if (anchorKey || !child.getAttribute) return;
      var rect = child.getBoundingClientRect ? child.getBoundingClientRect() : { top: 0, bottom: 1 };
      if (rect.bottom > listTop) {
        anchorKey = child.getAttribute('data-entry-key') || '';
        anchorOffset = rect.top - listTop;
      }
    });
    return {
      follow: nearBottom(list),
      initial: !state.transcriptKey,
      focusKey: focusKey,
      anchorKey: anchorKey,
      anchorOffset: anchorOffset,
    };
  }

  function restoreViewport(list, shot) {
    function restorePosition() {
      if (shot.initial || shot.follow) {
        list.scrollTop = list.scrollHeight;
      } else if (shot.anchorKey) {
        Array.prototype.forEach.call(list.childNodes, function restoreAnchor(child) {
          if (!child.getAttribute || child.getAttribute('data-entry-key') !== shot.anchorKey) return;
          if (!child.getBoundingClientRect || !list.getBoundingClientRect) return;
          list.scrollTop += child.getBoundingClientRect().top - list.getBoundingClientRect().top - shot.anchorOffset;
        });
      }
    }
    restorePosition();
    if (shot.focusKey) {
      Array.prototype.forEach.call(list.querySelectorAll('[data-entry-key]'), function restoreFocus(child) {
        if (child.getAttribute('data-entry-key') !== shot.focusKey) return;
        var summary = child.querySelector && child.querySelector('summary');
        if (summary && summary.focus) summary.focus({ preventScroll: true });
      });
      restorePosition();
    }
  }

  function textStatus(message) {
    var status = formatTime(message.timestamp);
    if (message.outgoing && message.read) status += ' · read';
    return status;
  }

  function textBubble(message) {
    var bubble = make('div', 'message ' + (message.outgoing ? 'outgoing' : 'incoming'));
    bubble.appendChild(make('p', '', message.message));
    bubble.appendChild(make('small', '', textStatus(message)));
    return bubble;
  }

  function renderMessages() {
    var list = byID('messageList');
    var inbox = window.BitBookPaymentInbox;
    var peerRequests = (state.paymentRequests || []).filter(function matchPeer(row) {
      return row.payee_peer_id === state.activeChat;
    });
    var entries = inbox ? inbox.mergeTranscript(state.messages, peerRequests) : (state.messages || []).map(function asText(message, index) {
      return { kind: 'text', time: Date.parse(message.timestamp) || 0, index: index, message: message };
    });
    var occurrences = Object.create(null);
    var keyedEntries = entries.map(function assignEntryKey(entry) {
      return { entry: entry, key: entryKey(entry, occurrences) };
    });
    var key = JSON.stringify([state.historyError, keyedEntries.map(function keyEntry(item) {
      var entry = item.entry;
      if (entry.kind === 'request') {
        return ['r', entry.request_id, entry.digest, entry.request && entry.request.status];
      }
      var message = entry.message || {};
      return ['t', item.key, message.timestamp, message.message, message.read, message.outgoing];
    })]);
    if (key === state.transcriptKey) return;
    var shot = captureViewport(list);
    var existing = Object.create(null);
    Array.prototype.forEach.call(list.childNodes, function indexExisting(node) {
      var stored = node.getAttribute && node.getAttribute('data-entry-key');
      if (stored) existing[stored] = node;
    });
    state.transcriptKey = key;
    var next = [];
    if (state.historyError) {
      next.push(make('p', 'inlineNotice', 'Messages could not be loaded'));
    }
    if (!entries.length && !state.historyError) {
      next.push(emptyState('Say hello', 'Messages are signed and sent directly to this peer.'));
    } else if (!entries.length) {
      next.push(emptyState('Messages could not be loaded', 'BitBook will try again automatically.'));
    }
    keyedEntries.forEach(function addEntry(item) {
      var entry = item.entry;
      var storedKey = item.key;
      var reused = existing[storedKey];
      if (reused) {
        if (entry.kind === 'request' && entry.request) {
          var meta = reused.querySelector && reused.querySelector('.paymentRequestMeta');
          if (meta) meta.textContent = entry.request.network + ' · ' + entry.request.status;
        } else if (entry.kind === 'text' && entry.message) {
          var status = reused.querySelector && reused.querySelector('small');
          if (status) status.textContent = textStatus(entry.message);
        }
        next.push(reused);
        return;
      }
      var node;
      if (entry.kind === 'request' && inbox) {
        node = inbox.renderCard(entry.request, { open: false });
      } else {
        node = textBubble(entry.message);
      }
      node.setAttribute('data-entry-key', storedKey);
      next.push(node);
    });
    while (list.firstChild) list.removeChild(list.firstChild);
    next.forEach(function append(node) { list.appendChild(node); });
    restoreViewport(list, shot);
  }

  function renderChat() {
    var conversation = byID('conversation');
    var conversationList = byID('conversationList');
    var active = Boolean(state.activeChat);
    conversation.classList.toggle('hidden', !active);
    conversationList.classList.toggle('hidden', active);
    byID('sendMessageButton').disabled = !state.connected || !active;
    renderPaymentNotice();
    if (!active) {
      state.transcriptKey = '';
      clear(byID('messageList'));
      return;
    }
    var profile = profileFor(state.activeChat);
    byID('chatPeerName').textContent = core.displayName(profile, state.activeChat);
    byID('chatPeerID').textContent = core.shortID(state.activeChat, 8);
    byID('chatPeerID').title = state.activeChat;
    renderMessages();
  }

  function renderAll() {
    renderProfile();
    renderFollowing();
    renderPosts();
    renderNetwork();
    renderConversations();
    renderChat();
  }

  async function refreshFeed(showResult) {
    if (!state.connected) return;
    byID('refreshFeedButton').disabled = true;
    try {
      await Promise.all([loadProfile(), loadFollowing(), loadOwnPosts()]);
      await hydratePeers();
      var unavailable = await loadRemotePosts();
      renderAll();
      if (showResult) {
        toast(unavailable ? unavailable + ' followed peer' + (unavailable === 1 ? ' is' : 's are') + ' currently unavailable.' : 'Feed refreshed.');
      }
    } catch (error) {
      toast(error.message, 'error');
    } finally {
      byID('refreshFeedButton').disabled = false;
    }
  }

  function abortOwnedPass() {
    if (!state.chatPass) return;
    if (state.chatPass.convAbort) {
      try { state.chatPass.convAbort.abort(); } catch (error) { /* owned */ }
    }
    if (state.chatPass.histAbort) {
      try { state.chatPass.histAbort.abort(); } catch (error) { /* owned */ }
    }
  }

  function clearTypingOwnership() {
    if (state.typingTimer) {
      window.clearTimeout(state.typingTimer);
      state.typingTimer = null;
    }
    if (!state.disposed) {
      var status = byID('chatStatus');
      if (status) status.textContent = 'Direct and signed';
    }
  }

  function abortReadReceipt() {
    var owner = state.readReceipt;
    state.readReceipt = null;
    if (owner && owner.controller) {
      try { owner.controller.abort(); } catch (error) { /* owned */ }
    }
  }

  function invalidateConversationOwnership() {
    clearTypingOwnership();
    abortReadReceipt();
    state.readAcknowledged = Object.create(null);
    state.readRetryPending = null;
  }

  function abortChatWork() {
    if (state.chatTimer) {
      window.clearTimeout(state.chatTimer);
      state.chatTimer = null;
    }
    abortOwnedPass();
    if (state.chatAbort) {
      try { state.chatAbort.abort(); } catch (error) { /* owned */ }
      state.chatAbort = null;
    }
    if (state.openAbort) {
      try { state.openAbort.abort(); } catch (error) { /* owned */ }
      state.openAbort = null;
    }
    invalidateConversationOwnership();
    state.chatPending = false;
  }

  function beginSession() {
    state.sessionGen += 1;
    state.chatGen += 1;
    state.boundPeerId = '';
    state.paymentNotice = '';
    state.paymentRequests = [];
    state.paymentKey = '';
    state.conversationKey = '';
    state.transcriptKey = '';
    state.historyError = false;
    state.config = {};
    state.profile = {};
    state.following = [];
    state.profiles = Object.create(null);
    state.ownPosts = [];
    state.remotePosts = {};
    state.peers = [];
    state.conversations = [];
    state.activeChat = '';
    state.messages = [];
    abortChatWork();
    state.chatInflight = false;
    state.chatPass = null;
    if (state.paymentReader && state.paymentReader.invalidate) state.paymentReader.invalidate();
    renderConversations();
    renderChat();
  }

  function scheduleChatReconcile() {
    if (state.chatTimer) window.clearTimeout(state.chatTimer);
    if (state.disposed || document.hidden) return;
    state.chatTimer = window.setTimeout(function onChatPoll() {
      state.chatTimer = null;
      state.readRetryPending = null;
      requestChatUpdate();
    }, 5000);
  }

  function passCurrent(passId, session, url) {
    return !state.disposed && state.chatPass && state.chatPass.id === passId &&
      state.sessionGen === session && state.apiURL === url;
  }

  function requestChatUpdate() {
    if (state.disposed || document.hidden) return;
    if (state.chatInflight) {
      state.chatPending = true;
      return;
    }
    runChatPass();
  }

  function runChatPass() {
    if (state.disposed || document.hidden) return;
    var session = state.sessionGen;
    var url = state.apiURL;
    var chatGen = state.chatGen;
    var peer = state.activeChat;
    var passId = state.chatPassId + 1;
    state.chatPassId = passId;
    var convAbort = new AbortController();
    var histAbort = peer ? new AbortController() : null;
    state.chatPass = { id: passId, convAbort: convAbort, histAbort: histAbort };
    state.chatInflight = true;
    state.chatPending = false;

    var convDone = api('/ob/chatconversations', { signal: convAbort.signal }).then(function onConversations(rows) {
      if (!passCurrent(passId, session, url)) return;
      state.conversations = rows || [];
      renderConversations();
    }).catch(function onConversationsFail() {
      if (!passCurrent(passId, session, url)) return;
    });

    var histDone = Promise.resolve();
    if (peer && histAbort) {
      histDone = api('/ob/chatmessages/' + encodeURIComponent(peer), { signal: histAbort.signal }).then(function onHistory(rows) {
        if (!passCurrent(passId, session, url) || state.chatGen !== chatGen || state.activeChat !== peer) return;
        state.messages = rows || [];
        state.historyError = false;
        renderChat();
        maybeQueueRead(session, url, chatGen, peer);
      }).catch(function onHistoryFail() {
        if (!passCurrent(passId, session, url) || state.chatGen !== chatGen || state.activeChat !== peer) return;
        state.historyError = true;
        renderChat();
      });
    }

    Promise.all([convDone, histDone]).then(function finishPass() {
      if (!state.chatPass || state.chatPass.id !== passId) return;
      state.chatInflight = false;
      state.chatPass = null;
      if (state.disposed || state.sessionGen !== session) return;
      if (document.hidden) return;
      if (state.chatPending) {
        state.chatPending = false;
        runChatPass();
      } else {
        scheduleChatReconcile();
      }
    });
  }

  function maybeQueueRead(session, url, chatGen, peer) {
    if (state.disposed || document.hidden) return;
    if (state.sessionGen !== session || state.apiURL !== url) return;
    if (state.chatGen !== chatGen || state.activeChat !== peer) return;
    if (state.readReceipt || state.readRetryPending) return;
    var occurrences = Object.create(null);
    var unreadIds = [];
    state.messages.forEach(function collectUnread(message) {
      var key = messageEntryKey(message || {}, occurrences);
      if (!message || message.outgoing || message.read || state.readAcknowledged[key]) return;
      unreadIds.push(key);
    });
    if (!unreadIds.length) return;
    var controller = new AbortController();
    var owner = {
      session: session,
      url: url,
      chatGen: chatGen,
      peer: peer,
      controller: controller,
      unreadIds: unreadIds,
    };
    state.readReceipt = owner;
    api('/ob/markchatasread/' + encodeURIComponent(peer), {
      method: 'POST',
      signal: controller.signal,
    }).then(function onMarked() {
      if (state.readReceipt !== owner || state.disposed) return;
      if (state.sessionGen !== owner.session || state.apiURL !== owner.url) return;
      if (state.chatGen !== owner.chatGen || state.activeChat !== owner.peer) return;
      owner.unreadIds.forEach(function rememberRead(key) { state.readAcknowledged[key] = true; });
      state.readReceipt = null;
      state.readRetryPending = null;
      requestChatUpdate();
    }).catch(function onReadFailed() {
      if (state.readReceipt !== owner) return;
      state.readReceipt = null;
      if (state.disposed || state.sessionGen !== owner.session || state.apiURL !== owner.url) return;
      if (state.chatGen !== owner.chatGen || state.activeChat !== owner.peer) return;
      state.readRetryPending = owner;
    });
  }

  function reconcileChat() {
    requestChatUpdate();
  }

  function closeSocket() {
    if (state.socketTimer) window.clearTimeout(state.socketTimer);
    state.socketTimer = null;
    if (state.socket) {
      var socket = state.socket;
      state.socket = null;
      socket.close();
    }
  }

  function connectSocket() {
    if (state.disposed) return;
    closeSocket();
    var socket = new WebSocket(core.socketURL(state.apiURL));
    var session = state.sessionGen;
    var url = state.apiURL;
    state.socket = socket;
    socket.addEventListener('open', function socketOpen() {
      if (state.disposed || state.socket !== socket) return;
      setConnection(true, 'Live');
    });
    socket.addEventListener('message', function socketMessage(event) {
      if (state.disposed || state.socket !== socket || state.sessionGen !== session || state.apiURL !== url) return;
      var payload;
      try {
        payload = JSON.parse(event.data);
      } catch (error) {
        return;
      }
      handleSocketEvent(payload, session, url, socket);
    });
    socket.addEventListener('close', function socketClosed() {
      if (state.disposed || state.socket !== socket) return;
      state.socket = null;
      if (!state.connected) return;
      setConnection(true, 'HTTP only');
      state.socketTimer = window.setTimeout(connectSocket, 3000);
    });
    socket.addEventListener('error', function socketError() {
      socket.close();
    });
  }

  function handleSocketEvent(payload, session, url, socket) {
    if (state.disposed || state.socket !== socket || state.sessionGen !== session || state.apiURL !== url) return;
    if (payload.message) {
      var message = payload.message;
      if (state.activeChat !== message.peerId) {
        toast('New message from ' + core.displayName(profileFor(message.peerId), message.peerId) + '.');
      }
      requestChatUpdate();
      return;
    }
    if (payload.messageTyping && state.activeChat === payload.messageTyping.peerId) {
      var name = core.displayName(profileFor(state.activeChat), state.activeChat);
      byID('chatStatus').textContent = name + ' is typing…';
      if (state.typingTimer) window.clearTimeout(state.typingTimer);
      var typingSession = state.sessionGen;
      var typingURL = state.apiURL;
      var typingChatGen = state.chatGen;
      var typingPeer = state.activeChat;
      var typingTimer = window.setTimeout(function resetTyping() {
        if (state.typingTimer !== typingTimer) return;
        state.typingTimer = null;
        if (state.disposed || state.sessionGen !== typingSession || state.apiURL !== typingURL) return;
        if (state.chatGen !== typingChatGen || state.activeChat !== typingPeer) return;
        byID('chatStatus').textContent = 'Direct and signed';
      }, 2500);
      state.typingTimer = typingTimer;
      return;
    }
    if (payload.messageRead) {
      requestChatUpdate();
      return;
    }
    if (payload.notification) {
      var notice = payload.notification;
      fetchProfile(notice.peerId).then(function afterProfile() {
        if (state.disposed || state.sessionGen !== session || state.apiURL !== url) return;
        toast(core.displayName(profileFor(notice.peerId), notice.peerId) + (notice.type === 'follow' ? ' followed you.' : ' unfollowed you.'));
      });
    }
  }

  async function connect() {
    beginSession();
    var gen = state.sessionGen;
    setConnection(false, 'Connecting…');
    closeSocket();
    try {
      state.apiURL = core.normalizeAPIURL(state.apiURL);
      var config = await api('/ob/config');
      if (gen !== state.sessionGen) return;
      state.config = config || {};
      state.boundPeerId = state.config.peerID || '';
      setConnection(true, 'Connected');
      if (state.paymentReader) {
        state.paymentReader.invalidate();
        state.paymentReader.start();
      }
      connectSocket();
      requestChatUpdate();
      try {
        await Promise.all([loadProfile(), loadFollowing(), loadOwnPosts(), loadPeers()]);
      } catch (ancillary) { /* payments still bind */ }
      if (gen !== state.sessionGen) return;
      renderAll();
      try {
        await hydratePeers();
        await loadRemotePosts();
      } catch (ignored) { /* cached names only */ }
      if (gen !== state.sessionGen) return;
      renderAll();
      if (!state.profile.name && !state.profile.handle && !localStorage.getItem('bitbook.profilePrompted')) {
        localStorage.setItem('bitbook.profilePrompted', 'true');
        openProfileDialog();
      }
    } catch (error) {
      if (gen !== state.sessionGen) return;
      setConnection(false, 'Daemon offline');
      state.config = {};
      state.boundPeerId = '';
      state.paymentRequests = [];
      state.paymentNotice = 'Payment requests unavailable for this identity';
      state.peers = [];
      renderAll();
      toast(error.message, 'error');
    }
  }

  function openProfileDialog() {
    byID('profileNameInput').value = state.profile.name || '';
    byID('profileHandleInput').value = (state.profile.handle || '').replace(/^@/, '');
    byID('profileAboutInput').value = state.profile.about || '';
    byID('profileDialog').showModal();
  }

  async function saveProfile() {
    if (!state.connected) return toast('Connect to the daemon first.', 'error');
    var button = byID('saveProfileButton');
    button.disabled = true;
    try {
      state.profile = await api('/ob/profile', {
        method: 'PUT',
        body: JSON.stringify({
          name: byID('profileNameInput').value.trim(),
          handle: byID('profileHandleInput').value.trim().replace(/^@/, ''),
          about: byID('profileAboutInput').value.trim(),
        }),
      });
      byID('profileDialog').close();
      renderAll();
      toast('Profile saved and signed.');
    } catch (error) {
      toast(error.message, 'error');
    } finally {
      button.disabled = false;
    }
  }

  async function publishPost() {
    var input = byID('postInput');
    var text = input.value.trim();
    if (!text) return;
    var button = byID('publishPostButton');
    button.disabled = true;
    try {
      var result = await api('/ob/post', { method: 'POST', body: JSON.stringify({ status: text }) });
      input.value = '';
      updatePostCount();
      await loadOwnPosts();
      renderPosts();
      toast(result && result.published ? 'Post signed and published.' : 'Post signed and saved locally.');
    } catch (error) {
      toast(error.message, 'error');
    } finally {
      button.disabled = !state.connected;
    }
  }

  async function deletePost(identifier) {
    if (!identifier || !window.confirm('Delete this post from your published state?')) return;
    try {
      await api('/ob/post/' + encodeURIComponent(identifier), { method: 'DELETE' });
      await loadOwnPosts();
      renderPosts();
      toast('Post deleted.');
    } catch (error) {
      toast(error.message, 'error');
    }
  }

  async function follow(peerID) {
    var id = (peerID || '').trim();
    if (!id) return;
    try {
      var result = await api('/ob/follow', { method: 'POST', body: JSON.stringify({ id: id }) });
      byID('followPeerInput').value = '';
      await loadFollowing();
      await fetchProfile(id);
      await loadRemotePosts();
      renderAll();
      toast(result && result.queued ? 'Follow saved; notification queued for the offline peer.' : 'Now following ' + core.displayName(profileFor(id), id) + '.');
    } catch (error) {
      toast(error.message, 'error');
    }
  }

  async function unfollow(peerID) {
    try {
      var result = await api('/ob/unfollow', { method: 'POST', body: JSON.stringify({ id: peerID }) });
      await loadFollowing();
      delete state.remotePosts[peerID];
      renderAll();
      toast(result && result.queued ? 'Unfollow saved; notification queued.' : 'Peer unfollowed.');
    } catch (error) {
      toast(error.message, 'error');
    }
  }

  function openChat(peerID) {
    var id = (peerID || '').trim();
    if (!id) return;
    if (id === state.config.peerID) return toast('You cannot message yourself.', 'error');
    if (state.activeChat !== id) {
      invalidateConversationOwnership();
      state.chatGen += 1;
      if (state.chatPass && state.chatPass.histAbort) {
        try { state.chatPass.histAbort.abort(); } catch (error) { /* owned */ }
      }
      state.messages = [];
      state.historyError = false;
      state.transcriptKey = '';
      byID('messageInput').value = '';
    }
    state.activeChat = id;
    renderConversations();
    renderChat();
    requestChatUpdate();
  }

  function closeChat() {
    invalidateConversationOwnership();
    state.chatGen += 1;
    if (state.chatPass && state.chatPass.histAbort) {
      try { state.chatPass.histAbort.abort(); } catch (error) { /* owned */ }
    }
    state.activeChat = '';
    state.messages = [];
    state.transcriptKey = '';
    renderConversations();
    renderChat();
    requestChatUpdate();
  }

  async function sendMessage() {
    var input = byID('messageInput');
    var message = input.value.trim();
    if (!message || !state.activeChat) return;
    var session = state.sessionGen;
    var url = state.apiURL;
    var chatGen = state.chatGen;
    var recipient = state.activeChat;
    var button = byID('sendMessageButton');
    button.disabled = true;
    try {
      var result = await api('/ob/chat', {
        method: 'POST',
        body: JSON.stringify({ peerId: recipient, message: message }),
      });
      if (state.disposed || state.sessionGen !== session || state.apiURL !== url) return;
      if (state.chatGen === chatGen && state.activeChat === recipient) {
        if (input.value === message) input.value = '';
        toast(result && result.queued ? 'Peer is offline; message queued securely on this daemon.' : 'Message delivered.');
        requestChatUpdate();
      }
    } catch (error) {
      if (state.disposed || state.sessionGen !== session) return;
      if (state.chatGen === chatGen && state.activeChat === recipient) toast(error.message, 'error');
    } finally {
      button.disabled = !state.connected || !state.activeChat;
    }
  }

  function sendTyping() {
    if (!state.connected || !state.activeChat || !byID('messageInput').value.trim()) return;
    if (Date.now() - state.lastTypingAt < 2500) return;
    state.lastTypingAt = Date.now();
    api('/ob/chat', {
      method: 'POST',
      body: JSON.stringify({ peerId: state.activeChat, message: '' }),
    }).catch(function ignoreTypingFailure() {});
  }

  function updatePostCount() {
    byID('postCount').textContent = byID('postInput').value.length + ' / 280';
  }

  function showView(name) {
    Array.prototype.forEach.call(document.querySelectorAll('.tab'), function updateTab(tab) {
      tab.classList.toggle('active', tab.getAttribute('data-view') === name);
    });
    Array.prototype.forEach.call(document.querySelectorAll('.view'), function updateView(view) {
      view.classList.toggle('active', view.id === name + 'View');
    });
    if (name === 'network' && state.connected) {
      loadPeers().then(function networkLoaded() { renderNetwork(); }).catch(function networkFailed(error) { toast(error.message, 'error'); });
    }
  }

  function bindPaymentInbox() {
    if (!window.BitBookPaymentInbox || !window.bitbookWallet) return;
    state.paymentReader = window.BitBookPaymentInbox.createReader({
      bridge: {
        get: function getInbox() { return window.bitbookWallet.getPaymentInbox(); },
      },
      onSnapshot: function onPaymentSnapshot(dto) {
        if (state.disposed) return;
        var bound = window.BitBookPaymentInbox.bindSnapshot(dto, state.boundPeerId);
        var key = JSON.stringify({
          notice: bound.notice,
          rows: bound.requests.map(function rowKey(row) {
            return [row.request_id, row.digest, row.status, row.amount_display, row.memo];
          }),
        });
        if (key === state.paymentKey) return;
        state.paymentKey = key;
        state.paymentRequests = bound.requests;
        state.paymentNotice = bound.notice;
        renderPaymentNotice();
        renderConversations();
        renderChat();
      },
    });
    state.paymentReader.start();
  }

  function onAppVisibility() {
    if (state.disposed) return;
    if (document.hidden) {
      if (state.chatTimer) {
        window.clearTimeout(state.chatTimer);
        state.chatTimer = null;
      }
      return;
    }
    requestChatUpdate();
  }

  function disposeApp() {
    if (state.disposed) return;
    state.disposed = true;
    state.sessionGen += 1;
    state.chatGen += 1;
    abortChatWork();
    state.chatInflight = false;
    state.chatPass = null;
    closeSocket();
    document.removeEventListener('visibilitychange', onAppVisibility);
    window.removeEventListener('pagehide', disposeApp);
    window.removeEventListener('unload', disposeApp);
    if (state.paymentReader) {
      state.paymentReader.dispose();
      state.paymentReader = null;
    }
  }

  function bindEvents() {
    bindPaymentInbox();
    byID('homeButton').addEventListener('click', function showFeed() { showView('feed'); });
    Array.prototype.forEach.call(document.querySelectorAll('.tab'), function bindTab(tab) {
      tab.addEventListener('click', function switchView() { showView(tab.getAttribute('data-view')); });
    });
    document.addEventListener('visibilitychange', onAppVisibility);
    byID('connectionButton').addEventListener('click', function openConnectionDialog() {
      byID('apiURLInput').value = state.apiURL;
      byID('connectionDialog').showModal();
    });
    byID('editProfileButton').addEventListener('click', openProfileDialog);
    byID('saveProfileButton').addEventListener('click', function saveProfileClick(event) {
      event.preventDefault();
      saveProfile();
    });
    byID('saveConnectionButton').addEventListener('click', function saveConnection(event) {
      event.preventDefault();
      try {
        state.apiURL = core.normalizeAPIURL(byID('apiURLInput').value);
        localStorage.setItem('bitbook.apiURL', state.apiURL);
        byID('connectionDialog').close();
        connect();
      } catch (error) {
        toast(error.message, 'error');
      }
    });
    byID('postInput').addEventListener('input', updatePostCount);
    byID('postInput').addEventListener('keydown', function publishShortcut(event) {
      if ((event.ctrlKey || event.metaKey) && event.key === 'Enter') publishPost();
    });
    byID('publishPostButton').addEventListener('click', publishPost);
    byID('refreshFeedButton').addEventListener('click', function refreshClick() { refreshFeed(true); });
    byID('followPeerButton').addEventListener('click', function followClick() { follow(byID('followPeerInput').value); });
    byID('followPeerInput').addEventListener('keydown', function followEnter(event) {
      if (event.key === 'Enter') follow(byID('followPeerInput').value);
    });
    byID('addPeerButton').addEventListener('click', function goToNetwork() {
      showView('network');
      byID('followPeerInput').focus();
    });
    byID('newChatButton').addEventListener('click', function newChat() {
      byID('chatPeerInput').value = '';
      byID('peerDialog').showModal();
      byID('chatPeerInput').focus();
    });
    byID('openChatButton').addEventListener('click', function openChatClick(event) {
      event.preventDefault();
      var peerID = byID('chatPeerInput').value;
      if (!peerID.trim()) return;
      byID('peerDialog').close();
      openChat(peerID);
    });
    byID('closeChatButton').addEventListener('click', closeChat);
    byID('sendMessageButton').addEventListener('click', sendMessage);
    byID('messageInput').addEventListener('input', sendTyping);
    byID('messageInput').addEventListener('keydown', function sendShortcut(event) {
      if (event.key === 'Enter' && !event.shiftKey) {
        event.preventDefault();
        sendMessage();
      }
    });
  }

  bindEvents();
  window.addEventListener('pagehide', disposeApp);
  window.addEventListener('unload', disposeApp);
  updatePostCount();
  renderAll();
  connect();
}());
