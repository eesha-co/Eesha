// EeshaApp - iOS entry point
// This is a placeholder for the iOS native shell.
// The actual Servo/Eesha rendering is handled by the Rust library.

import UIKit

@main
class AppDelegate: UIResponder, UIApplicationDelegate {
    var window: UIWindow?

    func application(
        _ application: UIApplication,
        didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]?
    ) -> Bool {
        // The Servo/Eesha engine is initialized from the Rust side
        // via the objc2 bindings defined in Cargo.toml
        return true
    }
}
