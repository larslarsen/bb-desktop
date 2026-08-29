(function expose(root, factory) {
  const api = factory();
  if (typeof module === 'object' && module.exports) {
    module.exports = api;
  } else {
    root.BitBookCore = api;
  }
}(typeof self !== 'undefined' ? self : this, function createCore() {
  'use strict';

  function normalizeAPIURL(value) {
    const raw = String(value || '').trim() || 'http://127.0.0.1:4002';
    const parsed = new URL(raw);
    if (parsed.protocol !== 'http:' && parsed.protocol !== 'https:') {
      throw new Error('The daemon URL must use HTTP or HTTPS.');
    }
    parsed.pathname = parsed.pathname.replace(/\/+$/, '');
    parsed.search = '';
    parsed.hash = '';
    return parsed.toString().replace(/\/$/, '');
  }

  function socketURL(apiURL) {
    const parsed = new URL(normalizeAPIURL(apiURL));
    parsed.protocol = parsed.protocol === 'https:' ? 'wss:' : 'ws:';
    parsed.pathname = '/ws';
    return parsed.toString();
  }

  function shortID(value, size) {
    const text = String(value || '');
    const width = size || 8;
    if (text.length <= width * 2 + 1) return text;
    return text.slice(0, width) + '…' + text.slice(-width);
  }

  function postText(post) {
    if (!post || typeof post !== 'object') return '';
    return post.status || post.longForm || '';
  }

  function sortFeed(items) {
    return (items || []).slice().sort((left, right) => {
      const leftTime = Date.parse(left.timestamp || 0) || 0;
      const rightTime = Date.parse(right.timestamp || 0) || 0;
      return rightTime - leftTime;
    });
  }

  function displayName(profile, fallback) {
    if (profile && profile.name) return profile.name;
    if (profile && profile.handle) return '@' + profile.handle;
    return shortID(fallback || (profile && profile.peerID) || 'Unknown');
  }

  return {
    displayName,
    normalizeAPIURL,
    postText,
    shortID,
    socketURL,
    sortFeed,
  };
}));
