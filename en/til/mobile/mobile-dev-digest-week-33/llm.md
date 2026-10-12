# Mobile development digest: Flutter 3.47 & Dart 3.13, 5 years of Compose & tools

> In-depth mobile development digest: Flutter 3.47 and Dart 3.13 releases, 5 years of Jetpack Compose, Apple promo guidelines

- **URL:** https://eagle2000.space/en/til/mobile/mobile-dev-digest-week-33/
- **Date:** 2026-08-17

- **Tags:** ios, android, swift, kotlin, flutter, dart, security, mobile

---
In this edition of our mobile development digest, I cover major updates across the mobile engineering landscape: the releases of Flutter 3.47 and Dart 3.13, celebrating 5 years of Jetpack Compose, Apple App Store header guidelines, and useful open-source tools.

## Releases & announcements: Flutter 3.47 & Dart 3.13

### Flutter 3.47
Flutter 3.47 brings key architectural modularity and performance enhancements:

- **Decoupling Material & Cupertino into standalone packages:** The Material and Cupertino widget libraries have been decoupled from the core framework into standalone packages on pub.dev: [material_ui](https://pub.dev/packages/material_ui) and [cupertino_ui](https://pub.dev/packages/cupertino_ui). This enables faster UI updates independent of SDK release cycles and reduces core engine footprint:

```dart
// ❌ Legacy monolithic SDK import
import 'package:flutter/material.dart';

// ✅ In Flutter 3.47: standalone packages from pub.dev
import 'package:material_ui/material_ui.dart';
import 'package:cupertino_ui/cupertino_ui.dart';
```

#### Step-by-step package migration:
1. **Automated import migration via Dart Fix:**
   ```bash
   dart fix --apply --code=migrate_design_widgets
   ```
   This command automatically adds `material_ui` / `cupertino_ui` to `pubspec.yaml` and migrates legacy `package:flutter/material.dart` imports to `package:material_ui/material_ui.dart`.

2. **Compatibility Bridge for legacy dependencies:**
   If third-party packages or subtrees still depend on `package:flutter/material.dart`, wrap your root app or subtree using `MaterialUiCompatibilityBridge` (or `CupertinoUiCompatibilityBridge`) so `ThemeData` and localizations resolve seamlessly:

   ```dart
   import 'package:material_ui/material_ui.dart';

   MaterialApp(
     builder: (context, child) {
       return MaterialUiCompatibilityBridge(child: child!);
     },
     home: const HomeScreen(),
   );
   ```

- **Impeller renderer enabled by default on Desktop:** The next-generation Impeller rendering engine—designed to eliminate runtime shader compilation jank—is now officially enabled by default for Desktop platforms (macOS, Windows, and Linux).
- **Flavors & Multi-window support for Desktop:** Desktop developers gain official Flutter Flavors support alongside an experimental multi-window API.
- **Apple Autumn Release Preparation:** Pre-emptively updated for smooth compatibility with upcoming autumn iOS and macOS system updates.
- 📌 **Links:** [Official Flutter 3.47 Announcement](https://flutter.dev/blog/whats-new-in-flutter-3-47) | [Package material_ui](https://pub.dev/packages/material_ui) | [Package cupertino_ui](https://pub.dev/packages/cupertino_ui) | [Breaking Changes](https://docs.flutter.dev/release/breaking-changes#released-in-flutter-3-47)

### Dart 3.13
Alongside Flutter, Dart 3.13 introduced significant language and runtime improvements:

- **Stable Primary Constructors:** Primary Constructors syntax is officially stable ([documentation](https://dart.dev/language/primary-constructors)). Class fields and parameter initialization can now be written concisely directly in the class header:

```dart
// ❌ Previously in Dart: explicit fields, constructor declaration, and initializer list
class Point {
  final int x;
  final int y;

  Point(this.x, this.y) : assert(x >= 0 && y >= 0);
}

// ✅ In Dart 3.13 (Primary Constructors): primary constructor directly in the class header
class Point(var int x, var int y) {
  this : assert(x >= 0 && y >= 0) {
    print('Point initialized at ($x, $y)');
  }
}

// Primary constructor with named parameters:
class User({required var String name});

// Constant primary constructor with initializer list (this : z = x + y):
class const ConstPoint(final int x, final int y) {
  final int z;
  this : z = x + y;
}
```

- **WASM Deferred Loading (Experimental in Dart 3.13):** The Dart WebAssembly compiler introduced experimental support for deferred module loading. By default, compiled apps do not use deferred loading—it is enabled using the `--enable-deferred-loading` flag:

```bash
$ dart compile wasm --enable-deferred-loading bin/main.dart
```

Enabling deferred loading requires the JS app loader to supply a `loadDeferredModules` callback to fetch module bytes on demand:

```javascript
const app = await compileStreaming(fetch('main.wasm'));
const instantiatedApp = await app.instantiate({}, {
  loadDeferredModules: (modules, handleModuleByteSource) => {
    return Promise.all(
      modules.map((m) => fetch(m).then((r) => handleModuleByteSource(m, r)))
    );
  },
});
instantiatedApp.invokeMain();
```

- **Native Library Tree-Shaking:** Introduced compiler annotations to automatically tree-shake unused C/C++ native library functions when building via FFI.
- 📌 **Link:** [Official Dart 3.13 Announcement](https://dart.dev/blog/announcing-dart-3-13) | [Primary Constructors Specification Guide](https://dart.dev/language/primary-constructors)

---

## Android & Jetpack Compose

- [Five years of Jetpack Compose 1.0 🎉](https://android-developers.googleblog.com/2026/07/five-years-of-jetpack-compose.html) — Google reflects on five years since the first stable release of Jetpack Compose, tracing its evolution from an experimental declarative framework into the default standard for modern Android UI development.
- [Stop writing Compose like it’s 2024–10 recent updates every Android developer should know](https://proandroiddev.com/stop-writing-compose-like-its-2024-10-recent-updates-every-android-developer-should-know-302c23b61e36) — A comprehensive breakdown of 10 modern Jetpack Compose updates and best practices, helping developers replace outdated 2024 habits for better UI performance and cleaner state handling.

---

## iOS & App Store

- [Apple guidelines for App Store promotional headers](https://developer.apple.com/news/?id=kug6m2ea) — Apple published official guidelines and best practices for creating promotional header assets for App Store features. The document provides visual examples and layout requirements to maximize impression-to-install conversion.

---

## Bonus: Open-source discovery

- **[open-video-editor](https://github.com/devhyper/open-video-editor)** — A handy open-source Android video editor licensed under GPL-3.0 that I recently stumbled upon. Built with **Media3** and **Jetpack Compose**, it supports video trimming, resolution scaling, rotation, and Weblate localization.

