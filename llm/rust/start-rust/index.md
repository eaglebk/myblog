# Введение в язык программирования Rust: Базовый синтаксис

> Простое и понятное введение в язык программирования Rust для начинающих: переменные, типы данных, функции и первые шаги в написании надежного кода.

- **URL:** https://eagle2000.space/rust/start-rust/
- **Date:** 2024-03-16

- **Tags:** rust, dev

---


<h1 class="relative group">Введение в Rust: установка, запуск и структура проекта
    <div id="введение-в-rust-установка-запуск-и-структура-проекта" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d0%b2%d0%b5%d0%b4%d0%b5%d0%bd%d0%b8%d0%b5-%d0%b2-rust-%d1%83%d1%81%d1%82%d0%b0%d0%bd%d0%be%d0%b2%d0%ba%d0%b0-%d0%b7%d0%b0%d0%bf%d1%83%d1%81%d0%ba-%d0%b8-%d1%81%d1%82%d1%80%d1%83%d0%ba%d1%82%d1%83%d1%80%d0%b0-%d0%bf%d1%80%d0%be%d0%b5%d0%ba%d1%82%d0%b0" aria-label="Якорь">#</a>
    </span>
    
</h1>
<p>Rust - это мощный системный язык программирования, в котором скорость и безопасность по управлению памятью стоит на первом месте. Разберемся, как установить и запустить Rust, а также рассмотрим из чего состоит базовый проект.</p>

<h2 class="relative group">Установка Rust
    <div id="установка-rust" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%83%d1%81%d1%82%d0%b0%d0%bd%d0%be%d0%b2%d0%ba%d0%b0-rust" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Люблю правило одной кнопки когда не приходится редактировать кучу конфигов и гулять по сайтам в поиске правильного порядка дейстий и ссылок.</p>
<p>Для установки Rust на вашем компьютере просто перейдите на официальный сайт Rust и следуйте инструкциям говорят все статьи. Но мы сделаем иначе.</p>
<p>На Linux и macOS системах запускаем:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-shell" data-lang="shell"><span class="line"><span class="cl">curl https://sh.rustup.rs -sSf <span class="p">|</span> sh</span></span></code></pre></div></div>
<p>для macOS можно запустить:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-shell" data-lang="shell"><span class="line"><span class="cl">brew install rustup</span></span></code></pre></div></div>
<p>для windows скачиваем экзешник по <a href="https://win.rustup.rs/"  target="_blank" rel="noreferrer">ссылке</a> и следуем сообщениям.</p>
<p>Таким несложным способом можно установить rust и его туллинг.</p>

<h2 class="relative group">Первое приложение
    <div id="первое-приложение" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d0%b5%d1%80%d0%b2%d0%be%d0%b5-%d0%bf%d1%80%d0%b8%d0%bb%d0%be%d0%b6%d0%b5%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Теперь пришло время создать наше первое приложение на расте.</p>
<p>Переходим в необходимую директорию (я люблю создавать главную папку dev в ней workspace/rust, которая содержит все проекты разбитые по технологиям).
После выбора директории запускаем команду <code>cargo new</code></p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-shell" data-lang="shell"><span class="line"><span class="cl">cargo new hello_world</span></span></code></pre></div></div>
<p>По умолчанию флаг &ndash;bin уже установлен и не нужно его указывать дополнительно. Он нужен для сборки бинарника. Также можно собрать библиотеку передав параметр &ndash;lib</p>
<p>Посмотрим, что у нас получилось</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-shell" data-lang="shell"><span class="line"><span class="cl">$ <span class="nb">cd</span> hello_world
</span></span><span class="line"><span class="cl">$ tree .
</span></span><span class="line"><span class="cl">.
</span></span><span class="line"><span class="cl">├── Cargo.toml
</span></span><span class="line"><span class="cl">└── src
</span></span><span class="line"><span class="cl">    └── main.rs
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl"><span class="m">1</span> directory, <span class="m">2</span> files</span></span></code></pre></div></div>
<p>Первый файл Cargo.toml. Откроем его</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-toml" data-lang="toml"><span class="line"><span class="cl"><span class="p">[</span><span class="nx">package</span><span class="p">]</span>
</span></span><span class="line"><span class="cl"><span class="nx">name</span> <span class="p">=</span> <span class="s2">&#34;hello_world&#34;</span>
</span></span><span class="line"><span class="cl"><span class="nx">version</span> <span class="p">=</span> <span class="s2">&#34;0.1.0&#34;</span>
</span></span><span class="line"><span class="cl"><span class="nx">edition</span> <span class="p">=</span> <span class="s2">&#34;2021&#34;</span>
</span></span><span class="line"><span class="cl">
</span></span><span class="line"><span class="cl"><span class="p">[</span><span class="nx">dependencies</span><span class="p">]</span></span></span></code></pre></div></div>
<p>Это так называемый манифест, как его назвали на официальном сайте. Типичный файл конфигурации проекта, а также зависимостей для большинства современных языков.</p>
<p>Заглянем в директорию src и посмотрим на файл main.rs</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">main</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Hello, world!&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p>Чтобы скомпилировать и запустить нашу программу, введите команду:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-shell" data-lang="shell"><span class="line"><span class="cl">$ cargo build</span></span></code></pre></div></div>
<p>Собрали, можно запускать:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-shell" data-lang="shell"><span class="line"><span class="cl">$ ./target/debug/hello_world
</span></span><span class="line"><span class="cl">Hello, world!</span></span></code></pre></div></div>
<p>Вывело стандартное приветствие, мы молодцы! Будем продолжать.</p>
<p>Хотите команду поменьше? Можно запустить программу через команду <code>cargo run</code></p>

<h2 class="relative group">Заключение
    <div id="заключение" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b7%d0%b0%d0%ba%d0%bb%d1%8e%d1%87%d0%b5%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Вот и все основы для начала работы с Rust. Этот язык имеет множество возможностей и дает большую свободу действий, так что вперед изучать и экспериментировать!</p>

