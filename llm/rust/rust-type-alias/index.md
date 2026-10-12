# Псевдонимы типов в Rust (type alias): синтаксис и практические кейсы

> Зачем использовать type alias в Rust. Упрощаем сложные сигнатуры функций, дженерики и Result-типы с помощью псевдонимов типов на понятных примерах.

- **URL:** https://eagle2000.space/rust/rust-type-alias/
- **Date:** 2025-05-25

- **Tags:** rust, dev

---


<h2 class="relative group">Введение
    <div id="введение" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d0%b2%d0%b5%d0%b4%d0%b5%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Псевдонимы типов <code>type aliases</code> — это способ дать существующему типу альтернативное, более понятное имя (аналог никнейм).</p>
<p>Вместо того чтобы читать &ldquo;i32&rdquo;, вы сразу видите: это <code>метры</code>, это <code>айдишник юзера</code> или это <code>цельсии</code>. Такой подход делает код яснее, безопаснее и проще в сопровождении.</p>

<h2 class="relative group">Пример
    <div id="пример" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%b8%d0%bc%d0%b5%d1%80" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Представим, что вы пишете программу для спортивных соревнований. Вам нужно оперировать длиной трассы.</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">main</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="n">race_distance</span>: <span class="kt">i32</span> <span class="o">=</span><span class="w"> </span><span class="mi">1600</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p>Rust по умолчанию определит тип i32 для переменной. Но <strong>1600 чего</strong>? Метров, милей, cантиметров? <em>Совсем неоднозначно.</em></p>
<p>Такое значение без контекста легко может привести к ошибке.</p>
<p>Используем псевдоним для типа i32.</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">type</span> <span class="nc">Meters</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="kt">i32</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">main</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="n">race_distance</span>: <span class="nc">Meters</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="mi">1600</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p>Теперь стало проще и из кода понятно: <strong>это метры</strong>.</p>
<hr>

<h2 class="relative group">Как объявить псевдоним для типа?
    <div id="как-объявить-псевдоним-для-типа" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%ba%d0%b0%d0%ba-%d0%be%d0%b1%d1%8a%d1%8f%d0%b2%d0%b8%d1%82%d1%8c-%d0%bf%d1%81%d0%b5%d0%b2%d0%b4%d0%be%d0%bd%d0%b8%d0%bc-%d0%b4%d0%bb%d1%8f-%d1%82%d0%b8%d0%bf%d0%b0" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Синтаксис простой:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">type</span> <span class="nc">NameAlias</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="err">Существующий</span><span class="n">_тип</span><span class="p">;</span></span></span></code></pre></div></div>

  



<div
  
    class="flex px-4 py-3 rounded-md shadow bg-primary-100 dark:bg-primary-900"
  
  >
  <span
    
      class="text-primary-400 pe-3 flex items-center"
    
    >
    <span class="relative block icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512"><path fill="currentColor" d="M112.1 454.3c0 6.297 1.816 12.44 5.284 17.69l17.14 25.69c5.25 7.875 17.17 14.28 26.64 14.28h61.67c9.438 0 21.36-6.401 26.61-14.28l17.08-25.68c2.938-4.438 5.348-12.37 5.348-17.7L272 415.1h-160L112.1 454.3zM191.4 .0132C89.44 .3257 16 82.97 16 175.1c0 44.38 16.44 84.84 43.56 115.8c16.53 18.84 42.34 58.23 52.22 91.45c.0313 .25 .0938 .5166 .125 .7823h160.2c.0313-.2656 .0938-.5166 .125-.7823c9.875-33.22 35.69-72.61 52.22-91.45C351.6 260.8 368 220.4 368 175.1C368 78.61 288.9-.2837 191.4 .0132zM192 96.01c-44.13 0-80 35.89-80 79.1C112 184.8 104.8 192 96 192S80 184.8 80 176c0-61.76 50.25-111.1 112-111.1c8.844 0 16 7.159 16 16S200.8 96.01 192 96.01z"/></svg>
</span>
  </span>

  <span
    
      class="dark:text-neutral-300"
    
    >Старайтесь придерживаться именования <code>CamelСase</code> -&gt; слова пишутся слитно без пробелов, при этом каждое слово внутри фразы пишется с прописной буквы</span>
</div>


<h2 class="relative group">Повторное использование
    <div id="повторное-использование" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d0%be%d0%b2%d1%82%d0%be%d1%80%d0%bd%d0%be%d0%b5-%d0%b8%d1%81%d0%bf%d0%be%d0%bb%d1%8c%d0%b7%d0%be%d0%b2%d0%b0%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Псевдонимы можно легко переиспользовать:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">type</span> <span class="nc">Meters</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="kt">i32</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">main</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="n">race_one_distance</span>: <span class="nc">Meters</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="mi">1600</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="n">race_two_distance</span>: <span class="nc">Meters</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="mi">3000</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p>Такой подход особенно полезен при работе с единицами измерения, координатами, идентификаторами и другими значениями, которые иначе были бы просто <code>i32</code>, <code>u64</code>, <code>String</code> и так далее.</p>

<h2 class="relative group">Псевдонимы для сложных типов
    <div id="псевдонимы-для-сложных-типов" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%81%d0%b5%d0%b2%d0%b4%d0%be%d0%bd%d0%b8%d0%bc%d1%8b-%d0%b4%d0%bb%d1%8f-%d1%81%d0%bb%d0%be%d0%b6%d0%bd%d1%8b%d1%85-%d1%82%d0%b8%d0%bf%d0%be%d0%b2" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Псевдонимы особенно удобны, когда у вас есть <strong>длинные или вложенные типы</strong>. Например, колбэки (функции обратного вызова в аргументах):</p>

<h3 class="relative group">Без псевдонима
    <div id="без-псевдонима" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b1%d0%b5%d0%b7-%d0%bf%d1%81%d0%b5%d0%b2%d0%b4%d0%be%d0%bd%d0%b8%d0%bc%d0%b0" aria-label="Якорь">#</a>
    </span>
    
</h3>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">run_callback</span><span class="p">(</span><span class="n">cb</span>: <span class="nc">fn</span><span class="p">(</span><span class="kt">i32</span><span class="p">)</span><span class="w"> </span>-&gt; <span class="kt">bool</span><span class="p">)</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">if</span><span class="w"> </span><span class="n">cb</span><span class="p">(</span><span class="mi">10</span><span class="p">)</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Callback returned true&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>

<h3 class="relative group">С псевдонимом
    <div id="с-псевдонимом" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%81-%d0%bf%d1%81%d0%b5%d0%b2%d0%b4%d0%be%d0%bd%d0%b8%d0%bc%d0%be%d0%bc" aria-label="Якорь">#</a>
    </span>
    
</h3>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">type</span> <span class="nc">IntPredicate</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="k">fn</span><span class="p">(</span><span class="kt">i32</span><span class="p">)</span><span class="w"> </span>-&gt; <span class="kt">bool</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">run_callback</span><span class="p">(</span><span class="n">cb</span>: <span class="nc">IntPredicate</span><span class="p">)</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">if</span><span class="w"> </span><span class="n">cb</span><span class="p">(</span><span class="mi">10</span><span class="p">)</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Callback returned true&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p>Теперь это читается гораздо проще.</p>
<hr>

<h2 class="relative group">Где еще удобно использовать псевдонимы?
    <div id="где-еще-удобно-использовать-псевдонимы" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b3%d0%b4%d0%b5-%d0%b5%d1%89%d0%b5-%d1%83%d0%b4%d0%be%d0%b1%d0%bd%d0%be-%d0%b8%d1%81%d0%bf%d0%be%d0%bb%d1%8c%d0%b7%d0%be%d0%b2%d0%b0%d1%82%d1%8c-%d0%bf%d1%81%d0%b5%d0%b2%d0%b4%d0%be%d0%bd%d0%b8%d0%bc%d1%8b" aria-label="Якорь">#</a>
    </span>
    
</h2>

<h3 class="relative group">Типы ошибок
    <div id="типы-ошибок" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%82%d0%b8%d0%bf%d1%8b-%d0%be%d1%88%d0%b8%d0%b1%d0%be%d0%ba" aria-label="Якорь">#</a>
    </span>
    
</h3>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">type</span> <span class="nb">Result</span><span class="o">&lt;</span><span class="n">T</span><span class="o">&gt;</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">std</span>::<span class="n">result</span>::<span class="nb">Result</span><span class="o">&lt;</span><span class="n">T</span><span class="p">,</span><span class="w"> </span><span class="n">MyError</span><span class="o">&gt;</span><span class="p">;</span></span></span></code></pre></div></div>

<h3 class="relative group">Функции-обработчики обратного вызова (колбэки/callbacks)
    <div id="функции-обработчики-обратного-вызова-колбэкиcallbacks" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%84%d1%83%d0%bd%d0%ba%d1%86%d0%b8%d0%b8-%d0%be%d0%b1%d1%80%d0%b0%d0%b1%d0%be%d1%82%d1%87%d0%b8%d0%ba%d0%b8-%d0%be%d0%b1%d1%80%d0%b0%d1%82%d0%bd%d0%be%d0%b3%d0%be-%d0%b2%d1%8b%d0%b7%d0%be%d0%b2%d0%b0-%d0%ba%d0%be%d0%bb%d0%b1%d1%8d%d0%ba%d0%b8callbacks" aria-label="Якорь">#</a>
    </span>
    
</h3>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">type</span> <span class="nc">EventHandler</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="k">fn</span><span class="p">(</span><span class="n">Event</span><span class="p">)</span><span class="w"> </span>-&gt; <span class="p">();</span></span></span></code></pre></div></div>

<h3 class="relative group">При работе с деньгами
    <div id="при-работе-с-деньгами" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%b8-%d1%80%d0%b0%d0%b1%d0%be%d1%82%d0%b5-%d1%81-%d0%b4%d0%b5%d0%bd%d1%8c%d0%b3%d0%b0%d0%bc%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h3>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">type</span> <span class="nc">Rubles</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="kt">f64</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">calculate_total</span><span class="p">(</span><span class="n">price</span>: <span class="nc">Rubles</span><span class="p">,</span><span class="w"> </span><span class="n">discount</span>: <span class="nc">Rubles</span><span class="p">)</span><span class="w"> </span>-&gt; <span class="nc">Rubles</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">price</span><span class="w"> </span><span class="o">-</span><span class="w"> </span><span class="n">discount</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>

<h3 class="relative group">Идентификаторы в веб-разработке
    <div id="идентификаторы-в-веб-разработке" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b8%d0%b4%d0%b5%d0%bd%d1%82%d0%b8%d1%84%d0%b8%d0%ba%d0%b0%d1%82%d0%be%d1%80%d1%8b-%d0%b2-%d0%b2%d0%b5%d0%b1-%d1%80%d0%b0%d0%b7%d1%80%d0%b0%d0%b1%d0%be%d1%82%d0%ba%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h3>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">type</span> <span class="nc">UserId</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="nb">String</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">get_user</span><span class="p">(</span><span class="n">id</span>: <span class="nc">UserId</span><span class="p">)</span><span class="w"> </span><span class="p">{</span><span class="w"> </span><span class="o">..</span><span class="p">.</span><span class="w"> </span><span class="p">}</span></span></span></code></pre></div></div>

<h3 class="relative group">Разработка игр
    <div id="разработка-игр" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%80%d0%b0%d0%b7%d1%80%d0%b0%d0%b1%d0%be%d1%82%d0%ba%d0%b0-%d0%b8%d0%b3%d1%80" aria-label="Якорь">#</a>
    </span>
    
</h3>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">type</span> <span class="nc">HealthPoints</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="kt">u32</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">player_health</span>: <span class="nc">HealthPoints</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="mi">100</span><span class="p">;</span></span></span></code></pre></div></div>

  
  
  
  



<div
  
    class="flex px-4 py-3 rounded-md shadow bg-primary-100 dark:bg-primary-900"
  
  >
  <span
    
      class="text-primary-400 pe-3 flex items-center"
    
    >
    <span class="relative block icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512"><path fill="currentColor" d="M112.1 454.3c0 6.297 1.816 12.44 5.284 17.69l17.14 25.69c5.25 7.875 17.17 14.28 26.64 14.28h61.67c9.438 0 21.36-6.401 26.61-14.28l17.08-25.68c2.938-4.438 5.348-12.37 5.348-17.7L272 415.1h-160L112.1 454.3zM191.4 .0132C89.44 .3257 16 82.97 16 175.1c0 44.38 16.44 84.84 43.56 115.8c16.53 18.84 42.34 58.23 52.22 91.45c.0313 .25 .0938 .5166 .125 .7823h160.2c.0313-.2656 .0938-.5166 .125-.7823c9.875-33.22 35.69-72.61 52.22-91.45C351.6 260.8 368 220.4 368 175.1C368 78.61 288.9-.2837 191.4 .0132zM192 96.01c-44.13 0-80 35.89-80 79.1C112 184.8 104.8 192 96 192S80 184.8 80 176c0-61.76 50.25-111.1 112-111.1c8.844 0 16 7.159 16 16S200.8 96.01 192 96.01z"/></svg>
</span>
  </span>

  <span
    
      class="dark:text-neutral-300"
    
    ><p>Важные моменты:</p>
<ul>
<li>Не злоупотребляйте ими - если тип и так понятен (например, <code>count: i32</code>), псевдоним не нужен</li>
<li>Используйте там, где важно подчеркнуть смысл данных</li>
<li>Помните, что это просто переименование - компилятор всё равно видит исходный тип</li>
</ul></span>
</div>


<h2 class="relative group">Итог
    <div id="итог" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b8%d1%82%d0%be%d0%b3" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Используйте алиасы типов везде, где базовый тип не раскрывает достаточного смысла или где повторяется сложная сигнатура.</p>
<p>В своей практике я почти в каждом языке встречал алиасы для типов и использовал их в работе, это полезная возможность.</p>

