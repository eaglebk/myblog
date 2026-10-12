# Как работает Vec в Rust: устройство, методы, оптимизация

> Полное руководство по Vec в Rust: устройство, методы, частые ошибки и оптимизация. Всё это — в форме космического путешествия, где капитан учит робота RUST-Y обращаться с динамическими массивами без паники и перерасхода памяти.

- **URL:** https://eagle2000.space/rust/rust-vec/
- **Date:** 2025-09-01

- **Tags:** rust, dev

---


<h2 class="relative group">Вступление
    <div id="вступление" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d1%81%d1%82%d1%83%d0%bf%d0%bb%d0%b5%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Иногда нам нужен список который не зажат в рамки фиксированного размера. В Rust для этого имеется <code>Vec&lt;T&gt; </code>динамический контейнер, который растёт и сжимается по мере необходимости.</p>
<p>Чтобы сделать знакомство с ним более запоминающимся, мы отправимся в путешествие с увлекательным сюжетом.</p>
<p>Главное действующее лицо - капитан космического корабля <em>«Vectoria»</em>. Его задача - подготовить корабль к миссии: собрать экипаж, загрузить оборудование, очистить и обслужить системы и наконец отправиться в полёт.</p>
<p>Вместе с капитаном будет робот-помощник RUST-Y или просто Расти. Он умеет писать программы на Rust, но часто совершает типичные ошибки новичка. Капитану придется направлять его и показывать правильные решения.</p>

  



<div
  
    class="flex px-4 py-3 rounded-md shadow bg-primary-100 dark:bg-primary-900"
  
  >
  <span
    
      class="text-primary-400 pe-3 flex items-center"
    
    >
    <span class="relative block icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path fill="currentColor" d="M256 32C114.6 32 .0272 125.1 .0272 240c0 49.63 21.35 94.98 56.97 130.7c-12.5 50.37-54.27 95.27-54.77 95.77c-2.25 2.25-2.875 5.734-1.5 8.734C1.979 478.2 4.75 480 8 480c66.25 0 115.1-31.76 140.6-51.39C181.2 440.9 217.6 448 256 448c141.4 0 255.1-93.13 255.1-208S397.4 32 256 32z"/></svg>
</span>
  </span>

  <span
    
      class="dark:text-neutral-300"
    
    >А ещё в конце статьи вас ждёт испытание: мини-квиз, который покажет насколько хорошо вы справились с миссией и разобрались с Vec.</span>
</div>

<p>Готовы? Тогда начнём с самого простого шага: списка экипажа.</p>

<h2 class="relative group">Часть 1. Знакомство с Vec — первые шаги
    <div id="часть-1-знакомство-с-vec--первые-шаги" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%87%d0%b0%d1%81%d1%82%d1%8c-1-%d0%b7%d0%bd%d0%b0%d0%ba%d0%be%d0%bc%d1%81%d1%82%d0%b2%d0%be-%d1%81-vec--%d0%bf%d0%b5%d1%80%d0%b2%d1%8b%d0%b5-%d1%88%d0%b0%d0%b3%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Вас вызывают в штаб. Перед стартом новой космической миссии вам поручают задание - предоставить список экипажа. Пока известно только четыре члена команды, ещё двое прибудут позже.</p>
<p>Вы даете задание Расти — составь список экипажа. Ваша задача - научить его вести списки правильно, ведь от этого зависит порядок на борту и успех всей миссии.</p>
### 💻 Попытка RUST-Y — ошибка новичка


#### Шаг 1


```rust
// ?compile_fail
fn main() {
    // Робот записывает имена по отдельности
    let crew1 = "Капитан Нова";
    let crew2 = "Инженер Спаркс";
    let crew3 = "Доктор Люмен";
    let crew4 = "Биолог Рост";

    // Он решает: "создам массив"
    let crew = [crew1, crew2, crew3, crew4];

    // Но сразу лезет к пятому элементу...
    let crew5 = crew[4];

    println!("Пятый член экипажа: {crew5}");
}
```


**Результат: программа упадёт с паникой.**
- Почему? Потому что массив в Rust имеет фиксированный размер — здесь ровно 4 элемента, а доступ к индексу [4] недопустим.
- 🔹 Паника — это механизм аварийного завершения, когда нарушено условие безопасности (например, выход за границы памяти).


#### Шаг 2


```rust
fn main() {
    // Список экипажа в гибкой коллекции
    let crew = vec!["Капитан Нова", "Инженер Спаркс", "Доктор Люмен", "Биолог Рост"];

    println!("Экипаж: {:?}", crew);
    println!("Длина: {}", crew.len());
    println!("Вместимость: {}", crew.capacity());
}
```


**Исправление капитана — Vec**
- Капитан объясняет:
Расти, массив — это как тетрадь на 4 страницы. Если в экипаже будет больше людей, страниц уже не хватит.
- Нам нужен гибкий список, который умеет расти. В Rust для этого есть Vec.
- Под капотом он хранит три числа:
- 1. указатель (`ptr`) на блок памяти, 
- 2. длину (`len`) — сколько элементов реально в списке,
- 3. вместимость (`capacity`) — сколько ещё можно добавить без расширения памяти.


<p>Запустите код в шаге 2 и вы увидите примерно такой вывод:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-text" data-lang="text"><span class="line"><span class="cl">Экипаж: [&#34;Капитан Нова&#34;, &#34;Инженер Спаркс&#34;, &#34;Доктор Люмен&#34;, &#34;Биолог Рост&#34;]
</span></span><span class="line"><span class="cl">Длина: 4
</span></span><span class="line"><span class="cl">Вместимость: 4</span></span></code></pre></div></div>
<blockquote><p>Представьте, что у вас есть грузовой отсек корабля.</p>
<ul>
<li>В нём 4 контейнера — это len.</li>
<li>Отсек рассчитан максимум на 4 контейнера — это capacity.</li>
<li>Если вы добавите ещё один контейнер, корабль &ldquo;расширит отсек&rdquo; (реаллокация): выделит новый, более просторный отсек, перенесёт туда все контейнеры и продолжит работу.</li>
</ul>
</blockquote>### 💻 Попробую показать это в коде:



```rust
fn main() {
    let mut crew = Vec::new();

    for i in 1..=5 {
        crew.push(format!("Член экипажа #{i}"));
        println!(
            "Добавлен {i}-й: len={}, capacity={}",
            crew.len(),
            crew.capacity()
        );
    }
}
```


**Запустите код и вы увидите, что capacity растёт скачками (примерно в 2 раза).**
- Это нужно, чтобы `push` в среднем был очень быстрым (`O(1)`).



<h3 class="relative group">Описание методов и конструкций
    <div id="описание-методов-и-конструкций" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%be%d0%bf%d0%b8%d1%81%d0%b0%d0%bd%d0%b8%d0%b5-%d0%bc%d0%b5%d1%82%d0%be%d0%b4%d0%be%d0%b2-%d0%b8-%d0%ba%d0%be%d0%bd%d1%81%d1%82%d1%80%d1%83%d0%ba%d1%86%d0%b8%d0%b9" aria-label="Якорь">#</a>
    </span>
    
</h3>
<ul>
<li>
<p><code>push</code>
Добавляет элемент в конец вектора. Если вектора не хватает <code>(len == capacity)</code>, он автоматически расширяет память.</p>
</li>
<li>
<p><code>len</code>
Возвращает текущее количество элементов в <code>Vec</code>.</p>
</li>
<li>
<p><code>capacity</code>
Показывает, сколько элементов можно ещё добавить без перераспределения памяти.</p>
</li>
<li>
<p><code>1..=5</code>
Это <strong>диапазон</strong> (range). В Rust <code>..= </code>означает &ldquo;включительно&rdquo;, то есть от 1 до 5 включительно.
Если бы мы написали <code>1..5</code>, то получили бы числа 1, 2, 3, 4.</p>
</li>
</ul>

<h3 class="relative group">Несколько способов создать Vec
    <div id="несколько-способов-создать-vec" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bd%d0%b5%d1%81%d0%ba%d0%be%d0%bb%d1%8c%d0%ba%d0%be-%d1%81%d0%bf%d0%be%d1%81%d0%be%d0%b1%d0%be%d0%b2-%d1%81%d0%be%d0%b7%d0%b4%d0%b0%d1%82%d1%8c-vec" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>В Rust есть несколько способов создать динамический вектор.</p>

<h4 class="relative group">1. Пустой вектор
    <div id="1-пустой-вектор" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#1-%d0%bf%d1%83%d1%81%d1%82%d0%be%d0%b9-%d0%b2%d0%b5%d0%ba%d1%82%d0%be%d1%80" aria-label="Якорь">#</a>
    </span>
    
</h4>
<p>Когда мы пока не знаем, какие элементы будут:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="k">mut</span><span class="w"> </span><span class="n">crew</span>: <span class="nb">Vec</span><span class="o">&lt;</span><span class="nb">String</span><span class="o">&gt;</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="nb">Vec</span>::<span class="n">new</span><span class="p">();</span></span></span></code></pre></div></div>
<p>Здесь мы явно указали тип <code>(Vec&lt;String&gt;)</code>. Без него компилятор не поймёт, что именно мы хотим хранить.</p>

<h4 class="relative group">2. Макрос vec![]
    <div id="2-макрос-vec" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#2-%d0%bc%d0%b0%d0%ba%d1%80%d0%be%d1%81-vec" aria-label="Якорь">#</a>
    </span>
    
</h4>
<p>Самый удобный способ, если сразу известны элементы:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">crew</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="fm">vec!</span><span class="p">[</span><span class="s">&#34;Алиса&#34;</span><span class="p">,</span><span class="w"> </span><span class="s">&#34;Борис&#34;</span><span class="p">,</span><span class="w"> </span><span class="s">&#34;Саяна&#34;</span><span class="p">,</span><span class="w"> </span><span class="s">&#34;Дмитрий&#34;</span><span class="p">];</span></span></span></code></pre></div></div>

<h4 class="relative group">3. С фиксированным числом одинаковых значений
    <div id="3-с-фиксированным-числом-одинаковых-значений" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#3-%d1%81-%d1%84%d0%b8%d0%ba%d1%81%d0%b8%d1%80%d0%be%d0%b2%d0%b0%d0%bd%d0%bd%d1%8b%d0%bc-%d1%87%d0%b8%d1%81%d0%bb%d0%be%d0%bc-%d0%be%d0%b4%d0%b8%d0%bd%d0%b0%d0%ba%d0%be%d0%b2%d1%8b%d1%85-%d0%b7%d0%bd%d0%b0%d1%87%d0%b5%d0%bd%d0%b8%d0%b9" aria-label="Якорь">#</a>
    </span>
    
</h4>
<p>Макрос <code>vec!</code> можно использовать и так:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">crew</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="fm">vec!</span><span class="p">[</span><span class="s">&#34;Неизвестный&#34;</span><span class="p">;</span><span class="w"> </span><span class="mi">3</span><span class="p">];</span></span></span></code></pre></div></div>
<p>Получим вектор [&ldquo;Неизвестный&rdquo;, &ldquo;Неизвестный&rdquo;, &ldquo;Неизвестный&rdquo;].</p>

<h4 class="relative group">4. Через итераторы и коллекции
    <div id="4-через-итераторы-и-коллекции" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#4-%d1%87%d0%b5%d1%80%d0%b5%d0%b7-%d0%b8%d1%82%d0%b5%d1%80%d0%b0%d1%82%d0%be%d1%80%d1%8b-%d0%b8-%d0%ba%d0%be%d0%bb%d0%bb%d0%b5%d0%ba%d1%86%d0%b8%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h4>
<p>Например, можно создать последовательность чисел и собрать её в вектор:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">numbers</span>: <span class="nb">Vec</span><span class="o">&lt;</span><span class="kt">i32</span><span class="o">&gt;</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="p">(</span><span class="mi">1</span><span class="o">..=</span><span class="mi">5</span><span class="p">).</span><span class="n">collect</span><span class="p">();</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;</span><span class="si">{:?}</span><span class="s">&#34;</span><span class="p">,</span><span class="w"> </span><span class="n">numbers</span><span class="p">);</span><span class="w"> </span><span class="c1">// [1, 2, 3, 4, 5]
</span></span></span></code></pre></div></div>

<h4 class="relative group">5. С заранее известной ёмкостью
    <div id="5-с-заранее-известной-ёмкостью" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#5-%d1%81-%d0%b7%d0%b0%d1%80%d0%b0%d0%bd%d0%b5%d0%b5-%d0%b8%d0%b7%d0%b2%d0%b5%d1%81%d1%82%d0%bd%d0%be%d0%b9-%d1%91%d0%bc%d0%ba%d0%be%d1%81%d1%82%d1%8c%d1%8e" aria-label="Якорь">#</a>
    </span>
    
</h4>
<p>Если мы знаем, что скоро придёт много элементов, выгодно сразу зарезервировать память:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="k">mut</span><span class="w"> </span><span class="n">crew</span>: <span class="nb">Vec</span><span class="o">&lt;</span><span class="nb">String</span><span class="o">&gt;</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="nb">Vec</span>::<span class="n">with_capacity</span><span class="p">(</span><span class="mi">10</span><span class="p">);</span></span></span></code></pre></div></div>
<p>Это создаст пустой вектор, у которого <code>len = 0</code>, но <code>capacity = 10</code>.</p>

  



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
    
    >На практике чаще всего используют vec![], а остальные способы нужны для оптимизаций или специфических случаев.</span>
</div>


<h2 class="relative group">Часть 2. Проверка экипажа перед тренировкой (паника и безопасный доступ)
    <div id="часть-2-проверка-экипажа-перед-тренировкой-паника-и-безопасный-доступ" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%87%d0%b0%d1%81%d1%82%d1%8c-2-%d0%bf%d1%80%d0%be%d0%b2%d0%b5%d1%80%d0%ba%d0%b0-%d1%8d%d0%ba%d0%b8%d0%bf%d0%b0%d0%b6%d0%b0-%d0%bf%d0%b5%d1%80%d0%b5%d0%b4-%d1%82%d1%80%d0%b5%d0%bd%d0%b8%d1%80%d0%be%d0%b2%d0%ba%d0%be%d0%b9-%d0%bf%d0%b0%d0%bd%d0%b8%d0%ba%d0%b0-%d0%b8-%d0%b1%d0%b5%d0%b7%d0%be%d0%bf%d0%b0%d1%81%d0%bd%d1%8b%d0%b9-%d0%b4%d0%be%d1%81%d1%82%d1%83%d0%bf" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Перед первой тренировкой капитан просит RUST-Y вызвать члена экипажа по индексу из списка.
Но если робот ошибётся с номером, программа завершится аварийно. Это важный момент: такие сбои недопустимы.</p>
### 💻 Попытка RUST-Y - ошибка новичка


#### Шаг 1


```rust
// ?should_panic
fn main() {
    let crew = vec![
        "Капитан Нова",
        "Инженер Спаркс",
        "Доктор Люмен",
        "Биолог Рост",
        "Пилот Вега",
        "Связист Хекс",
    ];

    // Попытка вызвать участника с индексом 99
    let who = crew[99]; // ❌ паника
    println!("На тренировку выходит: {}", who);
}
```


**Результат: программа упадёт с паникой.**
- Что произошло? В Rust доступ по индексу (`crew[99]`) проверяется на корректность. Если индекс вне диапазона, программа завершается **паникой**..
- В отличие от C/C++, где можно случайно прочитать "левые байты" или сломать память Rust останавливает программу и не даёт ей работать некорректно.


#### Шаг 2


```rust
fn main() {
    let crew = vec![
        "Капитан Нова",
        "Инженер Спаркс",
        "Доктор Люмен",
        "Биолог Рост",
        "Пилот Вега",
        "Связист Хекс",
    ];

    match crew.get(99) {
        Some(name) => println!("На тренировку выходит: {}", name),
        None => println!("Ошибка: участника с таким номером не существует."),
    }
}
```


**Исправление капитана — безопасный доступ**
- Разбор:
- [] возвращает ссылку &T, но вызывает панику, если индекс неверный.
- `.get(index)` возвращает `Option<&T>`: `Some(&T)` — элемент найден, `None` — индекс за пределами.



  



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
    
    ><p>Паника в Rust — это защита и повод проверить ваш код.</p>
<p>Если индекс может быть неверным, используйте <code>.get()</code>.</p>
</span>
</div>


<h3 class="relative group">Немного про Option
    <div id="немного-про-option" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bd%d0%b5%d0%bc%d0%bd%d0%be%d0%b3%d0%be-%d0%bf%d1%80%d0%be-option" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>В статье про функциональный Rust мы уже встречались с <code>Option</code> как с типом, который позволяет явно сообщить: &ldquo;значение может быть, а может не быть&rdquo; такой аналог null-safety из других языков.</p>
<p>В примере выше <code>Option</code> используется так:</p>
<ul>
<li><code>Some(value)</code> — элемент в векторе есть.</li>
<li><code>None</code> — элемента нет (например, индекс за пределами).</li>
</ul>
<p>Это хорошее решение языка: вместо &ldquo;магических значений&rdquo; или &ldquo;падений программы&rdquo; мы получаем явную модель отсутствия данных.</p>

<h2 class="relative group">Часть 3. Личные коды экипажа (итерация по вектору и изменение элементов)
    <div id="часть-3-личные-коды-экипажа-итерация-по-вектору-и-изменение-элементов" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%87%d0%b0%d1%81%d1%82%d1%8c-3-%d0%bb%d0%b8%d1%87%d0%bd%d1%8b%d0%b5-%d0%ba%d0%be%d0%b4%d1%8b-%d1%8d%d0%ba%d0%b8%d0%bf%d0%b0%d0%b6%d0%b0-%d0%b8%d1%82%d0%b5%d1%80%d0%b0%d1%86%d0%b8%d1%8f-%d0%bf%d0%be-%d0%b2%d0%b5%d0%ba%d1%82%d0%be%d1%80%d1%83-%d0%b8-%d0%b8%d0%b7%d0%bc%d0%b5%d0%bd%d0%b5%d0%bd%d0%b8%d0%b5-%d1%8d%d0%bb%d0%b5%d0%bc%d0%b5%d0%bd%d1%82%d0%be%d0%b2" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Перед полетом необходимо пройти инструктаж и капитан требует, чтобы каждый член экипажа получил личный код доступа в формате: <code>R-XXX Имя</code></p>
<p>Например:</p>
<ul>
<li>R-001 Капитан Нова</li>
<li>R-002 Инженер Спаркс</li>
</ul>
<p>Робот RUST-Y пробует сделать это с помощью цикла и сразу натыкается на ошибку компилятора.</p>
### 💻 Попытка RUST-Y


#### Шаг 1


```rust
// ?compile_fail
fn main() {
    let mut names = vec![
        String::from("Капитан Нова"),
        String::from("Инженер Спаркс"),
        String::from("Доктор Люмен"),
        String::from("Биолог Рост"),
        String::from("Пилот Вега"),
        String::from("Связист Хекс"),
    ];

    // ❌ Ошибочный подход: iter() даёт неизменяемые ссылки (&String)
    for (i, name) in names.iter().enumerate() {
        *name = format!("R-{:03} {}", i + 1, name);
        println!("Подготовка кода для: {name} (#{})", i + 1);
    }
}
```


**Почему не компилируется**
- `names.iter()` возвращает неизменяемые ссылки `&String`.
- Изменять значения по таким ссылкам запрещено.
- Чтобы изменять элементы вектора, нужны изменяемые ссылки `&mut String.`


#### Шаг 2


```rust
fn main() {

    let mut names = vec![
        String::from("Капитан Нова"),
        String::from("Инженер Спаркс"),
        String::from("Доктор Люмен"),
        String::from("Биолог Рост"),
        String::from("Пилот Вега"),
        String::from("Связист Хекс"),
    ];
for (i, name) in names.iter_mut().enumerate() {
        *name = format!("R-{:03} {}", i + 1, *name);
    }

    println!("Коды присвоены:\n{:#?}", names);
}
```


**Исправление капитана — iter_mut()**
- Капитан объясняет:
Теперь тип name стал &mut String и значение по ссылке можно изменять



<h3 class="relative group">Разбор новых понятий
    <div id="разбор-новых-понятий" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%80%d0%b0%d0%b7%d0%b1%d0%be%d1%80-%d0%bd%d0%be%d0%b2%d1%8b%d1%85-%d0%bf%d0%be%d0%bd%d1%8f%d1%82%d0%b8%d0%b9" aria-label="Якорь">#</a>
    </span>
    
</h3>

<h4 class="relative group">iter() vs iter_mut()
    <div id="iter-vs-iter_mut" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#iter-vs-iter_mut" aria-label="Якорь">#</a>
    </span>
    
</h4>
<ul>
<li><code>iter(&amp;self)</code> -&gt;<code> Iter&lt;'_, T&gt;</code> — даёт <code>&amp;T</code> (чтение).</li>
<li><code>iter_mut(&amp;mut self)</code> -&gt; <code>IterMut&lt;'_, T&gt;</code> — даёт <code>&amp;mut T</code> (изменение).</li>
</ul>
<blockquote><p>⚠️ Пока выполняется iter_mut(), нельзя иметь другие заимствования — это правило единственной изменяемой ссылки в Rust.</p>
</blockquote>
<h4 class="relative group">enumerate()
    <div id="enumerate" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#enumerate" aria-label="Якорь">#</a>
    </span>
    
</h4>
<ul>
<li>Оборачивает итератор и добавляет индекс.</li>
<li>Индексы начинаются с 0, поэтому для &ldquo;человекопонятной нумерации&rdquo; используем <code>i + 1</code>.</li>
</ul>

<h4 class="relative group">Изменение элемента через *name
    <div id="изменение-элемента-через-name" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b8%d0%b7%d0%bc%d0%b5%d0%bd%d0%b5%d0%bd%d0%b8%d0%b5-%d1%8d%d0%bb%d0%b5%d0%bc%d0%b5%d0%bd%d1%82%d0%b0-%d1%87%d0%b5%d1%80%d0%b5%d0%b7-name" aria-label="Якорь">#</a>
    </span>
    
</h4>
<ul>
<li>В цикле <code>name</code> имеет тип <code>&amp;mut String</code>.</li>
<li>Чтобы присвоить новое значение, используем разыменование</li>
</ul>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="o">*</span><span class="n">name</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="fm">format!</span><span class="p">(</span><span class="s">&#34;R-</span><span class="si">{:03}</span><span class="s"> </span><span class="si">{}</span><span class="s">&#34;</span><span class="p">,</span><span class="w"> </span><span class="n">i</span><span class="w"> </span><span class="o">+</span><span class="w"> </span><span class="mi">1</span><span class="p">,</span><span class="w"> </span><span class="o">*</span><span class="n">name</span><span class="p">);</span></span></span></code></pre></div></div>

<h4 class="relative group">format!
    <div id="format" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#format" aria-label="Якорь">#</a>
    </span>
    
</h4>
<ul>
<li>Макрос для сборки строк по шаблону.</li>
<li>Возвращает новый <code>String</code>.</li>
<li>Экономия на ручных конкатенациях (суммирования строк).</li>
</ul>

<h2 class="relative group">Часть 4. Массовая загрузка корабля и проверка приборов
    <div id="часть-4-массовая-загрузка-корабля-и-проверка-приборов" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%87%d0%b0%d1%81%d1%82%d1%8c-4-%d0%bc%d0%b0%d1%81%d1%81%d0%be%d0%b2%d0%b0%d1%8f-%d0%b7%d0%b0%d0%b3%d1%80%d1%83%d0%b7%d0%ba%d0%b0-%d0%ba%d0%be%d1%80%d0%b0%d0%b1%d0%bb%d1%8f-%d0%b8-%d0%bf%d1%80%d0%be%d0%b2%d0%b5%d1%80%d0%ba%d0%b0-%d0%bf%d1%80%d0%b8%d0%b1%d0%be%d1%80%d0%be%d0%b2" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Подготовка к миссии выходит на новый уровень.</p>
<p>Корабль <em>Vectoria</em> должен принять на борт сотни контейнеров и провести проверку бортовых приборов.</p>
<p>Робот Расти пытается записать данные как есть, но работа получается &ldquo;рваной&rdquo; — список растёт с перебоями и задержками.</p>
<p>Капитан объясняет, как избежать лишних перераспределений памяти при массовой загрузке.</p>
### 💻 Попытка RUST-Y — загрузка


#### Шаг 1


```rust
use std::time::Instant;

fn main() {
    // Симулируем поток данных от датчиков
    let incoming: Vec<String> = (1..=20_000)
        .map(|i| format!("Прибор #{i:05}"))
        .collect();

    let start = Instant::now();
    let mut manifest: Vec<String> = Vec::new();

    let mut last_cap = manifest.capacity();
    let mut reallocs = 0;

    for item in incoming {
        manifest.push(item); // добавляем по одному
        if manifest.capacity() != last_cap {
            reallocs += 1;
            last_cap = manifest.capacity();
        }
    }

    println!(
        "Загрузка завершена: len={}, capacity={}, reallocs={}, elapsed={:?}",
        manifest.len(),
        manifest.capacity(),
        reallocs,
        start.elapsed()
    );
}
```


**Что происходит:**
- `Vec` начинает с минимальной ёмкости.
- При каждом переполнении выделяется новый блок памяти → копирование старых данных.
- На больших объёмах видно множество реаллокаций.


#### Шаг 2


```rust
use std::time::Instant;

fn main() {
    let incoming: Vec<String> = (1..=20_000)
        .map(|i| format!("Прибор #{i:05}"))
        .collect();
    let mut reallocs = 0;
    let start = Instant::now();

// Предвыделяем память под все элементы заранее
    let mut manifest: Vec<String> = Vec::with_capacity(incoming.len());

    // Массовая вставка
    manifest.extend(incoming);
    reallocs += 1;

    println!(
        "Загрузка завершена: len={}, capacity={}, reallocs={}, elapsed={:?}",
        manifest.len(),
        manifest.capacity(),
        reallocs,
        start.elapsed()
    );
}
```


**Исправление капитана — предвыделение памяти)**
- Капитан объясняет, что теперь:
- нет скачков ёмкости,
- данные загружаются быстрее,
- память выделяется ровно один раз.



<h3 class="relative group">Разбор кода для 2 шага
    <div id="разбор-кода-для-2-шага" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%80%d0%b0%d0%b7%d0%b1%d0%be%d1%80-%d0%ba%d0%be%d0%b4%d0%b0-%d0%b4%d0%bb%d1%8f-2-%d1%88%d0%b0%d0%b3%d0%b0" aria-label="Якорь">#</a>
    </span>
    
</h3>
<ul>
<li><code>push</code> добавляет по одному элементу.</li>
<li><code>extend</code> добавляет коллекцию или итератор целиком.</li>
<li><code>append(&amp;mut other)</code> переносит данные из другого вектора без копирования (после этого other опустошается).</li>
<li><code>with_capacity(n)</code> сразу резервирует место под n элементов.</li>
<li><code>reserve(n) / reserve_exact(n)</code> — динамическое выделение при работе порциями.</li>
</ul>

  



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
    
    ><p><strong>Аналогия с роботом и складом</strong></p>
<p>Представьте складской отсек:</p>
<ul>
<li>RUST-Y тащит ящики один за одним, каждый раз переставляя стены отсека когда не хватает объема помещения (реаллокации).</li>
<li>Капитан заранее строит отсек нужного размера — и загрузка идёт без перебоев.</li>
</ul>
</span>
</div>


<h2 class="relative group">Часть 5. Буфер телеметрии — только последние данные
    <div id="часть-5-буфер-телеметрии--только-последние-данные" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%87%d0%b0%d1%81%d1%82%d1%8c-5-%d0%b1%d1%83%d1%84%d0%b5%d1%80-%d1%82%d0%b5%d0%bb%d0%b5%d0%bc%d0%b5%d1%82%d1%80%d0%b8%d0%b8--%d1%82%d0%be%d0%bb%d1%8c%d0%ba%d0%be-%d0%bf%d0%be%d1%81%d0%bb%d0%b5%d0%b4%d0%bd%d0%b8%d0%b5-%d0%b4%d0%b0%d0%bd%d0%bd%d1%8b%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Корабль <em>Vectoria</em> готовится к полёту. Приборы начинают выдавать поток телеметрии: десятки и сотни значений в секунду.</p>
<p>Хранить всё подряд — невозможно: память ограничена.</p>
<p>Задача: держать в буфере только последние N записей, а старые сбрасывать.</p>
<p>Расти пишет свой вариант и, как обычно, попадает в ловушку новичка.</p>
### 💻 Попытка RUST-Y — неограниченный рост


#### Шаг 1


```rust
fn main() {
    let mut telemetry: Vec<i32> = Vec::new();

    // Симулируем поток данных
    for i in 0..200 {
        telemetry.push(i);
    }

    println!("Собрано {} значений", telemetry.len());
}
```


**Проблема:**
- буфер растёт бесконечно,
- через некоторое время память будет исчерпана.


#### Шаг 2


```rust
fn main() {
    let mut telemetry: Vec<i32> = Vec::new();
    let max_len = 100;

    for i in 0..200 {
        telemetry.push(i);

        // если размер превысил лимит, убираем старые записи
        if telemetry.len() > max_len {
            telemetry.remove(0); // ❌ неэффективно, сдвигает элементы
        }
    }

    println!("Буфер телеметрии: {:?}", telemetry);
}
```


**Исправление капитана — ограниченный буфер**
- Работает, но `remove(0)` сдвигает все элементы - на больших объёмах это дорого.


#### Шаг 3


```rust
fn main() {
    let mut telemetry: Vec<i32> = Vec::new();
    let max_len = 100;

    for i in 0..200 {
        telemetry.push(i);

        if telemetry.len() > max_len {
            let overflow = telemetry.len() - max_len;
            telemetry.drain(0..overflow);
        }
    }

    println!("Буфер телеметрии: {:?}", telemetry);
}
```


**Ещё лучше — `drain`**
- `drain(0..overflow)` удаляет сразу несколько элементов с начала.
- В буфере всегда остаются последние `max_len` записей.


#### Шаг 4


```rust
fn main() {
    let mut telemetry: Vec<i32> = Vec::new();
    let max_len = 100;

    for i in 0..200 {
        telemetry.push(i);
    }
let last100: Vec<i32> = telemetry.iter().rev().take(100).cloned().collect();


    println!("Буфер телеметрии: {:?}", last100);
}
```


**Альтернативное решение — срез последних элементов**
- Иногда проще просто брать «хвост» вектора:



<h2 class="relative group">Часть 6. Итоги и старт миссии 🚀
    <div id="часть-6-итоги-и-старт-миссии-" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%87%d0%b0%d1%81%d1%82%d1%8c-6-%d0%b8%d1%82%d0%be%d0%b3%d0%b8-%d0%b8-%d1%81%d1%82%d0%b0%d1%80%d1%82-%d0%bc%d0%b8%d1%81%d1%81%d0%b8%d0%b8-" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Итак все системы корабля <em>Vectoria</em> проверены.</p>
<ul>
<li>Экипаж собран и получил личные коды.</li>
<li>Приборы загружены и протестированы.</li>
<li>Буфер телеметрии настроен чтобы хранить только актуальные данные.</li>
</ul>
<p>Робот RUST-Y научился многому: он уже не делает типичных ошибок новичка и готов сопровождать миссию в полёте.</p>
<p>Капитан даёт сигнал: <strong>&ldquo;К старту готовы!&rdquo;</strong></p>
<p>Проверьте себя: пройдите финальный квиз ниже.</p>
<p>Время прохождения ограничено, но дано с запасом. Вопросы каждый раз перемешиваются и вы можете проходить квиз несколько раз.</p>
<blockquote><p>Спасибо, что прочитали статью про векторы в Rust до конца, надеюсь что смог объяснить как их правильно готовить! :D</p>
</blockquote><div id="typeit-f4cefc7ff453e11fe1a04e6c0f91d75e"></div>

<script>
    document.addEventListener("DOMContentLoaded", function () {
      var strings = ["","Продолжение следует и удачи, капитан!",""];
      
      new TypeIt("#typeit-f4cefc7ff453e11fe1a04e6c0f91d75e", {
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

<p>🛰️</p>
### 📝 Rust Vec

**1. Что произойдёт при выполнении кода?**

- [ ] Напечатает пустую строку
- [ ] Вернёт None
- [x] Программа упадёт с паникой
- [ ] Автоматически расширит вектор

> 💡 *Индекс 10 вне диапазона. Безопасный доступ — через .get(10).*


**2. Как создать пустой вектор строк?**

- [ ] Vec::new()
- [ ] vec!()
- [ ] Vec::<String>::new()
- [x] Все варианты верны

> 💡 *Все варианты валидны (но тип должен быть выведен или указан).*


**3. Что хранит Vec под капотом?**

- [ ] Только список элементов
- [x] Указатель на данные, длину и вместимость
- [ ] Только длину и данные
- [ ] Только вместимость

> 💡 *Vec хранит указатель на буфер, длину (len) и вместимость (capacity).*


**4. Чем массив [T; N] отличается от Vec<T>?**

- [x] Массив фиксированного размера, Vec — динамический
- [ ] Vec не может хранить строки
- [ ] Массивы медленнее чем Vec
- [ ] Vec работает только с числами

> 💡 *Массивы в Rust имеют фиксированный размер, а Vec может расти и уменьшаться.*


**5. Что вернёт следующий код?**

- [ ] Панику
- [ ] Some("")
- [x] None
- [ ] Компилятор выдаст ошибку

> 💡 *Метод .get() возвращает Option. Для индекса вне диапазона это будет None.*


**6. Чем отличается crew[5] от crew.get(5)?**

- [ ] Разницы нет, оба возвращают ссылку
- [x] crew[5] может вызвать панику, crew.get(5) возвращает Option
- [ ] crew[5] всегда безопасен
- [ ] crew.get(5) расширяет вектор

> 💡 *`crew[5]` может паниковать, а `crew.get(5)` вернёт `None`, если элемента нет.*


**7. Почему этот код не компилируется?**

- [ ] iter() не поддерживает строки
- [x] iter() возвращает &String, а не &mut String
- [ ] enumerate() нельзя использовать вместе с iter()
- [ ] Вектор по умолчанию неизменяемый

> 💡 *iter() возвращает только неизменяемые ссылки (&T). Чтобы изменять элементы, нужно использовать iter_mut().*


**8. Какой тип имеет переменная name в этом цикле?**

- [ ] String
- [ ] &String
- [x] &mut String
- [ ] Option<&mut String>

> 💡 *iter_mut() возвращает изменяемые ссылки, поэтому name имеет тип &mut String.*


**9. Что делает метод enumerate()?**

- [ ] Превращает вектор в массив
- [x] Добавляет индекс к каждому элементу при обходе
- [ ] Создаёт новый Vec<(usize, T)>
- [ ] Изменяет элементы по месту

> 💡 *enumerate() оборачивает итератор и добавляет индекс к каждому элементу.*


**10. Чем отличается push от extend?**

- [x] push добавляет один элемент, extend — целую коллекцию
- [ ] Разницы нет, это синонимы
- [ ] extend всегда копирует данные, а push — нет
- [ ] push требует &mut Vec, а extend — нет

> 💡 *push работает по одному элементу, а extend позволяет добавить сразу коллекцию или итератор.*


**11. Что произойдёт в этом коде?**

- [ ] [1, 2] [3, 4]
- [x] [1, 2, 3, 4] []
- [ ] [1, 2, 3, 4] [3, 4]
- [ ] Ошибка компиляции

> 💡 *append перемещает элементы из b в a. После этого b очищается, поэтому результат: [1, 2, 3, 4] [].*


**12. Что делает метод with_capacity(n)?**

- [ ] Создаёт вектор длиной n
- [x] Резервирует место для n элементов, но оставляет длину равной 0
- [ ] Создаёт массив фиксированного размера
- [ ] Всегда удваивает вместимость

> 💡 *with_capacity(n) создаёт пустой вектор: len = 0, capacity = n.*


**13. Почему загрузка в Vec через push может работать медленнее?**

- [ ] Каждый push требует вызова format!
- [x] При переполнении Vec приходится выделять новый блок памяти и копировать элементы
- [ ] push всегда компилируется медленнее чем extend
- [ ] Vec не поддерживает большие коллекции

> 💡 *При переполнении capacity Vec делает реаллокацию: выделяет новый блок памяти и копирует туда все элементы.*


**14. Что делает метод truncate(n)?**

- [ ] Убирает все элементы
- [x] Оставляет только первые n элементов
- [ ] Удаляет элементы с начала
- [ ] Всегда очищает вектор

> 💡 *truncate(n) обрезает вектор, оставляя только первые n элементов.*


**15. Чем remove(0) хуже, чем drain(0..1) для больших буферов?**

- [ ] remove(0) не работает с векторами
- [x] remove(0) сдвигает все элементы, drain может убрать сразу диапазон
- [ ] Разницы нет
- [ ] drain всегда медленнее

> 💡 *remove(0) сдвигает все элементы влево. drain эффективнее, особенно если нужно удалить сразу несколько.*


**16. Как оставить в векторе только последние N элементов при росте?**

- [ ] Через truncate(N)
- [x] Через drain(0..len-N)
- [ ] Через remove(len-N)
- [ ] Через get(N)

> 💡 *Самый удобный способ — drain(0..len-N): он удаляет лишние элементы в начале, оставляя последние N.*


**17. Почему буфер с push без удаления не подходит для телеметрии?**

- [ ] push не работает для чисел
- [x] Буфер будет расти бесконечно и съест всю память
- [ ] push автоматически удаляет старые значения
- [ ] Vec ограничен максимум 100 элементами

> 💡 *Если только добавлять новые элементы через push, вектор будет расти без ограничений и рано или поздно исчерпает память.*




