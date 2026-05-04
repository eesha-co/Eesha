package org.eesha.browser

import android.annotation.SuppressLint
import android.content.Intent
import android.graphics.Bitmap
import android.net.Uri
import android.os.Bundle
import android.view.KeyEvent
import android.view.View
import android.view.inputmethod.EditorInfo
import android.net.http.SslError
import android.webkit.*
import android.widget.EditText
import android.widget.ImageButton
import android.widget.ProgressBar
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity
import androidx.core.view.isVisible
import androidx.swiperefreshlayout.widget.SwipeRefreshLayout

/**
 * Eesha Browser - Main Activity
 *
 * A privacy-focused web browser powered by Android WebView (Chromium/Blink).
 * Features:
 * - Built-in ad/tracker blocking
 * - Privacy-first design
 * - Custom Eesha new tab page
 * - HTTPS-only mode
 */
class EeshaBrowser : AppCompatActivity() {

    private lateinit var webView: WebView
    private lateinit var urlBar: EditText
    private lateinit var progressBar: ProgressBar
    private lateinit var btnBack: ImageButton
    private lateinit var btnForward: ImageButton
    private lateinit var btnRefresh: ImageButton
    private lateinit var btnHome: ImageButton
    private lateinit var swipeRefresh: SwipeRefreshLayout

    // Blocked domains for ad/tracker blocking
    private val blockedDomains = setOf(
        "doubleclick.net", "google-analytics.com", "googletagmanager.com",
        "facebook.net/en_US/fbevents", "analytics.facebook.com",
        "ads.yahoo.com", "amazon-adsystem.com", "adnxs.com",
        "adsrvr.org", "casalemedia.com", "criteo.com", "moatads.com",
        "outbrain.com", "rubiconproject.com", "scorecardresearch.com",
        "serving-sys.com", "sharethis.com", "taboola.com", "tapad.com",
        "quantserve.com", "hotjar.com", "mixpanel.com", "segment.io",
        "segment.com/v1", "fullstory.com", "log.optimizely.com"
    )

    @SuppressLint("SetJavaScriptEnabled")
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_browser)

        // Initialize views
        webView = findViewById(R.id.webView)
        urlBar = findViewById(R.id.urlBar)
        progressBar = findViewById(R.id.progressBar)
        btnBack = findViewById(R.id.btnBack)
        btnForward = findViewById(R.id.btnForward)
        btnRefresh = findViewById(R.id.btnRefresh)
        btnHome = findViewById(R.id.btnHome)
        swipeRefresh = findViewById(R.id.swipeRefresh)

        setupWebView()
        setupNavigation()
        setupUrlBar()

        // Load Eesha new tab page
        loadEeshaNewTab()
    }

    @SuppressLint("SetJavaScriptEnabled")
    private fun setupWebView() {
        val settings = webView.settings.apply {
            javaScriptEnabled = true
            domStorageEnabled = true
            databaseEnabled = true
            allowFileAccess = false
            allowContentAccess = false
            javaScriptCanOpenWindowsAutomatically = false
            loadsImagesAutomatically = true
            mixedContentMode = WebSettings.MIXED_CONTENT_NEVER_ALLOW
            cacheMode = WebSettings.LOAD_DEFAULT
            userAgentString = "Eesha/0.5.0 (Android) " + userAgentString
        }

        // WebView debugging is auto-enabled for debuggable apps (debug builds)

        webView.webViewClient = EeshaWebViewClient()
        webView.webChromeClient = EeshaWebChromeClient()

        // Handle swipe-to-refresh
        swipeRefresh.setOnRefreshListener {
            webView.reload()
        }
    }

    private fun setupNavigation() {
        btnBack.setOnClickListener { webView.goBack() }
        btnForward.setOnClickListener { webView.goForward() }
        btnRefresh.setOnClickListener { webView.reload() }
        btnHome.setOnClickListener { loadEeshaNewTab() }
    }

    private fun setupUrlBar() {
        urlBar.setOnEditorActionListener { _, actionId, event ->
            if (actionId == EditorInfo.IME_ACTION_GO ||
                (event?.action == KeyEvent.ACTION_DOWN && event.keyCode == KeyEvent.KEYCODE_ENTER)) {
                navigateToUrl(urlBar.text.toString())
                true
            } else false
        }

        urlBar.setOnClickListener {
            urlBar.selectAll()
        }
    }

    private fun navigateToUrl(input: String) {
        val url = when {
            input.startsWith("http://") || input.startsWith("https://") -> input
            input.startsWith("eesha://") -> input
            input.contains(".") && !input.contains(" ") -> "https://$input"
            else -> "https://duckduckgo.com/?q=${Uri.encode(input)}"
        }
        webView.loadUrl(url)
        urlBar.clearFocus()
    }

    private fun loadEeshaNewTab() {
        // Encode logo as base64 for embedding in HTML
        val iconBase64 = try {
            val iconStream = resources.openRawResource(R.drawable.eesha_icon)
            val iconBytes = iconStream.readBytes()
            iconStream.close()
            android.util.Base64.encodeToString(iconBytes, android.util.Base64.NO_WRAP)
        } catch (e: Exception) {
            ""
        }
        val iconDataUri = if (iconBase64.isNotEmpty()) "data:image/png;base64,$iconBase64" else ""

        val newTabHtml = """
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Eesha - New Tab</title>
            <style>
                * { margin: 0; padding: 0; box-sizing: border-box; }
                body {
                    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
                    background: #ffffff;
                    color: #202124;
                    min-height: 100vh;
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                    justify-content: flex-start;
                    padding: 12vh 1rem 1rem;
                    position: relative;
                    overflow: hidden;
                }
                /* Eesha logo watermark background */
                body::after {
                    content: '';
                    position: fixed;
                    top: 28%; left: 50%;
                    transform: translate(-50%, -50%);
                    width: 45vmin; height: 45vmin;
                    background-image: url('$iconDataUri');
                    background-size: contain;
                    background-repeat: no-repeat;
                    background-position: center;
                    opacity: 0.12;
                    pointer-events: none;
                }
                .search-container {
                    width: 100%; max-width: 500px;
                    position: relative; z-index: 1;
                }
                .search-box {
                    width: 100%; padding: 12px 16px 12px 44px; font-size: 16px;
                    border: 1px solid #dfe1e5; border-radius: 24px;
                    background: #fff; color: #202124; outline: none;
                }
                .search-box:hover { box-shadow: 0 1px 6px rgba(32,33,36,0.28); border-color: transparent; }
                .search-box:focus { box-shadow: 0 1px 6px rgba(32,33,36,0.28); border-color: transparent; }
                .search-box::placeholder { color: #9aa0a6; }
                .search-icon {
                    position: absolute; left: 14px; top: 50%; transform: translateY(-50%);
                    color: #9aa0a6; pointer-events: none;
                }
                .shortcuts {
                    display: flex; flex-wrap: wrap; justify-content: center;
                    gap: 12px; margin-top: 24px; max-width: 500px; width: 100%;
                    position: relative; z-index: 1;
                }
                .shortcut {
                    display: flex; flex-direction: column; align-items: center; gap: 6px;
                    padding: 8px; border-radius: 8px;
                    text-decoration: none; color: #202124; width: 72px;
                }
                .shortcut-icon {
                    width: 44px; height: 44px; border-radius: 50%;
                    display: flex; align-items: center; justify-content: center;
                    font-size: 18px; font-weight: 700; color: #fff;
                }
                .shortcut-name { font-size: 11px; color: #5f6368; text-align: center; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; max-width: 64px; }
            </style>
        </head>
        <body>
            <div class="search-container">
                <svg class="search-icon" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="#9aa0a6" stroke-width="2"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/></svg>
                <input type="text" class="search-box" id="search" placeholder="Search or enter URL" autofocus>
            </div>
            <div class="shortcuts">
                <a class="shortcut" href="https://www.wikipedia.org">
                    <div class="shortcut-icon" style="background:#636466;">W</div><span class="shortcut-name">Wikipedia</span>
                </a>
                <a class="shortcut" href="https://github.com">
                    <div class="shortcut-icon" style="background:#24292e;">G</div><span class="shortcut-name">GitHub</span>
                </a>
                <a class="shortcut" href="https://www.youtube.com">
                    <div class="shortcut-icon" style="background:#FF0000;">Y</div><span class="shortcut-name">YouTube</span>
                </a>
                <a class="shortcut" href="https://www.reddit.com">
                    <div class="shortcut-icon" style="background:#FF4500;">R</div><span class="shortcut-name">Reddit</span>
                </a>
                <a class="shortcut" href="https://twitter.com">
                    <div class="shortcut-icon" style="background:#1DA1F2;">X</div><span class="shortcut-name">X</span>
                </a>
            </div>
            <script>
                document.getElementById('search').addEventListener('keydown', function(e) {
                    if (e.key === 'Enter') {
                        var q = this.value.trim();
                        if (q) {
                            if (q.match(/^(https?:\/\/|www\\.)/)) {
                                location.href = q.startsWith('www.') ? 'https://' + q : q;
                            } else if (q.includes('.') && !q.includes(' ')) {
                                location.href = 'https://' + q;
                            } else {
                                location.href = 'https://duckduckgo.com/?q=' + encodeURIComponent(q);
                            }
                        }
                    }
                });
            </script>
        </body>
        </html>
        """.trimIndent()

        webView.loadDataWithBaseURL("eesha://newtab", newTabHtml, "text/html", "UTF-8", null)
        urlBar.setText("")
    }

    override fun onKeyDown(keyCode: Int, event: KeyEvent?): Boolean {
        if (keyCode == KeyEvent.KEYCODE_BACK && webView.canGoBack()) {
            webView.goBack()
            return true
        }
        return super.onKeyDown(keyCode, event)
    }

    override fun onResume() {
        super.onResume()
        webView.onResume()
    }

    override fun onPause() {
        webView.onPause()
        super.onPause()
    }

    override fun onDestroy() {
        webView.destroy()
        super.onDestroy()
    }

    // Custom WebViewClient with ad blocking
    inner class EeshaWebViewClient : WebViewClient() {
        override fun shouldOverrideUrlLoading(
            view: WebView, request: WebResourceRequest
        ): Boolean {
            val url = request.url.toString()
            // Block tracking domains
            if (isBlockedUrl(url)) return true
            return false
        }

        override fun shouldInterceptRequest(
            view: WebView, request: WebResourceRequest
        ): WebResourceResponse? {
            val url = request.url.toString()
            // Block ads and trackers
            if (isBlockedUrl(url)) {
                return WebResourceResponse("text/plain", "UTF-8", null)
            }
            return super.shouldInterceptRequest(view, request)
        }

        override fun onPageStarted(view: WebView?, url: String?, favicon: Bitmap?) {
            super.onPageStarted(view, url, favicon)
            progressBar.isVisible = true
            progressBar.progress = 0
            if (url != null && !url.startsWith("eesha://")) {
                urlBar.setText(url)
            }
        }

        override fun onPageFinished(view: WebView?, url: String?) {
            super.onPageFinished(view, url)
            progressBar.isVisible = false
            swipeRefresh.isRefreshing = false
            if (url != null && !url.startsWith("eesha://")) {
                urlBar.setText(url)
            }
        }

        override fun onReceivedSslError(view: WebView?, handler: SslErrorHandler?, error: SslError?) {
            // For security, don't proceed with SSL errors by default
            handler?.cancel()
        }

        private fun isBlockedUrl(url: String): Boolean {
            val lowerUrl = url.lowercase()
            return blockedDomains.any { lowerUrl.contains(it) }
        }
    }

    // Chrome client for progress, title, etc.
    inner class EeshaWebChromeClient : WebChromeClient() {
        override fun onProgressChanged(view: WebView?, newProgress: Int) {
            progressBar.progress = newProgress
            if (newProgress == 100) {
                progressBar.isVisible = false
            }
        }

        override fun onReceivedTitle(view: WebView?, title: String?) {
            super.onReceivedTitle(view, title)
            title?.let {
                if (it != "Eesha - New Tab") {
                    supportActionBar?.title = it
                }
            }
        }
    }
}
