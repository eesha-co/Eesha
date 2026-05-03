# Eesha Browser

A privacy-focused web browser powered by the **Blink** rendering engine (Chromium).

## Architecture

| Platform | Engine | Technology |
|----------|--------|------------|
| Desktop (Linux/Windows/macOS) | **Blink** | CEF (Chromium Embedded Framework) |
| Android | **Blink** | Android WebView (Chromium) |
| iOS | **WebKit** | WKWebView (Apple mandate) |

## Features

- 🔒 **Privacy-first**: Built-in ad and tracker blocking
- 🚫 **No Google telemetry**: All Google services removed
- 🔍 **DuckDuckGo**: Default search engine (privacy-focused)
- 🛡️ **HTTPS-only**: Enforces secure connections
- 🎨 **Custom new tab**: Eesha-branded with quick shortcuts
- 📱 **Cross-platform**: Desktop + Android + iOS

## Project Structure

```
├── desktop/          # CEF-based desktop browser (C++)
│   ├── src/          # Source code
│   ├── include/      # Headers
│   ├── cmake/        # CEF download module
│   └── resources/    # Desktop resources
├── android/          # WebView-based Android browser (Kotlin)
│   └── app/          # Android app module
├── ios/              # WKWebView-based iOS browser (Swift)
│   └── Eesha/        # iOS app source
├── shared/           # Shared assets
│   ├── icons/        # Eesha icons and logos
│   └── resources/    # Shared resources
└── .github/
    └── workflows/    # CI/CD pipelines
```

## Building

### Desktop (Linux/macOS/Windows)

```bash
cd desktop
mkdir build && cd build
cmake .. -DCEF_ROOT=/path/to/cef
make -j$(nproc)
```

CEF binary distribution is automatically downloaded if not provided.

### Android

```bash
cd android
./gradlew assembleDebug
```

### iOS

Open `ios/Eesha.xcodeproj` in Xcode and build.

## License

Apache-2.0 OR MIT
