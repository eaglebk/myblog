# Директивы и атрибуты компилятора в Rust: Полное руководство

> Как работают директивы и атрибуты компилятора (#![allow], #[derive], #[inline]) в Rust? Гайд с синтаксисом, примерами и советами по чистке кода.

- **URL:** https://eagle2000.space/rust/rust-directives/
- **Date:** 2025-05-26

- **Tags:** rust, dev

---


<h2 class="relative group">Введение
    <div id="введение" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d0%b2%d0%b5%d0%b4%d0%b5%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Сегодня мы поговорим о <strong>директивах компилятора</strong>, которые в языке Rust называются <strong>атрибутами</strong> (<code>attributes</code>). Это специальные аннотации в коде, которые указывают компилятору, как следует обрабатывать определённые участки исходного кода.</p>
<p>Говоря простыми словами: <strong>директива — это инструкция компилятору</strong>: &ldquo;обрати внимание, здесь есть особое правило&rdquo;.</p>

<h2 class="relative group">Синтаксис директив
    <div id="синтаксис-директив" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%81%d0%b8%d0%bd%d1%82%d0%b0%d0%ba%d1%81%d0%b8%d1%81-%d0%b4%d0%b8%d1%80%d0%b5%d0%ba%d1%82%d0%b8%d0%b2" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Атрибуты оформляются следующим образом:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="cp">#[attribute]</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="cp">#[attribute(value)]</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="cp">#[attribute(key = </span><span class="s">&#34;value&#34;</span><span class="cp">)]</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="cp">#[attribute(value1, value2)]</span></span></span></code></pre></div></div>
<p>Также существуют глобальные атрибуты, которые применяются ко всему файлу:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="cp">#![attribute]</span></span></span></code></pre></div></div>

<h2 class="relative group">Пример: игнорирование предупреждений об неиспользуемых переменных
    <div id="пример-игнорирование-предупреждений-об-неиспользуемых-переменных" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%b8%d0%bc%d0%b5%d1%80-%d0%b8%d0%b3%d0%bd%d0%be%d1%80%d0%b8%d1%80%d0%be%d0%b2%d0%b0%d0%bd%d0%b8%d0%b5-%d0%bf%d1%80%d0%b5%d0%b4%d1%83%d0%bf%d1%80%d0%b5%d0%b6%d0%b4%d0%b5%d0%bd%d0%b8%d0%b9-%d0%be%d0%b1-%d0%bd%d0%b5%d0%b8%d1%81%d0%bf%d0%be%d0%bb%d1%8c%d0%b7%d1%83%d0%b5%d0%bc%d1%8b%d1%85-%d0%bf%d0%b5%d1%80%d0%b5%d0%bc%d0%b5%d0%bd%d0%bd%d1%8b%d1%85" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Компилятор Rust по умолчанию &ldquo;ругается&rdquo;, если вы создаёте переменную, но не используете её:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">miles</span>: <span class="kt">i32</span> <span class="o">=</span><span class="w"> </span><span class="mi">1266</span><span class="p">;</span><span class="w"> </span><span class="c1">// warning: unused variable
</span></span></span></code></pre></div></div>
<p>Чтобы проигнорировать (подавить) это предупреждение, можно использовать директиву <code>#[allow(unused_variables)]</code></p>

<h3 class="relative group">Применение к одной переменной
    <div id="применение-к-одной-переменной" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%b8%d0%bc%d0%b5%d0%bd%d0%b5%d0%bd%d0%b8%d0%b5-%d0%ba-%d0%be%d0%b4%d0%bd%d0%be%d0%b9-%d0%bf%d0%b5%d1%80%d0%b5%d0%bc%d0%b5%d0%bd%d0%bd%d0%be%d0%b9" aria-label="Якорь">#</a>
    </span>
    
</h3>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="cp">#[allow(unused_variables)]</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">miles</span>: <span class="kt">i32</span> <span class="o">=</span><span class="w"> </span><span class="mi">1266</span><span class="p">;</span></span></span></code></pre></div></div>

<h3 class="relative group">Применение к нескольким переменным
    <div id="применение-к-нескольким-переменным" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%b8%d0%bc%d0%b5%d0%bd%d0%b5%d0%bd%d0%b8%d0%b5-%d0%ba-%d0%bd%d0%b5%d1%81%d0%ba%d0%be%d0%bb%d1%8c%d0%ba%d0%b8%d0%bc-%d0%bf%d0%b5%d1%80%d0%b5%d0%bc%d0%b5%d0%bd%d0%bd%d1%8b%d0%bc" aria-label="Якорь">#</a>
    </span>
    
</h3>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="cp">#[allow(unused_variables)]</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">miles</span>: <span class="kt">i32</span> <span class="o">=</span><span class="w"> </span><span class="mi">1266</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="cp">#[allow(unused_variables)]</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">kilometers</span>: <span class="kt">i32</span> <span class="o">=</span><span class="w"> </span><span class="mi">1300</span><span class="p">;</span></span></span></code></pre></div></div>

<h3 class="relative group">Применение ко всей функции
    <div id="применение-ко-всей-функции" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%b8%d0%bc%d0%b5%d0%bd%d0%b5%d0%bd%d0%b8%d0%b5-%d0%ba%d0%be-%d0%b2%d1%81%d0%b5%d0%b9-%d1%84%d1%83%d0%bd%d0%ba%d1%86%d0%b8%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h3>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="cp">#[allow(unused_variables)]</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">main</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="n">miles</span>: <span class="kt">i32</span> <span class="o">=</span><span class="w"> </span><span class="mi">1266</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="n">kilometers</span>: <span class="kt">i32</span> <span class="o">=</span><span class="w"> </span><span class="mi">1300</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>

<h3 class="relative group">Применение ко всему файлу
    <div id="применение-ко-всему-файлу" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%b8%d0%bc%d0%b5%d0%bd%d0%b5%d0%bd%d0%b8%d0%b5-%d0%ba%d0%be-%d0%b2%d1%81%d0%b5%d0%bc%d1%83-%d1%84%d0%b0%d0%b9%d0%bb%d1%83" aria-label="Якорь">#</a>
    </span>
    
</h3>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="cp">#![allow(unused_variables)]</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">main</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="n">miles</span>: <span class="kt">i32</span> <span class="o">=</span><span class="w"> </span><span class="mi">1266</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="n">kilometers</span>: <span class="kt">i32</span> <span class="o">=</span><span class="w"> </span><span class="mi">1300</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>

<h2 class="relative group">Где еще применяются директивы?
    <div id="где-еще-применяются-директивы" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b3%d0%b4%d0%b5-%d0%b5%d1%89%d0%b5-%d0%bf%d1%80%d0%b8%d0%bc%d0%b5%d0%bd%d1%8f%d1%8e%d1%82%d1%81%d1%8f-%d0%b4%d0%b8%d1%80%d0%b5%d0%ba%d1%82%d0%b8%d0%b2%d1%8b" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Rust предоставляет множество директив, каждая из которых решает свою задачу. Вот примеры полезных атрибутов:</p>
<ul>
<li><code>#[allow(dead_code)]</code>
Отключает предупреждение о неиспользуемом коде (например, функциях или структурах):</li>
</ul>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="cp">#[allow(dead_code)]</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">unused_function</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Эта функция не используется, но компилятор не ругается.&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<ul>
<li><code>#[cfg(target_os = &quot;linux&quot;)]</code>
Код будет скомпилирован только для Linux:</li>
</ul>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="cp">#[cfg(target_os = </span><span class="s">&#34;linux&#34;</span><span class="cp">)]</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">platform_specific</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Этот код скомпилируется только на Linux&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<ul>
<li><code>#[inline(always)]</code>
Просит компилятор всегда инлайнить функцию:</li>
</ul>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="cp">#[inline(always)]</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">fast_function</span><span class="p">(</span><span class="n">x</span>: <span class="kt">i32</span><span class="p">)</span><span class="w"> </span>-&gt; <span class="kt">i32</span> <span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">x</span><span class="w"> </span><span class="o">*</span><span class="w"> </span><span class="mi">2</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>

<h2 class="relative group">Варианты применения директив
    <div id="варианты-применения-директив" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d0%b0%d1%80%d0%b8%d0%b0%d0%bd%d1%82%d1%8b-%d0%bf%d1%80%d0%b8%d0%bc%d0%b5%d0%bd%d0%b5%d0%bd%d0%b8%d1%8f-%d0%b4%d0%b8%d1%80%d0%b5%d0%ba%d1%82%d0%b8%d0%b2" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Директивы в Rust могут быть применены к:</p>
<ul>
<li>переменным</li>
<li>функциям</li>
<li>структурам</li>
<li>модулям</li>
<li>impl блокам</li>
<li>трейтам</li>
<li>константам</li>
<li>ко всему файлу (через <code>#![...]</code>)</li>
</ul>

<h2 class="relative group">Вывод
    <div id="вывод" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d1%8b%d0%b2%d0%be%d0%b4" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Директивы компилятора или атрибуты - это удобная фитча языка Rust которая часто используется в проектах и надо уметь их использовать. Со временем вы встретите их большое множество, но будете уже вооружены.</p>

