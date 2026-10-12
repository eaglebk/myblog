# Крейт дня: Strum - автоматический перебор и конвертация enum в Rust на примере esp-generate

> Разбираем популярный крейт strum: как итерироваться по вариантам enum, конвертировать enum в строки и обратно на реальном примере из инструмента esp-generate.

- **URL:** https://eagle2000.space/rust/crate-of-the-day-strum/
- **Date:** 2026-08-16

- **Tags:** rust, crate-of-the-day, enums, macros, esp32

---

<div class="lead text-neutral-500 dark:text-neutral-400 !mb-9 text-xl">
  «Крейт дня» - краткий разбор популярных и практичных библиотек экосистемы Rust, которые избавляют от написания рутинного кода и делают архитектуру чище.
</div>

<p>Сегодня на рассмотрении - <strong><code>strum</code></strong> (набор процедурных макросов для работы с <code>enum</code> в Rust).</p>
<hr>

<h2 class="relative group">Проблема: enum в Rust не имеет итератора
    <div id="проблема-enum-в-rust-не-имеет-итератора" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%be%d0%b1%d0%bb%d0%b5%d0%bc%d0%b0-enum-%d0%b2-rust-%d0%bd%d0%b5-%d0%b8%d0%bc%d0%b5%d0%b5%d1%82-%d0%b8%d1%82%d0%b5%d1%80%d0%b0%d1%82%d0%be%d1%80%d0%b0" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Сам <code>enum</code> широко используется в языке, он отлично подходит для описания состояний или вариантов. Однако из коробки в языке <strong>нет встроенного способа пройтись циклом по всему Enum</strong>:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="c1">// Хотим пройтись по всем входящим в enum вариантам в цикле:
</span></span></span><span class="line"><span class="cl"><span class="k">for</span><span class="w"> </span><span class="n">color</span><span class="w"> </span><span class="k">in</span><span class="w"> </span><span class="n">Color</span>::<span class="n">all_variants</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w"> </span><span class="o">..</span><span class="p">.</span><span class="w"> </span><span class="p">}</span><span class="w"> </span><span class="c1">// ❌ Ошибка! Такого метода нет
</span></span></span></code></pre></div></div>
<p>Если вы разрабатываете CLI-утилиту, генератор аналогичный <code>esp generator</code> или веб-сервис, вам часто требуется:</p>
<ol>
<li>Вывести список всех поддерживаемых опций в <code>--help</code> или консольный меню-селектор.</li>
<li>Проверить входящую строку от пользователя или из файла конфигурации и распарсить её в <code>enum</code>.</li>
<li>Преобразовать имя enum в удобный строковый вид (Например для enum Color: <code>&quot;black&quot;</code> или <code>&quot;white&quot;</code>).</li>
</ol>
<p>Без сторонних библиотек разработчикам приходится писать все вручную чтобы поддерживать массив <code>const ALL_COLORS: &amp;[Color] = &amp;[...]</code>.</p>
<blockquote><p><strong>Главный риск такого подхода:</strong> как только в проект добавляется новый вариант для enum, например <code>GREY</code> кто-то обязательно забудет дописать его в массив.</p>
</blockquote><hr>

<h2 class="relative group">Решение от сообщества: реальный пример из проекта <code>esp-generate</code>
    <div id="решение-от-сообщества-реальный-пример-из-проекта-esp-generate" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%80%d0%b5%d1%88%d0%b5%d0%bd%d0%b8%d0%b5-%d0%be%d1%82-%d1%81%d0%be%d0%be%d0%b1%d1%89%d0%b5%d1%81%d1%82%d0%b2%d0%b0-%d1%80%d0%b5%d0%b0%d0%bb%d1%8c%d0%bd%d1%8b%d0%b9-%d0%bf%d1%80%d0%b8%d0%bc%d0%b5%d1%80-%d0%b8%d0%b7-%d0%bf%d1%80%d0%be%d0%b5%d0%ba%d1%82%d0%b0-esp-generate" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Официальный генератор проектов для микроконтроллеров Espressif — <strong><code>esp-generate</code></strong> (от команды <code>esp-rs</code>) решает эту задачу с помощью крейта <code>strum</code>.</p>
<p>Вот как объявлен их <code>enum Chip</code> в исходном коде библиотеки:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="cp">#[derive(
</span></span></span><span class="line"><span class="cl"><span class="cp">    Debug,
</span></span></span><span class="line"><span class="cl"><span class="cp">    Clone,
</span></span></span><span class="line"><span class="cl"><span class="cp">    Copy,
</span></span></span><span class="line"><span class="cl"><span class="cp">    PartialEq,
</span></span></span><span class="line"><span class="cl"><span class="cp">    Eq,
</span></span></span><span class="line"><span class="cl"><span class="cp">    Hash,
</span></span></span><span class="line"><span class="cl"><span class="cp">    Serialize,
</span></span></span><span class="line"><span class="cl"><span class="cp">    Deserialize,
</span></span></span><span class="line"><span class="cl"><span class="cp">    clap::ValueEnum,
</span></span></span><span class="line"><span class="cl"><span class="cp">    strum::EnumIter,
</span></span></span><span class="line"><span class="cl"><span class="cp">    strum::Display,
</span></span></span><span class="line"><span class="cl"><span class="cp">    strum::EnumString,
</span></span></span><span class="line"><span class="cl"><span class="cp">)]</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="cp">#[serde(rename_all = </span><span class="s">&#34;kebab-case&#34;</span><span class="cp">)]</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="cp">#[strum(serialize_all = </span><span class="s">&#34;kebab-case&#34;</span><span class="cp">)]</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">pub</span><span class="w"> </span><span class="k">enum</span> <span class="nc">Chip</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">Esp32</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">Esp32c2</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">Esp32c3</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">Esp32c5</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">Esp32c6</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">Esp32c61</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">Esp32h2</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">Esp32s2</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">Esp32s3</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p>Всего три макроса из <code>strum</code> перекрывают все потребности работы со строками и итерацией!</p>
<hr>

<h2 class="relative group">Основные фишки Strum
    <div id="основные-фишки-strum" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%be%d1%81%d0%bd%d0%be%d0%b2%d0%bd%d1%8b%d0%b5-%d1%84%d0%b8%d1%88%d0%ba%d0%b8-strum" aria-label="Якорь">#</a>
    </span>
    
</h2>

<h3 class="relative group">1. Итерирование по вариантам enum (<code>strum::EnumIter</code>)
    <div id="1-итерирование-по-вариантам-enum-strumenumiter" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#1-%d0%b8%d1%82%d0%b5%d1%80%d0%b8%d1%80%d0%be%d0%b2%d0%b0%d0%bd%d0%b8%d0%b5-%d0%bf%d0%be-%d0%b2%d0%b0%d1%80%d0%b8%d0%b0%d0%bd%d1%82%d0%b0%d0%bc-enum-strumenumiter" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Макрос <code>EnumIter</code> в сочетании с типажом <code>strum::IntoEnumIterator</code> добавляет к <code>enum</code> метод <code>.iter()</code>:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">use</span><span class="w"> </span><span class="n">strum</span>::<span class="n">IntoEnumIterator</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">for</span><span class="w"> </span><span class="n">chip</span><span class="w"> </span><span class="k">in</span><span class="w"> </span><span class="n">Chip</span>::<span class="n">iter</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Поддерживаемый микроконтроллер: </span><span class="si">{chip}</span><span class="s">&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p>Теперь при добавлении любого нового варианта (например, <code>Esp32p4</code>) он <strong>автоматически</strong> начнет участвовать во всех циклах программы без правки кода!</p>
<hr>

<h3 class="relative group">2. Преобразование в строку (<code>strum::Display</code>)
    <div id="2-преобразование-в-строку-strumdisplay" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#2-%d0%bf%d1%80%d0%b5%d0%be%d0%b1%d1%80%d0%b0%d0%b7%d0%be%d0%b2%d0%b0%d0%bd%d0%b8%d0%b5-%d0%b2-%d1%81%d1%82%d1%80%d0%be%d0%ba%d1%83-strumdisplay" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Макрос <code>Display</code> избавляет от необходимости вручную писать <code>impl std::fmt::Display for Chip</code>.</p>
<p>Атрибут <code>#[strum(serialize_all = &quot;kebab-case&quot;)]</code> настраивает правильное имя для вывода:</p>
<ul>
<li><code>Chip::Esp32c3</code> превращается в <code>&quot;esp32c3&quot;</code>.</li>
<li><code>Chip::Esp32s3</code> превращается в <code>&quot;esp32s3&quot;</code>.</li>
</ul>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;</span><span class="si">{}</span><span class="s">&#34;</span><span class="p">,</span><span class="w"> </span><span class="n">Chip</span>::<span class="n">Esp32c3</span><span class="p">);</span><span class="w"> </span><span class="c1">// Выведет: esp32c3
</span></span></span></code></pre></div></div>
<hr>

<h3 class="relative group">3. Парсинг строки в Enum (<code>strum::EnumString</code>)
    <div id="3-парсинг-строки-в-enum-strumenumstring" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#3-%d0%bf%d0%b0%d1%80%d1%81%d0%b8%d0%bd%d0%b3-%d1%81%d1%82%d1%80%d0%be%d0%ba%d0%b8-%d0%b2-enum-strumenumstring" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Макрос <code>EnumString</code> реализует типаж <code>std::str::FromStr</code>. Это позволяет легко распарсить пользовательский ввод:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">use</span><span class="w"> </span><span class="n">std</span>::<span class="kt">str</span>::<span class="n">FromStr</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">chip</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">Chip</span>::<span class="n">from_str</span><span class="p">(</span><span class="s">&#34;esp32c3&#34;</span><span class="p">)</span><span class="o">?</span><span class="p">;</span><span class="w"> </span><span class="c1">// Ok(Chip::Esp32c3)
</span></span></span><span class="line"><span class="cl"><span class="c1">// Или стандартным методом parse():
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">chip</span>: <span class="nc">Chip</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="s">&#34;esp32c3&#34;</span><span class="p">.</span><span class="n">parse</span><span class="p">()</span><span class="o">?</span><span class="p">;</span></span></span></code></pre></div></div>
<hr>

<h2 class="relative group">Другие полезные макросы в strum
    <div id="другие-полезные-макросы-в-strum" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b4%d1%80%d1%83%d0%b3%d0%b8%d0%b5-%d0%bf%d0%be%d0%bb%d0%b5%d0%b7%d0%bd%d1%8b%d0%b5-%d0%bc%d0%b0%d0%ba%d1%80%d0%be%d1%81%d1%8b-%d0%b2-strum" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Крейт <code>strum</code> не ограничивается этой тройкой:</p>
<ul>
<li><strong><code>EnumCount</code></strong>: добавляет константу (Например <code>Chip::COUNT</code>) показывающую общее количество вариантов <code>enum</code> во время компиляции.</li>
<li><strong><code>VariantNames</code></strong>: добавляет слайс  с именами всех вариантов (Например <code>Chip::VARIANTS</code> - <code>[&quot;esp32&quot;, &quot;esp32c2&quot;]</code>).</li>
<li><strong><code>EnumProperty</code> / <code>EnumMessage</code></strong>: позволяет прикреплять произвольные метаданные или понятные описания прямо к вариантам <code>enum</code> через атрибуты <code>#[strum(message = &quot;...&quot;)]</code>.</li>
</ul>
<hr>

<h2 class="relative group">Полный пример кода
    <div id="полный-пример-кода" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d0%be%d0%bb%d0%bd%d1%8b%d0%b9-%d0%bf%d1%80%d0%b8%d0%bc%d0%b5%d1%80-%d0%ba%d0%be%d0%b4%d0%b0" aria-label="Якорь">#</a>
    </span>
    
</h2>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">use</span><span class="w"> </span><span class="n">strum</span>::<span class="p">{</span><span class="n">EnumIter</span><span class="p">,</span><span class="w"> </span><span class="n">EnumString</span><span class="p">,</span><span class="w"> </span><span class="n">Display</span><span class="p">,</span><span class="w"> </span><span class="n">IntoEnumIterator</span><span class="p">};</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">use</span><span class="w"> </span><span class="n">std</span>::<span class="kt">str</span>::<span class="n">FromStr</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="cp">#[derive(Debug, PartialEq, Eq, EnumIter, Display, EnumString)]</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="cp">#[strum(serialize_all = </span><span class="s">&#34;kebab-case&#34;</span><span class="cp">)]</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">pub</span><span class="w"> </span><span class="k">enum</span> <span class="nc">Chip</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">Esp32</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">Esp32c2</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">Esp32c3</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">Esp32c5</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">Esp32c6</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">Esp32c61</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">Esp32h2</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">Esp32s2</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">Esp32s3</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">main</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="c1">// 1. Итерируемся по всему enum
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;=== ESP Chips ===&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">for</span><span class="w"> </span><span class="n">chip</span><span class="w"> </span><span class="k">in</span><span class="w"> </span><span class="n">Chip</span>::<span class="n">iter</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;- </span><span class="si">{chip}</span><span class="s">&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="c1">// 2. Парсинг из строки обратно в enum
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="n">input</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="s">&#34;esp32c3&#34;</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">match</span><span class="w"> </span><span class="n">Chip</span>::<span class="n">from_str</span><span class="p">(</span><span class="n">input</span><span class="p">)</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="nb">Ok</span><span class="p">(</span><span class="n">chip</span><span class="p">)</span><span class="w"> </span><span class="o">=&gt;</span><span class="w"> </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;</span><span class="se">\n</span><span class="s">Успешно спарсили &#39;</span><span class="si">{input}</span><span class="s">&#39; в вариант: </span><span class="si">{:?}</span><span class="s">&#34;</span><span class="p">,</span><span class="w"> </span><span class="n">chip</span><span class="p">),</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="nb">Err</span><span class="p">(</span><span class="n">_</span><span class="p">)</span><span class="w"> </span><span class="o">=&gt;</span><span class="w"> </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;</span><span class="se">\n</span><span class="s">Неизвестный чип!&#34;</span><span class="p">),</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>

<h3 class="relative group">Вывод программы:
    <div id="вывод-программы" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d1%8b%d0%b2%d0%be%d0%b4-%d0%bf%d1%80%d0%be%d0%b3%d1%80%d0%b0%d0%bc%d0%bc%d1%8b" aria-label="Якорь">#</a>
    </span>
    
</h3>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-text" data-lang="text"><span class="line"><span class="cl">=== Поддерживаемые микроконтроллеры ESP ===
</span></span><span class="line"><span class="cl">- esp32
</span></span><span class="line"><span class="cl">- esp32c2
</span></span><span class="line"><span class="cl">- esp32c3
</span></span><span class="line"><span class="cl">- esp32c5
</span></span><span class="line"><span class="cl">- esp32c6
</span></span><span class="line"><span class="cl">- esp32c61
</span></span><span class="line"><span class="cl">- esp32h2
</span></span><span class="line"><span class="cl">- esp32s2
</span></span><span class="line"><span class="cl">- esp32s3
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl">Успешно преобразовали &#39;esp32c3&#39; в: Esp32c3</span></span></code></pre></div></div>
<hr>

<h2 class="relative group">🎯 Проверь свои знания!
    <div id="-проверь-свои-знания" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#-%d0%bf%d1%80%d0%be%d0%b2%d0%b5%d1%80%d1%8c-%d1%81%d0%b2%d0%be%d0%b8-%d0%b7%d0%bd%d0%b0%d0%bd%d0%b8%d1%8f" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Пройдите короткий тест, чтобы закрепить материал о работе с <code>strum</code>:</p>
### 📝 Крейт Strum в Rust

**1. Какой макрос strum добавляет метод .iter() для enum?**

- [ ] strum::Display
- [x] strum::EnumIter
- [ ] strum::EnumString
- [ ] strum::EnumCount

> 💡 *Макрос strum::EnumIter генерирует реализацию типажа IntoEnumIterator, который предоставляет метод .iter() для получения итератора по вариантам enum.*


**2. Как атрибут #[strum(serialize_all = "kebab-case")] преобразует вариант Esp32c3 в строку?**

- [ ] Преобразует в ESP32C3
- [x] Преобразует в esp32c3
- [ ] Преобразует в esp_32_c3
- [ ] Оставляет без изменений

> 💡 *Правило kebab-case переводит имя варианта в нижний регистр через дефисы (Esp32c3 в данном случае становится esp32c3).*


**3. Какую роль выполняет derive-макрос strum::EnumString?**

- [ ] Выводит enum в консоль через std::fmt::Display
- [ ] Считает количество элементов в enum
- [x] Реализует трейт std::str::FromStr для парсинга строк в варианты enum
- [ ] Создает строковый массив всех имен вариантов enum

> 💡 *EnumString генерирует парсер FromStr, благодаря чему строки вроде "esp32c3" можно конвертировать методом .parse::<Chip>() обратно в вариант enum.*


**4. В чём главное преимущество strum::EnumIter перед ручным массивом variants: &[Chip]?**

- [ ] Работает только во время выполнения
- [x] При добавлении нового варианта в enum он автоматически попадает во все циклы итерации
- [ ] Ускоряет компиляцию программы в 2 раза
- [ ] Позволяет хранить варианты в памяти в виде связного списка

> 💡 *Поскольку strum генерирует итератор во время компиляции, при добавлении любого нового варианта в enum код итерации обновится автоматически без риска забыть элемент.*



<hr>

<h2 class="relative group">Как добавить в проект
    <div id="как-добавить-в-проект" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%ba%d0%b0%d0%ba-%d0%b4%d0%be%d0%b1%d0%b0%d0%b2%d0%b8%d1%82%d1%8c-%d0%b2-%d0%bf%d1%80%d0%be%d0%b5%d0%ba%d1%82" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Достаточно прописать в <code>Cargo.toml</code>:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-toml" data-lang="toml"><span class="line"><span class="cl"><span class="p">[</span><span class="nx">dependencies</span><span class="p">]</span>
</span></span><span class="line"><span class="cl"><span class="nx">strum</span> <span class="p">=</span> <span class="p">{</span> <span class="nx">version</span> <span class="p">=</span> <span class="s2">&#34;0.28&#34;</span><span class="p">,</span> <span class="nx">features</span> <span class="p">=</span> <span class="p">[</span><span class="s2">&#34;derive&#34;</span><span class="p">]</span> <span class="p">}</span></span></span></code></pre></div></div>
<p>🔗 <strong>Репозиторий на GitHub:</strong> <a href="https://github.com/Petrochev/strum"  target="_blank" rel="noreferrer">Petrochev/strum</a><br>
📦 <strong>Crates.io:</strong> <a href="https://crates.io/crates/strum"  target="_blank" rel="noreferrer">crates.io/crates/strum</a><br>
🦀 <strong>Пример из esp-generate:</strong> <a href="https://github.com/esp-rs/esp-generate"  target="_blank" rel="noreferrer">esp-rs/esp-generate</a></p>

