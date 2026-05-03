// app.js - Eesha Browser Renderer Logic
// Handles the browser chrome UI (tabs, URL bar, navigation)

(function () {
  'use strict';

  // ─── DOM References ────────────────────────────────────────────────────────
  const tabBar = document.getElementById('tabBar');
  const tabsContainer = document.getElementById('tabsContainer');
  const newTabBtn = document.getElementById('newTabBtn');
  const urlInput = document.getElementById('urlInput');
  const urlInputWrapper = document.getElementById('urlInputWrapper');
  const securityIndicator = document.getElementById('securityIndicator');
  const backBtn = document.getElementById('backBtn');
  const forwardBtn = document.getElementById('forwardBtn');
  const reloadBtn = document.getElementById('reloadBtn');
  const homeBtn = document.getElementById('homeBtn');
  const bookmarkBtn = document.getElementById('bookmarkBtn');
  const settingsBtn = document.getElementById('settingsBtn');
  const progressBar = document.getElementById('progressBar');
  const progressBarContainer = document.getElementById('progressBarContainer');

  // ─── State ─────────────────────────────────────────────────────────────────
  let activeTabId = null;
  let tabs = [];
  let isLoading = false;
  let urlInputFocused = false;
  let bookmarks = [];

  // ─── Search Engine ─────────────────────────────────────────────────────────
  const SEARCH_ENGINE = 'https://duckduckgo.com/?q=';
  const NEWTAB_URL = 'eesha://newtab';
  const SETTINGS_URL = 'eesha://settings';

  // ─── Platform Detection ────────────────────────────────────────────────────
  function detectPlatform() {
    const platform = navigator.platform || '';
    if (platform.includes('Mac')) {
      document.body.classList.add('platform-mac');
    } else if (platform.includes('Win')) {
      document.body.classList.add('platform-win');
    } else {
      document.body.classList.add('platform-linux');
    }
  }
  detectPlatform();

  // ─── Tab Management ────────────────────────────────────────────────────────
  function createTabElement(tabData) {
    const tab = document.createElement('div');
    tab.className = 'tab' + (tabData.active ? ' active' : '');
    tab.dataset.tabId = tabData.tabId;

    const favicon = document.createElement('div');
    favicon.className = 'tab-favicon';
    // Use first letter as favicon placeholder
    favicon.textContent = getFaviconText(tabData.url, tabData.title);
    favicon.style.background = getFaviconColor(tabData.url);
    favicon.style.color = '#fff';
    favicon.style.fontSize = '10px';
    favicon.style.display = 'flex';
    favicon.style.alignItems = 'center';
    favicon.style.justifyContent = 'center';
    favicon.style.borderRadius = '3px';

    const title = document.createElement('span');
    title.className = 'tab-title';
    title.textContent = tabData.title || 'New Tab';

    const closeBtn = document.createElement('button');
    closeBtn.className = 'tab-close';
    closeBtn.innerHTML = '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>';

    closeBtn.addEventListener('click', (e) => {
      e.stopPropagation();
      window.eesha.closeTab(tabData.tabId);
    });

    tab.addEventListener('click', () => {
      window.eesha.switchTab(tabData.tabId);
    });

    tab.appendChild(favicon);
    tab.appendChild(title);
    tab.appendChild(closeBtn);

    return tab;
  }

  function getFaviconText(url, title) {
    if (!url || url === NEWTAB_URL) return '🏠';
    if (url === SETTINGS_URL) return '⚙';
    try {
      const hostname = new URL(url).hostname;
      return hostname.charAt(0).toUpperCase();
    } catch {
      return (title || '?').charAt(0).toUpperCase();
    }
  }

  function getFaviconColor(url) {
    if (!url || url === NEWTAB_URL) return '#e94560';
    if (url === SETTINGS_URL) return '#5a5a7a';
    // Generate a consistent color based on URL
    let hash = 0;
    try {
      const hostname = new URL(url).hostname;
      for (let i = 0; i < hostname.length; i++) {
        hash = hostname.charCodeAt(i) + ((hash << 5) - hash);
      }
    } catch {
      hash = 0;
    }
    const hue = Math.abs(hash % 360);
    return `hsl(${hue}, 55%, 45%)`;
  }

  function updateTabsList(tabsData) {
    tabs = tabsData;
    tabsContainer.innerHTML = '';

    tabsData.forEach((tabData) => {
      const tabEl = createTabElement(tabData);
      tabsContainer.appendChild(tabEl);
    });

    // Update active tab ID
    const activeTab = tabsData.find(t => t.active);
    if (activeTab) {
      activeTabId = activeTab.tabId;
      updateUrlBar(activeTab.url, activeTab.loading);
      updateNavButtons(activeTab);
      updateBookmarkButton(activeTab.url);
    }
  }

  function addTab(tabData) {
    tabs.push(tabData);
    const tabEl = createTabElement(tabData);
    tabsContainer.appendChild(tabEl);
    scrollToTab(tabEl);
  }

  function updateTab(tabId, updates) {
    const tabEl = tabsContainer.querySelector(`[data-tab-id="${tabId}"]`);
    if (!tabEl) return;

    if (updates.title !== undefined) {
      const titleEl = tabEl.querySelector('.tab-title');
      if (titleEl) titleEl.textContent = updates.title || 'New Tab';
    }

    if (updates.url !== undefined) {
      const faviconEl = tabEl.querySelector('.tab-favicon');
      if (faviconEl) {
        faviconEl.textContent = getFaviconText(updates.url, updates.title);
        faviconEl.style.background = getFaviconColor(updates.url);
      }
    }

    if (updates.loading !== undefined) {
      tabEl.classList.toggle('loading', updates.loading);
    }

    // Update our local tabs state
    const tabIdx = tabs.findIndex(t => t.tabId === tabId);
    if (tabIdx !== -1) {
      Object.assign(tabs[tabIdx], updates);
    }
  }

  function setActiveTab(tabId) {
    // Remove active from all
    tabsContainer.querySelectorAll('.tab').forEach(el => el.classList.remove('active'));
    // Add active to target
    const tabEl = tabsContainer.querySelector(`[data-tab-id="${tabId}"]`);
    if (tabEl) {
      tabEl.classList.add('active');
      scrollToTab(tabEl);
    }
    activeTabId = tabId;
  }

  function removeTab(tabId) {
    const tabEl = tabsContainer.querySelector(`[data-tab-id="${tabId}"]`);
    if (tabEl) {
      tabEl.style.opacity = '0';
      tabEl.style.transform = 'scale(0.9)';
      tabEl.style.transition = 'all 0.15s ease';
      setTimeout(() => tabEl.remove(), 150);
    }
    tabs = tabs.filter(t => t.tabId !== tabId);
  }

  function scrollToTab(tabEl) {
    if (tabEl) {
      tabEl.scrollIntoView({ behavior: 'smooth', block: 'nearest', inline: 'nearest' });
    }
  }

  // ─── URL Bar ───────────────────────────────────────────────────────────────
  function updateUrlBar(url, loading) {
    if (!urlInputFocused) {
      // Clean up the URL for display
      let displayUrl = url || '';
      if (displayUrl === NEWTAB_URL || displayUrl === 'eesha://newtab/') {
        displayUrl = '';
      } else if (displayUrl === SETTINGS_URL || displayUrl === 'eesha://settings/') {
        displayUrl = 'eesha://settings';
      }
      urlInput.value = displayUrl;
    }
    updateSecurityIndicator(url);
  }

  function updateSecurityIndicator(url) {
    securityIndicator.classList.remove('secure', 'insecure', 'internal');
    if (!url) return;

    if (url.startsWith('https://')) {
      securityIndicator.classList.add('secure');
    } else if (url.startsWith('http://')) {
      securityIndicator.classList.add('insecure');
    } else if (url.startsWith('eesha://')) {
      securityIndicator.classList.add('internal');
    }
  }

  function navigateFromUrlBar() {
    const query = urlInput.value.trim();
    if (!query) {
      window.eesha.navigate(NEWTAB_URL);
      return;
    }

    let url;
    if (/^https?:\/\//i.test(query)) {
      url = query;
    } else if (/^[a-zA-Z0-9][a-zA-Z0-9-]*\.[a-zA-Z]{2,}/.test(query)) {
      url = 'https://' + query;
    } else if (query.startsWith('eesha://')) {
      url = query;
    } else {
      url = SEARCH_ENGINE + encodeURIComponent(query);
    }

    window.eesha.navigate(url);
    urlInput.blur();
  }

  // ─── Navigation Buttons ────────────────────────────────────────────────────
  function updateNavButtons(tabData) {
    backBtn.classList.toggle('disabled', !tabData.canGoBack);
    forwardBtn.classList.toggle('disabled', !tabData.canGoForward);
  }

  // ─── Progress Bar ─────────────────────────────────────────────────────────
  function showLoading() {
    isLoading = true;
    progressBar.className = 'progress-bar loading';
    progressBarContainer.style.height = '2px';
    reloadBtn.querySelector('svg').style.animation = 'spin 1s linear infinite';
  }

  function hideLoading() {
    isLoading = false;
    progressBar.className = 'progress-bar complete';
    reloadBtn.querySelector('svg').style.animation = '';
    setTimeout(() => {
      progressBar.className = 'progress-bar hidden';
      setTimeout(() => {
        progressBarContainer.style.height = '0';
      }, 300);
    }, 200);
  }

  // ─── Bookmark Button ──────────────────────────────────────────────────────
  function updateBookmarkButton(url) {
    if (!url || url.startsWith('eesha://')) {
      bookmarkBtn.classList.remove('bookmarked');
      bookmarkBtn.style.opacity = '0.4';
      return;
    }
    bookmarkBtn.style.opacity = '1';
    window.eesha.isBookmarked(url).then((isMarked) => {
      bookmarkBtn.classList.toggle('bookmarked', isMarked);
    });
  }

  function toggleBookmark() {
    const activeTab = tabs.find(t => t.tabId === activeTabId);
    if (!activeTab || activeTab.url.startsWith('eesha://')) return;

    window.eesha.isBookmarked(activeTab.url).then((isMarked) => {
      if (isMarked) {
        window.eesha.removeBookmark(activeTab.url);
        bookmarkBtn.classList.remove('bookmarked');
      } else {
        window.eesha.addBookmark(activeTab.url, activeTab.title);
        bookmarkBtn.classList.add('bookmarked');
      }
    });
  }

  // ─── Event Listeners ───────────────────────────────────────────────────────

  // New Tab button
  newTabBtn.addEventListener('click', () => {
    window.eesha.createTab(NEWTAB_URL);
  });

  // URL Input
  urlInput.addEventListener('focus', () => {
    urlInputFocused = true;
    urlInput.select();
  });

  urlInput.addEventListener('blur', () => {
    urlInputFocused = false;
    // Restore URL display
    const activeTab = tabs.find(t => t.tabId === activeTabId);
    if (activeTab) {
      updateUrlBar(activeTab.url, activeTab.loading);
    }
  });

  urlInput.addEventListener('keydown', (e) => {
    if (e.key === 'Enter') {
      navigateFromUrlBar();
    } else if (e.key === 'Escape') {
      urlInput.blur();
    }
  });

  // Navigation buttons
  backBtn.addEventListener('click', () => window.eesha.goBack());
  forwardBtn.addEventListener('click', () => window.eesha.goForward());
  reloadBtn.addEventListener('click', () => window.eesha.reload());
  homeBtn.addEventListener('click', () => window.eesha.navigate(NEWTAB_URL));
  bookmarkBtn.addEventListener('click', toggleBookmark);
  settingsBtn.addEventListener('click', () => window.eesha.createTab(SETTINGS_URL));

  // Window controls (for Windows/Linux)
  const minimizeBtn = document.getElementById('minimizeBtn');
  const maximizeBtn = document.getElementById('maximizeBtn');
  const closeBtn = document.getElementById('closeBtn');

  if (minimizeBtn) {
    minimizeBtn.addEventListener('click', () => {
      // These would need a separate IPC to control the window
      // For now, they're visual placeholders
    });
  }

  // ─── IPC Event Handlers ────────────────────────────────────────────────────

  // URL change
  window.eesha.onUrlChange((data) => {
    if (data.tabId === activeTabId) {
      updateUrlBar(data.url, isLoading);
      updateNavButtons(data);
      updateBookmarkButton(data.url);
    }
    updateTab(data.tabId, { url: data.url });
  });

  // Title change
  window.eesha.onTitleChange((data) => {
    updateTab(data.tabId, { title: data.title });
  });

  // Loading state change
  window.eesha.onLoadingStateChange((data) => {
    updateTab(data.tabId, { loading: data.loading });
    if (data.tabId === activeTabId) {
      if (data.loading) {
        showLoading();
      } else {
        hideLoading();
      }
    }
  });

  // Tab created
  window.eesha.onTabCreated((data) => {
    addTab(data);
    setActiveTab(data.tabId);
  });

  // Tab switched
  window.eesha.onTabSwitched((data) => {
    setActiveTab(data.tabId);
    // Fetch latest tab info
    window.eesha.getActiveTab().then((tabData) => {
      if (tabData) {
        updateUrlBar(tabData.url, tabData.loading);
        updateNavButtons(tabData);
        updateBookmarkButton(tabData.url);
      }
    });
  });

  // Tab closed
  window.eesha.onTabClosed((data) => {
    removeTab(data.tabId);
  });

  // Bookmarks updated
  window.eesha.onBookmarksUpdated((data) => {
    bookmarks = data.bookmarks;
  });

  // Focus URL bar
  window.eesha.onFocusUrlBar(() => {
    urlInput.focus();
    urlInput.select();
  });

  // ─── Keyboard Shortcuts ────────────────────────────────────────────────────
  document.addEventListener('keydown', (e) => {
    // Ctrl+L - Focus URL bar
    if ((e.ctrlKey || e.metaKey) && e.key === 'l') {
      e.preventDefault();
      urlInput.focus();
      urlInput.select();
    }
    // Ctrl+T - New tab (also handled by menu, but ensure it works)
    if ((e.ctrlKey || e.metaKey) && e.key === 't') {
      // This is handled by the application menu
    }
    // Ctrl+W - Close tab
    if ((e.ctrlKey || e.metaKey) && e.key === 'w') {
      // This is handled by the application menu
    }
  });

  // ─── Initialize ────────────────────────────────────────────────────────────
  async function init() {
    try {
      // Get initial tabs
      const tabsData = await window.eesha.getTabs();
      if (tabsData && tabsData.length > 0) {
        updateTabsList(tabsData);
      }

      // Get initial bookmarks
      bookmarks = await window.eesha.getBookmarks();

      // Get active tab info
      const activeTab = await window.eesha.getActiveTab();
      if (activeTab) {
        activeTabId = activeTab.id;
        updateUrlBar(activeTab.url, activeTab.loading);
        updateNavButtons(activeTab);
        updateBookmarkButton(activeTab.url);
      }
    } catch (err) {
      console.error('Failed to initialize:', err);
    }
  }

  init();

  // ─── Double-click title bar to maximize ──────────────────────────────────
  tabBar.addEventListener('dblclick', (e) => {
    // Only if clicking on empty space in tab bar
    if (e.target === tabBar || e.target === tabsContainer) {
      // Would need IPC to toggle maximize
    }
  });

})();
