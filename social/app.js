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
    profiles: {},
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

  async function api(path, options) {
    var settings = options || {};
    var headers = Object.assign({}, settings.headers || {});
    if (settings.body !== undefined) headers['Content-Type'] = 'application/json';
    var response;
    try {
      response = await fetch(state.apiURL + path, Object.assign({}, settings, { headers: headers }));
    } catch (error) {
      throw new Error('Cannot reach the BitBook daemon at ' + state.apiURL + '.');
    }
    var raw = await response.text();
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
    try {
      var profile = await api('/ob/profile/' + encodeURIComponent(peerID));
      state.profiles[peerID] = profile || { peerID: peerID };
    } catch (error) {
      if (!state.profiles[peerID]) state.profiles[peerID] = { peerID: peerID };
    }
    return state.profiles[peerID];
  }

  async function loadProfile() {
    state.profile = await api('/ob/profile') || {};
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
    state.conversations = await api('/ob/chatconversations') || [];
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

  function renderConversations() {
    var list = byID('conversationList');
    clear(list);
    if (!state.conversations.length) {
      list.appendChild(emptyState('No conversations', 'Start a signed, peer-to-peer chat.'));
      return;
    }
    state.conversations.forEach(function addConversation(conversation) {
      var profile = profileFor(conversation.peerId);
      var row = make('button', 'conversationRow' + (state.activeChat === conversation.peerId ? ' selected' : ''));
      row.type = 'button';
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

  function renderMessages() {
    var list = byID('messageList');
    clear(list);
    if (!state.messages.length) {
      list.appendChild(emptyState('Say hello', 'Messages are signed and sent directly to this peer.'));
      return;
    }
    state.messages.slice().reverse().forEach(function addMessage(message) {
      var bubble = make('div', 'message ' + (message.outgoing ? 'outgoing' : 'incoming'));
      bubble.appendChild(make('p', '', message.message));
      var status = formatTime(message.timestamp);
      if (message.outgoing && message.read) status += ' · read';
      bubble.appendChild(make('small', '', status));
      list.appendChild(bubble);
    });
    list.scrollTop = list.scrollHeight;
  }

  function renderChat() {
    var conversation = byID('conversation');
    var conversationList = byID('conversationList');
    var active = Boolean(state.activeChat);
    conversation.classList.toggle('hidden', !active);
    conversationList.classList.toggle('hidden', active);
    byID('sendMessageButton').disabled = !state.connected || !active;
    if (!active) return;
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
    closeSocket();
    var socket = new WebSocket(core.socketURL(state.apiURL));
    state.socket = socket;
    socket.addEventListener('open', function socketOpen() {
      if (state.socket === socket) setConnection(true, 'Live');
    });
    socket.addEventListener('message', function socketMessage(event) {
      var payload;
      try {
        payload = JSON.parse(event.data);
      } catch (error) {
        return;
      }
      handleSocketEvent(payload);
    });
    socket.addEventListener('close', function socketClosed() {
      if (state.socket !== socket) return;
      state.socket = null;
      if (!state.connected) return;
      setConnection(true, 'HTTP only');
      state.socketTimer = window.setTimeout(connectSocket, 3000);
    });
    socket.addEventListener('error', function socketError() {
      socket.close();
    });
  }

  async function handleSocketEvent(payload) {
    if (payload.message) {
      var message = payload.message;
      await fetchProfile(message.peerId);
      if (state.activeChat === message.peerId) {
        await loadActiveMessages();
        markActiveRead();
      } else {
        toast('New message from ' + core.displayName(profileFor(message.peerId), message.peerId) + '.');
      }
      await loadConversations();
      renderConversations();
      renderChat();
      return;
    }
    if (payload.messageTyping && state.activeChat === payload.messageTyping.peerId) {
      var name = core.displayName(profileFor(state.activeChat), state.activeChat);
      byID('chatStatus').textContent = name + ' is typing…';
      if (state.typingTimer) window.clearTimeout(state.typingTimer);
      state.typingTimer = window.setTimeout(function resetTyping() {
        byID('chatStatus').textContent = 'Direct and signed';
      }, 2500);
      return;
    }
    if (payload.messageRead) {
      if (state.activeChat === payload.messageRead.peerId) await loadActiveMessages();
      renderChat();
      return;
    }
    if (payload.notification) {
      var notice = payload.notification;
      await fetchProfile(notice.peerId);
      toast(core.displayName(profileFor(notice.peerId), notice.peerId) + (notice.type === 'follow' ? ' followed you.' : ' unfollowed you.'));
    }
  }

  async function connect() {
    setConnection(false, 'Connecting…');
    closeSocket();
    try {
      state.apiURL = core.normalizeAPIURL(state.apiURL);
      state.config = await api('/ob/config');
      await Promise.all([loadProfile(), loadFollowing(), loadOwnPosts(), loadPeers(), loadConversations()]);
      setConnection(true, 'Connected');
      renderAll();
      connectSocket();
      await hydratePeers();
      await loadRemotePosts();
      renderAll();
      if (!state.profile.name && !state.profile.handle && !localStorage.getItem('bitbook.profilePrompted')) {
        localStorage.setItem('bitbook.profilePrompted', 'true');
        openProfileDialog();
      }
    } catch (error) {
      setConnection(false, 'Daemon offline');
      state.config = {};
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

  async function loadActiveMessages() {
    if (!state.activeChat) {
      state.messages = [];
      return;
    }
    state.messages = await api('/ob/chatmessages/' + encodeURIComponent(state.activeChat)) || [];
  }

  async function markActiveRead() {
    if (!state.activeChat) return;
    var hasUnread = state.messages.some(function unread(message) { return !message.outgoing && !message.read; });
    if (!hasUnread) return;
    try {
      await api('/ob/markchatasread/' + encodeURIComponent(state.activeChat), { method: 'POST' });
      await Promise.all([loadActiveMessages(), loadConversations()]);
      renderConversations();
      renderChat();
    } catch (error) {
      // Reading local history still works if its receipt cannot be delivered yet.
    }
  }

  async function openChat(peerID) {
    var id = (peerID || '').trim();
    if (!id) return;
    if (id === state.config.peerID) return toast('You cannot message yourself.', 'error');
    state.activeChat = id;
    state.messages = [];
    byID('messageInput').value = '';
    try {
      await Promise.all([fetchProfile(id), loadActiveMessages()]);
      renderConversations();
      renderChat();
      byID('messageInput').focus();
      markActiveRead();
    } catch (error) {
      state.activeChat = '';
      renderChat();
      toast(error.message, 'error');
    }
  }

  function closeChat() {
    state.activeChat = '';
    state.messages = [];
    renderConversations();
    renderChat();
  }

  async function sendMessage() {
    var input = byID('messageInput');
    var message = input.value.trim();
    if (!message || !state.activeChat) return;
    var recipient = state.activeChat;
    var button = byID('sendMessageButton');
    button.disabled = true;
    try {
      var result = await api('/ob/chat', {
        method: 'POST',
        body: JSON.stringify({ peerId: recipient, message: message }),
      });
      input.value = '';
      await Promise.all([loadActiveMessages(), loadConversations()]);
      renderConversations();
      renderChat();
      toast(result && result.queued ? 'Peer is offline; message queued securely on this daemon.' : 'Message delivered.');
    } catch (error) {
      toast(error.message, 'error');
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

  function bindEvents() {
    byID('homeButton').addEventListener('click', function showFeed() { showView('feed'); });
    Array.prototype.forEach.call(document.querySelectorAll('.tab'), function bindTab(tab) {
      tab.addEventListener('click', function switchView() { showView(tab.getAttribute('data-view')); });
    });
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
  updatePostCount();
  renderAll();
  connect();
}());
