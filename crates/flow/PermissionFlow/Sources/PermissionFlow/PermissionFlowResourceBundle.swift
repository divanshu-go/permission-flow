#if os(macOS)
import Foundation

/// Anchor class used by `Bundle(for:)` to locate the binary that this module
/// was linked into. This is more reliable than `Bundle.module` in non-standard
/// host layouts (e.g. Tauri apps where the resource bundle lives in
/// `Contents/Resources/` instead of next to `Bundle.main.bundleURL`).
@available(macOS 13.0, *)
private final class PermissionFlowResourceAnchor {}

@available(macOS 13.0, *)
extension Foundation.Bundle {
    /// Locates the `PermissionFlow_PermissionFlow.bundle` resource bundle
    /// across the layouts we ship into.
    ///
    /// Why this exists:
    /// SwiftPM auto-generates a `Bundle.module` accessor that searches exactly
    /// two paths — `Bundle.main.bundleURL/PermissionFlow_PermissionFlow.bundle`
    /// and a hard-coded developer-machine build path — and `fatalError`s when
    /// neither exists. That accessor cannot find the bundle when this package
    /// is linked into a non-SPM host (Tauri, Electron, etc.) that places the
    /// resource bundle in the standard macOS `.app/Contents/Resources/`
    /// location, so any first access to a localized string crashes the host.
    ///
    /// `Bundle.permissionFlow` searches a layered set of plausible locations
    /// and degrades gracefully to `Bundle.main` if none match, so a missing
    /// resource bundle yields untranslated default strings instead of a hard
    /// crash.
    ///
    /// Implementation: backed by a `static let` lazy initializer so the
    /// lookup runs exactly once per process. `Bundle` is a reference type and
    /// `Sendable`, so the result is concurrency-safe under Swift 6 strict
    /// checking without needing actor isolation.
    static var permissionFlow: Bundle { PermissionFlowBundleCache.resolved }
}

@available(macOS 13.0, *)
private enum PermissionFlowBundleCache {
    /// Resolved exactly once at first access. Swift runtime guarantees a
    /// thread-safe one-shot initialization for `static let`.
    static let resolved: Bundle = resolve()

    private static func resolve() -> Bundle {
        let bundleName = "PermissionFlow_PermissionFlow.bundle"

        // 1) Bundle(for:) — finds the binary that this Swift module was linked
        // into. For SPM static-lib + swift-rs hosts, the accompanying resource
        // bundle is typically next to that binary; for dynamic frameworks it's
        // the framework bundle's own `Resources/`. If the candidate contains
        // `.lproj` directories we treat it as the resource bundle and return
        // it directly.
        let anchorBundle = Bundle(for: PermissionFlowResourceAnchor.self)
        if isResourceBundle(anchorBundle) {
            return anchorBundle
        }
        // Some hosts put the resource bundle next to the anchor's bundle URL
        // rather than inside it.
        if let candidate = Bundle(url: anchorBundle.bundleURL.appendingPathComponent(bundleName)),
           isResourceBundle(candidate) {
            return candidate
        }

        // 2) SPM default layout: <main.bundleURL>/<bundleName>. This is what
        // SwiftPM's generated `Bundle.module` accessor checks. Kept here for
        // parity with non-Tauri SPM hosts.
        if let candidate = Bundle(url: Bundle.main.bundleURL.appendingPathComponent(bundleName)),
           isResourceBundle(candidate) {
            return candidate
        }

        // 3) Standard macOS .app layout: <main>/Contents/Resources/<bundleName>.
        // This is where Tauri (and most non-SPM hosts) place bundled resources
        // declared in their packaging manifest. THIS is the path the original
        // `Bundle.module` accessor failed to check, and is the entire reason
        // this resolver exists.
        if let resourceURL = Bundle.main.resourceURL,
           let candidate = Bundle(url: resourceURL.appendingPathComponent(bundleName)),
           isResourceBundle(candidate) {
            return candidate
        }

        // 4) Last-ditch fallbacks for unusual host layouts (frameworks, plug-in
        // bundles, etc.). These mostly exist so we never reach the fatalError
        // branch; an empty Bundle.main still answers `localizedString` with
        // the default value, which is the behavior `PermissionFlowLocalizer`
        // expects on cache miss.
        for path in candidateSearchPaths(bundleName: bundleName) {
            if let candidate = Bundle(path: path), isResourceBundle(candidate) {
                return candidate
            }
        }

        // Degrade gracefully. `Bundle.main.localizedString(forKey:value:table:)`
        // returns the `value` argument when the key is not found, so callers
        // that pass an English default still display correctly. The original
        // SwiftPM accessor would `Swift.fatalError` here.
        return Bundle.main
    }

    /// Treats a candidate `Bundle` as the PermissionFlow resource bundle if it
    /// exposes at least one localization. The shipped bundle always contains
    /// `en.lproj` (and others), so this is a reliable "this is the right one"
    /// probe that avoids depending on any specific string key existing.
    private static func isResourceBundle(_ bundle: Bundle?) -> Bool {
        guard let bundle else { return false }
        return bundle.localizations.isEmpty == false
    }

    /// Builds a small set of plausible filesystem paths the resource bundle may
    /// live at relative to the main bundle, for hosts that don't fit the
    /// patterns above.
    private static func candidateSearchPaths(bundleName: String) -> [String] {
        var paths: [String] = []
        let main = Bundle.main.bundleURL
        // <main>/Contents/Frameworks/<bundleName>
        paths.append(main.appendingPathComponent("Contents/Frameworks/\(bundleName)").path)
        // <main>/Contents/PlugIns/<bundleName>
        paths.append(main.appendingPathComponent("Contents/PlugIns/\(bundleName)").path)
        // <main parent>/<bundleName> — for executables that aren't inside a .app
        paths.append(main.deletingLastPathComponent().appendingPathComponent(bundleName).path)
        return paths
    }
}
#endif
