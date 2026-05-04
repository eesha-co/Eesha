// main.js - Eesha Browser Electron Main Process
// A privacy-first browser powered by Chromium via Electron

const {
  app,
  BaseWindow,
  WebContentsView,
  BrowserWindow,
  ipcMain,
  session,
  protocol,
  dialog,
  Menu,
  clipboard,
  globalShortcut,
  nativeImage,
} = require('electron');
const path = require('path');
const fs = require('fs');

// ─── Protocol Registration (MUST be before app.ready) ────────────────────────
protocol.registerSchemesAsPrivileged([
  {
    scheme: 'eesha',
    privileges: {
      standard: true,
      secure: true,
      supportFetchAPI: true,
      corsEnabled: true,
      bypassCSP: false,
    },
  },
]);

// ─── Constants ────────────────────────────────────────────────────────────────
const APP_VERSION = '0.5.0';
const USER_AGENT_SUFFIX = `Eesha/${APP_VERSION}`;
const SEARCH_ENGINE = 'https://duckduckgo.com/?q=';
const NEWTAB_URL = 'eesha://newtab';
const SETTINGS_URL = 'eesha://settings';
const BOOKMARKS_FILE = path.join(app.getPath('userData'), 'bookmarks.json');
const HISTORY_FILE = path.join(app.getPath('userData'), 'history.json');
const MAX_HISTORY_ENTRIES = 1000;
const CHROME_HEIGHT = 82; // Tab bar + URL bar height in pixels

// ─── Resource Paths ──────────────────────────────────────────────────────────
const SHARED_DIR = path.join(__dirname, '..', 'shared');
const ICONS_DIR = path.join(SHARED_DIR, 'icons');
const RESOURCES_DIR = path.join(SHARED_DIR, 'resources');

// ─── Ad/Tracker Blocklist ────────────────────────────────────────────────────
const BLOCKED_DOMAINS = [
  // Major ad networks
  'doubleclick.net', 'googlesyndication.com', 'googleadservices.com',
  'google-analytics.com', 'googletagmanager.com',
  'connect.facebook.net', 'analytics.facebook.com',
  'ads.yahoo.com', 'ad.yieldmanager.com',
  'amazon-adsystem.com', 'associates-amazon.com',
  // Tracker networks
  'scorecardresearch.com', 'quantserve.com', 'moatads.com',
  'adsafeprotected.com', 'chartbeat.com', 'hotjar.com',
  'mixpanel.com', 'segment.io', 'segment.com',
  'amplitude.com', 'fullstory.com', 'crazyegg.com',
  'optimizely.com', 'adobedtm.com',
  // Common ad servers
  'adnxs.com', 'adsrvr.org', 'adroll.com', 'criteo.com',
  'outbrain.com', 'taboola.com', 'bidswitch.net',
  'rubiconproject.com', 'pubmatic.com', 'openx.net',
  'casalemedia.com', 'indexexchange.com', 'sharethrough.com',
  'lijit.com', 'media.net', 'mookie1.com',
  'bgclife.com', 'popads.net', 'revcontent.com',
  'taboola.com', 'outbrain.com', 'zemanta.com',
  // Fingerprinting
  'fpjs.io', 'fpcollect.com', 'botd.dev',
];

const BLOCKED_RESOURCE_TYPES = ['script', 'image', 'stylesheet', 'xmlhttprequest', 'sub_frame', 'font', 'media'];

// ─── Data Store Helpers ───────────────────────────────────────────────────────
function loadJSON(filePath, defaultValue) {
  try {
    if (fs.existsSync(filePath)) {
      return JSON.parse(fs.readFileSync(filePath, 'utf-8'));
    }
  } catch (e) {
    console.error(`Error loading ${filePath}:`, e);
  }
  return defaultValue;
}

function saveJSON(filePath, data) {
  try {
    fs.writeFileSync(filePath, JSON.stringify(data, null, 2), 'utf-8');
  } catch (e) {
    console.error(`Error saving ${filePath}:`, e);
  }
}

let bookmarks = loadJSON(BOOKMARKS_FILE, []);
let history = loadJSON(HISTORY_FILE, []);

function saveBookmarks() { saveJSON(BOOKMARKS_FILE, bookmarks); }
function saveHistory() { saveJSON(HISTORY_FILE, history); }

// ─── Tab Manager ──────────────────────────────────────────────────────────────
let tabs = [];
let activeTabId = null;
let mainWindow = null;
let chromeView = null; // The browser chrome (tab bar, URL bar, etc.)
let tabIdCounter = 0;

function createTabId() {
  return ++tabIdCounter;
}

function getTabById(id) {
  return tabs.find(t => t.id === id);
}

function addHistoryEntry(url, title) {
  if (!url || url.startsWith('eesha://')) return;
  const entry = {
    url,
    title: title || url,
    timestamp: Date.now(),
  };
  // Remove duplicate entries for the same URL (keep the latest)
  history = history.filter(h => h.url !== url);
  history.unshift(entry);
  if (history.length > MAX_HISTORY_ENTRIES) {
    history = history.slice(0, MAX_HISTORY_ENTRIES);
  }
  saveHistory();
}

function isBookmarked(url) {
  return bookmarks.some(b => b.url === url);
}

function addBookmark(url, title) {
  if (isBookmarked(url)) return;
  bookmarks.push({ url, title: title || url, timestamp: Date.now() });
  saveBookmarks();
}

function removeBookmark(url) {
  bookmarks = bookmarks.filter(b => b.url !== url);
  saveBookmarks();
}

// ─── New Tab Page HTML ────────────────────────────────────────────────────────
function getNewTabHTML() {
  return `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>New Tab - Eesha</title>
  <link rel="icon" type="image/png" href="eesha://resources/resources/eesha-logo.png">
  <style>
    *, *::before, *::after {
      margin: 0;
      padding: 0;
      box-sizing: border-box;
    }

    body {
      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
      background: #ffffff;
      color: #202124;
      min-height: 100vh;
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: flex-start;
      overflow-x: hidden;
      position: relative;
    }

    /* Eesha logo watermark background */
    body::after {
      content: '';
      position: fixed;
      top: 28%;
      left: 50%;
      transform: translate(-50%, -50%);
      width: 45vmin;
      height: 45vmin;
      background-image: url('eesha://resources/resources/eesha-logo.png');
      background-size: contain;
      background-repeat: no-repeat;
      background-position: center;
      opacity: 0.12;
      pointer-events: none;
      z-index: 0;
    }

    .container {
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: 24px;
      max-width: 680px;
      width: 100%;
      padding: 15vh 32px 32px;
      position: relative;
      z-index: 1;
    }

    .search-wrapper {
      width: 100%;
      max-width: 584px;
      position: relative;
    }

    .search-bar {
      width: 100%;
      padding: 12px 16px 12px 48px;
      background: #ffffff;
      border: 1px solid #dfe1e5;
      border-radius: 24px;
      color: #202124;
      font-size: 16px;
      outline: none;
      transition: box-shadow 0.2s ease, border-color 0.2s ease;
    }

    .search-bar:hover {
      box-shadow: 0 1px 6px rgba(32,33,36,0.28);
      border-color: rgba(223,225,229,0);
    }

    .search-bar:focus {
      box-shadow: 0 1px 6px rgba(32,33,36,0.28);
      border-color: rgba(223,225,229,0);
    }

    .search-bar::placeholder { color: #9aa0a6; }

    .search-icon {
      position: absolute;
      left: 16px;
      top: 50%;
      transform: translateY(-50%);
      color: #9aa0a6;
      pointer-events: none;
    }

    .shortcuts {
      display: flex;
      flex-wrap: wrap;
      justify-content: center;
      gap: 16px;
      width: 100%;
      max-width: 584px;
      padding-top: 8px;
    }

    .shortcut {
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: 8px;
      padding: 8px;
      border-radius: 8px;
      text-decoration: none;
      color: #202124;
      cursor: pointer;
      transition: background 0.15s ease;
      width: 80px;
    }

    .shortcut:hover {
      background: #f1f3f4;
    }

    .shortcut-icon {
      width: 48px;
      height: 48px;
      border-radius: 50%;
      display: flex;
      align-items: center;
      justify-content: center;
      font-size: 20px;
      font-weight: 700;
      color: #fff;
      transition: transform 0.15s ease;
    }

    .shortcut:hover .shortcut-icon { transform: scale(1.08); }

    .shortcut-label {
      font-size: 12px;
      font-weight: 400;
      text-align: center;
      white-space: nowrap;
      overflow: hidden;
      text-overflow: ellipsis;
      max-width: 72px;
      color: #5f6368;
    }

    .footer {
      position: fixed;
      bottom: 16px;
      font-size: 11px;
      color: #9aa0a6;
      letter-spacing: 0.3px;
    }
  </style>
</head>
<body>
  <div class="container">
    <div class="search-wrapper">
      <input type="text" class="search-bar" id="searchInput"
        placeholder="Search with DuckDuckGo or enter a URL..." autocomplete="off" autofocus />
      <svg class="search-icon" width="20" height="20" viewBox="0 0 24 24" fill="none"
        stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <circle cx="11" cy="11" r="8"></circle>
        <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
      </svg>
    </div>

    <div class="shortcuts">
      <a class="shortcut" href="https://duckduckgo.com">
        <div class="shortcut-icon" style="background: #DE5833;">D</div>
        <span class="shortcut-label">DuckDuckGo</span>
      </a>
      <a class="shortcut" href="https://www.wikipedia.org">
        <div class="shortcut-icon" style="background: #636466;">W</div>
        <span class="shortcut-label">Wikipedia</span>
      </a>
      <a class="shortcut" href="https://github.com">
        <div class="shortcut-icon" style="background: #24292e;">
          <svg width="24" height="24" viewBox="0 0 24 24" fill="white"><path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0 0 24 12c0-6.63-5.37-12-12-12z"/></svg>
        </div>
        <span class="shortcut-label">GitHub</span>
      </a>
      <a class="shortcut" href="https://www.reddit.com">
        <div class="shortcut-icon" style="background: #FF4500;">R</div>
        <span class="shortcut-label">Reddit</span>
      </a>
      <a class="shortcut" href="https://www.youtube.com">
        <div class="shortcut-icon" style="background: #FF0000;">Y</div>
        <span class="shortcut-label">YouTube</span>
      </a>
      <a class="shortcut" href="https://news.ycombinator.com">
        <div class="shortcut-icon" style="background: #FF6600;">H</div>
        <span class="shortcut-label">HN</span>
      </a>
      <a class="shortcut" href="https://stackoverflow.com">
        <div class="shortcut-icon" style="background: #F48024;">S</div>
        <span class="shortcut-label">Stack Overflow</span>
      </a>
      <a class="shortcut" href="https://mastodon.social">
        <div class="shortcut-icon" style="background: #6364FF;">M</div>
        <span class="shortcut-label">Mastodon</span>
      </a>
    </div>
  </div>
  <div class="footer">Eesha Browser v${APP_VERSION}</div>
  <script>
    (function() {
      var searchInput = document.getElementById('searchInput');
      searchInput.addEventListener('keydown', function(e) {
        if (e.key === 'Enter') {
          var query = searchInput.value.trim();
          if (query) {
            if (/^https?:\\/\\//i.test(query)) {
              window.eesha.navigate(query);
            } else if (/^[a-zA-Z0-9-]+\\.[a-zA-Z]{2,}/.test(query)) {
              window.eesha.navigate('https://' + query);
            } else {
              window.eesha.navigate('${SEARCH_ENGINE}' + encodeURIComponent(query));
            }
          }
        }
      });
      document.querySelectorAll('.shortcut').forEach(function(link) {
        link.addEventListener('click', function(e) {
          e.preventDefault();
          window.eesha.navigate(this.href);
        });
      });
    })();
  </script>
</body>
</html>`;
}

// ─── Settings Page HTML ──────────────────────────────────────────────────────
function getSettingsHTML() {
  return `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Settings - Eesha</title>
  <style>
    *, *::before, *::after { margin: 0; padding: 0; box-sizing: border-box; }
    body {
      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
      background: #1a1a2e;
      color: #e0e0e0;
      min-height: 100vh;
      padding: 40px;
    }
    .container { max-width: 640px; margin: 0 auto; }
    h1 { font-size: 28px; color: #fff; margin-bottom: 32px; }
    h2 { font-size: 18px; color: #e94560; margin: 24px 0 12px; }
    .card {
      background: rgba(22, 33, 62, 0.6);
      border: 1px solid #2a2a4a;
      border-radius: 12px;
      padding: 20px;
      margin-bottom: 16px;
    }
    .setting-row {
      display: flex;
      justify-content: space-between;
      align-items: center;
      padding: 12px 0;
      border-bottom: 1px solid #2a2a4a;
    }
    .setting-row:last-child { border-bottom: none; }
    .setting-label { font-size: 14px; }
    .setting-desc { font-size: 12px; color: #5a5a7a; margin-top: 4px; }
    .btn {
      padding: 8px 16px;
      border-radius: 8px;
      border: none;
      cursor: pointer;
      font-size: 13px;
      font-weight: 500;
      transition: all 0.2s ease;
    }
    .btn-primary { background: #e94560; color: #fff; }
    .btn-primary:hover { background: #c73652; }
    .btn-danger { background: transparent; color: #e94560; border: 1px solid #e94560; }
    .btn-danger:hover { background: rgba(233, 69, 96, 0.1); }
    .info { font-size: 13px; color: #8888aa; }
    .version { color: #e94560; font-weight: 600; }
  </style>
</head>
<body>
  <div class="container">
    <h1>⚙ Settings</h1>

    <h2>Privacy</h2>
    <div class="card">
      <div class="setting-row">
        <div>
          <div class="setting-label">Ad & Tracker Blocking</div>
          <div class="setting-desc">Block ads, trackers, and fingerprinting scripts</div>
        </div>
        <span class="info">Active ✓</span>
      </div>
      <div class="setting-row">
        <div>
          <div class="setting-label">Search Engine</div>
          <div class="setting-desc">Default search provider</div>
        </div>
        <span class="info">DuckDuckGo</span>
      </div>
    </div>

    <h2>Data</h2>
    <div class="card">
      <div class="setting-row">
        <div>
          <div class="setting-label">Clear Browsing History</div>
          <div class="setting-desc">Remove all browsing history entries</div>
        </div>
        <button class="btn btn-danger" onclick="window.eesha.clearHistory()">Clear History</button>
      </div>
    </div>

    <h2>About</h2>
    <div class="card">
      <div class="setting-row">
        <div>
          <div class="setting-label">Eesha Browser</div>
          <div class="setting-desc">A privacy-first browser powered by Chromium</div>
        </div>
        <span class="version">v${APP_VERSION}</span>
      </div>
      <div class="setting-row">
        <div>
          <div class="setting-label">Engine</div>
          <div class="setting-desc">Chromium via Electron</div>
        </div>
        <span class="info">100% web compatible</span>
      </div>
    </div>
  </div>
</body>
</html>`;
}

// ─── Ad/Tracker Blocking Setup ───────────────────────────────────────────────
function setupAdBlocking(ses) {
  ses.webRequest.onBeforeRequest((details, callback) => {
    const url = new URL(details.url);
    const hostname = url.hostname;
    
    // Check if the domain matches any blocked domain
    const blocked = BLOCKED_DOMAINS.some(domain => {
      return hostname === domain || hostname.endsWith('.' + domain);
    });

    if (blocked) {
      callback({ cancel: true });
      return;
    }

    callback({ cancel: false });
  });
}

// ─── Splash Screen ──────────────────────────────────────────────────────────
let splashWindow = null;

function createSplashScreen() {
  const logoImage = path.join(RESOURCES_DIR, 'eesha-logo.png');
  
  splashWindow = new BrowserWindow({
    width: 480,
    height: 360,
    transparent: true,
    frame: false,
    resizable: false,
    center: true,
    show: false,
    skipTaskbar: true,
    alwaysOnTop: true,
  });

  const splashHtml = `<!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <style>
    * { margin: 0; padding: 0; box-sizing: border-box; }
    html, body {
      width: 100%; height: 100%;
      overflow: hidden;
      background: #1a1a2e;
      display: flex;
      align-items: center;
      justify-content: center;
    }
    .splash-content {
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: center;
      gap: 20px;
      z-index: 1;
      width: 100%;
      height: 100%;
      padding: 40px;
    }
    .splash-logo {
      max-width: 70%;
      max-height: 50%;
      object-fit: contain;
    }
    .splash-title {
      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
      font-size: 24px;
      font-weight: 700;
      color: #ffffff;
      letter-spacing: -0.5px;
    }
    .splash-tagline {
      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
      font-size: 10px;
      color: #8888aa;
      letter-spacing: 3px;
      text-transform: uppercase;
    }
    .splash-loader {
      margin-top: 8px;
      width: 120px;
      height: 3px;
      background: rgba(233, 69, 96, 0.2);
      border-radius: 2px;
      overflow: hidden;
    }
    .splash-loader-bar {
      height: 100%;
      width: 0%;
      background: #e94560;
      border-radius: 2px;
      animation: splash-load 2s ease-in-out forwards;
    }
    @keyframes splash-load {
      0% { width: 0%; }
      40% { width: 60%; }
      80% { width: 90%; }
      100% { width: 100%; }
    }
  </style>
</head>
<body>
  <div class="splash-content">
    <img class="splash-logo" src="${logoImage.replace(/\\/g, '/')}" alt="Eesha"
      onerror="this.style.display='none'">
    <div class="splash-title">Eesha</div>
    <div class="splash-tagline">Fast. Private. Yours.</div>
    <div class="splash-loader"><div class="splash-loader-bar"></div></div>
  </div>
</body>
</html>`;

  splashWindow.loadURL(`data:text/html;charset=utf-8,${encodeURIComponent(splashHtml)}`);
  splashWindow.once('ready-to-show', () => {
    splashWindow.show();
  });
}

function closeSplashScreen() {
  if (splashWindow && !splashWindow.isDestroyed()) {
    splashWindow.close();
    splashWindow = null;
  }
}

// ─── Create Main Window ──────────────────────────────────────────────────────
function createWindow() {
  // Load window icon
  const iconPath = path.join(ICONS_DIR, 'icon512x512.png');
  const windowIcon = fs.existsSync(iconPath) ? nativeImage.createFromPath(iconPath) : undefined;

  mainWindow = new BaseWindow({
    width: 1400,
    height: 900,
    minWidth: 800,
    minHeight: 600,
    title: 'Eesha',
    titleBarStyle: 'hidden',
    titleBarOverlay: false,
    backgroundColor: '#1a1a2e',
    show: false,
    icon: windowIcon,
    trafficLightPosition: { x: 12, y: 15 },
  });

  // Create the browser chrome view (tab bar, URL bar, navigation)
  chromeView = new WebContentsView({
    webPreferences: {
      preload: path.join(__dirname, 'preload.js'),
      contextIsolation: true,
      nodeIntegration: false,
      sandbox: false,
    },
  });

  mainWindow.contentView.addChildView(chromeView);

  // Load the browser chrome UI
  chromeView.webContents.loadFile(path.join(__dirname, 'renderer', 'index.html'));

  // Set up ad blocking on the default session
  setupAdBlocking(session.defaultSession);

  // Set custom user agent
  session.defaultSession.setUserAgent(
    session.defaultSession.getUserAgent().replace(/Electron\/\S+/, USER_AGENT_SUFFIX)
  );

  // Handle eesha:// protocol
  session.defaultSession.protocol.registerStringProtocol('eesha', (request, callback) => {
    const url = request.url;
    if (url === 'eesha://newtab' || url === 'eesha://newtab/') {
      callback({
        data: getNewTabHTML(),
        mimeType: 'text/html',
        charset: 'utf-8',
      });
    } else if (url === 'eesha://settings' || url === 'eesha://settings/') {
      callback({
        data: getSettingsHTML(),
        mimeType: 'text/html',
        charset: 'utf-8',
      });
    } else if (url.startsWith('eesha://resources/')) {
      // Serve resource files (icons, logos, splash images)
      const resourcePath = url.replace('eesha://resources/', '');
      const fullPath = path.join(SHARED_DIR, resourcePath);
      try {
        const data = fs.readFileSync(fullPath);
        const ext = path.extname(fullPath).toLowerCase();
        const mimeMap = {
          '.png': 'image/png',
          '.jpg': 'image/jpeg',
          '.jpeg': 'image/jpeg',
          '.svg': 'image/svg+xml',
          '.ico': 'image/x-icon',
          '.gif': 'image/gif',
          '.webp': 'image/webp',
        };
        callback({
          data: data,
          mimeType: mimeMap[ext] || 'application/octet-stream',
        });
      } catch (e) {
        callback({ data: 'Not Found', mimeType: 'text/plain' });
      }
    } else {
      callback({
        data: '<html><body><h1>Unknown eesha:// page</h1></body></html>',
        mimeType: 'text/html',
      });
    }
  });

  // ─── Create first tab ──────────────────────────────────────────────────
  createTab(NEWTAB_URL);

  // ─── Window layout ─────────────────────────────────────────────────────
  mainWindow.on('resize', () => {
    layoutViews();
  });

  // Show window when ready and close splash screen
  mainWindow.once('ready-to-show', () => {
    layoutViews();
    // Close splash and show main window
    setTimeout(() => {
      closeSplashScreen();
      mainWindow.show();
    }, 1500); // Give splash screen time to show
  });

  // Initial layout
  setTimeout(layoutViews, 100);

  // ─── Handle new window requests from web content ───────────────────────
  mainWindow.webContents.on('new-window', (event) => {
    event.preventDefault();
  });

  // ─── Clean up on close ─────────────────────────────────────────────────
  mainWindow.on('closed', () => {
    tabs = [];
    activeTabId = null;
    mainWindow = null;
    chromeView = null;
  });
}

// ─── Tab Operations ──────────────────────────────────────────────────────────
function createTab(url = NEWTAB_URL) {
  const id = createTabId();

  const contentView = new WebContentsView({
    webPreferences: {
      contextIsolation: true,
      nodeIntegration: false,
      sandbox: true,
    },
  });

  const tab = {
    id,
    url,
    title: 'New Tab',
    loading: false,
    canGoBack: false,
    canGoForward: false,
    contentView,
  };

  tabs.push(tab);

  // Add to window
  if (mainWindow && mainWindow.contentView) {
    mainWindow.contentView.addChildView(contentView);
  }

  // Load URL
  if (url === NEWTAB_URL) {
    contentView.webContents.loadURL('eesha://newtab');
  } else if (url === SETTINGS_URL) {
    contentView.webContents.loadURL('eesha://settings');
  } else {
    contentView.webContents.loadURL(url);
  }

  // ─── WebContents event handlers ────────────────────────────────────────
  contentView.webContents.on('did-navigate', (event, navUrl) => {
    tab.url = navUrl;
    tab.canGoBack = contentView.webContents.canGoBack();
    tab.canGoForward = contentView.webContents.canGoForward();
    addHistoryEntry(navUrl, tab.title);
    notifyChrome('url-change', { tabId: id, url: navUrl, canGoBack: tab.canGoBack, canGoForward: tab.canGoForward });
  });

  contentView.webContents.on('did-navigate-in-page', (event, navUrl) => {
    tab.url = navUrl;
    tab.canGoBack = contentView.webContents.canGoBack();
    tab.canGoForward = contentView.webContents.canGoForward();
    notifyChrome('url-change', { tabId: id, url: navUrl, canGoBack: tab.canGoBack, canGoForward: tab.canGoForward });
  });

  contentView.webContents.on('page-title-updated', (event, title) => {
    tab.title = title;
    notifyChrome('title-change', { tabId: id, title });
  });

  contentView.webContents.on('did-start-loading', () => {
    tab.loading = true;
    notifyChrome('loading-state', { tabId: id, loading: true });
  });

  contentView.webContents.on('did-stop-loading', () => {
    tab.loading = false;
    notifyChrome('loading-state', { tabId: id, loading: false });
  });

  contentView.webContents.on('did-fail-load', (event, errorCode, errorDesc, failedUrl) => {
    tab.loading = false;
    notifyChrome('loading-state', { tabId: id, loading: false });
  });

  // Handle new window requests (e.g., target="_blank")
  contentView.webContents.setWindowOpenHandler(({ url: openUrl }) => {
    createTab(openUrl);
    return { action: 'deny' };
  });

  // Activate this tab
  switchToTab(id);

  // Notify chrome about new tab
  notifyChrome('tab-created', {
    tabId: id,
    url,
    title: tab.title,
    active: true,
  });

  return id;
}

function switchToTab(id) {
  const previousTabId = activeTabId;
  activeTabId = id;

  // Remove all tab content views from display, then add the active one
  tabs.forEach(tab => {
    if (tab.contentView && mainWindow && mainWindow.contentView) {
      try {
        mainWindow.contentView.removeChildView(tab.contentView);
      } catch (e) {
        // View may not be a child, that's fine
      }
    }
  });

  // Add the active tab's content view
  const activeTab = getTabById(id);
  if (activeTab && activeTab.contentView && mainWindow && mainWindow.contentView) {
    mainWindow.contentView.addChildView(activeTab.contentView);
  }

  // Re-layout
  layoutViews();

  // Notify chrome
  notifyChrome('tab-switched', { tabId: id, previousTabId });
}

function closeTab(id) {
  const idx = tabs.findIndex(t => t.id === id);
  if (idx === -1) return;

  const tab = tabs[idx];

  // Remove content view
  if (tab.contentView && mainWindow && mainWindow.contentView) {
    try {
      mainWindow.contentView.removeChildView(tab.contentView);
    } catch (e) {
      // View may not be a child
    }
    tab.contentView.webContents.close();
  }

  tabs.splice(idx, 1);

  // If we closed the active tab, switch to another
  if (activeTabId === id) {
    if (tabs.length > 0) {
      const newIdx = Math.min(idx, tabs.length - 1);
      switchToTab(tabs[newIdx].id);
    } else {
      // No tabs left - create a new one
      createTab(NEWTAB_URL);
    }
  }

  notifyChrome('tab-closed', { tabId: id });
}

function layoutViews() {
  if (!mainWindow) return;
  const [width, height] = mainWindow.getContentSize();
  if (!width || !height) return;

  // Chrome view at top
  if (chromeView) {
    chromeView.setBounds({ x: 0, y: 0, width, height: CHROME_HEIGHT });
  }

  // Active tab content view below chrome
  const activeTab = getTabById(activeTabId);
  if (activeTab && activeTab.contentView) {
    activeTab.contentView.setBounds({
      x: 0,
      y: CHROME_HEIGHT,
      width,
      height: height - CHROME_HEIGHT,
    });
  }
}

function notifyChrome(channel, data) {
  if (chromeView && chromeView.webContents && !chromeView.webContents.isDestroyed()) {
    chromeView.webContents.send(channel, data);
  }
}

// ─── IPC Handlers ─────────────────────────────────────────────────────────────
function setupIPC() {
  // Navigation
  ipcMain.handle('navigate', (_, url) => {
    const tab = getTabById(activeTabId);
    if (!tab) return;
    tab.url = url;
    if (url === NEWTAB_URL) {
      tab.contentView.webContents.loadURL('eesha://newtab');
    } else if (url === SETTINGS_URL) {
      tab.contentView.webContents.loadURL('eesha://settings');
    } else {
      tab.contentView.webContents.loadURL(url);
    }
  });

  ipcMain.handle('go-back', () => {
    const tab = getTabById(activeTabId);
    if (tab && tab.contentView.webContents.canGoBack()) {
      tab.contentView.webContents.goBack();
    }
  });

  ipcMain.handle('go-forward', () => {
    const tab = getTabById(activeTabId);
    if (tab && tab.contentView.webContents.canGoForward()) {
      tab.contentView.webContents.goForward();
    }
  });

  ipcMain.handle('reload', () => {
    const tab = getTabById(activeTabId);
    if (tab) {
      tab.contentView.webContents.reload();
    }
  });

  ipcMain.handle('force-reload', () => {
    const tab = getTabById(activeTabId);
    if (tab) {
      tab.contentView.webContents.reloadIgnoringCache();
    }
  });

  // Tab management
  ipcMain.handle('create-tab', (_, url) => {
    return createTab(url || NEWTAB_URL);
  });

  ipcMain.handle('switch-tab', (_, tabId) => {
    switchToTab(tabId);
  });

  ipcMain.handle('close-tab', (_, tabId) => {
    closeTab(tabId);
  });

  ipcMain.handle('get-tabs', () => {
    return tabs.map(t => ({
      id: t.id,
      url: t.url,
      title: t.title,
      loading: t.loading,
      active: t.id === activeTabId,
    }));
  });

  // Bookmarks
  ipcMain.handle('get-bookmarks', () => bookmarks);

  ipcMain.handle('add-bookmark', (_, url, title) => {
    addBookmark(url, title);
    return bookmarks;
  });

  ipcMain.handle('remove-bookmark', (_, url) => {
    removeBookmark(url);
    return bookmarks;
  });

  ipcMain.handle('is-bookmarked', (_, url) => {
    return isBookmarked(url);
  });

  // History
  ipcMain.handle('get-history', () => history);

  ipcMain.handle('clear-history', () => {
    history = [];
    saveHistory();
    return true;
  });

  // Focus URL bar
  ipcMain.handle('focus-url-bar', () => {
    notifyChrome('focus-url-bar', {});
  });

  // Get current tab info
  ipcMain.handle('get-active-tab', () => {
    const tab = getTabById(activeTabId);
    if (!tab) return null;
    return {
      id: tab.id,
      url: tab.url,
      title: tab.title,
      loading: tab.loading,
      canGoBack: tab.contentView.webContents.canGoBack(),
      canGoForward: tab.contentView.webContents.canGoForward(),
    };
  });
}

// ─── Application Menu ────────────────────────────────────────────────────────
function setupMenu() {
  const template = [
    {
      label: 'Eesha',
      submenu: [
        { label: 'About Eesha', click: () => createTab(SETTINGS_URL) },
        { type: 'separator' },
        { label: 'Preferences', accelerator: 'CmdOrCtrl+,', click: () => createTab(SETTINGS_URL) },
        { type: 'separator' },
        { role: 'quit' },
      ],
    },
    {
      label: 'File',
      submenu: [
        { label: 'New Tab', accelerator: 'CmdOrCtrl+T', click: () => createTab(NEWTAB_URL) },
        { label: 'Close Tab', accelerator: 'CmdOrCtrl+W', click: () => { if (activeTabId) closeTab(activeTabId); } },
        { type: 'separator' },
        { label: 'New Window', accelerator: 'CmdOrCtrl+N', click: () => createWindow() },
      ],
    },
    {
      label: 'Edit',
      submenu: [
        { role: 'undo' },
        { role: 'redo' },
        { type: 'separator' },
        { role: 'cut' },
        { role: 'copy' },
        { role: 'paste' },
        { role: 'selectAll' },
      ],
    },
    {
      label: 'View',
      submenu: [
        { label: 'Reload', accelerator: 'CmdOrCtrl+R', click: () => { const tab = getTabById(activeTabId); if (tab) tab.contentView.webContents.reload(); } },
        { label: 'Force Reload', accelerator: 'CmdOrCtrl+Shift+R', click: () => { const tab = getTabById(activeTabId); if (tab) tab.contentView.webContents.reloadIgnoringCache(); } },
        { type: 'separator' },
        { label: 'Toggle DevTools', accelerator: 'F12', click: () => { const tab = getTabById(activeTabId); if (tab) tab.contentView.webContents.toggleDevTools(); } },
        { type: 'separator' },
        { label: 'Focus URL Bar', accelerator: 'CmdOrCtrl+L', click: () => notifyChrome('focus-url-bar', {}) },
      ],
    },
    {
      label: 'History',
      submenu: [
        { label: 'Back', accelerator: 'Alt+Left', click: () => { const tab = getTabById(activeTabId); if (tab && tab.contentView.webContents.canGoBack()) tab.contentView.webContents.goBack(); } },
        { label: 'Forward', accelerator: 'Alt+Right', click: () => { const tab = getTabById(activeTabId); if (tab && tab.contentView.webContents.canGoForward()) tab.contentView.webContents.goForward(); } },
        { type: 'separator' },
        { label: 'Show All History', click: () => createTab(SETTINGS_URL) },
      ],
    },
    {
      label: 'Bookmarks',
      submenu: [
        { label: 'Bookmark This Page', accelerator: 'CmdOrCtrl+D', click: () => {
          const tab = getTabById(activeTabId);
          if (tab) {
            addBookmark(tab.url, tab.title);
            notifyChrome('bookmarks-updated', { bookmarks });
          }
        }},
        { type: 'separator' },
        ...bookmarks.slice(0, 10).map(b => ({
          label: b.title || b.url,
          click: () => createTab(b.url),
        })),
      ],
    },
    {
      label: 'Window',
      submenu: [
        { role: 'minimize' },
        { role: 'zoom' },
        { type: 'separator' },
        { role: 'front' },
      ],
    },
  ];

  const menu = Menu.buildFromTemplate(template);
  Menu.setApplicationMenu(menu);
}

// ─── App Lifecycle ────────────────────────────────────────────────────────────
app.whenReady().then(() => {
  // Set up IPC handlers
  setupIPC();

  // Set up application menu
  setupMenu();

  // Show splash screen first
  createSplashScreen();

  // Create the main window (it will remain hidden until ready)
  createWindow();

  // macOS: recreate window when clicking dock icon
  app.on('activate', () => {
    if (BaseWindow.getAllWindows().length === 0) {
      createWindow();
    }
  });
});

// Quit when all windows are closed (except on macOS)
app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit();
  }
});

// ─── Keyboard Shortcuts ──────────────────────────────────────────────────────
app.on('browser-window-focus', () => {
  // These are handled by the menu accelerators above, but we register them
  // here as well for robustness
});

// Security: Prevent new window creation via web content
app.on('web-contents-created', (event, contents) => {
  contents.on('new-window', (event) => {
    event.preventDefault();
  });
});
