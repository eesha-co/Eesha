// preload.js - Secure IPC bridge between main and renderer processes
// Exposes a safe, minimal API via contextBridge

const { contextBridge, ipcRenderer } = require('electron');

contextBridge.exposeInMainWorld('eesha', {
  // ─── Navigation ─────────────────────────────────────────────────────────
  navigate: (url) => ipcRenderer.invoke('navigate', url),
  goBack: () => ipcRenderer.invoke('go-back'),
  goForward: () => ipcRenderer.invoke('go-forward'),
  reload: () => ipcRenderer.invoke('reload'),
  forceReload: () => ipcRenderer.invoke('force-reload'),

  // ─── Tab Management ─────────────────────────────────────────────────────
  createTab: (url) => ipcRenderer.invoke('create-tab', url),
  switchTab: (tabId) => ipcRenderer.invoke('switch-tab', tabId),
  closeTab: (tabId) => ipcRenderer.invoke('close-tab', tabId),
  getTabs: () => ipcRenderer.invoke('get-tabs'),
  getActiveTab: () => ipcRenderer.invoke('get-active-tab'),

  // ─── Bookmarks ──────────────────────────────────────────────────────────
  getBookmarks: () => ipcRenderer.invoke('get-bookmarks'),
  addBookmark: (url, title) => ipcRenderer.invoke('add-bookmark', url, title),
  removeBookmark: (url) => ipcRenderer.invoke('remove-bookmark', url),
  isBookmarked: (url) => ipcRenderer.invoke('is-bookmarked', url),

  // ─── History ────────────────────────────────────────────────────────────
  getHistory: () => ipcRenderer.invoke('get-history'),
  clearHistory: () => ipcRenderer.invoke('clear-history'),

  // ─── Focus ──────────────────────────────────────────────────────────────
  focusUrlBar: () => ipcRenderer.invoke('focus-url-bar'),

  // ─── Event Listeners ────────────────────────────────────────────────────
  onUrlChange: (callback) => {
    ipcRenderer.on('url-change', (_, data) => callback(data));
  },
  onTitleChange: (callback) => {
    ipcRenderer.on('title-change', (_, data) => callback(data));
  },
  onLoadingStateChange: (callback) => {
    ipcRenderer.on('loading-state', (_, data) => callback(data));
  },
  onTabCreated: (callback) => {
    ipcRenderer.on('tab-created', (_, data) => callback(data));
  },
  onTabSwitched: (callback) => {
    ipcRenderer.on('tab-switched', (_, data) => callback(data));
  },
  onTabClosed: (callback) => {
    ipcRenderer.on('tab-closed', (_, data) => callback(data));
  },
  onBookmarksUpdated: (callback) => {
    ipcRenderer.on('bookmarks-updated', (_, data) => callback(data));
  },
  onFocusUrlBar: (callback) => {
    ipcRenderer.on('focus-url-bar', () => callback());
  },
});
