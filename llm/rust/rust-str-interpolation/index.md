# Работа с println! и интерполяцией строк в Rust

> Узнайте о макросе println! и интерполяции строк в Rust. В этой статье мы рассмотрим, как использовать println! для вывода сообщений, добавлять динамические значения в строку и использовать именованные аргументы. Rust, println!, интерполяция строк, динамические значения, именованные аргументы

- **URL:** https://eagle2000.space/rust/rust-str-interpolation/
- **Date:** 2025-05-03

- **Tags:** rust, dev

---


<h1 class="relative group">Работа с println! и интерполяцией строк в Rust
    <div id="работа-с-println-и-интерполяцией-строк-в-rust" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%80%d0%b0%d0%b1%d0%be%d1%82%d0%b0-%d1%81-println-%d0%b8-%d0%b8%d0%bd%d1%82%d0%b5%d1%80%d0%bf%d0%be%d0%bb%d1%8f%d1%86%d0%b8%d0%b5%d0%b9-%d1%81%d1%82%d1%80%d0%be%d0%ba-%d0%b2-rust" aria-label="Якорь">#</a>
    </span>
    
</h1>
<p>Макрос <code>println!</code> позволяет напечатать необходимое выражение через <code>\n</code>.</p>
<p><code>\n</code> - это аналог ввода через Enter.</p>

<h2 class="relative group">Как это работает
    <div id="как-это-работает" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%ba%d0%b0%d0%ba-%d1%8d%d1%82%d0%be-%d1%80%d0%b0%d0%b1%d0%be%d1%82%d0%b0%d0%b5%d1%82" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Когда мы используем <code>println!</code>, например:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;sensors&#34;</span><span class="p">);</span></span></span></code></pre></div></div>
<p>На консоль будет выведено сообщение &ldquo;sensors&rdquo;.</p>

<h2 class="relative group">Добавление динамических значений в строку
    <div id="добавление-динамических-значений-в-строку" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b4%d0%be%d0%b1%d0%b0%d0%b2%d0%bb%d0%b5%d0%bd%d0%b8%d0%b5-%d0%b4%d0%b8%d0%bd%d0%b0%d0%bc%d0%b8%d1%87%d0%b5%d1%81%d0%ba%d0%b8%d1%85-%d0%b7%d0%bd%d0%b0%d1%87%d0%b5%d0%bd%d0%b8%d0%b9-%d0%b2-%d1%81%d1%82%d1%80%d0%be%d0%ba%d1%83" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Ранее мы добавляли в строку захардкоженные значения. Теперь мы можем добавить динамические значения, используя фигурные скобки <code>{}</code>. Это называется интерполяцией.</p>
<p>Пример:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">room_degree</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="mi">26</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Temperature in room is </span><span class="si">{}</span><span class="s"> degrees.&#34;</span><span class="p">,</span><span class="w"> </span><span class="n">room_degree</span><span class="p">);</span></span></span></code></pre></div></div>
<p>Rust интерполирует значение переменной <code>room_degree</code> в строку.</p>

<h2 class="relative group">Использование выражений в интерполяции
    <div id="использование-выражений-в-интерполяции" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b8%d1%81%d0%bf%d0%be%d0%bb%d1%8c%d0%b7%d0%be%d0%b2%d0%b0%d0%bd%d0%b8%d0%b5-%d0%b2%d1%8b%d1%80%d0%b0%d0%b6%d0%b5%d0%bd%d0%b8%d0%b9-%d0%b2-%d0%b8%d0%bd%d1%82%d0%b5%d1%80%d0%bf%d0%be%d0%bb%d1%8f%d1%86%d0%b8%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Значения для интерполяции могут быть также динамическими выражениями, которые будут вычислены автоматически.</p>
<p>Пример:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">room_degree</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="mi">26</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Temperature in the room is </span><span class="si">{}</span><span class="s"> degrees.&#34;</span><span class="p">,</span><span class="w"> </span><span class="n">room_degree</span><span class="w"> </span><span class="o">-</span><span class="w"> </span><span class="mi">2</span><span class="p">);</span></span></span></code></pre></div></div>
<p>В этом случае будет выведен текст: &ldquo;Temperature in the room is 24 degrees.&rdquo;</p>

<h2 class="relative group">Именованные аргументы
    <div id="именованные-аргументы" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b8%d0%bc%d0%b5%d0%bd%d0%be%d0%b2%d0%b0%d0%bd%d0%bd%d1%8b%d0%b5-%d0%b0%d1%80%d0%b3%d1%83%d0%bc%d0%b5%d0%bd%d1%82%d1%8b" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Начиная с Rust версии 1.58.0, можно использовать <a href="https://blog.rust-lang.org/2022/01/13/Rust-1.58.0/"  target="_blank" rel="noreferrer">именованные аргументы</a> или как называются в других языках - шаблонные литералы, для упрощения чтения и написания кода.</p>
<p>Пример:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">room_degree</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="mi">26</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Temperature in the room is </span><span class="si">{room_degree}</span><span class="s"> degrees.&#34;</span><span class="p">);</span></span></span></code></pre></div></div>

<h2 class="relative group">Ошибки и особенности
    <div id="ошибки-и-особенности" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%be%d1%88%d0%b8%d0%b1%d0%ba%d0%b8-%d0%b8-%d0%be%d1%81%d0%be%d0%b1%d0%b5%d0%bd%d0%bd%d0%be%d1%81%d1%82%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Если мы забудем указать круглые скобки, Rust не будет ничего вычислять, а просто выведет строку.</p>
<p>Пример:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Temperature in the room is room_degree degrees.&#34;</span><span class="p">);</span></span></span></code></pre></div></div>
<p>Выведет: &ldquo;Temperature in the room is room_degree degrees.&rdquo;</p>
<p>Также важно не допускать ошибку при указании имени переменной при интерполяции - компилятор будет ругаться.</p>

<h2 class="relative group">Использование нескольких значений
    <div id="использование-нескольких-значений" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b8%d1%81%d0%bf%d0%be%d0%bb%d1%8c%d0%b7%d0%be%d0%b2%d0%b0%d0%bd%d0%b8%d0%b5-%d0%bd%d0%b5%d1%81%d0%ba%d0%be%d0%bb%d1%8c%d0%ba%d0%b8%d1%85-%d0%b7%d0%bd%d0%b0%d1%87%d0%b5%d0%bd%d0%b8%d0%b9" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Если необходимо указать несколько значений в строке, просто прописываем их через запятую.</p>
<p>Пример:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">living_room_degree</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="mi">26</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">kitchen_degree</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="mi">24</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Temperature in the living room is </span><span class="si">{}</span><span class="s">. And temperature in the kitchen is </span><span class="si">{}</span><span class="s">&#34;</span><span class="p">,</span><span class="w"> </span><span class="n">living_room_degree</span><span class="p">,</span><span class="w"> </span><span class="n">kitchen_degree</span><span class="p">);</span></span></span></code></pre></div></div>
<p>Либо воспользуйтесь новым синтаксисом:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Temperature in the living room is </span><span class="si">{living_room_degree}</span><span class="s">. And temperature in the kitchen is </span><span class="si">{kitchen_degree}</span><span class="s">&#34;</span><span class="p">);</span></span></span></code></pre></div></div>

<h2 class="relative group">Позиционные аргументы
    <div id="позиционные-аргументы" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d0%be%d0%b7%d0%b8%d1%86%d0%b8%d0%be%d0%bd%d0%bd%d1%8b%d0%b5-%d0%b0%d1%80%d0%b3%d1%83%d0%bc%d0%b5%d0%bd%d1%82%d1%8b" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Rust был бы не растом если бы не имел ещё один способ для интерполяции строк. Такой способ есть это - <code>позиционные аргументы</code>. Мы можем &ldquo;вставлять&rdquo; значения переменных в строку (аналогия со слотами) указывая номер аргумента в фигурных скобках ( {0}, {1} и т.д. ). Это позволяет использовать одни и те же переменные в нескольких местах.</p>
<p>Пример:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">living_room_degree</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="mi">26</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">kitchen_degree</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="mi">24</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Temperature in the living room is </span><span class="si">{0}</span><span class="s">. And temperature in the kitchen is </span><span class="si">{1}</span><span class="s">. But living room is </span><span class="si">{0}</span><span class="s"> degrees.&#34;</span><span class="p">,</span><span class="w"> </span><span class="n">living_room_degree</span><span class="p">,</span><span class="w"> </span><span class="n">kitchen_degree</span><span class="p">);</span></span></span></code></pre></div></div>
<p>В этом случае будет выведен текст: &ldquo;Temperature in the living room is 26. And temperature in the kitchen is 24. But living room is 26 degrees.&rdquo;</p>

<h2 class="relative group">Форматирование строк с помощью padding и выравнивания
    <div id="форматирование-строк-с-помощью-padding-и-выравнивания" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%84%d0%be%d1%80%d0%bc%d0%b0%d1%82%d0%b8%d1%80%d0%be%d0%b2%d0%b0%d0%bd%d0%b8%d0%b5-%d1%81%d1%82%d1%80%d0%be%d0%ba-%d1%81-%d0%bf%d0%be%d0%bc%d0%be%d1%89%d1%8c%d1%8e-padding-%d0%b8-%d0%b2%d1%8b%d1%80%d0%b0%d0%b2%d0%bd%d0%b8%d0%b2%d0%b0%d0%bd%d0%b8%d1%8f" aria-label="Якорь">#</a>
    </span>
    
</h2>
<div class="flex mt-2">
  <span
    class="rounded-full bg-primary-500 dark:bg-primary-400 text-neutral-50 dark:text-neutral-800 px-1.5 py-[1px] text-xs font-normal">
    <span class="flex flex-row items-center"><span>🦀 <strong>rust skill</strong></span>
    </span>
  </span>
</div>
<p>Хотелось бы еще упомянуть о дополнительной возможности для форматирования строк с помощью println!. Вы можете использовать padding и выравнивание, чтобы &ldquo;украсить&rdquo; вывод данных.</p>

<h3 class="relative group">Синтаксис форматирования
    <div id="синтаксис-форматирования" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%81%d0%b8%d0%bd%d1%82%d0%b0%d0%ba%d1%81%d0%b8%d1%81-%d1%84%d0%be%d1%80%d0%bc%d0%b0%d1%82%d0%b8%d1%80%d0%be%d0%b2%d0%b0%d0%bd%d0%b8%d1%8f" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Общий синтаксис форматирования выглядит следующим образом:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-text" data-lang="text"><span class="line"><span class="cl">{variable:padding alignment minimum.maximum}</span></span></code></pre></div></div>
<p>Разберем составляющие этого синтаксиса:</p>
<ul>
<li>variable: имя переменной, которую вы хотите вывести.</li>
<li>padding: символ, который будет использоваться для заполнения пробелов.</li>
<li>alignment: выравнивание текста (влево &lt;, вправо &gt;, или по центру ^).</li>
<li>minimum: минимальная длина строки.</li>
<li>maximum: максимальная длина строки (предваряется точкой).</li>
</ul>

<h3 class="relative group">Примеры использования
    <div id="примеры-использования" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%b8%d0%bc%d0%b5%d1%80%d1%8b-%d0%b8%d1%81%d0%bf%d0%be%d0%bb%d1%8c%d0%b7%d0%be%d0%b2%d0%b0%d0%bd%d0%b8%d1%8f" aria-label="Якорь">#</a>
    </span>
    
</h3>

<h4 class="relative group">Выравнивание целого числа по правой стороне с нулями
    <div id="выравнивание-целого-числа-по-правой-стороне-с-нулями" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d1%8b%d1%80%d0%b0%d0%b2%d0%bd%d0%b8%d0%b2%d0%b0%d0%bd%d0%b8%d0%b5-%d1%86%d0%b5%d0%bb%d0%be%d0%b3%d0%be-%d1%87%d0%b8%d1%81%d0%bb%d0%b0-%d0%bf%d0%be-%d0%bf%d1%80%d0%b0%d0%b2%d0%be%d0%b9-%d1%81%d1%82%d0%be%d1%80%d0%be%d0%bd%d0%b5-%d1%81-%d0%bd%d1%83%d0%bb%d1%8f%d0%bc%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h4>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">num</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="mi">123</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;</span><span class="si">{:0&gt;5}</span><span class="s">&#34;</span><span class="p">,</span><span class="w"> </span><span class="n">num</span><span class="p">);</span></span></span></code></pre></div></div>
<p>Выведет: <code>00123</code></p>

<h4 class="relative group">Выравнивание строки по центру с padding и минимальной длиной
    <div id="выравнивание-строки-по-центру-с-padding-и-минимальной-длиной" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d1%8b%d1%80%d0%b0%d0%b2%d0%bd%d0%b8%d0%b2%d0%b0%d0%bd%d0%b8%d0%b5-%d1%81%d1%82%d1%80%d0%be%d0%ba%d0%b8-%d0%bf%d0%be-%d1%86%d0%b5%d0%bd%d1%82%d1%80%d1%83-%d1%81-padding-%d0%b8-%d0%bc%d0%b8%d0%bd%d0%b8%d0%bc%d0%b0%d0%bb%d1%8c%d0%bd%d0%be%d0%b9-%d0%b4%d0%bb%d0%b8%d0%bd%d0%be%d0%b9" aria-label="Якорь">#</a>
    </span>
    
</h4>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">title</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="s">&#34;Rust&#34;</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;</span><span class="si">{:-^10}</span><span class="s">&#34;</span><span class="p">,</span><span class="w"> </span><span class="n">title</span><span class="p">);</span></span></span></code></pre></div></div>
<p>Выведет: <code>----Rust----</code></p>

<h4 class="relative group">Выравнивание строк влево и вправо с padding
    <div id="выравнивание-строк-влево-и-вправо-с-padding" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d1%8b%d1%80%d0%b0%d0%b2%d0%bd%d0%b8%d0%b2%d0%b0%d0%bd%d0%b8%d0%b5-%d1%81%d1%82%d1%80%d0%be%d0%ba-%d0%b2%d0%bb%d0%b5%d0%b2%d0%be-%d0%b8-%d0%b2%d0%bf%d1%80%d0%b0%d0%b2%d0%be-%d1%81-padding" aria-label="Якорь">#</a>
    </span>
    
</h4>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">left_text</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="s">&#34;Left&#34;</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">right_text</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="s">&#34;Right&#34;</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;</span><span class="si">{:.&lt;15}{:.&gt;15}</span><span class="s">&#34;</span><span class="p">,</span><span class="w"> </span><span class="n">left_text</span><span class="p">,</span><span class="w"> </span><span class="n">right_text</span><span class="p">);</span></span></span></code></pre></div></div>
<p>Выведет: <code>Left.....................Right</code></p>

<h4 class="relative group">Использование именованных аргументов с padding и выравниванием
    <div id="использование-именованных-аргументов-с-padding-и-выравниванием" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b8%d1%81%d0%bf%d0%be%d0%bb%d1%8c%d0%b7%d0%be%d0%b2%d0%b0%d0%bd%d0%b8%d0%b5-%d0%b8%d0%bc%d0%b5%d0%bd%d0%be%d0%b2%d0%b0%d0%bd%d0%bd%d1%8b%d1%85-%d0%b0%d1%80%d0%b3%d1%83%d0%bc%d0%b5%d0%bd%d1%82%d0%be%d0%b2-%d1%81-padding-%d0%b8-%d0%b2%d1%8b%d1%80%d0%b0%d0%b2%d0%bd%d0%b8%d0%b2%d0%b0%d0%bd%d0%b8%d0%b5%d0%bc" aria-label="Якорь">#</a>
    </span>
    
</h4>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">city1</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="s">&#34;Москва&#34;</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">city2</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="s">&#34;Уфа&#34;</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;</span><span class="si">{city1:-&gt;10}</span><span class="s"> &lt;--&gt; </span><span class="si">{city2:-&lt;10}</span><span class="s">&#34;</span><span class="p">,</span><span class="w"> </span><span class="n">city1</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">city1</span><span class="p">,</span><span class="w"> </span><span class="n">city2</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">city2</span><span class="p">);</span></span></span></code></pre></div></div>
<p>Выведет: <code>-----Москва &lt;--&gt; Уфа-----</code></p>

