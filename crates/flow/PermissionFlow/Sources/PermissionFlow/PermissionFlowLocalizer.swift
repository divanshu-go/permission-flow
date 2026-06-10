#if os(macOS)
import Foundation

@available(macOS 13.0, *)
enum PermissionFlowLocalizer {
    /// Resolves a localized string from the best matching `.lproj` bundle for
    /// the injected locale. This keeps all custom locale selection in one
    /// place, while still letting the rest of the UI use plain localization
    /// keys and format strings.
    ///
    /// Uses `Bundle.permissionFlow` (see `PermissionFlowResourceBundle.swift`)
    /// instead of the SwiftPM-generated `Bundle.module` accessor, because the
    /// latter `fatalError`s when this package is linked into hosts that don't
    /// place the resource bundle at `Bundle.main.bundleURL/...` (e.g. Tauri
    /// apps, which use the standard `.app/Contents/Resources/` layout).
    static func string(
        _ key: String,
        defaultValue: String,
        localeIdentifier: String?
    ) -> String {
        localizedBundle(for: localeIdentifier)?
            .localizedString(forKey: key, value: defaultValue, table: nil)
            ?? Bundle.permissionFlow.localizedString(forKey: key, value: defaultValue, table: nil)
    }

    private static func localizedBundle(for localeIdentifier: String?) -> Bundle? {
        guard let localeIdentifier, localeIdentifier.isEmpty == false else {
            return nil
        }

        let preferences = localizationPreferences(for: localeIdentifier)
        guard let localization = Bundle.preferredLocalizations(
            from: Bundle.permissionFlow.localizations,
            forPreferences: preferences
        ).first,
        let path = Bundle.permissionFlow.path(forResource: localization, ofType: "lproj") else {
            return nil
        }

        return Bundle(path: path)
    }

    private static func localizationPreferences(for localeIdentifier: String) -> [String] {
        let normalizedIdentifier = localeIdentifier.replacingOccurrences(of: "_", with: "-")
        let locale = Locale(identifier: normalizedIdentifier)

        var preferences = [normalizedIdentifier]
        if let identifier = locale.language.languageCode?.identifier {
            if let script = locale.language.script?.identifier {
                preferences.append("\(identifier)-\(script)")
            }
            preferences.append(identifier)
        }

        return Array(NSOrderedSet(array: preferences)) as? [String] ?? preferences
    }
}
#endif
