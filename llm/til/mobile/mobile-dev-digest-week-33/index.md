# Дайджест мобильной разработки: Релиз Flutter 3.47 & Dart 3.13, 5 лет Compose, безопасность и утилиты

> Подробный дайджест мобильной разработки: релизы Flutter 3.47 и Dart 3.13, 5 лет Jetpack Compose, правила безопасности Semgrep, гайды Apple по промо в App Store и open-source утилиты.

- **URL:** https://eagle2000.space/til/mobile/mobile-dev-digest-week-33/
- **Date:** 2026-08-17

- **Tags:** ios, android, swift, kotlin, flutter, dart, security, mobile

---

<p>В этом выпуске собрал разбор ключевых новостей мобильной разработки: крупный релиз Flutter 3.47 и Dart 3.13, пятилетие Jetpack Compose, единый набор правил безопасности Semgrep, гайдлайны Apple по промо в App Store и простой opensource видеоредактор.</p>

<h2 class="relative group">Релизы и анонсы: Flutter 3.47 &amp; Dart 3.13
    <div id="релизы-и-анонсы-flutter-347--dart-313" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%80%d0%b5%d0%bb%d0%b8%d0%b7%d1%8b-%d0%b8-%d0%b0%d0%bd%d0%be%d0%bd%d1%81%d1%8b-flutter-347--dart-313" aria-label="Якорь">#</a>
    </span>
    
</h2>

<h3 class="relative group">Flutter 3.47
    <div id="flutter-347" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#flutter-347" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Вышла новая версия фреймворка Flutter! Разработчики подготовили несколько важнейших архитектурных улучшений:</p>
<ul>
<li><strong>Вынос <code>material_ui</code> и <code>cupertino_ui</code> в отдельные пакеты:</strong> Графические пакеты Material и Cupertino теперь вынесены из ядра фреймворка в отдельные независимые пакеты на pub.dev: <a href="https://pub.dev/packages/material_ui"  target="_blank" rel="noreferrer">material_ui</a> и <a href="https://pub.dev/packages/cupertino_ui"  target="_blank" rel="noreferrer">cupertino_ui</a>. Это позволяет обновлять элементы дизайна независимо от релизов SDK и использовать только нужные UI-модули:</li>
</ul>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-dart" data-lang="dart"><span class="line"><span class="cl"><span class="c1">// ❌ Старый монолитный импорт через Flutter SDK
</span></span></span><span class="line"><span class="cl"><span class="k">import</span> <span class="s1">&#39;package:flutter/material.dart&#39;</span><span class="p">;</span>
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl"><span class="c1">// ✅ В Flutter 3.47: независимые пакеты из pub.dev
</span></span></span><span class="line"><span class="cl"><span class="k">import</span> <span class="s1">&#39;package:material_ui/material_ui.dart&#39;</span><span class="p">;</span>
</span></span><span class="line"><span class="cl"><span class="k">import</span> <span class="s1">&#39;package:cupertino_ui/cupertino_ui.dart&#39;</span><span class="p">;</span></span></span></code></pre></div></div>

<h4 class="relative group">Пошаговая миграция на независимые пакеты:
    <div id="пошаговая-миграция-на-независимые-пакеты" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d0%be%d1%88%d0%b0%d0%b3%d0%be%d0%b2%d0%b0%d1%8f-%d0%bc%d0%b8%d0%b3%d1%80%d0%b0%d1%86%d0%b8%d1%8f-%d0%bd%d0%b0-%d0%bd%d0%b5%d0%b7%d0%b0%d0%b2%d0%b8%d1%81%d0%b8%d0%bc%d1%8b%d0%b5-%d0%bf%d0%b0%d0%ba%d0%b5%d1%82%d1%8b" aria-label="Якорь">#</a>
    </span>
    
</h4>
<ol>
<li>
<p><strong>Автоматическое обновление импортов через Dart Fix:</strong></p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-bash" data-lang="bash"><span class="line"><span class="cl">dart fix --apply --code<span class="o">=</span>migrate_design_widgets</span></span></code></pre></div></div>
<p>Команда автоматически добавит <code>material_ui</code> / <code>cupertino_ui</code> в <code>pubspec.yaml</code> и перепишет старые импорты <code>package:flutter/material.dart</code> на <code>package:material_ui/material_ui.dart</code>.</p>
</li>
<li>
<p><strong>Мост совместимости для легаси-пакетов (Compatibility Bridge):</strong>
Если сторонние зависимости все еще импортируют старый <code>package:flutter/material.dart</code>, оборачивайте дерево виджетов в мост <code>MaterialUiCompatibilityBridge</code> (или <code>CupertinoUiCompatibilityBridge</code>), чтобы стили и темы <code>ThemeData</code> пробрасывались корректно:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-dart" data-lang="dart"><span class="line"><span class="cl"><span class="k">import</span> <span class="s1">&#39;package:material_ui/material_ui.dart&#39;</span><span class="p">;</span>
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl"><span class="n">MaterialApp</span><span class="p">(</span>
</span></span><span class="line"><span class="cl">  <span class="nl">builder:</span> <span class="p">(</span><span class="n">context</span><span class="p">,</span> <span class="n">child</span><span class="p">)</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">    <span class="k">return</span> <span class="n">MaterialUiCompatibilityBridge</span><span class="p">(</span><span class="nl">child:</span> <span class="n">child</span><span class="o">!</span><span class="p">);</span>
</span></span><span class="line"><span class="cl">  <span class="p">},</span>
</span></span><span class="line"><span class="cl">  <span class="nl">home:</span> <span class="kd">const</span> <span class="n">HomeScreen</span><span class="p">(),</span>
</span></span><span class="line"><span class="cl"><span class="p">);</span></span></span></code></pre></div></div>
</li>
</ol>
<ul>
<li><strong>Impeller по умолчанию для Desktop-платформ:</strong> Графический рендерер нового поколения Impeller официально включен по умолчанию на macOS, Windows и Linux. Приложения получают плавные 60+ FPS и улучшенное аппаратное ускорение прямо из коробки.</li>
<li><strong>Поддержка Flavors и мультиоконности на Desktop:</strong> Разработчики настольных приложений на Flutter получили полноценную поддержку Flavors (конфигураций сборки dev/staging/prod) для десктопа и экспериментальный API для работы с несколькими окнами.</li>
<li><strong>Подготовка к осенним обновлениям Apple:</strong> В движок заранее внесены необходимые изменения для стабильной работы под предстоящими осенними релизами iOS и macOS.</li>
<li>📌 <strong>Ссылки:</strong> <a href="https://flutter.dev/blog/whats-new-in-flutter-3-47"  target="_blank" rel="noreferrer">Официальный анонс Flutter 3.47</a> | <a href="https://pub.dev/packages/material_ui"  target="_blank" rel="noreferrer">Пакет material_ui</a> | <a href="https://pub.dev/packages/cupertino_ui"  target="_blank" rel="noreferrer">Пакет cupertino_ui</a> | <a href="https://docs.flutter.dev/release/breaking-changes#released-in-flutter-3-47"  target="_blank" rel="noreferrer">Breaking Changes</a></li>
</ul>

<h3 class="relative group">Dart 3.13
    <div id="dart-313" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#dart-313" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Вместе с Flutter обновился и язык Dart:</p>
<ul>
<li><strong>Primary Constructors стали стабильными:</strong> Синтаксис первичных конструкторов (Primary Constructors) официально объявлен стабильным (<a href="https://dart.dev/language/primary-constructors"  target="_blank" rel="noreferrer">документация</a>). Объявление полей класса и их инициализация теперь сокращаются до лаконичной записи прямо в заголовке класса без избыточных конструкторов и двойных присвоений:</li>
</ul>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-dart" data-lang="dart"><span class="line"><span class="cl"><span class="c1">// ❌ Раньше в Dart: явные поля, объявленный конструктор и список инициализации
</span></span></span><span class="line"><span class="cl"><span class="kd">class</span> <span class="nc">Point</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">  <span class="kd">final</span> <span class="kt">int</span> <span class="n">x</span><span class="p">;</span>
</span></span><span class="line"><span class="cl">  <span class="kd">final</span> <span class="kt">int</span> <span class="n">y</span><span class="p">;</span>
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl">  <span class="n">Point</span><span class="p">(</span><span class="k">this</span><span class="p">.</span><span class="n">x</span><span class="p">,</span> <span class="k">this</span><span class="p">.</span><span class="n">y</span><span class="p">)</span> <span class="o">:</span> <span class="k">assert</span><span class="p">(</span><span class="n">x</span> <span class="o">&gt;=</span> <span class="m">0</span> <span class="o">&amp;&amp;</span> <span class="n">y</span> <span class="o">&gt;=</span> <span class="m">0</span><span class="p">);</span>
</span></span><span class="line"><span class="cl"><span class="p">}</span>
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl"><span class="c1">// ✅ В Dart 3.13 (Primary Constructors): первичный конструктор прямо в заголовке класса
</span></span></span><span class="line"><span class="cl"><span class="kd">class</span> <span class="nc">Point</span><span class="p">(</span><span class="kd">var</span> <span class="kt">int</span> <span class="n">x</span><span class="p">,</span> <span class="kd">var</span> <span class="kt">int</span> <span class="n">y</span><span class="p">)</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">  <span class="k">this</span> <span class="o">:</span> <span class="k">assert</span><span class="p">(</span><span class="n">x</span> <span class="o">&gt;=</span> <span class="m">0</span> <span class="o">&amp;&amp;</span> <span class="n">y</span> <span class="o">&gt;=</span> <span class="m">0</span><span class="p">)</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">    <span class="n">print</span><span class="p">(</span><span class="s1">&#39;Point initialized at (</span><span class="si">$</span><span class="n">x</span><span class="s1">, </span><span class="si">$</span><span class="n">y</span><span class="s1">)&#39;</span><span class="p">);</span>
</span></span><span class="line"><span class="cl">  <span class="p">}</span>
</span></span><span class="line"><span class="cl"><span class="p">}</span>
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl"><span class="c1">// Первичный конструктор с именованными параметрами:
</span></span></span><span class="line"><span class="cl"><span class="kd">class</span> <span class="nc">User</span><span class="p">({</span><span class="kd">required</span> <span class="kd">var</span> <span class="kt">String</span> <span class="n">name</span><span class="p">});</span>
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl"><span class="c1">// Константный первичный конструктор со списком инициализации (this : z = x + y):
</span></span></span><span class="line"><span class="cl"><span class="kd">class</span> <span class="nc">const</span> <span class="n">ConstPoint</span><span class="p">(</span><span class="kd">final</span> <span class="kt">int</span> <span class="n">x</span><span class="p">,</span> <span class="kd">final</span> <span class="kt">int</span> <span class="n">y</span><span class="p">)</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">  <span class="kd">final</span> <span class="kt">int</span> <span class="n">z</span><span class="p">;</span>
</span></span><span class="line"><span class="cl">  <span class="k">this</span> <span class="o">:</span> <span class="n">z</span> <span class="o">=</span> <span class="n">x</span> <span class="o">+</span> <span class="n">y</span><span class="p">;</span>
</span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<ul>
<li><strong>WASM Deferred Loading (Экспериментально в Dart 3.13):</strong> В компиляторе Dart WebAssembly появилась отложенная загрузка модулей. По умолчанию сборка не использует deferred loading — фича включается флагом <code>--enable-deferred-loading</code>:</li>
</ul>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-bash" data-lang="bash"><span class="line"><span class="cl">$ dart compile wasm --enable-deferred-loading bin/main.dart</span></span></code></pre></div></div>
<p>При этом загрузчик приложения на стороне JS перегружает коллбэк <code>loadDeferredModules</code> для подгрузки байтов модулей по требованию:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-javascript" data-lang="javascript"><span class="line"><span class="cl"><span class="kr">const</span> <span class="nx">app</span> <span class="o">=</span> <span class="kr">await</span> <span class="nx">compileStreaming</span><span class="p">(</span><span class="nx">fetch</span><span class="p">(</span><span class="s1">&#39;main.wasm&#39;</span><span class="p">));</span>
</span></span><span class="line"><span class="cl"><span class="kr">const</span> <span class="nx">instantiatedApp</span> <span class="o">=</span> <span class="kr">await</span> <span class="nx">app</span><span class="p">.</span><span class="nx">instantiate</span><span class="p">({},</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">  <span class="nx">loadDeferredModules</span><span class="o">:</span> <span class="p">(</span><span class="nx">modules</span><span class="p">,</span> <span class="nx">handleModuleByteSource</span><span class="p">)</span> <span class="p">=&gt;</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">    <span class="k">return</span> <span class="nb">Promise</span><span class="p">.</span><span class="nx">all</span><span class="p">(</span>
</span></span><span class="line"><span class="cl">      <span class="nx">modules</span><span class="p">.</span><span class="nx">map</span><span class="p">((</span><span class="nx">m</span><span class="p">)</span> <span class="p">=&gt;</span> <span class="nx">fetch</span><span class="p">(</span><span class="nx">m</span><span class="p">).</span><span class="nx">then</span><span class="p">((</span><span class="nx">r</span><span class="p">)</span> <span class="p">=&gt;</span> <span class="nx">handleModuleByteSource</span><span class="p">(</span><span class="nx">m</span><span class="p">,</span> <span class="nx">r</span><span class="p">)))</span>
</span></span><span class="line"><span class="cl">    <span class="p">);</span>
</span></span><span class="line"><span class="cl">  <span class="p">},</span>
</span></span><span class="line"><span class="cl"><span class="p">});</span>
</span></span><span class="line"><span class="cl"><span class="nx">instantiatedApp</span><span class="p">.</span><span class="nx">invokeMain</span><span class="p">();</span></span></span></code></pre></div></div>
<ul>
<li><strong>Tree-shaking нативных библиотек:</strong> Появилась новая аннотация компилятора для отсечения (tree-shaking) неиспользуемого нативного кода C/C++ при сборке через FFI. Также runtime-движок получил очередные оптимизации вызовов функций и работы с памятью.</li>
<li>📌 <strong>Ссылка:</strong> <a href="https://dart.dev/blog/announcing-dart-3-13"  target="_blank" rel="noreferrer">Официальный анонс Dart 3.13</a> | <a href="https://dart.dev/language/primary-constructors"  target="_blank" rel="noreferrer">Гайд по Primary Constructors</a></li>
</ul>
<hr>

<h2 class="relative group">Android &amp; Jetpack Compose
    <div id="android--jetpack-compose" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#android--jetpack-compose" aria-label="Якорь">#</a>
    </span>
    
</h2>
<ul>
<li><a href="https://android-developers.googleblog.com/2026/07/five-years-of-jetpack-compose.html"  target="_blank" rel="noreferrer">5 лет с релиза Jetpack Compose 1.0 🎉</a> — Google подводит итоги пяти лет с момента первого стабильного релиза Jetpack Compose. Из экспериментального фреймворка Compose превратился в стандарт современной Android-разработки. В статье команда делится ретроспективой архитектурных решений, эволюцией компилятора и планами на будущее.</li>
<li><a href="https://proandroiddev.com/stop-writing-compose-like-its-2024-10-recent-updates-every-android-developer-should-know-302c23b61e36"  target="_blank" rel="noreferrer">Stop Writing Compose Like It’s 2024–10 Recent Updates Every Android Developer Should Know</a> — отличный разбор 10 актуальных изменений в Jetpack Compose. Автор показывает, от каких устаревших привычек 2024 года (вроде неоптимального использования <code>remember</code> или старых шаблонов работы со State) пора отказаться ради производительности UI.</li>
</ul>
<hr>

<h2 class="relative group">iOS &amp; App Store
    <div id="ios--app-store" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#ios--app-store" aria-label="Якорь">#</a>
    </span>
    
</h2>
<ul>
<li><a href="https://developer.apple.com/news/?id=kug6m2ea"  target="_blank" rel="noreferrer">Рекомендации Apple по созданию хэдеров и визуальных материалов для продвижения приложений в App Store</a> — компания Apple выкатила официальные гайдлайны и примеры по созданию графических хэдеров для продвижения приложений в App Store (дополнение к новой функции продвижения, анонсированной на WWDC). В руководстве акцентируется внимание на правильном выборе визуальных элементов, позиционировании и контексте, которые напрямую влияют на конверсию показов в установки.</li>
</ul>
<hr>

<h2 class="relative group">Безопасность мобильных приложений
    <div id="безопасность-мобильных-приложений" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b1%d0%b5%d0%b7%d0%be%d0%bf%d0%b0%d1%81%d0%bd%d0%be%d1%81%d1%82%d1%8c-%d0%bc%d0%be%d0%b1%d0%b8%d0%bb%d1%8c%d0%bd%d1%8b%d1%85-%d0%bf%d1%80%d0%b8%d0%bb%d0%be%d0%b6%d0%b5%d0%bd%d0%b8%d0%b9" aria-label="Якорь">#</a>
    </span>
    
</h2>
<ul>
<li><a href="https://github.com/Sib-Coder/semgrep-mobile-rules"  target="_blank" rel="noreferrer">semgrep-mobile-rules</a> — открытый единый репозиторий правил Semgrep для статического анализа безопасности Android, iOS, React Native/Expo и Flutter-приложений. Проект создали разработчики из Sib-Coder, объединив в одном месте лучшие правила из OWASP MASTG и MobSF, избавившись от багов и ограничений классического <code>mobsfscan</code>.</li>
<li><a href="https://fi5t.xyz/posts/contactdirectory-abuse/"  target="_blank" rel="noreferrer">Злоупотребление возможностями ContactDirectory</a> — подробный разбор атак и методов злоупотребления системным разрешением <code>ContactDirectory</code> в Android. Автор показывает, как недобросовестные приложения могут извлекать личные данные пользователей и обходить механизмы приватности.</li>
<li><a href="https://habr.com/ru/amp/publications/1013714/"  target="_blank" rel="noreferrer">TrustYFox: путь от пет‑проекта до LLM‑инструмента для поиска уязвимостей</a> — статья о создании Open Source инструмента TrustYFox, использующего нейросетевых агентов для автоматизированного аудита безопасности мобильной кодовой базы.</li>
</ul>
<hr>

<h2 class="relative group">Бонус: Интересный Open Source проект
    <div id="бонус-интересный-open-source-проект" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b1%d0%be%d0%bd%d1%83%d1%81-%d0%b8%d0%bd%d1%82%d0%b5%d1%80%d0%b5%d1%81%d0%bd%d1%8b%d0%b9-open-source-%d0%bf%d1%80%d0%be%d0%b5%d0%ba%d1%82" aria-label="Якорь">#</a>
    </span>
    
</h2>
<ul>
<li><strong><a href="https://github.com/devhyper/open-video-editor"  target="_blank" rel="noreferrer">open-video-editor</a></strong> — Наткнулся на интересный открытый видеоредактор для Android. Проект написан на Kotlin с использованием <strong>Media3</strong> и <strong>Jetpack Compose</strong>. Умеет кадрировать видео, менять разрешение, поворачивать и масштабировать ролики, а также поддерживает многозадачность и локализацию через Weblate.</li>
</ul>

