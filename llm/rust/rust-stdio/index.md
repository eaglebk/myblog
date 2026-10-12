# Ввод данных от пользователя в Rust: как использовать stdin

> Узнай, как считывать пользовательский ввод в Rust с помощью std::io::stdin().read_line. Простые примеры, обработка ошибок, удаление символа новой строки, тип Result<usize> и рекомендации по CLI

- **URL:** https://eagle2000.space/rust/rust-stdio/
- **Date:** 2025-06-08

- **Tags:** rust, dev

---


<h2 class="relative group">Введение
    <div id="введение" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d0%b2%d0%b5%d0%b4%d0%b5%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>В этой статье разберём как получать ввод от пользователя в терминальном приложении на Rust с использованием стандартной библиотеки.</p>
<p>Казалось бы, простая задача - спросить имя и поприветствовать пользователя. Но даже в этом примере кроются важные особенности языка: работа со стандартным вводом, ссылки, обработка ошибок и типы.</p>

<h2 class="relative group">Что потребуется?
    <div id="что-потребуется" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%87%d1%82%d0%be-%d0%bf%d0%be%d1%82%d1%80%d0%b5%d0%b1%d1%83%d0%b5%d1%82%d1%81%d1%8f" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Нам понадобится модуль <code>std::io</code> который входит в стандартную библиотеку Rust. Он предоставляет доступ к <code>stdin()</code> — стандартному вводу. Чтобы использовать его, нужно подключить:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">use</span><span class="w"> </span><span class="n">std</span>::<span class="n">io</span><span class="p">;</span></span></span></code></pre></div></div>

<h2 class="relative group">Пример: Приветствие пользователя
    <div id="пример-приветствие-пользователя" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%b8%d0%bc%d0%b5%d1%80-%d0%bf%d1%80%d0%b8%d0%b2%d0%b5%d1%82%d1%81%d1%82%d0%b2%d0%b8%d0%b5-%d0%bf%d0%be%d0%bb%d1%8c%d0%b7%d0%be%d0%b2%d0%b0%d1%82%d0%b5%d0%bb%d1%8f" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Начнём с минимального приложения:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">use</span><span class="w"> </span><span class="n">std</span>::<span class="n">io</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">main</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="k">mut</span><span class="w"> </span><span class="n">input</span><span class="w"> </span><span class="n"> `[1]`
</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="nb">String</span>::<span class="n">new</span><span class="p">();</span><span class="w"> </span><span class="n"> `[2]`
</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Enter your name:&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">io</span>::<span class="n">stdin</span><span class="p">().</span><span class="n">read_line</span><span class="p">(</span><span class="o">&amp;</span><span class="k">mut</span><span class="w"> </span><span class="n">input</span><span class="p">).</span><span class="n">unwrap</span><span class="p">();</span><span class="w"> </span><span class="n"> `[3]`
</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Hello </span><span class="si">{input}</span><span class="s">&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>

<h3 class="relative group">Что здесь важно:
    <div id="что-здесь-важно" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%87%d1%82%d0%be-%d0%b7%d0%b4%d0%b5%d1%81%d1%8c-%d0%b2%d0%b0%d0%b6%d0%bd%d0%be" aria-label="Якорь">#</a>
    </span>
    
</h3>
<ul>
<li><code>&amp;mut input</code>  *(строка 1)*
 - передаём изменяемую ссылку, потому что <code>read_line</code> требует возможность изменить содержимое.</li>
<li><code>String::new()</code>  *(строка 2)*
 - создаёт пустую строку для хранения ввода.</li>
<li><code>.unwrap()</code> - в данном случае упрощённая обработка <code>Result</code>. Позже заменим это на более надёжный способ.  *(строка 3)*
</li>
</ul>

<h2 class="relative group">Коротко о типах
    <div id="коротко-о-типах" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%ba%d0%be%d1%80%d0%be%d1%82%d0%ba%d0%be-%d0%be-%d1%82%d0%b8%d0%bf%d0%b0%d1%85" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Функция <code>read_line</code> возвращает <code>Result&lt;usize&gt;</code> где <code>usize</code> — количество байт прочитанных из ввода.</p>

  



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
    
    >Тип <code>usize</code> — это целое без знака, размер которого зависит от архитектуры машины (например, 64-битный на x86_64).</span>
</div>


<h2 class="relative group">Добавим отладочную информацию:
    <div id="добавим-отладочную-информацию" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b4%d0%be%d0%b1%d0%b0%d0%b2%d0%b8%d0%bc-%d0%be%d1%82%d0%bb%d0%b0%d0%b4%d0%be%d1%87%d0%bd%d1%83%d1%8e-%d0%b8%d0%bd%d1%84%d0%be%d1%80%d0%bc%d0%b0%d1%86%d0%b8%d1%8e" aria-label="Якорь">#</a>
    </span>
    
</h2>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">use</span><span class="w"> </span><span class="n">std</span>::<span class="n">io</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">main</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="k">mut</span><span class="w"> </span><span class="n">input</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="nb">String</span>::<span class="n">new</span><span class="p">();</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Enter your name:&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">match</span><span class="w"> </span><span class="n">io</span>::<span class="n">stdin</span><span class="p">().</span><span class="n">read_line</span><span class="p">(</span><span class="o">&amp;</span><span class="k">mut</span><span class="w"> </span><span class="n">input</span><span class="p">)</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="nb">Ok</span><span class="p">(</span><span class="n">n</span><span class="p">)</span><span class="w"> </span><span class="o">=&gt;</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">            </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;</span><span class="si">{n}</span><span class="s"> bytes read&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">            </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Hello </span><span class="si">{input}</span><span class="s">&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="nb">Err</span><span class="p">(</span><span class="n">error</span><span class="p">)</span><span class="w"> </span><span class="o">=&gt;</span><span class="w"> </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;error: </span><span class="si">{error}</span><span class="s">&#34;</span><span class="p">),</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>

<h3 class="relative group">Пример вывода:
    <div id="пример-вывода" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%b8%d0%bc%d0%b5%d1%80-%d0%b2%d1%8b%d0%b2%d0%be%d0%b4%d0%b0" aria-label="Якорь">#</a>
    </span>
    
</h3>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-text" data-lang="text"><span class="line"><span class="cl">Enter your name:
</span></span><span class="line"><span class="cl">developer
</span></span><span class="line"><span class="cl">10 bytes read
</span></span><span class="line"><span class="cl">Hello developer</span></span></code></pre></div></div>

  



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
    
    >Почему 10 байт? Строка <code>&quot;developer&quot;</code> — это 9 символов + 1 символ новой строки <code>\n</code> автоматически добавляемый после нажатия Enter.</span>
</div>


<h2 class="relative group">Практическое применение
    <div id="практическое-применение" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%b0%d0%ba%d1%82%d0%b8%d1%87%d0%b5%d1%81%d0%ba%d0%be%d0%b5-%d0%bf%d1%80%d0%b8%d0%bc%d0%b5%d0%bd%d0%b5%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Такой ввод часто используется в CLI-интерфейсах: для подтверждения действия, выбора режима работы, простых форм.</p>
<p>Особенности ввода в CLI:</p>
<ul>
<li><code>read_line</code> блокирует выполнение, пока пользователь не введёт строку.</li>
<li>Строка сохраняет символ новой строки <code>\n</code>, что иногда требуется удалять с помощью <code>.trim()</code>.</li>
</ul>
<blockquote><p>Совет: чтобы избавиться от <code>\n</code>, используйте <code>.trim()</code></p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">name</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">input</span><span class="p">.</span><span class="n">trim</span><span class="p">();</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Hello </span><span class="si">{name}</span><span class="s">&#34;</span><span class="p">);</span></span></span></code></pre></div></div>
</blockquote>
<h2 class="relative group">Заключение
    <div id="заключение" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b7%d0%b0%d0%ba%d0%bb%d1%8e%d1%87%d0%b5%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Мы познакомились с базовым способом взаимодействия с пользователем в консольном приложении на Rust. Хотя код получился коротким. Под капотом - работа с изменяемыми ссылками, возвратом <code>Result</code>, безопасной обработкой ввода и особенностями строк.</p>
<p>На этом всё. Пробуйте, меняйте, экспериментируйте.</p>
<div id="typeit-00e7a450b8a1ed0bb090fdc25bb068cb"></div>

<script>
    document.addEventListener("DOMContentLoaded", function () {
      var strings = ["","Продолжение следует...",""];
      
      new TypeIt("#typeit-00e7a450b8a1ed0bb090fdc25bb068cb", {
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


