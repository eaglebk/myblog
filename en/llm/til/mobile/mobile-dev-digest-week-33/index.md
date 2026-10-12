# Mobile development digest: Flutter 3.47 & Dart 3.13, 5 years of Compose & tools

> In-depth mobile development digest: Flutter 3.47 and Dart 3.13 releases, 5 years of Jetpack Compose, Apple promo guidelines

- **URL:** https://eagle2000.space/en/til/mobile/mobile-dev-digest-week-33/
- **Date:** 2026-08-17

- **Tags:** ios, android, swift, kotlin, flutter, dart, security, mobile

---

<p>In this edition of our mobile development digest, I cover major updates across the mobile engineering landscape: the releases of Flutter 3.47 and Dart 3.13, celebrating 5 years of Jetpack Compose, Apple App Store header guidelines, and useful open-source tools.</p>

<h2 class="relative group">Releases &amp; announcements: Flutter 3.47 &amp; Dart 3.13
    <div id="releases--announcements-flutter-347--dart-313" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#releases--announcements-flutter-347--dart-313" aria-label="Anchor">#</a>
    </span>
    
</h2>

<h3 class="relative group">Flutter 3.47
    <div id="flutter-347" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#flutter-347" aria-label="Anchor">#</a>
    </span>
    
</h3>
<p>Flutter 3.47 brings key architectural modularity and performance enhancements:</p>
<ul>
<li><strong>Decoupling Material &amp; Cupertino into standalone packages:</strong> The Material and Cupertino widget libraries have been decoupled from the core framework into standalone packages on pub.dev: <a href="https://pub.dev/packages/material_ui"  target="_blank" rel="noreferrer">material_ui</a> and <a href="https://pub.dev/packages/cupertino_ui"  target="_blank" rel="noreferrer">cupertino_ui</a>. This enables faster UI updates independent of SDK release cycles and reduces core engine footprint:</li>
</ul>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-dart" data-lang="dart"><span class="line"><span class="cl"><span class="c1">// ❌ Legacy monolithic SDK import
</span></span></span><span class="line"><span class="cl"><span class="k">import</span> <span class="s1">&#39;package:flutter/material.dart&#39;</span><span class="p">;</span>
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl"><span class="c1">// ✅ In Flutter 3.47: standalone packages from pub.dev
</span></span></span><span class="line"><span class="cl"><span class="k">import</span> <span class="s1">&#39;package:material_ui/material_ui.dart&#39;</span><span class="p">;</span>
</span></span><span class="line"><span class="cl"><span class="k">import</span> <span class="s1">&#39;package:cupertino_ui/cupertino_ui.dart&#39;</span><span class="p">;</span></span></span></code></pre></div></div>

<h4 class="relative group">Step-by-step package migration:
    <div id="step-by-step-package-migration" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#step-by-step-package-migration" aria-label="Anchor">#</a>
    </span>
    
</h4>
<ol>
<li>
<p><strong>Automated import migration via Dart Fix:</strong></p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-bash" data-lang="bash"><span class="line"><span class="cl">dart fix --apply --code<span class="o">=</span>migrate_design_widgets</span></span></code></pre></div></div>
<p>This command automatically adds <code>material_ui</code> / <code>cupertino_ui</code> to <code>pubspec.yaml</code> and migrates legacy <code>package:flutter/material.dart</code> imports to <code>package:material_ui/material_ui.dart</code>.</p>
</li>
<li>
<p><strong>Compatibility Bridge for legacy dependencies:</strong>
If third-party packages or subtrees still depend on <code>package:flutter/material.dart</code>, wrap your root app or subtree using <code>MaterialUiCompatibilityBridge</code> (or <code>CupertinoUiCompatibilityBridge</code>) so <code>ThemeData</code> and localizations resolve seamlessly:</p>
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
<li><strong>Impeller renderer enabled by default on Desktop:</strong> The next-generation Impeller rendering engine—designed to eliminate runtime shader compilation jank—is now officially enabled by default for Desktop platforms (macOS, Windows, and Linux).</li>
<li><strong>Flavors &amp; Multi-window support for Desktop:</strong> Desktop developers gain official Flutter Flavors support alongside an experimental multi-window API.</li>
<li><strong>Apple Autumn Release Preparation:</strong> Pre-emptively updated for smooth compatibility with upcoming autumn iOS and macOS system updates.</li>
<li>📌 <strong>Links:</strong> <a href="https://flutter.dev/blog/whats-new-in-flutter-3-47"  target="_blank" rel="noreferrer">Official Flutter 3.47 Announcement</a> | <a href="https://pub.dev/packages/material_ui"  target="_blank" rel="noreferrer">Package material_ui</a> | <a href="https://pub.dev/packages/cupertino_ui"  target="_blank" rel="noreferrer">Package cupertino_ui</a> | <a href="https://docs.flutter.dev/release/breaking-changes#released-in-flutter-3-47"  target="_blank" rel="noreferrer">Breaking Changes</a></li>
</ul>

<h3 class="relative group">Dart 3.13
    <div id="dart-313" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#dart-313" aria-label="Anchor">#</a>
    </span>
    
</h3>
<p>Alongside Flutter, Dart 3.13 introduced significant language and runtime improvements:</p>
<ul>
<li><strong>Stable Primary Constructors:</strong> Primary Constructors syntax is officially stable (<a href="https://dart.dev/language/primary-constructors"  target="_blank" rel="noreferrer">documentation</a>). Class fields and parameter initialization can now be written concisely directly in the class header:</li>
</ul>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-dart" data-lang="dart"><span class="line"><span class="cl"><span class="c1">// ❌ Previously in Dart: explicit fields, constructor declaration, and initializer list
</span></span></span><span class="line"><span class="cl"><span class="kd">class</span> <span class="nc">Point</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">  <span class="kd">final</span> <span class="kt">int</span> <span class="n">x</span><span class="p">;</span>
</span></span><span class="line"><span class="cl">  <span class="kd">final</span> <span class="kt">int</span> <span class="n">y</span><span class="p">;</span>
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl">  <span class="n">Point</span><span class="p">(</span><span class="k">this</span><span class="p">.</span><span class="n">x</span><span class="p">,</span> <span class="k">this</span><span class="p">.</span><span class="n">y</span><span class="p">)</span> <span class="o">:</span> <span class="k">assert</span><span class="p">(</span><span class="n">x</span> <span class="o">&gt;=</span> <span class="m">0</span> <span class="o">&amp;&amp;</span> <span class="n">y</span> <span class="o">&gt;=</span> <span class="m">0</span><span class="p">);</span>
</span></span><span class="line"><span class="cl"><span class="p">}</span>
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl"><span class="c1">// ✅ In Dart 3.13 (Primary Constructors): primary constructor directly in the class header
</span></span></span><span class="line"><span class="cl"><span class="kd">class</span> <span class="nc">Point</span><span class="p">(</span><span class="kd">var</span> <span class="kt">int</span> <span class="n">x</span><span class="p">,</span> <span class="kd">var</span> <span class="kt">int</span> <span class="n">y</span><span class="p">)</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">  <span class="k">this</span> <span class="o">:</span> <span class="k">assert</span><span class="p">(</span><span class="n">x</span> <span class="o">&gt;=</span> <span class="m">0</span> <span class="o">&amp;&amp;</span> <span class="n">y</span> <span class="o">&gt;=</span> <span class="m">0</span><span class="p">)</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">    <span class="n">print</span><span class="p">(</span><span class="s1">&#39;Point initialized at (</span><span class="si">$</span><span class="n">x</span><span class="s1">, </span><span class="si">$</span><span class="n">y</span><span class="s1">)&#39;</span><span class="p">);</span>
</span></span><span class="line"><span class="cl">  <span class="p">}</span>
</span></span><span class="line"><span class="cl"><span class="p">}</span>
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl"><span class="c1">// Primary constructor with named parameters:
</span></span></span><span class="line"><span class="cl"><span class="kd">class</span> <span class="nc">User</span><span class="p">({</span><span class="kd">required</span> <span class="kd">var</span> <span class="kt">String</span> <span class="n">name</span><span class="p">});</span>
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl"><span class="c1">// Constant primary constructor with initializer list (this : z = x + y):
</span></span></span><span class="line"><span class="cl"><span class="kd">class</span> <span class="nc">const</span> <span class="n">ConstPoint</span><span class="p">(</span><span class="kd">final</span> <span class="kt">int</span> <span class="n">x</span><span class="p">,</span> <span class="kd">final</span> <span class="kt">int</span> <span class="n">y</span><span class="p">)</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">  <span class="kd">final</span> <span class="kt">int</span> <span class="n">z</span><span class="p">;</span>
</span></span><span class="line"><span class="cl">  <span class="k">this</span> <span class="o">:</span> <span class="n">z</span> <span class="o">=</span> <span class="n">x</span> <span class="o">+</span> <span class="n">y</span><span class="p">;</span>
</span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<ul>
<li><strong>WASM Deferred Loading (Experimental in Dart 3.13):</strong> The Dart WebAssembly compiler introduced experimental support for deferred module loading. By default, compiled apps do not use deferred loading—it is enabled using the <code>--enable-deferred-loading</code> flag:</li>
</ul>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-bash" data-lang="bash"><span class="line"><span class="cl">$ dart compile wasm --enable-deferred-loading bin/main.dart</span></span></code></pre></div></div>
<p>Enabling deferred loading requires the JS app loader to supply a <code>loadDeferredModules</code> callback to fetch module bytes on demand:</p>
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
<li><strong>Native Library Tree-Shaking:</strong> Introduced compiler annotations to automatically tree-shake unused C/C++ native library functions when building via FFI.</li>
<li>📌 <strong>Link:</strong> <a href="https://dart.dev/blog/announcing-dart-3-13"  target="_blank" rel="noreferrer">Official Dart 3.13 Announcement</a> | <a href="https://dart.dev/language/primary-constructors"  target="_blank" rel="noreferrer">Primary Constructors Specification Guide</a></li>
</ul>
<hr>

<h2 class="relative group">Android &amp; Jetpack Compose
    <div id="android--jetpack-compose" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#android--jetpack-compose" aria-label="Anchor">#</a>
    </span>
    
</h2>
<ul>
<li><a href="https://android-developers.googleblog.com/2026/07/five-years-of-jetpack-compose.html"  target="_blank" rel="noreferrer">Five years of Jetpack Compose 1.0 🎉</a> — Google reflects on five years since the first stable release of Jetpack Compose, tracing its evolution from an experimental declarative framework into the default standard for modern Android UI development.</li>
<li><a href="https://proandroiddev.com/stop-writing-compose-like-its-2024-10-recent-updates-every-android-developer-should-know-302c23b61e36"  target="_blank" rel="noreferrer">Stop writing Compose like it’s 2024–10 recent updates every Android developer should know</a> — A comprehensive breakdown of 10 modern Jetpack Compose updates and best practices, helping developers replace outdated 2024 habits for better UI performance and cleaner state handling.</li>
</ul>
<hr>

<h2 class="relative group">iOS &amp; App Store
    <div id="ios--app-store" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#ios--app-store" aria-label="Anchor">#</a>
    </span>
    
</h2>
<ul>
<li><a href="https://developer.apple.com/news/?id=kug6m2ea"  target="_blank" rel="noreferrer">Apple guidelines for App Store promotional headers</a> — Apple published official guidelines and best practices for creating promotional header assets for App Store features. The document provides visual examples and layout requirements to maximize impression-to-install conversion.</li>
</ul>
<hr>

<h2 class="relative group">Bonus: Open-source discovery
    <div id="bonus-open-source-discovery" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#bonus-open-source-discovery" aria-label="Anchor">#</a>
    </span>
    
</h2>
<ul>
<li><strong><a href="https://github.com/devhyper/open-video-editor"  target="_blank" rel="noreferrer">open-video-editor</a></strong> — A handy open-source Android video editor licensed under GPL-3.0 that I recently stumbled upon. Built with <strong>Media3</strong> and <strong>Jetpack Compose</strong>, it supports video trimming, resolution scaling, rotation, and Weblate localization.</li>
</ul>

