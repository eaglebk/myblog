# Поиск данных в файле - Cli хелпер для мастера с помощью Rust

> Создаём утилиту командной строки на Rust для поиска устройств в конфигурационном файле по описанию. Пошаговое руководство: чтение аргументов, обработка строк, парсинг блоков, поиск и вывод.

- **URL:** https://eagle2000.space/rust/rust-cli-sh/
- **Date:** 2025-06-17

- **Tags:** rust, dev

---


<h2 class="relative group">Введение
    <div id="введение" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d0%b2%d0%b5%d0%b4%d0%b5%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Сегодня мы создадим консольную утилиту на Rust. По задумке она должна помочь мастеру находить устройства в конфигурации умного дома по короткому запросу.</p>
<blockquote><p>Например можно ввести: &ldquo;лампочка&rdquo; или &ldquo;розетка&rdquo;.</p>
<p>Это практический пример для отладки и настройки систем умного дома.</p>
</blockquote><p>Параллельно с этим мы разберём основы: работу с аргументами, чтение файла, парсинг, фильтрацию и безопасную обработку данных.</p>

<h2 class="relative group">Шаг 0: Создание проекта
    <div id="шаг-0-создание-проекта" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%88%d0%b0%d0%b3-0-%d1%81%d0%be%d0%b7%d0%b4%d0%b0%d0%bd%d0%b8%d0%b5-%d0%bf%d1%80%d0%be%d0%b5%d0%ba%d1%82%d0%b0" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Перед началом работы нам нужно создать структуру проекта. Rust использует инструмент Cargo (<em>о нем мы уже говорили в начальной статье, но можно и напомнить</em>) - это менеджер пакетов, сборщик и система управления зависимостями. Практически все проекты на Rust начинаются с <code>cargo new</code>.</p>
<p>Откроем терминал и выполним команды:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-sh" data-lang="sh"><span class="line"><span class="cl">cargo new smart_helper
</span></span><span class="line"><span class="cl"><span class="nb">cd</span> smart_helper</span></span></code></pre></div></div>

  



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
    
    >Команда создаёт директорию <code>smart_helper</code> с шаблоном проекта: <code>src/main.rs</code>, <code>Cargo.toml</code>, и <code>.gitignore</code>.</span>
</div>


<h3 class="relative group">Что внутри
    <div id="что-внутри" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%87%d1%82%d0%be-%d0%b2%d0%bd%d1%83%d1%82%d1%80%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>После команды <code>cargo new</code> структура проекта будет следующей:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-text" data-lang="text"><span class="line"><span class="cl">smart_helper/
</span></span><span class="line"><span class="cl">├── Cargo.toml
</span></span><span class="line"><span class="cl">└── src
</span></span><span class="line"><span class="cl">    └── main.rs</span></span></code></pre></div></div>

<h4 class="relative group">Cargo.toml
    <div id="cargotoml" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#cargotoml" aria-label="Якорь">#</a>
    </span>
    
</h4>
<p>Файл конфигурации, где указываются:</p>
<ul>
<li>имя и версия пакета</li>
<li>зависимости (dependencies)</li>
<li>тип проекта (бинарный или библиотека <em>в нашем случае бинарный</em>)</li>
</ul>
<p>На данный момент содержимое будет минимальным:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-toml" data-lang="toml"><span class="line"><span class="cl"><span class="p">[</span><span class="nx">package</span><span class="p">]</span>
</span></span><span class="line"><span class="cl"><span class="nx">name</span> <span class="p">=</span> <span class="s2">&#34;smart_helper&#34;</span>
</span></span><span class="line"><span class="cl"><span class="nx">version</span> <span class="p">=</span> <span class="s2">&#34;0.1.0&#34;</span>
</span></span><span class="line"><span class="cl"><span class="nx">edition</span> <span class="p">=</span> <span class="s2">&#34;2024&#34;</span>
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl"><span class="p">[</span><span class="nx">dependencies</span><span class="p">]</span></span></span></code></pre></div></div>

<h4 class="relative group">main.rs
    <div id="mainrs" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#mainrs" aria-label="Якорь">#</a>
    </span>
    
</h4>
<p>Файл <code>src/main.rs</code> — точка входа:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">main</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Hello, world!&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p>Мы его позже заменим на логику CLI-приложения.</p>

<h4 class="relative group">Запуск проекта
    <div id="запуск-проекта" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b7%d0%b0%d0%bf%d1%83%d1%81%d0%ba-%d0%bf%d1%80%d0%be%d0%b5%d0%ba%d1%82%d0%b0" aria-label="Якорь">#</a>
    </span>
    
</h4>
<p>Проверим, что проект компилируется:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-sh" data-lang="sh"><span class="line"><span class="cl">cargo run</span></span></code></pre></div></div>
<p>Вывод должен быть:</p>
<p><code>Hello, world!</code></p>
<blockquote><p>Совет: Никогда не начинай Rust-проект без cargo new. Это избавит от множества ручных ошибок и создаст правильную структуру сразу.</p>
</blockquote><p>Готово! У нас есть чистая заготовка для приложения. Теперь можно переходить к следующему шагу — работе с аргументами командной строки.</p>

<h2 class="relative group">Шаг 1: Получение аргументов командной строки
    <div id="шаг-1-получение-аргументов-командной-строки" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%88%d0%b0%d0%b3-1-%d0%bf%d0%be%d0%bb%d1%83%d1%87%d0%b5%d0%bd%d0%b8%d0%b5-%d0%b0%d1%80%d0%b3%d1%83%d0%bc%d0%b5%d0%bd%d1%82%d0%be%d0%b2-%d0%ba%d0%be%d0%bc%d0%b0%d0%bd%d0%b4%d0%bd%d0%be%d0%b9-%d1%81%d1%82%d1%80%d0%be%d0%ba%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Язык Rust предоставляет модуль std::env. Он позволяет взаимодействовать с окружением операционной системы. Кроме того он позволяет получить доступ к аргументам командной строки - именно этой возможностью мы и воспользуемся.</p>

<h3 class="relative group">Начнём с кода:
    <div id="начнём-с-кода" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bd%d0%b0%d1%87%d0%bd%d1%91%d0%bc-%d1%81-%d0%ba%d0%be%d0%b4%d0%b0" aria-label="Якорь">#</a>
    </span>
    
</h3>

<h4 class="relative group">Что потребуется подключить?
    <div id="что-потребуется-подключить" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%87%d1%82%d0%be-%d0%bf%d0%be%d1%82%d1%80%d0%b5%d0%b1%d1%83%d0%b5%d1%82%d1%81%d1%8f-%d0%bf%d0%be%d0%b4%d0%ba%d0%bb%d1%8e%d1%87%d0%b8%d1%82%d1%8c" aria-label="Якорь">#</a>
    </span>
    
</h4>
<p>Нам понадобится стандартная библиотека Rust. А именно подключенные модули:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">use</span><span class="w"> </span><span class="n">std</span>::<span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">env</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">error</span>::<span class="n">Error</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">fs</span>::<span class="n">File</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">io</span>::<span class="p">{</span><span class="n">BufRead</span><span class="p">,</span><span class="w"> </span><span class="n">BufReader</span><span class="p">},</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">};</span></span></span></code></pre></div></div>

<h4 class="relative group">Собираем аргументы
    <div id="собираем-аргументы" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%81%d0%be%d0%b1%d0%b8%d1%80%d0%b0%d0%b5%d0%bc-%d0%b0%d1%80%d0%b3%d1%83%d0%bc%d0%b5%d0%bd%d1%82%d1%8b" aria-label="Якорь">#</a>
    </span>
    
</h4>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">main</span><span class="p">()</span><span class="w"> </span>-&gt; <span class="nb">Result</span><span class="o">&lt;</span><span class="p">(),</span><span class="w"> </span><span class="nb">Box</span><span class="o">&lt;</span><span class="k">dyn</span><span class="w"> </span><span class="n">Error</span><span class="o">&gt;&gt;</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="n">args</span>: <span class="nb">Vec</span><span class="o">&lt;</span><span class="nb">String</span><span class="o">&gt;</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">env</span>::<span class="n">args</span><span class="p">().</span><span class="n">collect</span><span class="p">();</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">if</span><span class="w"> </span><span class="n">args</span><span class="p">.</span><span class="n">len</span><span class="p">()</span><span class="w"> </span><span class="o">!=</span><span class="w"> </span><span class="mi">3</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Использование: smart_helper &lt;config_file&gt; &lt;поисковая_фраза&gt;&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="k">return</span><span class="w"> </span><span class="nb">Ok</span><span class="p">(());</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="nb">Ok</span><span class="p">(())</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<ul>
<li><code>env::args()</code>  *(строка 1)*
 - функция из модуля <code>std::env</code>. Она возвращает итератор по аргументам, переданным в командной строке.</li>
<li>Аргументы возвращаются как <code>std::env::Args</code> - это &ldquo;ленивый итератор&rdquo; который мы собираем в вектор с помощью <code>.collect()</code> (<em>про итераторы мы поговорим в другой раз, сейчас надо представлять их как функцию которая возвращает следующий доступный элемент при вызове, а ленивый потому что функция не вызвается сразу, а только когда после неё будет вызван специальный метод, как в нашем случае collect</em>)</li>
<li><code>.collect()</code>  *(строка 2)*
 преобразует итератор в коллекцию <code>Vec&lt;String&gt;</code> - удобный для нас тип. Он позволяет обращаться к аргументам по индексу.</li>
<li>Первый элемент <code>args[0]</code> - это путь к исполняемому файлу. Поэтому пользовательские значения начинаются с args[1].</li>
</ul>
<p>
  



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
    
    >
<h4 class="relative group">Почему <code>Vec&lt;String&gt;?</code>
    <div id="почему-vecstring" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d0%be%d1%87%d0%b5%d0%bc%d1%83-vecstring" aria-label="Якорь">#</a>
    </span>
    
</h4>
<ul>
<li>env::args() возвращает элементы типа <code>String</code>, а не <code>&amp;str</code>, потому что данные копируются из системного ввода в память программы. Это значит:
<ul>
<li>нельзя использовать <code>&amp;str</code> так как у нас нет ссылки на оригинальные аргументы;</li>
<li><code>String</code> это безопасный способ владения строками в Rust;</li>
<li><code>Vec&lt;String&gt;</code> даёт произвольный доступ, например args[1], args[2], что невозможно с чистым итератором.</li>
</ul>
</li>
</ul>
</span>
</div>

</br></p>

<h4 class="relative group">Зачем мы проверяем длину через len?
    <div id="зачем-мы-проверяем-длину-через-len" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b7%d0%b0%d1%87%d0%b5%d0%bc-%d0%bc%d1%8b-%d0%bf%d1%80%d0%be%d0%b2%d0%b5%d1%80%d1%8f%d0%b5%d0%bc-%d0%b4%d0%bb%d0%b8%d0%bd%d1%83-%d1%87%d0%b5%d1%80%d0%b5%d0%b7-len" aria-label="Якорь">#</a>
    </span>
    
</h4>
<p><code>if args.len() != 3</code></p>
<p>Потому что мы ожидаем:</p>
<ol>
<li><code>args[0]</code> - путь к самому бинарнику;</li>
<li><code>args[1]</code> — путь к конфигурационному файлу;</li>
<li><code>args[2]</code> — строка поиска.
Если аргументов меньше или больше - приложение выведет подсказку и завершится корректно.</li>
</ol>

  



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
    
    >Если не проверять <code>args.len()</code> программа может аварийно завершиться при попытке обратиться к несуществующему элементу в <code>Vec</code>.</span>
</div>


<h3 class="relative group">Альтернативный вариант: итератор без collect
    <div id="альтернативный-вариант-итератор-без-collect" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b0%d0%bb%d1%8c%d1%82%d0%b5%d1%80%d0%bd%d0%b0%d1%82%d0%b8%d0%b2%d0%bd%d1%8b%d0%b9-%d0%b2%d0%b0%d1%80%d0%b8%d0%b0%d0%bd%d1%82-%d0%b8%d1%82%d0%b5%d1%80%d0%b0%d1%82%d0%be%d1%80-%d0%b1%d0%b5%d0%b7-collect" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Можно было бы сразу использовать <code>let mut args = env::args();</code> но это неудобно:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">config_path</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">args</span><span class="p">.</span><span class="n">nth</span><span class="p">(</span><span class="mi">1</span><span class="p">).</span><span class="n">unwrap</span><span class="p">();</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">query</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">args</span><span class="p">.</span><span class="n">next</span><span class="p">().</span><span class="n">unwrap</span><span class="p">();</span></span></span></code></pre></div></div>
<p>Этот стиль менее удобен для новичков и не позволяет легко вывести ошибку, если аргументов недостаточно. Поэтому в этом проекте мы используем явный <code>Vec&lt;String&gt;</code>.</p>

<h2 class="relative group">Шаг 2: Открытие и буферизация файла
    <div id="шаг-2-открытие-и-буферизация-файла" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%88%d0%b0%d0%b3-2-%d0%be%d1%82%d0%ba%d1%80%d1%8b%d1%82%d0%b8%d0%b5-%d0%b8-%d0%b1%d1%83%d1%84%d0%b5%d1%80%d0%b8%d0%b7%d0%b0%d1%86%d0%b8%d1%8f-%d1%84%d0%b0%d0%b9%d0%bb%d0%b0" aria-label="Якорь">#</a>
    </span>
    
</h2>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">file</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">File</span>::<span class="n">open</span><span class="p">(</span><span class="o">&amp;</span><span class="n">args</span><span class="p">[</span><span class="mi">1</span><span class="p">])</span><span class="o">?</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">reader</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">BufReader</span>::<span class="n">new</span><span class="p">(</span><span class="n">file</span><span class="p">);</span></span></span></code></pre></div></div>
<ul>
<li><code>BufReader</code> даёт построчный доступ без загрузки всего файла в память.
<ul>
<li>(<em>Про BufReader мы также еще поговорим отдельно</em>)</li>
</ul>
</li>
</ul>

<h2 class="relative group">Шаг 3: Разбор одного блока конфигурации
    <div id="шаг-3-разбор-одного-блока-конфигурации" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%88%d0%b0%d0%b3-3-%d1%80%d0%b0%d0%b7%d0%b1%d0%be%d1%80-%d0%be%d0%b4%d0%bd%d0%be%d0%b3%d0%be-%d0%b1%d0%bb%d0%be%d0%ba%d0%b0-%d0%ba%d0%be%d0%bd%d1%84%d0%b8%d0%b3%d1%83%d1%80%d0%b0%d1%86%d0%b8%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Мы подошли к важной части: нужно распарсить (разобрать на составляющие) один блок из текстового файла, который содержит описание устройства. Конфигурация у нас будет выглядеть вот так:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-txt" data-lang="txt"><span class="line"><span class="cl">device_id: light_hue_rgb_03
</span></span><span class="line"><span class="cl">name: Philips Hue RGB лампочка
</span></span><span class="line"><span class="cl">location: Гостиная (люстра)
</span></span><span class="line"><span class="cl">serial: PH-HUE-03
</span></span><span class="line"><span class="cl">features: [диммируемая, цветная]</span></span></code></pre></div></div>
<p>Блоки разделяются через строку <code>&quot;---&quot;</code></p>
<p>Наша задача - превратить этот блок в структуру <code>Device</code> с которой уже можно работать как с Rust-объектом.</p>

<h3 class="relative group">Структура устройства
    <div id="структура-устройства" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%81%d1%82%d1%80%d1%83%d0%ba%d1%82%d1%83%d1%80%d0%b0-%d1%83%d1%81%d1%82%d1%80%d0%be%d0%b9%d1%81%d1%82%d0%b2%d0%b0" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Сначала определим структуру, в которую будем собирать данные:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="cp">#[derive(Default)]</span><span class="w">  </span><span class="n"> `[3]`
</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">struct</span> <span class="nc">Device</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">device_id</span>: <span class="nb">String</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">name</span>: <span class="nb">String</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">location</span>: <span class="nb">String</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">serial</span>: <span class="nb">String</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">features</span>: <span class="nb">Vec</span><span class="o">&lt;</span><span class="nb">String</span><span class="o">&gt;</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>

  



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
    
    ><code>#[derive(Default)]</code>  <em>(строка 3)</em>
позволяет создать пустой <code>Device</code> по умолчанию вызовом <code>Device::default()</code> это удобно для постепенного заполнения.</span>
</div>


<h3 class="relative group">Парсинг блока
    <div id="парсинг-блока" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d0%b0%d1%80%d1%81%d0%b8%d0%bd%d0%b3-%d0%b1%d0%bb%d0%be%d0%ba%d0%b0" aria-label="Якорь">#</a>
    </span>
    
</h3>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">parse_device</span><span class="p">(</span><span class="n">block</span>: <span class="kp">&amp;</span><span class="kt">str</span><span class="p">)</span><span class="w"> </span>-&gt; <span class="nb">Option</span><span class="o">&lt;</span><span class="n">Device</span><span class="o">&gt;</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="k">mut</span><span class="w"> </span><span class="n">device</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">Device</span>::<span class="n">default</span><span class="p">();</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">for</span><span class="w"> </span><span class="n">line</span><span class="w"> </span><span class="k">in</span><span class="w"> </span><span class="n">block</span><span class="p">.</span><span class="n">lines</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w"> </span><span class="n"> `[4]`
</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="kd">let</span><span class="w"> </span><span class="n">parts</span>: <span class="nb">Vec</span><span class="o">&lt;&amp;</span><span class="kt">str</span><span class="o">&gt;</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">line</span><span class="p">.</span><span class="n">splitn</span><span class="p">(</span><span class="mi">2</span><span class="p">,</span><span class="w"> </span><span class="sc">&#39;:&#39;</span><span class="p">).</span><span class="n">collect</span><span class="p">();</span><span class="w"> </span><span class="n"> `[5]`
</span><span class="w"> 
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="k">if</span><span class="w"> </span><span class="n">parts</span><span class="p">.</span><span class="n">len</span><span class="p">()</span><span class="w"> </span><span class="o">!=</span><span class="w"> </span><span class="mi">2</span><span class="w"> </span><span class="p">{</span><span class="w"> </span><span class="k">continue</span><span class="p">;</span><span class="w"> </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="kd">let</span><span class="w"> </span><span class="n">key</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">parts</span><span class="p">[</span><span class="mi">0</span><span class="p">].</span><span class="n">trim</span><span class="p">();</span><span class="w"> </span><span class="n"> `[6]`
</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="kd">let</span><span class="w"> </span><span class="n">value</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">parts</span><span class="p">[</span><span class="mi">1</span><span class="p">].</span><span class="n">trim</span><span class="p">();</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="k">match</span><span class="w"> </span><span class="n">key</span><span class="w"> </span><span class="p">{</span><span class="w"> </span><span class="n"> `[7]`
</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">            </span><span class="s">&#34;device_id&#34;</span><span class="w"> </span><span class="o">=&gt;</span><span class="w"> </span><span class="n">device</span><span class="p">.</span><span class="n">device_id</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">value</span><span class="p">.</span><span class="n">to_string</span><span class="p">(),</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">            </span><span class="s">&#34;name&#34;</span><span class="w"> </span><span class="o">=&gt;</span><span class="w"> </span><span class="n">device</span><span class="p">.</span><span class="n">name</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">value</span><span class="p">.</span><span class="n">to_string</span><span class="p">(),</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">            </span><span class="s">&#34;location&#34;</span><span class="w"> </span><span class="o">=&gt;</span><span class="w"> </span><span class="n">device</span><span class="p">.</span><span class="n">location</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">value</span><span class="p">.</span><span class="n">to_string</span><span class="p">(),</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">            </span><span class="s">&#34;serial&#34;</span><span class="w"> </span><span class="o">=&gt;</span><span class="w"> </span><span class="n">device</span><span class="p">.</span><span class="n">serial</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">value</span><span class="p">.</span><span class="n">to_string</span><span class="p">(),</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">            </span><span class="s">&#34;features&#34;</span><span class="w"> </span><span class="o">=&gt;</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">                </span><span class="kd">let</span><span class="w"> </span><span class="n">cleaned</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">value</span><span class="p">.</span><span class="n">trim_matches</span><span class="p">(</span><span class="o">&amp;</span><span class="p">[</span><span class="sc">&#39;[&#39;</span><span class="p">,</span><span class="w"> </span><span class="sc">&#39;]&#39;</span><span class="p">][</span><span class="o">..</span><span class="p">]);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">                </span><span class="n">device</span><span class="p">.</span><span class="n">features</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">cleaned</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">                    </span><span class="p">.</span><span class="n">split</span><span class="p">(</span><span class="sc">&#39;,&#39;</span><span class="p">)</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">                    </span><span class="p">.</span><span class="n">map</span><span class="p">(</span><span class="o">|</span><span class="n">s</span><span class="o">|</span><span class="w"> </span><span class="n">s</span><span class="p">.</span><span class="n">trim</span><span class="p">().</span><span class="n">to_string</span><span class="p">())</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">                    </span><span class="p">.</span><span class="n">collect</span><span class="p">();</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">            </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">            </span><span class="n">_</span><span class="w"> </span><span class="o">=&gt;</span><span class="w"> </span><span class="p">{}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="nb">Some</span><span class="p">(</span><span class="n">device</span><span class="p">)</span><span class="w"> </span><span class="n"> `[8]`
</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p>Много кода получилось, давайте немного остановимся и разберём строки подробнее:</p>

<h4 class="relative group"><code>block.lines()</code>
    <div id="blocklines" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#blocklines" aria-label="Якорь">#</a>
    </span>
    
</h4>
<p> *(строка 4)*
 Функция lines() создаёт итератор по строкам.</p>
<p>Например из этого блока конфига:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-txt" data-lang="txt"><span class="line"><span class="cl">device_id: light01
</span></span><span class="line"><span class="cl">name: Умная лампочка</span></span></code></pre></div></div>
<p>получим:</p>
<p><code>[&quot;device_id: light01&quot;, &quot;name: Умная лампочка&quot;]</code></p>

<h4 class="relative group"><code>splitn(2, ':')</code>
    <div id="splitn2-" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#splitn2-" aria-label="Якорь">#</a>
    </span>
    
</h4>
<p> *(строка 5)*
 Используем splitn(2, &lsquo;:&rsquo;) — это важно потому что эта функция разбивает строку <strong>только один раз</strong> даже если значение содержит <code>:</code></p>
<p>Пример:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-txt" data-lang="txt"><span class="line"><span class="cl">location: Гостиная: зона 2</span></span></code></pre></div></div>
<p>Результат:
<code>[&quot;location&quot;, &quot;Гостиная: зона 2&quot;]</code></p>

  



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
    
    >Если бы мы использовали split(&rsquo;:&rsquo;), результат был бы некорректен — &lsquo;Гостиная&rsquo; и &rsquo; зона 2&rsquo; попали бы в разные части.</span>
</div>


<h4 class="relative group"><code>trim()</code>
    <div id="trim" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#trim" aria-label="Якорь">#</a>
    </span>
    
</h4>
<p> *(строка 6)*
 <code>trim()</code> убирает пробелы и символы перевода строки с обеих сторон:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">key</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">parts</span><span class="p">[</span><span class="mi">0</span><span class="p">].</span><span class="n">trim</span><span class="p">();</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">value</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">parts</span><span class="p">[</span><span class="mi">1</span><span class="p">].</span><span class="n">trim</span><span class="p">();</span></span></span></code></pre></div></div>
<p>Это избавляет от проблем при работе с файлами, особенно если строки написаны вручную.</p>

<h4 class="relative group"><code>match key</code>
    <div id="match-key" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#match-key" aria-label="Якорь">#</a>
    </span>
    
</h4>
<p> *(строка 7)*
 Мы проходим по всем строкам и в зависимости от поля присваиваем значение в device.</p>
<p>Разберём что происходит:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="s">&#34;device_id&#34;</span><span class="w"> </span><span class="o">=&gt;</span><span class="w"> </span><span class="n">device</span><span class="p">.</span><span class="n">device_id</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">value</span><span class="p">.</span><span class="n">to_string</span><span class="p">()</span></span></span></code></pre></div></div>
<ul>
<li><code>.to_string()</code> копирует <code>&amp;str</code> в <code>String</code> — потому что структура содержит <code>String</code> а не ссылку.</li>
</ul>

<h4 class="relative group">Разбор <code>features</code>
    <div id="разбор-features" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%80%d0%b0%d0%b7%d0%b1%d0%be%d1%80-features" aria-label="Якорь">#</a>
    </span>
    
</h4>
<p>Поле features хранится в виде строки:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-txt" data-lang="txt"><span class="line"><span class="cl">[диммируемая, цветная]</span></span></code></pre></div></div>
<p>Чтобы превратить её в <code>Vec&lt;String&gt;</code>:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">cleaned</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">value</span><span class="p">.</span><span class="n">trim_matches</span><span class="p">(</span><span class="o">&amp;</span><span class="p">[</span><span class="sc">&#39;[&#39;</span><span class="p">,</span><span class="w"> </span><span class="sc">&#39;]&#39;</span><span class="p">][</span><span class="o">..</span><span class="p">]);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="n">device</span><span class="p">.</span><span class="n">features</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">cleaned</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">.</span><span class="n">split</span><span class="p">(</span><span class="sc">&#39;,&#39;</span><span class="p">)</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">.</span><span class="n">map</span><span class="p">(</span><span class="o">|</span><span class="n">s</span><span class="o">|</span><span class="w"> </span><span class="n">s</span><span class="p">.</span><span class="n">trim</span><span class="p">().</span><span class="n">to_string</span><span class="p">())</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">.</span><span class="n">collect</span><span class="p">();</span></span></span></code></pre></div></div>

<h5 class="relative group">Пояснение
    <div id="пояснение" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d0%be%d1%8f%d1%81%d0%bd%d0%b5%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h5>
<ul>
<li><code>trim_matches(&amp;['[', ']'])</code> убирает квадратные скобки.</li>
<li><code>split(',')</code> разбивает строку по запятой.</li>
<li><code>map(|s| s.trim().to_string())</code> — убирает пробелы, превращает в <code>String</code>.</li>
<li><code>collect()</code> собирает вектор.</li>
</ul>

<h4 class="relative group">Возврат результата
    <div id="возврат-результата" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d0%be%d0%b7%d0%b2%d1%80%d0%b0%d1%82-%d1%80%d0%b5%d0%b7%d1%83%d0%bb%d1%8c%d1%82%d0%b0%d1%82%d0%b0" aria-label="Якорь">#</a>
    </span>
    
</h4>
<p>В конце  *(строка 8)*
 <code>Some(device)</code></p>
<p>Мы не обрабатываем ошибки парсинга явно — просто пропускаем некорректные строки.</p>

  



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
    
    >Можете заметить что мы вручную разбираем структуру без сторонних библиотек. Это полезно для понимания базовой обработки строк и соответствия ключам.</span>
</div>


<h2 class="relative group">Шаг 4: Основной цикл чтения и фильтрации
    <div id="шаг-4-основной-цикл-чтения-и-фильтрации" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%88%d0%b0%d0%b3-4-%d0%be%d1%81%d0%bd%d0%be%d0%b2%d0%bd%d0%be%d0%b9-%d1%86%d0%b8%d0%ba%d0%bb-%d1%87%d1%82%d0%b5%d0%bd%d0%b8%d1%8f-%d0%b8-%d1%84%d0%b8%d0%bb%d1%8c%d1%82%d1%80%d0%b0%d1%86%d0%b8%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>На данный момент мы подготовили структуру <code>Device</code>, научились её заполнять на основе одного текстового блока. Теперь надо прочитать весь файл, разбить его на блоки и выбрать те, что соответствуют поисковой строке.</p>
<p>Вот фрагмент кода, реализующий эту логику:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="k">mut</span><span class="w"> </span><span class="n">buffer</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="nb">String</span>::<span class="n">new</span><span class="p">();</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="k">mut</span><span class="w"> </span><span class="n">matches</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="nb">Vec</span>::<span class="n">new</span><span class="p">();</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">query</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">args</span><span class="p">[</span><span class="mi">2</span><span class="p">].</span><span class="n">to_lowercase</span><span class="p">();</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">for</span><span class="w"> </span><span class="n">line</span><span class="w"> </span><span class="k">in</span><span class="w"> </span><span class="n">reader</span><span class="p">.</span><span class="n">lines</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="n">line</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">line</span><span class="o">?</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">if</span><span class="w"> </span><span class="n">line</span><span class="p">.</span><span class="n">trim</span><span class="p">()</span><span class="w"> </span><span class="o">==</span><span class="w"> </span><span class="s">&#34;---&#34;</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="k">if</span><span class="w"> </span><span class="kd">let</span><span class="w"> </span><span class="nb">Some</span><span class="p">(</span><span class="n">device</span><span class="p">)</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">parse_device</span><span class="p">(</span><span class="o">&amp;</span><span class="n">buffer</span><span class="p">)</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">            </span><span class="k">if</span><span class="w"> </span><span class="n">device</span><span class="p">.</span><span class="n">name</span><span class="p">.</span><span class="n">to_lowercase</span><span class="p">().</span><span class="n">contains</span><span class="p">(</span><span class="o">&amp;</span><span class="n">query</span><span class="p">)</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">                </span><span class="n">matches</span><span class="p">.</span><span class="n">push</span><span class="p">(</span><span class="n">device</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">            </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="n">buffer</span><span class="p">.</span><span class="n">clear</span><span class="p">();</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">}</span><span class="w"> </span><span class="k">else</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="n">buffer</span><span class="p">.</span><span class="n">push_str</span><span class="p">(</span><span class="o">&amp;</span><span class="n">line</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="n">buffer</span><span class="p">.</span><span class="n">push</span><span class="p">(</span><span class="sc">&#39;\n&#39;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>

<h3 class="relative group">Что же здесь происходит?
    <div id="что-же-здесь-происходит" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%87%d1%82%d0%be-%d0%b6%d0%b5-%d0%b7%d0%b4%d0%b5%d1%81%d1%8c-%d0%bf%d1%80%d0%be%d0%b8%d1%81%d1%85%d0%be%d0%b4%d0%b8%d1%82" aria-label="Якорь">#</a>
    </span>
    
</h3>
<ul>
<li><code>let mut buffer = String::new()</code>
<ul>
<li>Создаём временную строку, в которую будем записывать строки блока конфигурации до тех пор пока не встретим строку-разделитель <code>---</code>.</li>
</ul>
</li>
</ul>
<blockquote><p>Каждый такой буфер содержит один логический блок (устройство) в текстовом представлении.</p>
</blockquote><ul>
<li>
<p><code>let mut matches = Vec::new()</code></p>
<ul>
<li>Вектор, куда мы будем складывать <strong>устройства, прошедшие фильтр</strong>. Это наш итоговый результат.</li>
</ul>
</li>
<li>
<p><code>let query = args[2].to_lowercase()</code></p>
<ul>
<li>Переводим строку поиска в нижний регистр — это позволяет искать независимо от регистра:
<code>lamp == Лампочка == ЛАМПОЧКА</code></li>
</ul>
</li>
</ul>

<h3 class="relative group">Проход по файлу
    <div id="проход-по-файлу" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%be%d1%85%d0%be%d0%b4-%d0%bf%d0%be-%d1%84%d0%b0%d0%b9%d0%bb%d1%83" aria-label="Якорь">#</a>
    </span>
    
</h3>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">for</span><span class="w"> </span><span class="n">line</span><span class="w"> </span><span class="k">in</span><span class="w"> </span><span class="n">reader</span><span class="p">.</span><span class="n">lines</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="n">line</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">line</span><span class="o">?</span><span class="p">;</span></span></span></code></pre></div></div>
<p>Мы читаем файл построчно — <code>BufReader</code> здесь работает максимально эффективно. Каждая строка —&gt; <code>Result&lt;String&gt;</code>, поэтому мы используем ? для автоматического выхода в случае ошибки.</p>
<p>Граница блока:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">if</span><span class="w"> </span><span class="n">line</span><span class="p">.</span><span class="n">trim</span><span class="p">()</span><span class="w"> </span><span class="o">==</span><span class="w"> </span><span class="s">&#34;---&#34;</span></span></span></code></pre></div></div>
<p>Когда встречаем строку <code>---</code>, это значит, что закончился блок. Всё, что накоплено в buffer —&gt; это одно устройство.</p>

<h3 class="relative group">Парсинг + фильтрация
    <div id="парсинг--фильтрация" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d0%b0%d1%80%d1%81%d0%b8%d0%bd%d0%b3--%d1%84%d0%b8%d0%bb%d1%8c%d1%82%d1%80%d0%b0%d1%86%d0%b8%d1%8f" aria-label="Якорь">#</a>
    </span>
    
</h3>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">if</span><span class="w"> </span><span class="kd">let</span><span class="w"> </span><span class="nb">Some</span><span class="p">(</span><span class="n">device</span><span class="p">)</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">parse_device</span><span class="p">(</span><span class="o">&amp;</span><span class="n">buffer</span><span class="p">)</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">if</span><span class="w"> </span><span class="n">device</span><span class="p">.</span><span class="n">name</span><span class="p">.</span><span class="n">to_lowercase</span><span class="p">().</span><span class="n">contains</span><span class="p">(</span><span class="o">&amp;</span><span class="n">query</span><span class="p">)</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="n">matches</span><span class="p">.</span><span class="n">push</span><span class="p">(</span><span class="n">device</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>

<h4 class="relative group">Разбор
    <div id="разбор" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%80%d0%b0%d0%b7%d0%b1%d0%be%d1%80" aria-label="Якорь">#</a>
    </span>
    
</h4>
<ul>
<li><code>parse_device()</code> пытается разобрать buffer в структуру Device</li>
<li><code>if let Some(...)</code> — обрабатывает случай если результат удачный (<code>Some</code>)</li>
<li><code>device.name.to_lowercase().contains(&amp;query)</code> — ищет подстроку без учёта регистра</li>
<li><code>matches.push(device)</code> — сохраняем устройство, которое подошло под критерии</li>
</ul>

<h4 class="relative group">Почему buffer.clear()?
    <div id="почему-bufferclear" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d0%be%d1%87%d0%b5%d0%bc%d1%83-bufferclear" aria-label="Якорь">#</a>
    </span>
    
</h4>
<p>После обработки одного блока мы <strong>обнуляем буфер</strong> чтобы начать записывать следующий.</p>

<h4 class="relative group">Что же делать с последним блоком?
    <div id="что-же-делать-с-последним-блоком" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%87%d1%82%d0%be-%d0%b6%d0%b5-%d0%b4%d0%b5%d0%bb%d0%b0%d1%82%d1%8c-%d1%81-%d0%bf%d0%be%d1%81%d0%bb%d0%b5%d0%b4%d0%bd%d0%b8%d0%bc-%d0%b1%d0%bb%d0%be%d0%ba%d0%be%d0%bc" aria-label="Якорь">#</a>
    </span>
    
</h4>
<p>Если в конце файла нет завершающей строки <code>---</code>, последний блок так и останется в <code>buffer</code>. Поэтому отдельно обрабатываем его после цикла:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">if</span><span class="w"> </span><span class="o">!</span><span class="n">buffer</span><span class="p">.</span><span class="n">trim</span><span class="p">().</span><span class="n">is_empty</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">if</span><span class="w"> </span><span class="kd">let</span><span class="w"> </span><span class="nb">Some</span><span class="p">(</span><span class="n">device</span><span class="p">)</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">parse_device</span><span class="p">(</span><span class="o">&amp;</span><span class="n">buffer</span><span class="p">)</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="k">if</span><span class="w"> </span><span class="n">device</span><span class="p">.</span><span class="n">name</span><span class="p">.</span><span class="n">to_lowercase</span><span class="p">().</span><span class="n">contains</span><span class="p">(</span><span class="o">&amp;</span><span class="n">query</span><span class="p">)</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">            </span><span class="n">matches</span><span class="p">.</span><span class="n">push</span><span class="p">(</span><span class="n">device</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<blockquote><p>Это защита от потери последнего элемента - частая ошибка при обработке по разделителям.</p>
</blockquote>
<h2 class="relative group">Шаг 5: Вывод результата
    <div id="шаг-5-вывод-результата" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%88%d0%b0%d0%b3-5-%d0%b2%d1%8b%d0%b2%d0%be%d0%b4-%d1%80%d0%b5%d0%b7%d1%83%d0%bb%d1%8c%d1%82%d0%b0%d1%82%d0%b0" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Наконец мы собрали все подходящие устройства в <code>Vec&lt;Device&gt;</code> под названием matches. Теперь нужно вывести их в консоль.</p>
<p>Rust не имеет встроенного форматирования таблиц или цветного вывода — всё делается вручную через <code>println!</code>.</p>
<p>Вот полный блок кода:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">if</span><span class="w"> </span><span class="n">matches</span><span class="p">.</span><span class="n">is_empty</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;🚫 Ничего не найдено по запросу &#39;</span><span class="si">{}</span><span class="s">&#39;&#34;</span><span class="p">,</span><span class="w"> </span><span class="n">query</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w"> </span><span class="k">else</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;🔍 Найдено </span><span class="si">{}</span><span class="s"> совпадений:&#34;</span><span class="p">,</span><span class="w"> </span><span class="n">matches</span><span class="p">.</span><span class="n">len</span><span class="p">());</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">for</span><span class="w"> </span><span class="n">device</span><span class="w"> </span><span class="k">in</span><span class="w"> </span><span class="n">matches</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Название: </span><span class="si">{}</span><span class="s">&#34;</span><span class="p">,</span><span class="w"> </span><span class="n">device</span><span class="p">.</span><span class="n">name</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;ID: </span><span class="si">{}</span><span class="s">&#34;</span><span class="p">,</span><span class="w"> </span><span class="n">device</span><span class="p">.</span><span class="n">device_id</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Местоположение: </span><span class="si">{}</span><span class="s">&#34;</span><span class="p">,</span><span class="w"> </span><span class="n">device</span><span class="p">.</span><span class="n">location</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Серийный номер: </span><span class="si">{}</span><span class="s">&#34;</span><span class="p">,</span><span class="w"> </span><span class="n">device</span><span class="p">.</span><span class="n">serial</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Возможности: </span><span class="si">{}</span><span class="s">&#34;</span><span class="p">,</span><span class="w"> </span><span class="n">device</span><span class="p">.</span><span class="n">features</span><span class="p">.</span><span class="n">join</span><span class="p">(</span><span class="s">&#34;, &#34;</span><span class="p">));</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>

<h3 class="relative group">Проверка на пустой результат
    <div id="проверка-на-пустой-результат" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%be%d0%b2%d0%b5%d1%80%d0%ba%d0%b0-%d0%bd%d0%b0-%d0%bf%d1%83%d1%81%d1%82%d0%be%d0%b9-%d1%80%d0%b5%d0%b7%d1%83%d0%bb%d1%8c%d1%82%d0%b0%d1%82" aria-label="Якорь">#</a>
    </span>
    
</h3>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">if</span><span class="w"> </span><span class="n">matches</span><span class="p">.</span><span class="n">is_empty</span><span class="p">()</span></span></span></code></pre></div></div>
<p>Мы не хотим запускать цикл, если ничего не найдено. Метод <code>.is_empty()</code> проверяет, что длина вектора равна нулю.</p>

  



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
    
    >Это читаемый и эффективный способ, эквивалент matches.len() == 0, но он более привычный и используемый.</span>
</div>


<h3 class="relative group">Вывод количества совпадений
    <div id="вывод-количества-совпадений" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d1%8b%d0%b2%d0%be%d0%b4-%d0%ba%d0%be%d0%bb%d0%b8%d1%87%d0%b5%d1%81%d1%82%d0%b2%d0%b0-%d1%81%d0%be%d0%b2%d0%bf%d0%b0%d0%b4%d0%b5%d0%bd%d0%b8%d0%b9" aria-label="Якорь">#</a>
    </span>
    
</h3>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;🔍 Найдено </span><span class="si">{}</span><span class="s"> совпадений:&#34;</span><span class="p">,</span><span class="w"> </span><span class="n">matches</span><span class="p">.</span><span class="n">len</span><span class="p">());</span></span></span></code></pre></div></div>
<p>Форматирование через макрос <code>println!</code> с подстановкой переменной. Здесь<code>matches.len()</code> — возвращает <code>usize</code>, который можно спокойно вставить в форматируемую строку.</p>

<h2 class="relative group">Пример конфигурационного файла
    <div id="пример-конфигурационного-файла" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%b8%d0%bc%d0%b5%d1%80-%d0%ba%d0%be%d0%bd%d1%84%d0%b8%d0%b3%d1%83%d1%80%d0%b0%d1%86%d0%b8%d0%be%d0%bd%d0%bd%d0%be%d0%b3%d0%be-%d1%84%d0%b0%d0%b9%d0%bb%d0%b0" aria-label="Якорь">#</a>
    </span>
    
</h2>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-txt" data-lang="txt"><span class="line"><span class="cl">device_id: light_hue_rgb_01
</span></span><span class="line"><span class="cl">name: Умная лампочка Philips Hue RGB
</span></span><span class="line"><span class="cl">location: Гостиная
</span></span><span class="line"><span class="cl">serial: PH-HUE-01
</span></span><span class="line"><span class="cl">features: [диммируемая, цветная]
</span></span><span class="line"><span class="cl">---
</span></span><span class="line"><span class="cl">device_id: plug_lenovo_16a
</span></span><span class="line"><span class="cl">name: Розетка умная Lenovo
</span></span><span class="line"><span class="cl">location: Кухня
</span></span><span class="line"><span class="cl">serial: LN-PLUG-44891
</span></span><span class="line"><span class="cl">features: [мониторинг_питания]</span></span></code></pre></div></div>

<h2 class="relative group">Пример работы
    <div id="пример-работы" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%b8%d0%bc%d0%b5%d1%80-%d1%80%d0%b0%d0%b1%d0%be%d1%82%d1%8b" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Если в файле конфига есть подходящие устройства и мы запустили:</p>
<p><code>cargo run devices.txt ламп</code></p>
<p>Вывод может быть таким:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-yaml" data-lang="yaml"><span class="line"><span class="cl"><span class="nt">🔍 Найдено 1 совпадений</span><span class="p">:</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="nt">Название</span><span class="p">:</span><span class="w"> </span><span class="l">Умная лампочка Philips Hue RGB</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="nt">ID</span><span class="p">:</span><span class="w"> </span><span class="l">light_hue_rgb_01</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="nt">Местоположение</span><span class="p">:</span><span class="w"> </span><span class="l">Гостиная (люстра)</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="nt">Серийный номер</span><span class="p">:</span><span class="w"> </span><span class="l">PH-HUE-01</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="nt">Возможности</span><span class="p">:</span><span class="w"> </span><span class="l">диммируемая, цветная</span></span></span></code></pre></div></div>
<p>Готово! 🚀 Мы не просто написали утилиту, а разобрали <strong>все ключевые части Rust-приложения</strong>: от создания проекта и ввода данных до построчного чтения, обработки и красивого вывода.</p>

<h2 class="relative group">Заключение
    <div id="заключение" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b7%d0%b0%d0%ba%d0%bb%d1%8e%d1%87%d0%b5%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Мы создали полноценную CLI-утилиту на Rust для поиска устройств по описанию из конфигурационного файла. Приложение построено на стандартной библиотеке, без сторонних зависимостей. При этом оно уже решает реальную задачу.</p>
<p>Что мы изучили в процессе написания этого проекта:</p>
<ul>
<li>Работа с аргументами командной строки</li>
<li>Построчное чтение файла</li>
<li>Поиск по подстроке</li>
<li>Парсинг структур вручную</li>
</ul>

  



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
    
    >Мы написали простой, но качественный инструмент. А ещё — разобрали важные аспекты Rust, от итераторов до ошибок, от строк до структур. Это именно тот путь, по которому изучается язык.</span>
</div>

<br>
<p>👇🏻 Полный код проекта вы можете найти в репозитории</p>
<div class="github-card-wrapper">
    <a id="github-b24e42b6dbb7f09a80c9b9f35ee214a9" target="_blank" href="https://github.com/eaglebk/smart_helper" class="cursor-pointer">
      <div
        class="w-full md:w-auto p-0 m-0 border border-neutral-200 dark:border-neutral-700 border rounded-md shadow-2xl"><div class="w-full nozoom">
            <img
              src="https://opengraph.githubassets.com/0/eaglebk/smart_helper"
              alt="GitHub Repository Thumbnail"
              class="nozoom mt-0 mb-0 w-full h-full object-cover">
          </div><div class="w-full md:w-auto pt-3 p-5">
          <div class="flex items-center">
            <span class="text-2xl text-neutral-800 dark:text-neutral me-2">
              <span class="relative block icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 496 512"><path fill="currentColor" d="M165.9 397.4c0 2-2.3 3.6-5.2 3.6-3.3.3-5.6-1.3-5.6-3.6 0-2 2.3-3.6 5.2-3.6 3-.3 5.6 1.3 5.6 3.6zm-31.1-4.5c-.7 2 1.3 4.3 4.3 4.9 2.6 1 5.6 0 6.2-2s-1.3-4.3-4.3-5.2c-2.6-.7-5.5.3-6.2 2.3zm44.2-1.7c-2.9.7-4.9 2.6-4.6 4.9.3 2 2.9 3.3 5.9 2.6 2.9-.7 4.9-2.6 4.6-4.6-.3-1.9-3-3.2-5.9-2.9zM244.8 8C106.1 8 0 113.3 0 252c0 110.9 69.8 205.8 169.5 239.2 12.8 2.3 17.3-5.6 17.3-12.1 0-6.2-.3-40.4-.3-61.4 0 0-70 15-84.7-29.8 0 0-11.4-29.1-27.8-36.6 0 0-22.9-15.7 1.6-15.4 0 0 24.9 2 38.6 25.8 21.9 38.6 58.6 27.5 72.9 20.9 2.3-16 8.8-27.1 16-33.7-55.9-6.2-112.3-14.3-112.3-110.5 0-27.5 7.6-41.3 23.6-58.9-2.6-6.5-11.1-33.3 2.6-67.9 20.9-6.5 69 27 69 27 20-5.6 41.5-8.5 62.8-8.5s42.8 2.9 62.8 8.5c0 0 48.1-33.6 69-27 13.7 34.7 5.2 61.4 2.6 67.9 16 17.7 25.8 31.5 25.8 58.9 0 96.5-58.9 104.2-114.8 110.5 9.2 7.9 17 22.9 17 46.4 0 33.7-.3 75.4-.3 83.6 0 6.5 4.6 14.4 17.3 12.1C428.2 457.8 496 362.9 496 252 496 113.3 383.5 8 244.8 8zM97.2 352.9c-1.3 1-1 3.3.7 5.2 1.6 1.6 3.9 2.3 5.2 1 1.3-1 1-3.3-.7-5.2-1.6-1.6-3.9-2.3-5.2-1zm-10.8-8.1c-.7 1.3.3 2.9 2.3 3.9 1.6 1 3.6.7 4.3-.7.7-1.3-.3-2.9-2.3-3.9-2-.6-3.6-.3-4.3.7zm32.4 35.6c-1.6 1.3-1 4.3 1.3 6.2 2.3 2.3 5.2 2.6 6.5 1 1.3-1.3.7-4.3-1.3-6.2-2.2-2.3-5.2-2.6-6.5-1zm-11.4-14.7c-1.6 1-1.6 3.6 0 5.9 1.6 2.3 4.3 3.3 5.6 2.3 1.6-1.3 1.6-3.9 0-6.2-1.4-2.3-4-3.3-5.6-2z"/></svg>
</span>
            </span>
            <div
              id="github-b24e42b6dbb7f09a80c9b9f35ee214a9-full_name"
              class="m-0 font-bold text-xl text-neutral-800 decoration-primary-500 hover:underline hover:underline-offset-2 dark:text-neutral">
              eaglebk/smart_helper
            </div>
          </div>

          <p id="github-b24e42b6dbb7f09a80c9b9f35ee214a9-description" class="m-0 mt-2 text-md text-neutral-800 dark:text-neutral">
            CLI на Rust для поиска устройств в текстовых конфигурациях. Учебный проект
          </p>

          <div class="m-0 mt-2 flex items-center">
            <span class="mr-1 inline-block h-3 w-3 rounded-full language-dot" data-language="Rust"></span>
            <div class="m-0 mr-5 text-md text-neutral-800 dark:text-neutral">
              Rust
            </div>

            <span class="text-md mr-1 text-neutral-800 dark:text-neutral">
              <span class="relative block icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 576 512"><path fill="currentColor" d="M287.9 0C297.1 0 305.5 5.25 309.5 13.52L378.1 154.8L531.4 177.5C540.4 178.8 547.8 185.1 550.7 193.7C553.5 202.4 551.2 211.9 544.8 218.2L433.6 328.4L459.9 483.9C461.4 492.9 457.7 502.1 450.2 507.4C442.8 512.7 432.1 513.4 424.9 509.1L287.9 435.9L150.1 509.1C142.9 513.4 133.1 512.7 125.6 507.4C118.2 502.1 114.5 492.9 115.1 483.9L142.2 328.4L31.11 218.2C24.65 211.9 22.36 202.4 25.2 193.7C28.03 185.1 35.5 178.8 44.49 177.5L197.7 154.8L266.3 13.52C270.4 5.249 278.7 0 287.9 0L287.9 0zM287.9 78.95L235.4 187.2C231.9 194.3 225.1 199.3 217.3 200.5L98.98 217.9L184.9 303C190.4 308.5 192.9 316.4 191.6 324.1L171.4 443.7L276.6 387.5C283.7 383.7 292.2 383.7 299.2 387.5L404.4 443.7L384.2 324.1C382.9 316.4 385.5 308.5 391 303L476.9 217.9L358.6 200.5C350.7 199.3 343.9 194.3 340.5 187.2L287.9 78.95z"/></svg></span>
            </span>
            <div id="github-b24e42b6dbb7f09a80c9b9f35ee214a9-stargazers" class="m-0 mr-5 text-md text-neutral-800 dark:text-neutral">
              0
            </div>

            <span class="text-md mr-1 text-neutral-800 dark:text-neutral">
              <span class="relative block icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path fill="currentColor" d="M80 104c13.3 0 24-10.7 24-24s-10.7-24-24-24S56 66.7 56 80s10.7 24 24 24zm80-24c0 32.8-19.7 61-48 73.3V192c0 17.7 14.3 32 32 32H304c17.7 0 32-14.3 32-32V153.3C307.7 141 288 112.8 288 80c0-44.2 35.8-80 80-80s80 35.8 80 80c0 32.8-19.7 61-48 73.3V192c0 53-43 96-96 96H256v70.7c28.3 12.3 48 40.5 48 73.3c0 44.2-35.8 80-80 80s-80-35.8-80-80c0-32.8 19.7-61 48-73.3V288H144c-53 0-96-43-96-96V153.3C19.7 141 0 112.8 0 80C0 35.8 35.8 0 80 0s80 35.8 80 80zm208 24c13.3 0 24-10.7 24-24s-10.7-24-24-24s-24 10.7-24 24s10.7 24 24 24zM248 432c0-13.3-10.7-24-24-24s-24 10.7-24 24s10.7 24 24 24s24-10.7 24-24z"/></svg></span>
            </span>
            <div id="github-b24e42b6dbb7f09a80c9b9f35ee214a9-forks" class="m-0 mr-5 text-md text-neutral-800 dark:text-neutral">
              0
            </div>
          </div>
        </div>
      </div>
      
      
      <script
        async
        type="text/javascript"
        src="/js/fetch-repo.min.dc5533c50cefd50405344b235937142271f26229fe39cbee27fd4960e8bb897a0beebfad77a1091ca91cd0d1fb14e70fc37cc114dd9674fb2c32e0ab512ec8a4.js"
        integrity="sha512-3FUzxQzv1QQFNEsjWTcUInHyYin&#43;OcvuJ/1JYOi7iXoL7r&#43;td6EJHKkc0NH7FOcPw3zBFN2WdPssMuCrUS7IpA=="
        data-repo-url="https://api.github.com/repos/eaglebk/smart_helper"
        data-repo-id="github-b24e42b6dbb7f09a80c9b9f35ee214a9"></script>
    </a>
  </div>
<div id="typeit-3ed909d3e57b632dc5d32da69ca261d2"></div>

<script>
    document.addEventListener("DOMContentLoaded", function () {
      var strings = ["","Продолжение следует...",""];
      
      new TypeIt("#typeit-3ed909d3e57b632dc5d32da69ca261d2", {
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


