# Заимствование (Borrowing) в Rust

> Руководство по заимствованию в Rust - механизму работы с данными без передачи владения. Узнайте, как использовать ссылки &T и &mut T, избегать типичных ошибок и писать безопасный код.

- **URL:** https://eagle2000.space/rust/rust-borrowing/
- **Date:** 2025-06-02

- **Tags:** rust, dev

---


<h2 class="relative group">Введение
    <div id="введение" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d0%b2%d0%b5%d0%b4%d0%b5%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Сегодня нам предстоит разобраться с темой заимствования. Как безопасно работать с данными без передачи владения.</p>
<p>Но прежде чем погружаться в эту тему разберём, что такое ссылка и её базовый синтаксис.</p>

<h2 class="relative group">Что такое ссылки в Rust?
    <div id="что-такое-ссылки-в-rust" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%87%d1%82%d0%be-%d1%82%d0%b0%d0%ba%d0%be%d0%b5-%d1%81%d1%81%d1%8b%d0%bb%d0%ba%d0%b8-%d0%b2-rust" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>В Rust ссылка <strong>&amp;</strong> (reference) — это указатель на значение, который <strong>не владеет данными</strong>. Рассмотрим базовый пример:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">x</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="mi">42</span><span class="p">;</span><span class="w">          </span><span class="c1">// Создаём значение
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">x_ref</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="o">&amp;</span><span class="n">x</span><span class="p">;</span><span class="w">      </span><span class="c1">// Создаём ссылку (заимствование)
</span></span></span><span class="line"><span class="cl"><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;</span><span class="si">{}</span><span class="s">&#34;</span><span class="p">,</span><span class="w"> </span><span class="o">*</span><span class="n">x_ref</span><span class="p">);</span><span class="w"> </span><span class="c1">// Доступ к значению через * 
</span></span></span></code></pre></div></div>

<h3 class="relative group">Ключевые особенности ссылок
    <div id="ключевые-особенности-ссылок" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%ba%d0%bb%d1%8e%d1%87%d0%b5%d0%b2%d1%8b%d0%b5-%d0%be%d1%81%d0%be%d0%b1%d0%b5%d0%bd%d0%bd%d0%be%d1%81%d1%82%d0%b8-%d1%81%d1%81%d1%8b%d0%bb%d0%be%d0%ba" aria-label="Якорь">#</a>
    </span>
    
</h3>
<ul>
<li><strong>Нулевая стоимость</strong> — ссылки не копируют данные, работая аналогично указателям (но с проверками безопасности)</li>
<li><strong>Существует два типа ссылок</strong>:</li>
</ul>
<ol>
<li><code>&amp;T</code> — неизменяемая ссылка (read-only) <code>только чтение</code></li>
<li><code>&amp;mut T</code> — изменяемая ссылка (read-write) <code>чтение и запись</code></li>
</ol>
<ul>
<li><strong>Явное разыменование:</strong>
<ul>
<li><code>*x_ref</code> — получает значение (аналогично C++, но с проверками)</li>
</ul>
</li>
</ul>
<blockquote><p>Важно! В большинстве случаев Rust сам разыменовывает ссылки автоматически. Например, в <code>println!(&quot;{}&quot;, x_ref)</code> символ <code>*</code> не требуется.</p>
</blockquote>
  



<div
  
    class="flex px-4 py-3 rounded-md shadow bg-primary-100 dark:bg-primary-900"
  
  >
  <span
    
      class="text-primary-400 pe-3 flex items-center"
    
    >
    <span class="relative block icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512">
<path fill="currentColor"  d="M159.3 5.4c7.8-7.3 19.9-7.2 27.7 .1c27.6 25.9 53.5 53.8 77.7 84c11-14.4 23.5-30.1 37-42.9c7.9-7.4 20.1-7.4 28 .1c34.6 33 63.9 76.6 84.5 118c20.3 40.8 33.8 82.5 33.8 111.9C448 404.2 348.2 512 224 512C98.4 512 0 404.1 0 276.5c0-38.4 17.8-85.3 45.4-131.7C73.3 97.7 112.7 48.6 159.3 5.4zM225.7 416c25.3 0 47.7-7 68.8-21c42.1-29.4 53.4-88.2 28.1-134.4c-2.8-5.6-5.6-11.2-9.8-16.8l-50.6 58.8s-81.4-103.6-87.1-110.6C133.1 243.8 112 273.2 112 306.8C112 375.4 162.6 416 225.7 416z"/></svg></span>
  </span>

  <span
    
      class="dark:text-neutral-300"
    
    ><em><strong>Совет:</strong> Хотите глубже понять основы? Прочитайте <a href="../rust-ownership/" >предыдущую статью</a> перед продолжением.</em></span>
</div>


<h2 class="relative group">Зачем нужно заимствование?
    <div id="зачем-нужно-заимствование" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b7%d0%b0%d1%87%d0%b5%d0%bc-%d0%bd%d1%83%d0%b6%d0%bd%d0%be-%d0%b7%d0%b0%d0%b8%d0%bc%d1%81%d1%82%d0%b2%d0%be%d0%b2%d0%b0%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Заимствование в языке Rust (borrowing) — это механизм, позволяющий временно использовать данные, не становясь их владельцем.</p>
<p>Система заимствования решает три ключевые задачи:</p>
<ol>
<li><strong>Исключает гонки данных</strong> (data races) в многопоточном коде.</li>
<li><strong>Гарантирует безопасность памяти</strong> без сборщика мусора</li>
<li><strong>Избегает лишнего копирования</strong> больших структур данных</li>
</ol>
<p>В отличие от владения (ownership), заимствование не передаёт права на использование и освобожение памяти — оно лишь даёт доступ к данным на время.</p>

<h2 class="relative group">Неизменяемые ссылки (&amp;T) в Rust
    <div id="неизменяемые-ссылки-t-в-rust" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bd%d0%b5%d0%b8%d0%b7%d0%bc%d0%b5%d0%bd%d1%8f%d0%b5%d0%bc%d1%8b%d0%b5-%d1%81%d1%81%d1%8b%d0%bb%d0%ba%d0%b8-t-%d0%b2-rust" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p><strong>Аналогия из жизни</strong>: чтение книги в библиотеке. Вы можете только прочитать её, но не изменять.</p>

<h3 class="relative group">Практический пример
    <div id="практический-пример" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%b0%d0%ba%d1%82%d0%b8%d1%87%d0%b5%d1%81%d0%ba%d0%b8%d0%b9-%d0%bf%d1%80%d0%b8%d0%bc%d0%b5%d1%80" aria-label="Якорь">#</a>
    </span>
    
</h3>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">calculate_length</span><span class="p">(</span><span class="n">s</span>: <span class="kp">&amp;</span><span class="nb">String</span><span class="p">)</span><span class="w"> </span>-&gt; <span class="kt">usize</span> <span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">s</span><span class="p">.</span><span class="n">len</span><span class="p">()</span><span class="w">  </span><span class="c1">// Чтение без изменения
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>

<h3 class="relative group">Еще один пример
    <div id="еще-один-пример" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b5%d1%89%d0%b5-%d0%be%d0%b4%d0%b8%d0%bd-%d0%bf%d1%80%d0%b8%d0%bc%d0%b5%d1%80" aria-label="Якорь">#</a>
    </span>
    
</h3>
### 💻 Пример с автомобилем



```rust
fn borrow_car(borrower: &String) {
    println!("{} from borrow_car", borrower);
}

fn main() {
    let owner = String::from("automobile");
    borrow_car(&owner);
    println!("{} from main", owner); // всё ещё доступен
}
```


**Вместо того чтобы передавать владение переменной (move), можно передать ссылку на неё: `&` или `&mut`.**
- `borrow_car` использует ссылку `&String`
- В `main` переменная `owner` всё ещё жива — потому что её никто не «забрал»



  



<div
  
    class="flex px-4 py-3 rounded-md shadow bg-primary-100 dark:bg-primary-900"
  
  >
  <span
    
      class="text-primary-400 pe-3 flex items-center"
    
    >
    <span class="relative block icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path fill="currentColor" d="M506.3 417l-213.3-364c-16.33-28-57.54-28-73.98 0l-213.2 364C-10.59 444.9 9.849 480 42.74 480h426.6C502.1 480 522.6 445 506.3 417zM232 168c0-13.25 10.75-24 24-24S280 154.8 280 168v128c0 13.25-10.75 24-23.1 24S232 309.3 232 296V168zM256 416c-17.36 0-31.44-14.08-31.44-31.44c0-17.36 14.07-31.44 31.44-31.44s31.44 14.08 31.44 31.44C287.4 401.9 273.4 416 256 416z"/></svg>
</span>
  </span>

  <span
    
      class="dark:text-neutral-300"
    
    ><p><strong>Правила работы с &amp;T:</strong></p>
<ul>
<li>Можно создавать <strong>сколько угодно</strong> неизменяемых ссылок одновременно.</li>
<li>При существовании<code> &amp;T</code> изменяемые ссылки (<code>&amp;mut T</code>) запрещены</li>
<li>Нельзя изменять данные через <code>&amp;T</code></li>
</ul></span>
</div>


<h2 class="relative group">Изменяемые ссылки (&amp;mut T) в Rust
    <div id="изменяемые-ссылки-mut-t-в-rust" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b8%d0%b7%d0%bc%d0%b5%d0%bd%d1%8f%d0%b5%d0%bc%d1%8b%d0%b5-%d1%81%d1%81%d1%8b%d0%bb%d0%ba%d0%b8-mut-t-%d0%b2-rust" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p><strong>Аналогия из жизни</strong>: Эксклюзивный доступ к документу для редактирования - только один автор может изменять данные.</p>

<h3 class="relative group">Пример использования
    <div id="пример-использования" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%b8%d0%bc%d0%b5%d1%80-%d0%b8%d1%81%d0%bf%d0%be%d0%bb%d1%8c%d0%b7%d0%be%d0%b2%d0%b0%d0%bd%d0%b8%d1%8f" aria-label="Якорь">#</a>
    </span>
    
</h3>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">add_suffix</span><span class="p">(</span><span class="n">s</span>: <span class="kp">&amp;</span><span class="nc">mut</span><span class="w"> </span><span class="nb">String</span><span class="p">)</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">s</span><span class="p">.</span><span class="n">push_str</span><span class="p">(</span><span class="s">&#34;_suffix&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="k">mut</span><span class="w"> </span><span class="n">data</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="nb">String</span>::<span class="n">from</span><span class="p">(</span><span class="s">&#34;text&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="n">add_suffix</span><span class="p">(</span><span class="o">&amp;</span><span class="k">mut</span><span class="w"> </span><span class="n">data</span><span class="p">);</span></span></span></code></pre></div></div>

  



<div
  
    class="flex px-4 py-3 rounded-md shadow bg-primary-100 dark:bg-primary-900"
  
  >
  <span
    
      class="text-primary-400 pe-3 flex items-center"
    
    >
    <span class="relative block icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path fill="currentColor" d="M506.3 417l-213.3-364c-16.33-28-57.54-28-73.98 0l-213.2 364C-10.59 444.9 9.849 480 42.74 480h426.6C502.1 480 522.6 445 506.3 417zM232 168c0-13.25 10.75-24 24-24S280 154.8 280 168v128c0 13.25-10.75 24-23.1 24S232 309.3 232 296V168zM256 416c-17.36 0-31.44-14.08-31.44-31.44c0-17.36 14.07-31.44 31.44-31.44s31.44 14.08 31.44 31.44C287.4 401.9 273.4 416 256 416z"/></svg>
</span>
  </span>

  <span
    
      class="dark:text-neutral-300"
    
    ><p><strong>Жёсткие ограничения:</strong></p>
<ul>
<li>Только <strong>одна</strong> изменяемая ссылка в одной области видимости.</li>
<li>Нельзя одновременно иметь <code>&amp;mut T</code> и <code>&amp;T</code>.</li>
<li>Гарантирует отсутствие гонок данных.</li>
</ul></span>
</div>


<h3 class="relative group">Ошибки заимствования: как их избежать
    <div id="ошибки-заимствования-как-их-избежать" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%be%d1%88%d0%b8%d0%b1%d0%ba%d0%b8-%d0%b7%d0%b0%d0%b8%d0%bc%d1%81%d1%82%d0%b2%d0%be%d0%b2%d0%b0%d0%bd%d0%b8%d1%8f-%d0%ba%d0%b0%d0%ba-%d0%b8%d1%85-%d0%b8%d0%b7%d0%b1%d0%b5%d0%b6%d0%b0%d1%82%d1%8c" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Компилятор Rust предотвращает распространённые ошибки:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="k">mut</span><span class="w"> </span><span class="n">vec</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="fm">vec!</span><span class="p">[</span><span class="mi">1</span><span class="p">,</span><span class="w"> </span><span class="mi">2</span><span class="p">,</span><span class="w"> </span><span class="mi">3</span><span class="p">];</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">first</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="o">&amp;</span><span class="n">vec</span><span class="p">[</span><span class="mi">0</span><span class="p">];</span><span class="w">   </span><span class="c1">// Неизменяемая ссылка
</span></span></span><span class="line"><span class="cl"><span class="n">vec</span><span class="p">.</span><span class="n">push</span><span class="p">(</span><span class="mi">4</span><span class="p">);</span><span class="w">           </span><span class="c1">// Ошибка! Попытка изменения
</span></span></span></code></pre></div></div>
<p><strong>Решение:</strong> ограничить область видимости ссылки или переписать код, чтобы не нужно было изменять исходную коллекцию.</p>

<h3 class="relative group">Время жизни (Lifetimes) и заимствование
    <div id="время-жизни-lifetimes-и-заимствование" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d1%80%d0%b5%d0%bc%d1%8f-%d0%b6%d0%b8%d0%b7%d0%bd%d0%b8-lifetimes-%d0%b8-%d0%b7%d0%b0%d0%b8%d0%bc%d1%81%d1%82%d0%b2%d0%be%d0%b2%d0%b0%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Время жизни (<code>'a</code>) определяет, как долго ссылка остаётся действительной:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">longest</span><span class="o">&lt;</span><span class="na">&#39;a</span><span class="o">&gt;</span><span class="p">(</span><span class="n">x</span>: <span class="kp">&amp;</span><span class="na">&#39;a</span> <span class="kt">str</span><span class="p">,</span><span class="w"> </span><span class="n">y</span>: <span class="kp">&amp;</span><span class="na">&#39;a</span> <span class="kt">str</span><span class="p">)</span><span class="w"> </span>-&gt; <span class="kp">&amp;</span><span class="na">&#39;a</span> <span class="kt">str</span> <span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">if</span><span class="w"> </span><span class="n">x</span><span class="p">.</span><span class="n">len</span><span class="p">()</span><span class="w"> </span><span class="o">&gt;</span><span class="w"> </span><span class="n">y</span><span class="p">.</span><span class="n">len</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w"> </span><span class="n">x</span><span class="w"> </span><span class="p">}</span><span class="w"> </span><span class="k">else</span><span class="w"> </span><span class="p">{</span><span class="w"> </span><span class="n">y</span><span class="w"> </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p><strong>Разберем чуть подробнее:</strong></p>
<ul>
<li><code>'a </code>— время жизни, общее для <code>x</code>, <code>y</code> и возвращаемого значения.</li>
<li>Компилятор гарантирует, что возвращаемая ссылка будет валидной.</li>
</ul>
<p>В следующих статьях я еще вернусь к теме времени жизни потому что часто программисты сталкиваются с ошибками в этой части.</p>

<h2 class="relative group">Примеры взятые из практики
    <div id="примеры-взятые-из-практики" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%b8%d0%bc%d0%b5%d1%80%d1%8b-%d0%b2%d0%b7%d1%8f%d1%82%d1%8b%d0%b5-%d0%b8%d0%b7-%d0%bf%d1%80%d0%b0%d0%ba%d1%82%d0%b8%d0%ba%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h2>
<ol>
<li>Команда работает над презентацией</li>
</ol>
<ul>
<li>Только один человек может редактировать презентацию в Google Docs → <code>&amp;mut</code></li>
<li>Остальные — могут только просматривать → <code>&amp;</code></li>
<li><strong>Какую в этом случае проблему решают ссылки:</strong> если один редактирует, а другой одновременно читает - возникает риск рассинхронизации (<em>Rust не даст это сделать</em>)</li>
</ul>
<ol start="2">
<li>Web-сервер</li>
</ol>
<ul>
<li>Eсть глобальная конфигурация <code>Config</code></li>
<li>Каждый запрос читает <code>&amp;Config</code></li>
<li><strong>Изменить</strong> можно только через <code>&amp;mut Config</code>, например, при hot-reload</li>
</ul>

<h2 class="relative group">Практические советы по заимствованию
    <div id="практические-советы-по-заимствованию" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%b0%d0%ba%d1%82%d0%b8%d1%87%d0%b5%d1%81%d0%ba%d0%b8%d0%b5-%d1%81%d0%be%d0%b2%d0%b5%d1%82%d1%8b-%d0%bf%d0%be-%d0%b7%d0%b0%d0%b8%d0%bc%d1%81%d1%82%d0%b2%d0%be%d0%b2%d0%b0%d0%bd%d0%b8%d1%8e" aria-label="Якорь">#</a>
    </span>
    
</h2>
<ol>
<li><strong>Всегда начинайте</strong> с <code>&amp;T</code> по умолчанию — если не нужно изменять данные</li>
<li><strong>Избегайте</strong> <code>&amp;mut T</code> <code>в больших областях видимости</code> — это ограничивает гибкость</li>
<li><strong>Доверяйте компилятору</strong> - если код компилируется, заимствование безопасно Он не даст вам запустить программу где произойдет ошибка при заимствовании</li>
<li><strong>Учитесь читать вывод компилятора об ошибках</strong> - он точно указывазывает на проблемы в коде</li>
</ol>

<h2 class="relative group">Заключение
    <div id="заключение" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b7%d0%b0%d0%ba%d0%bb%d1%8e%d1%87%d0%b5%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Заимствование в Rust — это то без чего этот язык сложно представить.
Как и с владением вам предстоит постоянно с ним сталкиваться и работать. Эти концепции заложены в язык и благодаря им невозможно будет освободить память дважды, не будет висячих указателей, сборка мусора не требуется, а также у программы будет предсказуемое поведение в рантайме.</p>
<p>Поверьте это большие проблемы в приложениях написанных на других языках. Множество уязвимостей связанно как с переполнением буфера, так и в отказе от обслуживания из-за висячих указателей. (<em>В будущем я еще напишу об этих темах отдельно</em>)</p>
<p>Я советую вам придумать для себя аналогии которые близки вашему представлению и образу жизни и тогда восприниматься этот механизм работы с памятью в языке Rust будет проще.</p>
<p>На сегодня это все!</p>
<div id="typeit-a5372711e6fd66fdc4e0b13a2aa38f40"></div>

<script>
    document.addEventListener("DOMContentLoaded", function () {
      var strings = ["","Продолжение следует...",""];
      
      new TypeIt("#typeit-a5372711e6fd66fdc4e0b13a2aa38f40", {
        strings: strings,
        speed:  50 ,
        lifeLike:  false ,
        startDelay:  0 ,
        breakLines:  true ,
        waitUntilVisible:  true ,
        loop:  false 
      }).go();
    });
</script>


