# Eesha Browser

A privacy-focused web browser powered by **Blink/Chromium** — built for the future of an independent web.

> **Strategy**: Blink NOW → Ladybird later. Ship a competitive browser today, build toward engine independence tomorrow.

## Architecture

| Platform | Engine | Technology |
|----------|--------|------------|
| Desktop (Linux/Windows/macOS) | **Blink** | Electron (Chromium 134+) |
| Android | **Blink** | Android WebView (Chromium) |
| iOS | **WebKit** | WKWebView (Apple mandate) |

## Features

- 🔒 **Privacy-first**: Built-in ad and tracker blocking (50+ domains)
- 🚫 **No Google telemetry**: All Google tracking services removed
- 🔍 **DuckDuckGo**: Default search engine (privacy-focused)
- 🛡️ **HTTPS upgrade**: Enforces secure connections
- 🎨 **Custom new tab**: Eesha-branded with search and quick shortcuts
- 📑 **Multi-tab browsing**: Full tab management on desktop
- ⭐ **Bookmarks**: Save and organize your favorite sites
- 📜 **History**: Browsing history with search
- 📱 **Cross-platform**: Desktop + Android + iOS
- 🌙 **Dark theme**: Beautiful Eesha dark UI

## Project Structure

```
├── desktop/          # Electron-based desktop browser
│   ├── main.js       # Electron main process
│   ├── preload.js    # Secure IPC bridge
│   └── renderer/     # Browser UI (HTML/CSS/JS)
├── android/          # WebView-based Android browser (Kotlin)
│   └── app/          # Android app module
├── ios/              # WKWebView-based iOS browser (Swift)
│   └── Eesha/        # iOS app source
├── shared/           # Shared assets
│   ├── icons/        # Eesha icons and logos
│   └── resources/    # Shared HTML pages
└── .github/
    └── workflows/    # CI/CD pipelines
```

## Installation

### Desktop

**Linux:**
1. Download the `.AppImage` from [Releases](https://github.com/eesha-co/Eesha/releases)
2. `chmod +x Eesha-*.AppImage`
3. `./Eesha-*.AppImage`

**Windows:**
1. Download the `.exe` installer from [Releases](https://github.com/eesha-co/Eesha/releases)
2. Run the installer

**macOS:**
1. Download the `.dmg` from [Releases](https://github.com/eesha-co/Eesha/releases)
2. Open the DMG, drag Eesha to Applications

### Android

1. Download the `.apk` from [Releases](https://github.com/eesha-co/Eesha/releases)
2. Enable "Install from unknown sources" in your device settings
3. Install the APK

### iOS

> iOS requires building from source with Xcode and an Apple Developer account.

1. Clone the repository
2. `cd ios && brew install xcodegen && ./create_xcodeproj.sh`
3. Open the generated `.xcodeproj` in Xcode
4. Select your team and build

## Building from Source

### Desktop

```bash
cd desktop
npm install
npm start          # Development
npm run build:all  # Production builds
```

### Android

```bash
cd android
./gradlew assembleDebug
```

### iOS

```bash
cd ios
brew install xcodegen
./create_xcodeproj.sh
# Then open in Xcode
```

## Roadmap

| Phase | Timeline | Goal |
|-------|----------|------|
| **v0.2** (now) | Week 1 | Working browser on all platforms |
| **v0.3** | Month 1 | Polish UI, extension support |
| **v0.5** | Month 3 | Privacy features, ad blocker v2 |
| **v1.0** | Month 6 | Stable, daily-drivable browser |
| **v2.0** | Year 1+ | Evaluate Ladybird integration |

## License

Apache-2.0 OR MIT
