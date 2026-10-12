# Дайджест мобильной разработки: Релиз Flutter 3.47 & Dart 3.13, 5 лет Compose, безопасность и утилиты

> Подробный дайджест мобильной разработки: релизы Flutter 3.47 и Dart 3.13, 5 лет Jetpack Compose, правила безопасности Semgrep, гайды Apple по промо в App Store и open-source утилиты.

- **URL:** https://eagle2000.space/til/mobile/mobile-dev-digest-week-33/
- **Date:** 2026-08-17

- **Tags:** ios, android, swift, kotlin, flutter, dart, security, mobile

---
В этом выпуске собрал разбор ключевых новостей мобильной разработки: крупный релиз Flutter 3.47 и Dart 3.13, пятилетие Jetpack Compose, единый набор правил безопасности Semgrep, гайдлайны Apple по промо в App Store и простой opensource видеоредактор.

## Релизы и анонсы: Flutter 3.47 & Dart 3.13

### Flutter 3.47
Вышла новая версия фреймворка Flutter! Разработчики подготовили несколько важнейших архитектурных улучшений:

- **Вынос `material_ui` и `cupertino_ui` в отдельные пакеты:** Графические пакеты Material и Cupertino теперь вынесены из ядра фреймворка в отдельные независимые пакеты на pub.dev: [material_ui](https://pub.dev/packages/material_ui) и [cupertino_ui](https://pub.dev/packages/cupertino_ui). Это позволяет обновлять элементы дизайна независимо от релизов SDK и использовать только нужные UI-модули:

```dart
// ❌ Старый монолитный импорт через Flutter SDK
import 'package:flutter/material.dart';

// ✅ В Flutter 3.47: независимые пакеты из pub.dev
import 'package:material_ui/material_ui.dart';
import 'package:cupertino_ui/cupertino_ui.dart';
```

#### Пошаговая миграция на независимые пакеты:
1. **Автоматическое обновление импортов через Dart Fix:**
   ```bash
   dart fix --apply --code=migrate_design_widgets
   ```
   Команда автоматически добавит `material_ui` / `cupertino_ui` в `pubspec.yaml` и перепишет старые импорты `package:flutter/material.dart` на `package:material_ui/material_ui.dart`.

2. **Мост совместимости для легаси-пакетов (Compatibility Bridge):**
   Если сторонние зависимости все еще импортируют старый `package:flutter/material.dart`, оборачивайте дерево виджетов в мост `MaterialUiCompatibilityBridge` (или `CupertinoUiCompatibilityBridge`), чтобы стили и темы `ThemeData` пробрасывались корректно:

   ```dart
   import 'package:material_ui/material_ui.dart';

   MaterialApp(
     builder: (context, child) {
       return MaterialUiCompatibilityBridge(child: child!);
     },
     home: const HomeScreen(),
   );
   ```

- **Impeller по умолчанию для Desktop-платформ:** Графический рендерер нового поколения Impeller официально включен по умолчанию на macOS, Windows и Linux. Приложения получают плавные 60+ FPS и улучшенное аппаратное ускорение прямо из коробки.
- **Поддержка Flavors и мультиоконности на Desktop:** Разработчики настольных приложений на Flutter получили полноценную поддержку Flavors (конфигураций сборки dev/staging/prod) для десктопа и экспериментальный API для работы с несколькими окнами.
- **Подготовка к осенним обновлениям Apple:** В движок заранее внесены необходимые изменения для стабильной работы под предстоящими осенними релизами iOS и macOS.
- 📌 **Ссылки:** [Официальный анонс Flutter 3.47](https://flutter.dev/blog/whats-new-in-flutter-3-47) | [Пакет material_ui](https://pub.dev/packages/material_ui) | [Пакет cupertino_ui](https://pub.dev/packages/cupertino_ui) | [Breaking Changes](https://docs.flutter.dev/release/breaking-changes#released-in-flutter-3-47)

### Dart 3.13
Вместе с Flutter обновился и язык Dart:

- **Primary Constructors стали стабильными:** Синтаксис первичных конструкторов (Primary Constructors) официально объявлен стабильным ([документация](https://dart.dev/language/primary-constructors)). Объявление полей класса и их инициализация теперь сокращаются до лаконичной записи прямо в заголовке класса без избыточных конструкторов и двойных присвоений:

```dart
// ❌ Раньше в Dart: явные поля, объявленный конструктор и список инициализации
class Point {
  final int x;
  final int y;

  Point(this.x, this.y) : assert(x >= 0 && y >= 0);
}

// ✅ В Dart 3.13 (Primary Constructors): первичный конструктор прямо в заголовке класса
class Point(var int x, var int y) {
  this : assert(x >= 0 && y >= 0) {
    print('Point initialized at ($x, $y)');
  }
}

// Первичный конструктор с именованными параметрами:
class User({required var String name});

// Константный первичный конструктор со списком инициализации (this : z = x + y):
class const ConstPoint(final int x, final int y) {
  final int z;
  this : z = x + y;
}
```

- **WASM Deferred Loading (Экспериментально в Dart 3.13):** В компиляторе Dart WebAssembly появилась отложенная загрузка модулей. По умолчанию сборка не использует deferred loading — фича включается флагом `--enable-deferred-loading`:

```bash
$ dart compile wasm --enable-deferred-loading bin/main.dart
```

При этом загрузчик приложения на стороне JS перегружает коллбэк `loadDeferredModules` для подгрузки байтов модулей по требованию:

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

- **Tree-shaking нативных библиотек:** Появилась новая аннотация компилятора для отсечения (tree-shaking) неиспользуемого нативного кода C/C++ при сборке через FFI. Также runtime-движок получил очередные оптимизации вызовов функций и работы с памятью.
- 📌 **Ссылка:** [Официальный анонс Dart 3.13](https://dart.dev/blog/announcing-dart-3-13) | [Гайд по Primary Constructors](https://dart.dev/language/primary-constructors)

---

## Android & Jetpack Compose

- [5 лет с релиза Jetpack Compose 1.0 🎉](https://android-developers.googleblog.com/2026/07/five-years-of-jetpack-compose.html) — Google подводит итоги пяти лет с момента первого стабильного релиза Jetpack Compose. Из экспериментального фреймворка Compose превратился в стандарт современной Android-разработки. В статье команда делится ретроспективой архитектурных решений, эволюцией компилятора и планами на будущее.
- [Stop Writing Compose Like It’s 2024–10 Recent Updates Every Android Developer Should Know](https://proandroiddev.com/stop-writing-compose-like-its-2024-10-recent-updates-every-android-developer-should-know-302c23b61e36) — отличный разбор 10 актуальных изменений в Jetpack Compose. Автор показывает, от каких устаревших привычек 2024 года (вроде неоптимального использования `remember` или старых шаблонов работы со State) пора отказаться ради производительности UI.

---

## iOS & App Store

- [Рекомендации Apple по созданию хэдеров и визуальных материалов для продвижения приложений в App Store](https://developer.apple.com/news/?id=kug6m2ea) — компания Apple выкатила официальные гайдлайны и примеры по созданию графических хэдеров для продвижения приложений в App Store (дополнение к новой функции продвижения, анонсированной на WWDC). В руководстве акцентируется внимание на правильном выборе визуальных элементов, позиционировании и контексте, которые напрямую влияют на конверсию показов в установки.

---

## Безопасность мобильных приложений

- [semgrep-mobile-rules](https://github.com/Sib-Coder/semgrep-mobile-rules) — открытый единый репозиторий правил Semgrep для статического анализа безопасности Android, iOS, React Native/Expo и Flutter-приложений. Проект создали разработчики из Sib-Coder, объединив в одном месте лучшие правила из OWASP MASTG и MobSF, избавившись от багов и ограничений классического `mobsfscan`.
- [Злоупотребление возможностями ContactDirectory](https://fi5t.xyz/posts/contactdirectory-abuse/) — подробный разбор атак и методов злоупотребления системным разрешением `ContactDirectory` в Android. Автор показывает, как недобросовестные приложения могут извлекать личные данные пользователей и обходить механизмы приватности.
- [TrustYFox: путь от пет‑проекта до LLM‑инструмента для поиска уязвимостей](https://habr.com/ru/amp/publications/1013714/) — статья о создании Open Source инструмента TrustYFox, использующего нейросетевых агентов для автоматизированного аудита безопасности мобильной кодовой базы.

---

## Бонус: Интересный Open Source проект

- **[open-video-editor](https://github.com/devhyper/open-video-editor)** — Наткнулся на интересный открытый видеоредактор для Android. Проект написан на Kotlin с использованием **Media3** и **Jetpack Compose**. Умеет кадрировать видео, менять разрешение, поворачивать и масштабировать ролики, а также поддерживает многозадачность и локализацию через Weblate.

