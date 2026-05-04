import UIKit
import WebKit

/**
 * Eesha Browser - Main Browser View Controller
 *
 * A privacy-focused web browser powered by WKWebView (WebKit).
 * On iOS, Apple requires all browsers to use WebKit — even Chrome uses it on iOS.
 *
 * Features:
 * - Built-in ad/tracker blocking via content blocker
 * - Privacy-first design
 * - Custom Eesha new tab page
 * - HTTPS upgrade
 */

class BrowserViewController: UIViewController, WKNavigationDelegate, WKUIDelegate, UITextFieldDelegate {

    private var webView: WKWebView!
    private var urlBar: UITextField!
    private var progressBar: UIProgressView!
    private var btnBack: UIButton!
    private var btnForward: UIButton!
    private var btnRefresh: UIButton!
    private var btnHome: UIButton!
    private var navigationBar: UIView!

    // Blocked domains for ad/tracker blocking
    private let blockedDomains = [
        "doubleclick.net", "google-analytics.com", "googletagmanager.com",
        "facebook.net/en_US/fbevents", "analytics.facebook.com",
        "ads.yahoo.com", "amazon-adsystem.com", "adnxs.com",
        "adsrvr.org", "casalemedia.com", "criteo.com", "moatads.com",
        "outbrain.com", "rubiconproject.com", "scorecardresearch.com",
        "serving-sys.com", "sharethis.com", "taboola.com", "tapad.com",
        "quantserve.com", "hotjar.com", "mixpanel.com"
    ]

    override func viewDidLoad() {
        super.viewDidLoad()
        setupUI()
        setupWebView()
        loadEeshaNewTab()
    }

    private func setupUI() {
        view.backgroundColor = UIColor(red: 0.102, green: 0.102, blue: 0.180, alpha: 1.0)

        // Navigation bar
        navigationBar = UIView()
        navigationBar.backgroundColor = UIColor(red: 0.102, green: 0.102, blue: 0.180, alpha: 1.0)
        navigationBar.translatesAutoresizingMaskIntoConstraints = false
        view.addSubview(navigationBar)

        // Back button
        btnBack = UIButton(type: .system)
        btnBack.setTitle("◀", for: .normal)
        btnBack.tintColor = .white
        btnBack.addTarget(self, action: #selector(goBack), for: .touchUpInside)
        btnBack.translatesAutoresizingMaskIntoConstraints = false

        // Forward button
        btnForward = UIButton(type: .system)
        btnForward.setTitle("▶", for: .normal)
        btnForward.tintColor = .white
        btnForward.addTarget(self, action: #selector(goForward), for: .touchUpInside)
        btnForward.translatesAutoresizingMaskIntoConstraints = false

        // URL bar
        urlBar = UITextField()
        urlBar.backgroundColor = UIColor(red: 0.188, green: 0.169, blue: 0.388, alpha: 1.0)
        urlBar.textColor = .white
        urlBar.attributedPlaceholder = NSAttributedString(
            string: "Search or enter URL",
            attributes: [.foregroundColor: UIColor(white: 1, alpha: 0.4)]
        )
        urlBar.font = UIFont.systemFont(ofSize: 14)
        urlBar.layer.cornerRadius = 18
        urlBar.clipsToBounds = true
        urlBar.returnKeyType = .go
        urlBar.autocorrectionType = .no
        urlBar.autocapitalizationType = .none
        urlBar.spellCheckingType = .no
        urlBar.keyboardType = .webSearch
        urlBar.delegate = self
        urlBar.leftView = UIView(frame: CGRect(x: 0, y: 0, width: 12, height: 36))
        urlBar.leftViewMode = .always
        urlBar.translatesAutoresizingMaskIntoConstraints = false

        // Refresh button
        btnRefresh = UIButton(type: .system)
        btnRefresh.setTitle("↻", for: .normal)
        btnRefresh.tintColor = .white
        btnRefresh.addTarget(self, action: #selector(refresh), for: .touchUpInside)
        btnRefresh.translatesAutoresizingMaskIntoConstraints = false

        // Home button
        btnHome = UIButton(type: .system)
        btnHome.setTitle("⌂", for: .normal)
        btnHome.tintColor = .white
        btnHome.addTarget(self, action: #selector(goHome), for: .touchUpInside)
        btnHome.translatesAutoresizingMaskIntoConstraints = false

        navigationBar.addSubview(btnBack)
        navigationBar.addSubview(btnForward)
        navigationBar.addSubview(urlBar)
        navigationBar.addSubview(btnRefresh)
        navigationBar.addSubview(btnHome)

        // Progress bar
        progressBar = UIProgressView(progressViewStyle: .bar)
        progressBar.progressTintColor = UIColor(red: 0.914, green: 0.271, blue: 0.376, alpha: 1.0)
        progressBar.trackTintColor = .clear
        progressBar.translatesAutoresizingMaskIntoConstraints = false
        view.addSubview(progressBar)

        // Layout constraints
        NSLayoutConstraint.activate([
            navigationBar.topAnchor.constraint(equalTo: view.safeAreaLayoutGuide.topAnchor),
            navigationBar.leadingAnchor.constraint(equalTo: view.leadingAnchor),
            navigationBar.trailingAnchor.constraint(equalTo: view.trailingAnchor),
            navigationBar.heightAnchor.constraint(equalToConstant: 48),

            btnBack.leadingAnchor.constraint(equalTo: navigationBar.leadingAnchor, constant: 8),
            btnBack.centerYAnchor.constraint(equalTo: navigationBar.centerYAnchor),
            btnBack.widthAnchor.constraint(equalToConstant: 36),

            btnForward.leadingAnchor.constraint(equalTo: btnBack.trailingAnchor, constant: 4),
            btnForward.centerYAnchor.constraint(equalTo: navigationBar.centerYAnchor),
            btnForward.widthAnchor.constraint(equalToConstant: 36),

            urlBar.leadingAnchor.constraint(equalTo: btnForward.trailingAnchor, constant: 4),
            urlBar.centerYAnchor.constraint(equalTo: navigationBar.centerYAnchor),
            urlBar.heightAnchor.constraint(equalToConstant: 36),

            btnRefresh.leadingAnchor.constraint(equalTo: urlBar.trailingAnchor, constant: 4),
            btnRefresh.centerYAnchor.constraint(equalTo: navigationBar.centerYAnchor),
            btnRefresh.widthAnchor.constraint(equalToConstant: 36),

            btnHome.leadingAnchor.constraint(equalTo: btnRefresh.trailingAnchor, constant: 4),
            btnHome.centerYAnchor.constraint(equalTo: navigationBar.centerYAnchor),
            btnHome.widthAnchor.constraint(equalToConstant: 36),
            btnHome.trailingAnchor.constraint(equalTo: navigationBar.trailingAnchor, constant: -8),

            progressBar.topAnchor.constraint(equalTo: navigationBar.bottomAnchor),
            progressBar.leadingAnchor.constraint(equalTo: view.leadingAnchor),
            progressBar.trailingAnchor.constraint(equalTo: view.trailingAnchor),
            progressBar.heightAnchor.constraint(equalToConstant: 2),
        ])
    }

    private func setupWebView() {
        let config = WKWebViewConfiguration()
        config.websiteDataStore = .default()
        config.preferences.javaScriptCanOpenWindowsAutomatically = false

        webView = WKWebView(frame: .zero, configuration: config)
        webView.navigationDelegate = self
        webView.uiDelegate = self
        webView.translatesAutoresizingMaskIntoConstraints = false
        webView.isOpaque = false
        webView.backgroundColor = UIColor(red: 0.059, green: 0.047, blue: 0.161, alpha: 1.0)
        view.addSubview(webView)

        NSLayoutConstraint.activate([
            webView.topAnchor.constraint(equalTo: progressBar.bottomAnchor),
            webView.leadingAnchor.constraint(equalTo: view.leadingAnchor),
            webView.trailingAnchor.constraint(equalTo: view.trailingAnchor),
            webView.bottomAnchor.constraint(equalTo: view.bottomAnchor),
        ])

        // Observe loading progress
        webView.addObserver(self, forKeyPath: "estimatedProgress", options: .new, context: nil)
        webView.addObserver(self, forKeyPath: "title", options: .new, context: nil)
    }

    override func observeValue(forKeyPath keyPath: String?, of object: Any?,
                                change: [NSKeyValueChangeKey: Any]?, context: UnsafeMutableRawPointer?) {
        if keyPath == "estimatedProgress" {
            progressBar.progress = Float(webView.estimatedProgress)
            progressBar.isHidden = webView.estimatedProgress >= 1.0
        } else if keyPath == "title" {
            if let title = webView.title, !title.isEmpty {
                navigationItem.title = title
            }
        }
    }

    // MARK: - Navigation

    func textFieldShouldReturn(_ textField: UITextField) -> Bool {
        textField.resignFirstResponder()
        if let input = textField.text?.trimmingCharacters(in: .whitespacesAndNewlines), !input.isEmpty {
            navigateToUrl(input)
        }
        return true
    }

    private func navigateToUrl(_ input: String) {
        let url: String
        if input.hasPrefix("http://") || input.hasPrefix("https://") {
            url = input
        } else if input.contains(".") && !input.contains(" ") {
            url = "https://\(input)"
        } else {
            url = "https://duckduckgo.com/?q=\(input.addingPercentEncoding(withAllowedCharacters: .urlQueryAllowed) ?? input)"
        }
        if let requestUrl = URL(string: url) {
            webView.load(URLRequest(url: requestUrl))
        }
    }

    @objc private func goBack() { webView.goBack() }
    @objc private func goForward() { webView.goForward() }
    @objc private func refresh() { webView.reload() }
    @objc private func goHome() { loadEeshaNewTab() }

    private func loadEeshaNewTab() {
        // Load FULL-RESOLUTION logo (677x369 eesha-logo.png) as base64 for watermark
        var logoDataUri = ""
        if let logoImage = UIImage(named: "EeshaLogo"),
           let pngData = logoImage.pngData() {
            logoDataUri = "data:image/png;base64,\(pngData.base64EncodedString())"
        } else if let iconUrl = Bundle.main.url(forResource: "eesha-logo", withExtension: "png", subdirectory: "Eesha"),
                  let iconData = try? Data(contentsOf: iconUrl) {
            logoDataUri = "data:image/png;base64,\(iconData.base64EncodedString())"
        }

        let html = """
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
                    color: #202124; min-height: 100vh;
                    display: flex; flex-direction: column; align-items: center;
                    justify-content: flex-start; padding: 12vh 1rem 1rem;
                    position: relative; overflow: hidden;
                }
                /* Eesha logo watermark background - full res, visible */
                body::after {
                    content: '';
                    position: fixed;
                    top: 25%; left: 50%;
                    transform: translate(-50%, -50%);
                    width: 70vmin; height: 38vmin;
                    background-image: url('\(logoDataUri)');
                    background-size: contain;
                    background-repeat: no-repeat;
                    background-position: center;
                    opacity: 0.18;
                    pointer-events: none;
                    z-index: 0;
                }
                .search-container {
                    width: 100%; max-width: 500px;
                    position: relative; z-index: 1;
                }
                .search-box {
                    width: 100%; padding: 14px 16px 14px 46px; font-size: 16px;
                    border: 1px solid #dfe1e5; border-radius: 24px;
                    background: #fff; color: #202124; outline: none;
                    transition: box-shadow 0.2s, border-color 0.2s;
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
                    gap: 16px; margin-top: 28px; max-width: 500px; width: 100%;
                    position: relative; z-index: 1;
                }
                .shortcut {
                    display: flex; flex-direction: column; align-items: center; gap: 8px;
                    padding: 8px; border-radius: 12px;
                    text-decoration: none; color: #202124; width: 76px;
                    transition: background 0.15s;
                }
                .shortcut:active { background: #f1f3f4; }
                .shortcut-icon {
                    width: 48px; height: 48px; border-radius: 50%;
                    display: flex; align-items: center; justify-content: center;
                    font-size: 20px; font-weight: 700; color: #fff;
                    box-shadow: 0 1px 3px rgba(0,0,0,0.12);
                }
                .shortcut-name {
                    font-size: 11px; color: #5f6368; text-align: center;
                    white-space: nowrap; overflow: hidden; text-overflow: ellipsis; max-width: 68px;
                }
                .footer {
                    position: fixed; bottom: 12px; left: 0; right: 0;
                    text-align: center; font-size: 11px; color: #9aa0a6;
                    z-index: 1; pointer-events: none;
                }
            </style>
        </head>
        <body>
            <div class="search-container">
                <svg class="search-icon" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#9aa0a6" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/></svg>
                <input type="text" class="search-box" id="search" placeholder="Search with DuckDuckGo or enter URL" autofocus>
            </div>
            <div class="shortcuts">
                <a class="shortcut" href="https://duckduckgo.com">
                    <div class="shortcut-icon" style="background:#DE5833;">D</div><span class="shortcut-name">DuckDuckGo</span>
                </a>
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
                <a class="shortcut" href="https://news.ycombinator.com">
                    <div class="shortcut-icon" style="background:#FF6600;">H</div><span class="shortcut-name">HN</span>
                </a>
                <a class="shortcut" href="https://stackoverflow.com">
                    <div class="shortcut-icon" style="background:#F48024;">S</div><span class="shortcut-name">Stack Overflow</span>
                </a>
            </div>
            <div class="footer">Eesha Browser</div>
            <script>
                document.getElementById('search').addEventListener('keydown', function(e) {
                    if (e.key === 'Enter') {
                        var q = this.value.trim();
                        if (q) {
                            if (q.match(/^(https?:\\/\\/|www\\.)/)) {
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
        """
        webView.loadHTMLString(html, baseURL: URL(string: "eesha://newtab"))
        urlBar.text = ""
    }

    // MARK: - WKNavigationDelegate

    func webView(_ webView: WKWebView, decidePolicyFor navigationAction: WKNavigationAction,
                 decisionHandler: @escaping (WKNavigationActionPolicy) -> Void) {
        guard let url = navigationAction.request.url else {
            decisionHandler(.allow)
            return
        }

        // Block tracking domains
        let urlStr = url.absoluteString.lowercased()
        if blockedDomains.contains(where: { urlStr.contains($0) }) {
            decisionHandler(.cancel)
            return
        }

        decisionHandler(.allow)
    }

    func webView(_ webView: WKWebView, didStartProvisionalNavigation navigation: WKNavigation!) {
        if let url = webView.url?.absoluteString, !url.hasPrefix("eesha://") {
            urlBar.text = url
        }
    }

    func webView(_ webView: WKWebView, didFinish navigation: WKNavigation!) {
        if let url = webView.url?.absoluteString, !url.hasPrefix("eesha://") {
            urlBar.text = url
        }
    }

    deinit {
        webView.removeObserver(self, forKeyPath: "estimatedProgress")
        webView.removeObserver(self, forKeyPath: "title")
    }
}
