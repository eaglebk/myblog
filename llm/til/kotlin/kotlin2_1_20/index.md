# Обзор Kotlin 2.1.20: Главные обновления и фичи языка

> Подробный разбор ключевых улучшений компилятора K2, исправлений багов и новых возможностей языка Kotlin версии 2.1.20 для Android и Backend разработки.

- **URL:** https://eagle2000.space/til/kotlin/kotlin2_1_20/
- **Date:** 2025-03-26

- **Tags:** Kotlin

---

<p>На днях вышел очередной релиз Kotlin 2.1.20, который принес массу интересных обновлений. Давайте разберемся, что нового появилось в этой версии.</p>
<hr>

<h4 class="relative group"><strong>1. Основные изменения</strong>
    <div id="1-основные-изменения" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#1-%d0%be%d1%81%d0%bd%d0%be%d0%b2%d0%bd%d1%8b%d0%b5-%d0%b8%d0%b7%d0%bc%d0%b5%d0%bd%d0%b5%d0%bd%d0%b8%d1%8f" aria-label="Якорь">#</a>
    </span>
    
</h4>
<p><strong>🔧 K2-компилятор и KAPT</strong></p>
<ul>
<li>
<p><strong>KAPT по умолчанию на K2</strong>: Новый плагин kapt по умолчанию – вроде бы мелочь, но для тех, кто возится с аннотациями и препроцессингом, это может быть существенно. Раньше приходилось возиться с кучей конфигураций, теперь обещают больше стабильности.</p>
<p><em>Откат на K1</em>: Добавьте в <code>gradle.properties</code>:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-properties" data-lang="properties"><span class="line"><span class="cl"><span class="na">kapt.use.k2</span><span class="o">=</span><span class="s">false</span></span></span></code></pre></div></div>
</li>
</ul>
<p><strong>⚡ Kotlin/Native: Производительность</strong></p>
<ul>
<li><strong>Инлайн-оптимизация</strong> (экспериментально):<br>
Инлайн-оптимизации в Kotlin/Native – вещь, которую оценят только настоящие перфекционисты. Миллисекунды? Да. Принципиально? Абсолютно.</li>
</ul>
<p>Эта оптимизация превосходит стандартный инлайнер LLVM и ускоряет выполнение программ.</p>
<p><em>Как включить:</em></p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-bash" data-lang="bash"><span class="line"><span class="cl">-Xbinary<span class="o">=</span><span class="nv">preCodegenInlineThreshold</span><span class="o">=</span><span class="m">40</span></span></span></code></pre></div></div>
<ul>
<li>Значение 40 — оптимальный порог для баланса между оптимизацией и временем компиляции.</li>
<li>Можно экспериментировать с другими значениями (чем выше, тем агрессивнее оптимизация).</li>
</ul>
<p><em>Результаты</em>: +9.5% к скорости выполнения (по тестам JetBrains).</p>
<p><strong>🌐 WebAssembly</strong></p>
<ul>
<li><strong>Кастомные форматтеры по умолчанию</strong> в dev-сборках.
Дефолтные кастомные форматтеры в Wasm – это уже интересно. Web-разработчики получают немного больше контроля без лишних плясок с бубном.</li>
</ul>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-kotlin" data-lang="kotlin"><span class="line"><span class="cl"><span class="c1">// build.gradle.kts
</span></span></span><span class="line"><span class="cl"><span class="n">kotlin</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">    <span class="n">wasmJs</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">        <span class="c1">// ...
</span></span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl">        <span class="n">compilerOptions</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">            <span class="n">freeCompilerArgs</span><span class="p">.</span><span class="n">add</span><span class="p">(</span><span class="s2">&#34;-Xwasm-debugger-custom-formatters&#34;</span><span class="p">)</span>
</span></span><span class="line"><span class="cl">        <span class="p">}</span>
</span></span><span class="line"><span class="cl">    <span class="p">}</span>
</span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<ul>
<li><strong>DWARF-отладка</strong> (для Wasm VM):
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-bash" data-lang="bash"><span class="line"><span class="cl">-Xwasm-generate-dwarf</span></span></code></pre></div></div>
<em>Что даёт</em>: инспекция переменных, пошаговая отладка.</li>
</ul>
<hr>

<h4 class="relative group"><strong>2. Экспериментальные возможности</strong>
    <div id="2-экспериментальные-возможности" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#2-%d1%8d%d0%ba%d1%81%d0%bf%d0%b5%d1%80%d0%b8%d0%bc%d0%b5%d0%bd%d1%82%d0%b0%d0%bb%d1%8c%d0%bd%d1%8b%d0%b5-%d0%b2%d0%be%d0%b7%d0%bc%d0%be%d0%b6%d0%bd%d0%be%d1%81%d1%82%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h4>
<p><strong>🛠 Поддержка Lombok @SuperBuilder и улучшения @Builder</strong></p>
<ul>
<li><strong>Kotlin Lombok plugin</strong> теперь поддерживает аннотацию @SuperBuilder, упрощая создание билдеров для иерархий классов. Раньше при наследовании приходилось вручную описывать логику для суперклассов — теперь это делается автоматически.</li>
</ul>
<p><strong>Что изменилось:</strong></p>
<ol>
<li>@SuperBuilder:
<ul>
<li>Автоматически наследует поля родительского класса.</li>
<li>Позволяет инициализировать поля суперкласса при создании объекта.</li>
</ul>
</li>
</ol>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-kotlin" data-lang="kotlin"><span class="line"><span class="cl">    <span class="nd">@SuperBuilder</span>
</span></span><span class="line"><span class="cl">    <span class="k">public</span> <span class="k">class</span> <span class="nc">Parent</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">        <span class="k">private</span> <span class="n">String</span> <span class="n">parentField</span><span class="p">;</span>
</span></span><span class="line"><span class="cl">    <span class="p">}</span>
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl">    <span class="nd">@SuperBuilder</span>
</span></span><span class="line"><span class="cl">    <span class="k">public</span> <span class="k">class</span> <span class="nc">Child</span> <span class="n">extends</span> <span class="n">Parent</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">        <span class="k">private</span> <span class="n">String</span> <span class="n">childField</span><span class="p">;</span>
</span></span><span class="line"><span class="cl">    <span class="p">}</span>
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl">    <span class="c1">// Использование в Kotlin:
</span></span></span><span class="line"><span class="cl">    <span class="k">val</span> <span class="py">child</span> <span class="p">=</span> <span class="nc">Child</span><span class="p">.</span><span class="n">builder</span><span class="p">()</span>
</span></span><span class="line"><span class="cl">        <span class="p">.</span><span class="n">parentField</span><span class="p">(</span><span class="s2">&#34;value&#34;</span><span class="p">)</span>
</span></span><span class="line"><span class="cl">        <span class="p">.</span><span class="n">childField</span><span class="p">(</span><span class="s2">&#34;anotherValue&#34;</span><span class="p">)</span>
</span></span><span class="line"><span class="cl">        <span class="p">.</span><span class="n">build</span><span class="p">()</span></span></span></code></pre></div></div>
<ol start="2">
<li><strong>@Builder на конструкторах:</strong></li>
</ol>
<ul>
<li>Теперь работает с конструкторами, предлагая гибкие варианты создания объектов.</li>
</ul>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-kotlin" data-lang="kotlin"><span class="line"><span class="cl"><span class="k">public</span> <span class="k">class</span> <span class="nc">User</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">    <span class="nd">@Builder</span>
</span></span><span class="line"><span class="cl">    <span class="k">public</span> <span class="n">User</span><span class="p">(</span><span class="n">String</span> <span class="n">name</span><span class="p">,</span> <span class="n">int</span> <span class="n">age</span><span class="p">)</span> <span class="p">{</span> <span class="o">..</span><span class="p">.</span> <span class="p">}</span>
</span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p><strong>📦 Новый DSL для Multiplatform</strong><br>
С Gradle 8.7 старый Application plugin несовместим с Kotlin Multiplatform. Появился новый экспериментальный DSL с блоком executable {}:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-kotlin" data-lang="kotlin"><span class="line"><span class="cl"><span class="n">kotlin</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">    <span class="n">jvm</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">        <span class="nd">@OptIn</span><span class="p">(</span><span class="n">ExperimentalKotlinGradlePluginApi</span><span class="o">::</span><span class="k">class</span><span class="p">)</span>
</span></span><span class="line"><span class="cl">        <span class="n">binaries</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">            <span class="n">executable</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">                <span class="n">mainClass</span><span class="p">.</span><span class="k">set</span><span class="p">(</span><span class="s2">&#34;foo.MainKt&#34;</span><span class="p">)</span>
</span></span><span class="line"><span class="cl">            <span class="p">}</span>
</span></span><span class="line"><span class="cl">            
</span></span><span class="line"><span class="cl">            <span class="c1">// Можно настроить несколько точек входа
</span></span></span><span class="line"><span class="cl">            <span class="n">executable</span><span class="p">(</span><span class="nc">KotlinCompilation</span><span class="p">.</span><span class="n">MAIN_COMPILATION_NAME</span><span class="p">,</span> <span class="s2">&#34;another&#34;</span><span class="p">)</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">                <span class="n">mainClass</span><span class="p">.</span><span class="k">set</span><span class="p">(</span><span class="s2">&#34;foo.MainAnotherKt&#34;</span><span class="p">)</span>
</span></span><span class="line"><span class="cl">            <span class="p">}</span>
</span></span><span class="line"><span class="cl">        <span class="p">}</span>
</span></span><span class="line"><span class="cl">    <span class="p">}</span>
</span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p><strong>Важно:</strong> Требуется <code>@OptIn(ExperimentalKotlinGradlePluginApi::class)</code></p>
<p><strong>📤 Кастомные публикации в Gradle</strong><br>
Добавление своих вариантов публикации для JVM и Multiplatform (экспериментально):<br>
Возможность добавлять кастомные варианты публикации. Звучит скучно? Для кого-то – да. Для того, кто хочет максимально тонко настроить сборку – почти как панацея.</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-kotlin" data-lang="kotlin"><span class="line"><span class="cl"><span class="n">plugins</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">    <span class="n">kotlin</span><span class="p">(</span><span class="s2">&#34;jvm&#34;</span><span class="p">)</span> <span class="c1">// или multiplatform
</span></span></span><span class="line"><span class="cl"><span class="p">}</span>
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl"><span class="n">kotlin</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">    <span class="nd">@OptIn</span><span class="p">(</span><span class="n">ExperimentalKotlinGradlePluginApi</span><span class="o">::</span><span class="k">class</span><span class="p">)</span>
</span></span><span class="line"><span class="cl">    <span class="n">publishing</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">        <span class="n">adhocSoftwareComponent</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">            <span class="c1">// Конфигурация кастомных вариантов
</span></span></span><span class="line"><span class="cl">        <span class="p">}</span>
</span></span><span class="line"><span class="cl">    <span class="p">}</span>
</span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p><strong>⏱️ Трекинг времени в stdlib</strong><br>
Новые классы <code>Clock</code> и <code>Instant</code> (аналоги из <code>kotlinx-datetime</code>):</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-kotlin" data-lang="kotlin"><span class="line"><span class="cl"><span class="nd">@OptIn</span><span class="p">(</span><span class="n">ExperimentalTime</span><span class="o">::</span><span class="k">class</span><span class="p">)</span>
</span></span><span class="line"><span class="cl"><span class="k">fun</span> <span class="nf">main</span><span class="p">()</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">    <span class="k">val</span> <span class="py">now</span> <span class="p">=</span> <span class="nc">Clock</span><span class="p">.</span><span class="nc">System</span><span class="p">.</span><span class="n">now</span><span class="p">()</span>
</span></span><span class="line"><span class="cl">    <span class="k">val</span> <span class="py">past</span> <span class="p">=</span> <span class="nc">Instant</span><span class="p">.</span><span class="n">parse</span><span class="p">(</span><span class="s2">&#34;2023-01-01T00:00:00Z&#34;</span><span class="p">)</span>
</span></span><span class="line"><span class="cl">    <span class="n">println</span><span class="p">(</span><span class="s2">&#34;Прошло времени: </span><span class="si">${now - past}</span><span class="s2">&#34;</span><span class="p">)</span>
</span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p><em>Конвертеры</em>:</p>
<ul>
<li><code>.toJavaInstant()</code> → <code>java.time.Instant</code></li>
<li><code>.toJSDate()</code> → JS <code>Date</code> (с потерей точности).</li>
</ul>
<hr>

<h4 class="relative group"><strong>3. Kotlin/Wasm: Миграция на Provider API</strong>
    <div id="3-kotlinwasm-миграция-на-provider-api" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#3-kotlinwasm-%d0%bc%d0%b8%d0%b3%d1%80%d0%b0%d1%86%d0%b8%d1%8f-%d0%bd%d0%b0-provider-api" aria-label="Якорь">#</a>
    </span>
    
</h4>
<p>Раньше:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-kotlin" data-lang="kotlin"><span class="line"><span class="cl"><span class="n">the</span><span class="p">&lt;</span><span class="n">NodeJsExtension</span><span class="p">&gt;().</span><span class="n">version</span> <span class="p">=</span> <span class="s2">&#34;2.0.0&#34;</span></span></span></code></pre></div></div>
<p>Теперь:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-kotlin" data-lang="kotlin"><span class="line"><span class="cl"><span class="n">the</span><span class="p">&lt;</span><span class="n">NodeJsEnvSpec</span><span class="p">&gt;().</span><span class="n">version</span><span class="p">.</span><span class="k">set</span><span class="p">(</span><span class="s2">&#34;2.0.0&#34;</span><span class="p">)</span></span></span></code></pre></div></div>
<p><em>Зачем</em>: Ленивые вычисления и улучшенная интеграция с задачами Gradle.</p>
<p><strong>🚨 Заменили устаревшее</strong></p>
<table>
	<thead>
			<tr>
					<th>Устарело</th>
					<th>Замена</th>
			</tr>
	</thead>
	<tbody>
			<tr>
					<td><code>wasmJsRun</code></td>
					<td><code>wasmJsBrowserDevelopmentRun</code></td>
			</tr>
			<tr>
					<td><code>wasmJsBrowserRun</code></td>
					<td><code>wasmJsBrowserDevelopmentRun</code></td>
			</tr>
			<tr>
					<td><code>wasmJsNodeRun</code></td>
					<td><code>wasmJsNodeDevelopmentRun</code></td>
			</tr>
			<tr>
					<td><code>wasmJsBrowserWebpack</code></td>
					<td><code>wasmJsBrowserProductionWebpack или wasmJsBrowserDistribution</code></td>
			</tr>
			<tr>
					<td><code>jsRun</code></td>
					<td><code>jsBrowserDevelopmentRun</code></td>
			</tr>
			<tr>
					<td><code>jsBrowserRun</code></td>
					<td><code>jsBrowserDevelopmentRun</code></td>
			</tr>
			<tr>
					<td><code>jsNodeRun</code></td>
					<td><code>jsNodeDevelopmentRun</code></td>
			</tr>
			<tr>
					<td><code>jsBrowserWebpack</code></td>
					<td><code>jsBrowserProductionWebpack или jsBrowserDistribution</code></td>
			</tr>
	</tbody>
</table>
<hr>

<h4 class="relative group"><strong>4. Стандартная библиотека</strong>
    <div id="4-стандартная-библиотека" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#4-%d1%81%d1%82%d0%b0%d0%bd%d0%b4%d0%b0%d1%80%d1%82%d0%bd%d0%b0%d1%8f-%d0%b1%d0%b8%d0%b1%d0%bb%d0%b8%d0%be%d1%82%d0%b5%d0%ba%d0%b0" aria-label="Якорь">#</a>
    </span>
    
</h4>
<ul>
<li><strong>Атомарные типы</strong>: Для конкурентных операций.</li>
<li><strong>UUID</strong>: Улучшенная генерация и сериализация.</li>
<li><strong>Трекинг времени</strong>: Точное измерение интервалов (см. раздел 2).</li>
</ul>
<hr>

<h4 class="relative group"><strong>5. Compose: Отладка и оптимизации</strong>
    <div id="5-compose-отладка-и-оптимизации" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#5-compose-%d0%be%d1%82%d0%bb%d0%b0%d0%b4%d0%ba%d0%b0-%d0%b8-%d0%be%d0%bf%d1%82%d0%b8%d0%bc%d0%b8%d0%b7%d0%b0%d1%86%d0%b8%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h4>
<ul>
<li><strong>Исходная информация включена по умолчанию</strong> для всех платформ.</li>
<li><strong>Поддержка default-аргументов в open-функциях</strong>:
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-kotlin" data-lang="kotlin"><span class="line"><span class="cl"><span class="k">open</span> <span class="k">class</span> <span class="nc">MyComponent</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">    <span class="nd">@Composable</span>
</span></span><span class="line"><span class="cl">    <span class="k">open</span> <span class="k">fun</span> <span class="nf">Content</span><span class="p">(</span><span class="n">text</span><span class="p">:</span> <span class="n">String</span> <span class="p">=</span> <span class="s2">&#34;default&#34;</span><span class="p">)</span> <span class="p">{</span> <span class="o">..</span><span class="p">.</span> <span class="p">}</span>
</span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<em>Важно</em>: Требуется Kotlin 2.1.20+ для корректной работы.</li>
<li><strong>Финальные функции могут быть restartable</strong>: Убрано ограничение для <code>final override</code>.</li>
<li><strong>Удаление ComposableSingletons из публичного API</strong>: Устранение утечек в inline-функциях.</li>
</ul>
<hr>

<h4 class="relative group"><strong>6. Gradle</strong>
    <div id="6-gradle" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#6-gradle" aria-label="Якорь">#</a>
    </span>
    
</h4>
<ul>
<li><strong>Поддержка версий</strong>: 7.6.3 – 8.11 (с предупреждениями в новых версиях).</li>
<li><strong>Изолированные проекты</strong> (pre-Alpha):
<ul>
<li>Включите через системное свойство.</li>
<li>JS/Wasm пока не поддерживаются.</li>
<li><em>Отключение для Multiplatform</em>:
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-properties" data-lang="properties"><span class="line"><span class="cl"><span class="na">kotlin.kmp.isolated-projects.support</span><span class="o">=</span><span class="s">disable</span></span></span></code></pre></div></div>
</li>
</ul>
</li>
<li><strong>Кастомные публикации</strong>: Экспериментальный API (см. раздел 2).</li>
</ul>
<hr>

<h4 class="relative group"><strong>7. Установка</strong>
    <div id="7-установка" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#7-%d1%83%d1%81%d1%82%d0%b0%d0%bd%d0%be%d0%b2%d0%ba%d0%b0" aria-label="Якорь">#</a>
    </span>
    
</h4>
<ul>
<li><strong>IntelliJ IDEA 2023.3+ и Android Studio Iguana</strong>: Плагин встроен.</li>
<li><strong>Обновление</strong>: Пропишите в <code>build.gradle</code>:
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-kotlin" data-lang="kotlin"><span class="line"><span class="cl"><span class="n">plugins</span> <span class="p">{</span>
</span></span><span class="line"><span class="cl">    <span class="n">kotlin</span><span class="p">(</span><span class="s2">&#34;jvm&#34;</span><span class="p">)</span> <span class="n">version</span> <span class="s2">&#34;2.1.20&#34;</span>
</span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
</li>
</ul>
<hr>

<h4 class="relative group"><strong>8. Breaking Changes</strong>
    <div id="8-breaking-changes" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#8-breaking-changes" aria-label="Якорь">#</a>
    </span>
    
</h4>
<ul>
<li><strong>Устарело</strong>:
<ul>
<li><code>withJava()</code> → Java-сорс-сеты создаются автоматически.</li>
<li><code>kotlin-android-extensions</code> → Полная блокировка (ошибка конфигурации).</li>
</ul>
</li>
<li><strong>Удалено</strong>: Свойство <code>kotlin.incremental.classpath.snapshot.enabled</code>.</li>
</ul>
<hr>

<h3 class="relative group"><strong>Итоги</strong>
    <div id="итоги" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b8%d1%82%d0%be%d0%b3%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Kotlin 2.1.20 фокусируется на <strong>производительности</strong> (Native, K2), <strong>упрощении конфигурации</strong> (Multiplatform DSL) и <strong>отладке</strong> (Wasm DWARF, Compose). Экспериментальные фичи вроде <code>@SuperBuilder</code> и нового DSL дают заглянуть в будущее языка.</p>
<p>🔗 <a href="https://github.com/JetBrains/kotlin/releases/tag/v2.1.20"  target="_blank" rel="noreferrer">https://github.com/JetBrains/kotlin/releases/tag/v2.1.20</a></p>
<p><strong>Внимание:</strong> Все экспериментальные фичи требуют явного <code>@OptIn</code>!</p>
<p>P.S. Если заметили баги с K2 — не паникуйте, всегда можно откатиться на K1 через <code>gradle.properties</code>.<br>
<strong>Тестируйте и программируйте в удовольствие!</strong> 🚀</p>
<hr>
### 📝 Тест по обновлению компилятора Kotlin 2.1.20

**1. Какая главная задача нового компилятора K2 в Kotlin 2.1.20?**

- [ ] Удаление поддержки Java
- [x] Существенное ускорение компиляции и усовершенствованный анализ типов
- [ ] Переход исключительно на JavaScript
- [ ] Автоматическое написание юнит-тестов

> 💡 *Компилятор K2 обеспечивает кратное ускорение сборки проектов и унифицирует анализ кода для Kotlin Multiplatform.*




