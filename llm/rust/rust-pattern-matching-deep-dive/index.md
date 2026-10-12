# Rust: тонкости Pattern Matching — деструктуризация, bindings и guards

> Глубокое погружение в сопоставление с образцом в Rust: exhaustiveness, деструктуризация, @-binding на примере dupe-krill, slice patterns в normalize_obj и guards.

- **URL:** https://eagle2000.space/rust/rust-pattern-matching-deep-dive/
- **Date:** 2026-10-05

- **Tags:** rust, dev, pattern-matching, destructuring, match-guards, if-let, while-let

---


<h2 class="relative group">Введение
    <div id="введение" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d0%b2%d0%b5%d0%b4%d0%b5%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Паттерн-матчинг в Rust часто воспринимают как аналог оператора <code>switch</code> из других языков. На деле это один из самых гибких инструментов в языке. Выражения <code>match</code>, <code>if let</code> и <code>let</code> умеют разбирать структуры прямо на лету, проверять условия без вложенных <code>if</code> и отсекать невалидные состояния ещё на этапе компиляции.</p>
<p>В прошлых статьях я разбирал базовые перечисления <a href="../rust-enums/" >rust-enums</a> и управляющие конструкции <a href="../rust-conditionals/" >rust-conditionals</a>.</p>
<p>Сегодня отойдём от банальных примеров и разберём, как сопоставление с образцом работает на практике: от <code>@</code>-биндингов в CLI-утилитах до разбора срезов и конечных автоматов в реальных open-source проектах.</p>
<hr>

<h2 class="relative group">1. Принцип полноты и деструктуризация
    <div id="1-принцип-полноты-и-деструктуризация" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#1-%d0%bf%d1%80%d0%b8%d0%bd%d1%86%d0%b8%d0%bf-%d0%bf%d0%be%d0%bb%d0%bd%d0%be%d1%82%d1%8b-%d0%b8-%d0%b4%d0%b5%d1%81%d1%82%d1%80%d1%83%d0%ba%d1%82%d1%83%d1%80%d0%b8%d0%b7%d0%b0%d1%86%d0%b8%d1%8f" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Главное требование компилятора к любому <code>match</code> — <strong>полнота проверок (exhaustiveness)</strong>. Если тип может принять три состояния, вы обязаны явно обработать все три, либо поставить заглушку <code>_</code>.</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">enum</span> <span class="nc">ConnectionState</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">Disconnected</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">Connecting</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">Connected</span><span class="w"> </span><span class="p">{</span><span class="w"> </span><span class="n">ip</span>: <span class="nb">String</span> <span class="p">},</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">handle_connection</span><span class="p">(</span><span class="n">state</span>: <span class="nc">ConnectionState</span><span class="p">)</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">match</span><span class="w"> </span><span class="n">state</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="n">ConnectionState</span>::<span class="n">Disconnected</span><span class="w"> </span><span class="o">=&gt;</span><span class="w"> </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Офлайн&#34;</span><span class="p">),</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="n">ConnectionState</span>::<span class="n">Connected</span><span class="w"> </span><span class="p">{</span><span class="w"> </span><span class="n">ip</span><span class="w"> </span><span class="p">}</span><span class="w"> </span><span class="o">=&gt;</span><span class="w"> </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Подключено к </span><span class="si">{ip}</span><span class="s">&#34;</span><span class="p">),</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="c1">// Ошибка компиляции: вариант `Connecting` не обработан!
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>

<h3 class="relative group">Разбор структур и кортежей на лету
    <div id="разбор-структур-и-кортежей-на-лету" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%80%d0%b0%d0%b7%d0%b1%d0%be%d1%80-%d1%81%d1%82%d1%80%d1%83%d0%ba%d1%82%d1%83%d1%80-%d0%b8-%d0%ba%d0%be%d1%80%d1%82%d0%b5%d0%b6%d0%b5%d0%b9-%d0%bd%d0%b0-%d0%bb%d0%b5%d1%82%d1%83" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Сопоставление работает не только в <code>match</code>, но и в обычных <code>let</code>-выражениях. Можно извлекать нужные поля из глубоко вложенных структур за одно действие:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">struct</span> <span class="nc">Address</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">street</span>: <span class="nb">String</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">city</span>: <span class="nb">String</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">struct</span> <span class="nc">Person</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">name</span>: <span class="nb">String</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">address</span>: <span class="nc">Address</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="c1">// Достаём имя и поля вложенного адреса сразу:
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">Person</span><span class="w"> </span><span class="p">{</span><span class="w"> 
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">name</span><span class="p">,</span><span class="w"> 
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">address</span>: <span class="nc">Address</span><span class="w"> </span><span class="p">{</span><span class="w"> </span><span class="n">street</span><span class="p">,</span><span class="w"> </span><span class="n">city</span><span class="w"> </span><span class="p">}</span><span class="w"> 
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">person</span><span class="p">;</span></span></span></code></pre></div></div>
<p>Лишние поля можно отбросить с помощью <code>_</code> или синтаксиса остатка <code>..</code>:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">Person</span><span class="w"> </span><span class="p">{</span><span class="w"> </span><span class="n">name</span><span class="p">,</span><span class="w"> </span><span class="n">address</span>: <span class="nc">Address</span><span class="w"> </span><span class="p">{</span><span class="w"> </span><span class="n">city</span><span class="p">,</span><span class="w"> </span><span class="o">..</span><span class="w"> </span><span class="p">}</span><span class="w"> </span><span class="p">}</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">person</span><span class="p">;</span></span></span></code></pre></div></div>

<h3 class="relative group">Заимствование через <code>ref</code> и несколько шаблонов через <code>|</code>
    <div id="заимствование-через-ref-и-несколько-шаблонов-через-" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b7%d0%b0%d0%b8%d0%bc%d1%81%d1%82%d0%b2%d0%be%d0%b2%d0%b0%d0%bd%d0%b8%d0%b5-%d1%87%d0%b5%d1%80%d0%b5%d0%b7-ref-%d0%b8-%d0%bd%d0%b5%d1%81%d0%ba%d0%be%d0%bb%d1%8c%d0%ba%d0%be-%d1%88%d0%b0%d0%b1%d0%bb%d0%be%d0%bd%d0%be%d0%b2-%d1%87%d0%b5%d1%80%d0%b5%d0%b7-" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Если нужно сопоставить значение, не забирая владение над его полями (когда тип не реализует <code>Copy</code>), пригодится ключевое слово <code>ref</code> или <code>ref mut</code>:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">point</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="p">(</span><span class="nb">String</span>::<span class="n">from</span><span class="p">(</span><span class="s">&#34;sensor_1&#34;</span><span class="p">),</span><span class="w"> </span><span class="mi">42</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">match</span><span class="w"> </span><span class="n">point</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">(</span><span class="k">ref</span><span class="w"> </span><span class="n">name</span><span class="p">,</span><span class="w"> </span><span class="mi">0</span><span class="p">)</span><span class="w"> </span><span class="o">=&gt;</span><span class="w"> </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Датчик </span><span class="si">{name}</span><span class="s"> на нулевой отметке&#34;</span><span class="p">),</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">(</span><span class="k">ref</span><span class="w"> </span><span class="n">name</span><span class="p">,</span><span class="w"> </span><span class="n">val</span><span class="p">)</span><span class="w"> </span><span class="k">if</span><span class="w"> </span><span class="n">val</span><span class="w"> </span><span class="o">&gt;</span><span class="w"> </span><span class="mi">100</span><span class="w"> </span><span class="o">=&gt;</span><span class="w"> </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Превышение на датчике </span><span class="si">{name}</span><span class="s">: </span><span class="si">{val}</span><span class="s">&#34;</span><span class="p">),</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">_</span><span class="w"> </span><span class="o">=&gt;</span><span class="w"> </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Штатный режим&#34;</span><span class="p">),</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="c1">// point по-прежнему доступен здесь, так как name был заимствован через ref
</span></span></span></code></pre></div></div>
<p>А если несколько веток должны выполнять одинаковый код, их объединяют через символ <code>|</code>:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">match</span><span class="w"> </span><span class="n">code</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="mi">1</span><span class="w"> </span><span class="o">|</span><span class="w"> </span><span class="mi">2</span><span class="w"> </span><span class="o">|</span><span class="w"> </span><span class="mi">3</span><span class="w"> </span><span class="o">=&gt;</span><span class="w"> </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Низкий приоритет&#34;</span><span class="p">),</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="mi">4</span><span class="w"> </span><span class="o">|</span><span class="w"> </span><span class="mi">5</span><span class="w"> </span><span class="o">=&gt;</span><span class="w"> </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Средний приоритет&#34;</span><span class="p">),</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">_</span><span class="w"> </span><span class="o">=&gt;</span><span class="w"> </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Критический&#34;</span><span class="p">),</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<hr>

<h2 class="relative group">2. <code>@</code>-связывание: пример из утилиты dupe-krill
    <div id="2--связывание-пример-из-утилиты-dupe-krill" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#2--%d1%81%d0%b2%d1%8f%d0%b7%d1%8b%d0%b2%d0%b0%d0%bd%d0%b8%d0%b5-%d0%bf%d1%80%d0%b8%d0%bc%d0%b5%d1%80-%d0%b8%d0%b7-%d1%83%d1%82%d0%b8%d0%bb%d0%b8%d1%82%d1%8b-dupe-krill" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Частая задача: проверить, что число или объект попадает в диапазон, и <strong>тут же использовать</strong> это значение в правой части ветки. Без <code>@</code>-связывания пришлось бы либо делать вложенный <code>if</code>, либо дублировать проверки.</p>
<p>Оператор <code>@</code> решает проблему элегантно: он проверяет шаблон и одновременно привязывает совпавшее значение к переменной.</p>
<p>Отличный пример из продакшена — утилита поиска дубликатов файлов <a href="https://github.com/kornelski/dupe-krill/blob/0e813b83c08543d4052523e05ca170bb1d155e91/src/ui.rs#L50"  target="_blank" rel="noreferrer">dupe-krill (автор Kornel Lesiński)</a>. Там длительность сканирования форматируется в красивую строку с помощью сопоставления диапазонов:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="c1">// Исходный код из dupe-krill (src/ui.rs):
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">nice_duration</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="k">match</span><span class="w"> </span><span class="n">scan_duration</span><span class="p">.</span><span class="n">as_secs</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">x</span><span class="w"> </span><span class="o">@</span><span class="w"> </span><span class="mi">0</span><span class="o">..=</span><span class="mi">5</span><span class="w"> </span><span class="o">=&gt;</span><span class="w"> </span><span class="fm">format!</span><span class="p">(</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="s">&#34;</span><span class="si">{:.1}</span><span class="s">s&#34;</span><span class="p">,</span><span class="w"> 
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="p">(</span><span class="n">x</span><span class="w"> </span><span class="o">*</span><span class="w"> </span><span class="mi">1_000_000_000</span><span class="w"> </span><span class="o">+</span><span class="w"> </span><span class="kt">u64</span>::<span class="n">from</span><span class="p">(</span><span class="n">scan_duration</span><span class="p">.</span><span class="n">subsec_nanos</span><span class="p">()))</span><span class="w"> </span><span class="k">as</span><span class="w"> </span><span class="kt">f64</span><span class="w"> </span><span class="o">/</span><span class="w"> </span><span class="mi">1_000_000_000</span><span class="k">f64</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">),</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">x</span><span class="w"> </span><span class="o">@</span><span class="w"> </span><span class="mi">5</span><span class="o">..=</span><span class="mi">59</span><span class="w"> </span><span class="o">=&gt;</span><span class="w"> </span><span class="fm">format!</span><span class="p">(</span><span class="s">&#34;</span><span class="si">{x}</span><span class="s">s&#34;</span><span class="p">),</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">x</span><span class="w"> </span><span class="o">=&gt;</span><span class="w"> </span><span class="fm">format!</span><span class="p">(</span><span class="s">&#34;</span><span class="si">{}</span><span class="s">m</span><span class="si">{}</span><span class="s">s&#34;</span><span class="p">,</span><span class="w"> </span><span class="n">x</span><span class="w"> </span><span class="o">/</span><span class="w"> </span><span class="mi">60</span><span class="p">,</span><span class="w"> </span><span class="n">x</span><span class="w"> </span><span class="o">%</span><span class="w"> </span><span class="mi">60</span><span class="p">),</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">};</span></span></span></code></pre></div></div>
<p>Обратите внимание:</p>
<ul>
<li>В первой ветке <code>x @ 0..=5</code> компилятор проверяет, что секунды укладываются в диапазон <code>0..=5</code>, и связывает это число с <code>x</code>, чтобы добавить наносекунды для точного вывода (<code>1.4s</code>).</li>
<li>Во второй ветке <code>x @ 5..=59</code> число форматируется как целые секунды (<code>42s</code>).</li>
<li>В последней ветке <code>x</code> просто забирает оставшиеся значения и делит их на минуты и секунды (<code>2m15s</code>).</li>
</ul>
<p>Код читается сверху вниз и не требует ни одной вспомогательной переменной.</p>
<hr>

<h2 class="relative group">3. Slice Patterns и Guards: парсер 3D-файлов в normalize_obj
    <div id="3-slice-patterns-и-guards-парсер-3d-файлов-в-normalize_obj" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#3-slice-patterns-%d0%b8-guards-%d0%bf%d0%b0%d1%80%d1%81%d0%b5%d1%80-3d-%d1%84%d0%b0%d0%b9%d0%bb%d0%be%d0%b2-%d0%b2-normalize_obj" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Охранные выражения (<strong>Match Guards</strong>) добавляют к ветке произвольное <code>if</code>-условие. Ветка сработает только тогда, когда совпал и шаблон, и условие guard.</p>
<p>Особенно мощно это выглядит в сочетании с <strong>сопоставлением срезов (Slice Patterns)</strong>.</p>
<p>Посмотрим на парсер формата 3D-моделей OFF из проекта <a href="https://github.com/JulianKnodt/normalize_obj/blob/a43e1c71a2f2d5c336ba72d840b9293a8496c66c/src/off.rs#L35"  target="_blank" rel="noreferrer">normalize_obj (Julian Knodt)</a>. Парсер считывает строки файла, разбивает их на слова через <code>.split_whitespace()</code> и сопоставляет полученный срез токенов:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="c1">// Фрагмент парсера из normalize_obj (src/off.rs):
</span></span></span><span class="line"><span class="cl"><span class="k">match</span><span class="w"> </span><span class="o">&amp;</span><span class="n">tokens</span><span class="p">.</span><span class="n">as_slice</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="c1">// 1. Игнорируем комментарии: первый токен начинается с &#39;#&#39;
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">[</span><span class="n">v</span><span class="p">,</span><span class="w"> </span><span class="o">..</span><span class="p">]</span><span class="w"> </span><span class="k">if</span><span class="w"> </span><span class="n">v</span><span class="p">.</span><span class="n">starts_with</span><span class="p">(</span><span class="sc">&#39;#&#39;</span><span class="p">)</span><span class="w"> </span><span class="o">=&gt;</span><span class="w"> </span><span class="k">continue</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="c1">// 2. Читаем заголовок формата:
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">[</span><span class="n">off</span><span class="p">]</span><span class="w"> </span><span class="k">if</span><span class="w"> </span><span class="n">off</span><span class="p">.</span><span class="n">to_lowercase</span><span class="p">()</span><span class="w"> </span><span class="o">==</span><span class="w"> </span><span class="s">&#34;off&#34;</span><span class="w"> </span><span class="o">=&gt;</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="n">curr</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">State</span>::<span class="n">Counts</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="c1">// 3. Фиксированный срез из 3 элементов: координаты вершины (x, y, z)
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">[</span><span class="n">x</span><span class="p">,</span><span class="w"> </span><span class="n">y</span><span class="p">,</span><span class="w"> </span><span class="n">z</span><span class="p">]</span><span class="w"> </span><span class="k">if</span><span class="w"> </span><span class="n">curr</span><span class="w"> </span><span class="o">==</span><span class="w"> </span><span class="n">State</span>::<span class="n">Verts</span><span class="w"> </span><span class="o">=&gt;</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="kd">let</span><span class="w"> </span><span class="n">x</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">x</span><span class="p">.</span><span class="n">parse</span>::<span class="o">&lt;</span><span class="kt">f64</span><span class="o">&gt;</span><span class="p">()</span><span class="o">?</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="kd">let</span><span class="w"> </span><span class="n">y</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">y</span><span class="p">.</span><span class="n">parse</span>::<span class="o">&lt;</span><span class="kt">f64</span><span class="o">&gt;</span><span class="p">()</span><span class="o">?</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="kd">let</span><span class="w"> </span><span class="n">z</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">z</span><span class="p">.</span><span class="n">parse</span>::<span class="o">&lt;</span><span class="kt">f64</span><span class="o">&gt;</span><span class="p">()</span><span class="o">?</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="n">verts</span><span class="p">.</span><span class="n">push</span><span class="p">([</span><span class="n">x</span><span class="p">,</span><span class="w"> </span><span class="n">y</span><span class="p">,</span><span class="w"> </span><span class="n">z</span><span class="p">]);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="c1">// 4. Захват головы и хвоста среза через `@ ..`:
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">[</span><span class="n">cnt</span><span class="p">,</span><span class="w"> </span><span class="n">r</span><span class="w"> </span><span class="o">@</span><span class="w"> </span><span class="o">..</span><span class="p">]</span><span class="w"> </span><span class="k">if</span><span class="w"> </span><span class="n">curr</span><span class="w"> </span><span class="o">==</span><span class="w"> </span><span class="n">State</span>::<span class="n">Faces</span><span class="w"> </span><span class="o">=&gt;</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="kd">let</span><span class="w"> </span><span class="n">_cnt</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">cnt</span><span class="p">.</span><span class="n">parse</span>::<span class="o">&lt;</span><span class="kt">u32</span><span class="o">&gt;</span><span class="p">()</span><span class="o">?</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="kd">let</span><span class="w"> </span><span class="n">v_idxs</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">r</span><span class="p">.</span><span class="n">iter</span><span class="p">().</span><span class="n">map</span><span class="p">(</span><span class="o">|</span><span class="n">vi</span><span class="o">|</span><span class="w"> </span><span class="n">vi</span><span class="p">.</span><span class="n">parse</span>::<span class="o">&lt;</span><span class="kt">usize</span><span class="o">&gt;</span><span class="p">().</span><span class="n">unwrap</span><span class="p">()).</span><span class="n">collect</span><span class="p">();</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="n">faces</span><span class="p">.</span><span class="n">push</span><span class="p">(</span><span class="n">v_idxs</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">_</span><span class="w"> </span><span class="o">=&gt;</span><span class="w"> </span><span class="fm">panic!</span><span class="p">(</span><span class="s">&#34;Неожиданная строка в файле&#34;</span><span class="p">),</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p>Что здесь происходит:</p>
<ol>
<li><code>[v, ..]</code> проверяет, что в строке есть хотя бы одно слово <code>v</code>, а всё остальное <code>..</code> игнорирует. Guard <code>if v.starts_with('#')</code> мгновенно отсекает комментарии.</li>
<li><code>[x, y, z]</code> требует, чтобы в срезе было <strong>ровно три</strong> элемента. Если элементов два или четыре, эта ветка не сопоставится.</li>
<li><code>[cnt, r @ ..]</code> достаёт первый элемент как число граней <code>cnt</code>, а весь оставшийся срез привязывает к переменной <code>r</code>.</li>
</ol>
<p>Весь конечный автомат парсера уместился в один чистый <code>match</code> без ручных проверок длины массива <code>tokens.len() == 3</code>.</p>
<hr>

<h2 class="relative group">4. Интерактивные примеры: деструктуризация и guards
    <div id="4-интерактивные-примеры-деструктуризация-и-guards" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#4-%d0%b8%d0%bd%d1%82%d0%b5%d1%80%d0%b0%d0%ba%d1%82%d0%b8%d0%b2%d0%bd%d1%8b%d0%b5-%d0%bf%d1%80%d0%b8%d0%bc%d0%b5%d1%80%d1%8b-%d0%b4%d0%b5%d1%81%d1%82%d1%80%d1%83%d0%ba%d1%82%d1%83%d1%80%d0%b8%d0%b7%d0%b0%d1%86%d0%b8%d1%8f-%d0%b8-guards" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>В слайдере ниже можно изучить и запустить примеры с деструктуризацией и условиями:</p>
### 💻 Деструктуризация, @-связывание и Match Guards


#### Шаг 1


```rust
// Шаг 1: Глубокая деструктуризация структур и связывающие шаблоны (@)
struct Address {
    street: String,
    city: String,
}

struct Person {
    name: String,
    address: Address,
}

fn main() {
    let person = Person {
        name: String::from("Алексей"),
        address: Address {
            street: String::from("Ленина"),
            city: String::from("Москва"),
        },
    };

    // 1. Извлечение полей из вложенной структуры за одно действие:
    let Person { name, address: Address { street, city } } = person;
    println!("Имя: {name}, Город: {city}, Улица: {street}");

    // 2. Использование @-связывания для сохранения значения при проверке диапазона:
    let val = 15;
    match val {
        num @ 1..=10 => println!("Число {num} в диапазоне от 1 до 10"),
        num @ 11..=20 => println!("Число {num} в диапазоне от 11 до 20"),
        _ => println!("Другое число"),
    }
}
```


**1. Деструктуризация и @-связывание**
- Шаблоны позволяют разбирать вложенные структуры и перечисления за один шаг без временных переменных.
- Оператор `@` сохраняет привязанное значение в переменную одновременно с проверкой диапазона или шаблона.


#### Шаг 2


```rust
enum Signal {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

// Шаг 2: Охранные выражения (Match Guards) в перечислениях
fn main() {
    let signal = Signal::Move { x: 20, y: 20 };

    match signal {
        Signal::Quit => println!("Команда на выход"),
        // Match Guard с условием равенства координат:
        Signal::Move { x, y } if x == y => {
            println!("Диагональное перемещение в точку ({x}, {y})");
        }
        Signal::Move { x, y } => {
            println!("Обычное смещение в точку ({x}, {y})");
        }
        Signal::Write(text) => println!("Текст сигнала: {text}"),
    }
}
```


**2. Охранные выражения (Match Guards)**
- Match Guard (`if CONDITION`) добавляет дополнительное динамическое условие к ветке `match`.
- Компилятор учитывает порядок веток: ветку с guard следует ставить перед общей веткой.


<hr>

<h2 class="relative group">5. Цепочки <code>if let</code> (<code>let_chains</code>) и цикл <code>while let</code>
    <div id="5-цепочки-if-let-let_chains-и-цикл-while-let" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#5-%d1%86%d0%b5%d0%bf%d0%be%d1%87%d0%ba%d0%b8-if-let-let_chains-%d0%b8-%d1%86%d0%b8%d0%ba%d0%bb-while-let" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Когда полный <code>match</code> избыточен (например, нужно обработать только один позитивный вариант), на помощь приходит <code>if let</code>.</p>
<p>В современном Rust можно связывать несколько сопоставлений через оператор <code>&amp;&amp;</code> в единую цепочку (<strong>let chains</strong>):</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">result</span>: <span class="nb">Option</span><span class="o">&lt;</span><span class="nb">Result</span><span class="o">&lt;</span><span class="kt">i32</span><span class="p">,</span><span class="w"> </span><span class="o">&amp;</span><span class="kt">str</span><span class="o">&gt;&gt;</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="nb">Some</span><span class="p">(</span><span class="nb">Ok</span><span class="p">(</span><span class="mi">42</span><span class="p">));</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="c1">// Разворачиваем вложенные Option и Result в одном условии:
</span></span></span><span class="line"><span class="cl"><span class="k">if</span><span class="w"> </span><span class="kd">let</span><span class="w"> </span><span class="nb">Some</span><span class="p">(</span><span class="n">res</span><span class="p">)</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">result</span><span class="w"> </span><span class="o">&amp;&amp;</span><span class="w"> </span><span class="kd">let</span><span class="w"> </span><span class="nb">Ok</span><span class="p">(</span><span class="n">val</span><span class="p">)</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">res</span><span class="w"> </span><span class="o">&amp;&amp;</span><span class="w"> </span><span class="n">val</span><span class="w"> </span><span class="o">&gt;</span><span class="w"> </span><span class="mi">10</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Получено валидное значение: </span><span class="si">{val}</span><span class="s">&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p>А для работы с очередями, итераторами или каналами Tokio идеально подходит цикл <strong><code>while let</code></strong>:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="k">mut</span><span class="w"> </span><span class="n">stack</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="fm">vec!</span><span class="p">[</span><span class="mi">1</span><span class="p">,</span><span class="w"> </span><span class="mi">2</span><span class="p">,</span><span class="w"> </span><span class="mi">3</span><span class="p">];</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="c1">// Цикл выполняется, пока pop() возвращает Some(item):
</span></span></span><span class="line"><span class="cl"><span class="k">while</span><span class="w"> </span><span class="kd">let</span><span class="w"> </span><span class="nb">Some</span><span class="p">(</span><span class="n">top</span><span class="p">)</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">stack</span><span class="p">.</span><span class="n">pop</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Сняли со стека: </span><span class="si">{top}</span><span class="s">&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p>Попробуйте интерактивный пример с цепочками условий:</p>
### 💻 Цепочки if let и цикл while let


#### Шаг 1


```rust
// Шаг 1: Комбинирование условий в цепочках if let (let_chains)
fn main() {
    let result: Option<Result<i32, &str>> = Some(Ok(42));

    // Цепочка if let с комбинированием условий через оператор &&:
    if let Some(res) = result && let Ok(val) = res && val > 10 {
        println!("Успешно извлечено значение больше 10: {val}");
    }
}
```


**1. Цепочки if let (let_chains)**
- В современных версиях Rust можно объединять несколько сопоставлений `let` и булевых выражений через `&&`.
- Это избавляет от необходимости писать несколько вложенных блоков `if let`.


#### Шаг 2


```rust
// Шаг 2: Использование конструкции while let для обхода итераторов
fn main() {
    let numbers = vec![10, 20, 30, 40, 50];
    let mut iter = numbers.into_iter();

    // Цикл while let извлекает элементы, пока iter.next() возвращает Some(val)
    while let Some(val) = iter.next() {
        println!("Обработан элемент из итератора: {val}");
    }

    println!("Итератор завершил работу!");
}
```


**2. Лаконичный цикл while let**
- Конструкция `while let` выполняет цикл до тех пор, пока выражение соответствует заданному шаблону.
- Идеально подходит для работы с итераторами, стек-буферами и очередями данных.


<hr>

<h2 class="relative group">Проверь свои знания!
    <div id="проверь-свои-знания" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%be%d0%b2%d0%b5%d1%80%d1%8c-%d1%81%d0%b2%d0%be%d0%b8-%d0%b7%d0%bd%d0%b0%d0%bd%d0%b8%d1%8f" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Проверьте, насколько хорошо вы освоили тонкости Pattern Matching:</p>
### 📝 Продвинутый Pattern Matching

**1. Какую роль выполняет оператор `@` в шаблонах Pattern Matching?**

- [x] Сохраняет значение в переменную во время его проверки в диапазоне или шаблоне
- [ ] Принудительно удаляет объект из оперативной памяти
- [ ] Преобразует строку в массив байтов

> 💡 *Оператор `@` (например `num @ 1..=10`) связывает извлеченное значение с переменной при условии совпадения с заданным диапазоном.*


**2. Что представляет собой охранное выражение (Match Guard) в конструкции `match`?**

- [x] Дополнительное `if`-условие в ветке `match`, которое должно вернуть `true` для срабатывания данной ветки
- [ ] Специальный деструктор для очистки памяти
- [ ] Защита от перезаписи файлов конфигурации

> 💡 *Match Guard (`if CONDITION`) дополняет синтаксический шаблон динамическим логическим выражением.*


**3. В чем заключается преимущество цепочек `if let` (let_chains) с оператором `&&`?**

- [x] Они позволяют последовательно извлекать данные из нескольких шаблонов и проверять условия без вложенных блоков
- [ ] Они увеличивают точность вычислений чисел с плавающей точкой
- [ ] Они отключают проверку времён жизни компилятором

> 💡 *Цепочки `if let` упрощают код, избавляя от громоздкой вложенности при проверке нескольких `Option` или `Result`.*


**4. Для какой задачи наиболее удобна конструкция `while let`?**

- [x] Для выполнения цикла до тех пор, пока выражение совпадает с шаблоном (например, последовательный вызов `iter.next()`)
- [ ] Для создания макросов `macro_rules!`
- [ ] Для вызова C-функций через FFI

> 💡 *Конструкция `while let` естественным образом итерируется по значениям `Some(item)`, прерывая цикл при получении `None`.*




