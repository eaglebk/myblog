# Трейт-объекты: как работает динамический полиморфизм в Rust

> Разбираемся простыми словами, как устроены трейт-объекты (dyn Trait) в Rust: зачем нужен двойной указатель, как работает таблица методов vtable и какие правила Object Safety нужно знать на практике.

- **URL:** https://eagle2000.space/rust/rust-dyn-traits/
- **Date:** 2026-08-14

- **Tags:** rust, dev, traits, dyn, vtable, middle

---


<h2 class="relative group">Введение
    <div id="введение" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d0%b2%d0%b5%d0%b4%d0%b5%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>В прошлых статьях: <a href="../rust-raii-box-deref" >Статья про RAII, Box, Deref</a>, <a href="../rust-traits" >Введение в трейты</a> мы разобрали базовые трейты и работу с кучей через указатели <code>Box&lt;T&gt;</code>. По умолчанию Rust стремится сделать код максимально быстрым: при использовании обобщенных типов (дженериков) компилятор выполняет мономорфизацию. Он берет функцию и создает ее копию под каждый конкретный тип, получая прямые вызовы.</p>
<p>Однако в реальной разработке бывают задачи, когда типы данных не известны заранее на этапе сборки.</p>
<p>Допустим, мы проектируем систему обработки уведомлений или фоновых задач. У нас есть разные структуры: <code>NetworkTask</code> (сетевой запрос), <code>DbTask</code> (запись в базу) и <code>TimerTask</code> (срабатывание таймера). Все они реализуют общий трейт <code>Task</code> с методом <code>execute()</code>. Мы хотим сложить эти задачи в один список (вектор) и обработать их в цикле.</p>
<p>Но как поместить разные типы в один <code>Vec</code>? Ведь вектор в Rust требует, чтобы все его элементы имели строго одинаковый тип и размер в памяти.</p>
<p>Здесь нам на помощь приходят <strong>трейт-объекты</strong> и ключевое слово <code>dyn</code>. В этой статье мы разберем, как устроен динамический вызов методов в Rust, откуда берутся «толстые» указатели и как компилятор держит всё под контролем.</p>
<hr>

<h2 class="relative group">1. Статический и динамический подход
    <div id="1-статический-и-динамический-подход" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#1-%d1%81%d1%82%d0%b0%d1%82%d0%b8%d1%87%d0%b5%d1%81%d0%ba%d0%b8%d0%b9-%d0%b8-%d0%b4%d0%b8%d0%bd%d0%b0%d0%bc%d0%b8%d1%87%d0%b5%d1%81%d0%ba%d0%b8%d0%b9-%d0%bf%d0%be%d0%b4%d1%85%d0%be%d0%b4" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Чтобы выбрать правильный инструмент, сравним эти два подхода:</p>
<table>
	<thead>
			<tr>
					<th>Свойство</th>
					<th>Статический полиморфизм (Generics)</th>
					<th>Динамический полиморфизм (<code>dyn Trait</code>)</th>
			</tr>
	</thead>
	<tbody>
			<tr>
					<td><strong>Выбор метода</strong></td>
					<td>При компиляции</td>
					<td>Во время работы программы (рантайм)</td>
			</tr>
			<tr>
					<td><strong>Размер бинарного файла</strong></td>
					<td>Чуть больше (компилятор копирует код под каждый тип)</td>
					<td>Меньше (один экземпляр функции на все типы)</td>
			</tr>
			<tr>
					<td><strong>Коллекции в памяти</strong></td>
					<td>Только один тип на вектор (например, строго <code>Vec&lt;i32&gt;</code>)</td>
					<td>Разные типы в одном векторе (<code>Vec&lt;Box&lt;dyn Task&gt;&gt;</code>)</td>
			</tr>
			<tr>
					<td><strong>Скорость вызова</strong></td>
					<td>Прямой вызов (максимальная скорость)</td>
					<td>Вызов через таблицу методов (небольшой накладной расход)</td>
			</tr>
	</tbody>
</table>

<h3 class="relative group">Пример разнородной коллекции
    <div id="пример-разнородной-коллекции" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%b8%d0%bc%d0%b5%d1%80-%d1%80%d0%b0%d0%b7%d0%bd%d0%be%d1%80%d0%be%d0%b4%d0%bd%d0%be%d0%b9-%d0%ba%d0%be%d0%bb%d0%bb%d0%b5%d0%ba%d1%86%d0%b8%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Чтобы сложить разные типы в один вектор, мы оборачиваем их в указатель <code>Box</code> и указываем ключевое слово <code>dyn</code>:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">trait</span><span class="w"> </span><span class="n">Task</span><span class="w"> </span><span class="p">{</span><span class="w"> 
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">fn</span> <span class="nf">execute</span><span class="p">(</span><span class="o">&amp;</span><span class="bp">self</span><span class="p">);</span><span class="w"> 
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">struct</span> <span class="nc">NetworkTask</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">struct</span> <span class="nc">DbTask</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="c1">// Реализуем Task для этих структур...
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="c1">// Складываем разные типы в один вектор
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">tasks</span>: <span class="nb">Vec</span><span class="o">&lt;</span><span class="nb">Box</span><span class="o">&lt;</span><span class="k">dyn</span><span class="w"> </span><span class="n">Task</span><span class="o">&gt;&gt;</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="fm">vec!</span><span class="p">[</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="nb">Box</span>::<span class="n">new</span><span class="p">(</span><span class="n">NetworkTask</span><span class="p">),</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="nb">Box</span>::<span class="n">new</span><span class="p">(</span><span class="n">DbTask</span><span class="p">),</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">];</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">for</span><span class="w"> </span><span class="n">task</span><span class="w"> </span><span class="k">in</span><span class="w"> </span><span class="n">tasks</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">task</span><span class="p">.</span><span class="n">execute</span><span class="p">();</span><span class="w"> </span><span class="c1">// Какая именно функция вызовется - вычисляется во время работы
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<div class="admonition relative overflow-hidden rounded-lg border-l-4 my-3 px-4 py-3 shadow-sm" data-type="note">
      <div class="flex items-center gap-2 font-semibold text-inherit">
        <div class="flex shrink-0 h-5 w-5 items-center justify-center text-lg"><span class="relative block icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path fill="currentColor" d="M256 0C114.6 0 0 114.6 0 256s114.6 256 256 256s256-114.6 256-256S397.4 0 256 0zM256 128c17.67 0 32 14.33 32 32c0 17.67-14.33 32-32 32S224 177.7 224 160C224 142.3 238.3 128 256 128zM296 384h-80C202.8 384 192 373.3 192 360s10.75-24 24-24h16v-64H224c-13.25 0-24-10.75-24-24S210.8 224 224 224h32c13.25 0 24 10.75 24 24v88h16c13.25 0 24 10.75 24 24S309.3 384 296 384z"/></svg>
</span></div>
        <div class="grow">
          Примечание
        </div>
      </div><div class="admonition-content mt-3 text-base leading-relaxed text-inherit"><p>Если список всех возможных типов известен заранее и не будет расширяться, хорошей альтернативой <code>dyn Trait</code> служит обычный <code>enum</code>.</p></div></div><hr>

<h2 class="relative group">2. Размеры типов и <code>?Sized</code>
    <div id="2-размеры-типов-и-sized" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#2-%d1%80%d0%b0%d0%b7%d0%bc%d0%b5%d1%80%d1%8b-%d1%82%d0%b8%d0%bf%d0%be%d0%b2-%d0%b8-sized" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>В Rust почти все типы имеют фиксированный размер в байтах, известный компилятору заранее. Их называют <code>Sized</code>. К примеру, <code>i64</code> всегда занимает 8 байт, структура из двух <code>i32</code> - 8 байт. Компилятор точно знает, сколько места выделить под них на стеке.</p>
<p>Но есть типы, размер которых вычисляется только во время работы программы. Их называют DST (Dynamically Sized Types):</p>
<ul>
<li>Слайсы текста <code>str</code></li>
<li>Слайсы массивов <code>[T]</code></li>
<li>Трейт-объекты вида <code>dyn Task</code></li>
</ul>
<div class="admonition relative overflow-hidden rounded-lg border-l-4 my-3 px-4 py-3 shadow-sm" data-type="caution">
      <div class="flex items-center gap-2 font-semibold text-inherit">
        <div class="flex shrink-0 h-5 w-5 items-center justify-center text-lg"><span class="relative block icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512">
<path fill="currentColor"  d="M159.3 5.4c7.8-7.3 19.9-7.2 27.7 .1c27.6 25.9 53.5 53.8 77.7 84c11-14.4 23.5-30.1 37-42.9c7.9-7.4 20.1-7.4 28 .1c34.6 33 63.9 76.6 84.5 118c20.3 40.8 33.8 82.5 33.8 111.9C448 404.2 348.2 512 224 512C98.4 512 0 404.1 0 276.5c0-38.4 17.8-85.3 45.4-131.7C73.3 97.7 112.7 48.6 159.3 5.4zM225.7 416c25.3 0 47.7-7 68.8-21c42.1-29.4 53.4-88.2 28.1-134.4c-2.8-5.6-5.6-11.2-9.8-16.8l-50.6 58.8s-81.4-103.6-87.1-110.6C133.1 243.8 112 273.2 112 306.8C112 375.4 162.6 416 225.7 416z"/></svg></span></div>
        <div class="grow">
          Предостережение
        </div>
      </div><div class="admonition-content mt-3 text-base leading-relaxed text-inherit"><p>Переменные типа DST запрещено хранить на стеке напрямую или передавать по значению. Переменная вида <code>let x: dyn Task</code> не скомпилируется, потому что Rust не знает, сколько байт выделить на стеке. Динамические типы всегда живут за указателями или ссылками: <code>&amp;str</code>, <code>&amp;[T]</code>, <code>Box&lt;dyn Task&gt;</code>.</p></div></div><p>По умолчанию компилятор считает, что любой параметр дженерика должен иметь фиксированный размер (<code>T: Sized</code>). Если мы хотим написать функцию, которая работает с динамическими типами по ссылке, нужно явно отменить это ограничение с помощью синтаксиса <code>?Sized</code> (&ldquo;размер может быть неизвестен&rdquo;):</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="c1">// T может быть как фиксированного размера, так и динамическим (например, &amp;str или &amp;dyn Debug)
</span></span></span><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">print_val</span><span class="o">&lt;</span><span class="n">T</span>: <span class="o">?</span><span class="nb">Sized</span><span class="w"> </span><span class="o">+</span><span class="w"> </span><span class="n">std</span>::<span class="n">fmt</span>::<span class="n">Debug</span><span class="o">&gt;</span><span class="p">(</span><span class="n">val</span>: <span class="kp">&amp;</span><span class="nc">T</span><span class="p">)</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;</span><span class="si">{:?}</span><span class="s">&#34;</span><span class="p">,</span><span class="w"> </span><span class="n">val</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<hr>

<h2 class="relative group">3. Как устроены «толстые» указатели и таблица методов (vtable)
    <div id="3-как-устроены-толстые-указатели-и-таблица-методов-vtable" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#3-%d0%ba%d0%b0%d0%ba-%d1%83%d1%81%d1%82%d1%80%d0%be%d0%b5%d0%bd%d1%8b-%d1%82%d0%be%d0%bb%d1%81%d1%82%d1%8b%d0%b5-%d1%83%d0%ba%d0%b0%d0%b7%d0%b0%d1%82%d0%b5%d0%bb%d0%b8-%d0%b8-%d1%82%d0%b0%d0%b1%d0%bb%d0%b8%d1%86%d0%b0-%d0%bc%d0%b5%d1%82%d0%be%d0%b4%d0%be%d0%b2-vtable" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Обычная ссылка в 64-битной системе (например, <code>&amp;i32</code> или <code>Box&lt;u64&gt;</code>) занимает в памяти <strong>8 байт</strong> (одно машинное слово). Это просто адрес в памяти.</p>
<p>Но ссылка на динамический тип <code>&amp;dyn Task</code> или <code>Box&lt;dyn Task&gt;</code> занимает <strong>16 байт (два машинных слова)</strong>. За такой двойной размер их называют <strong>толстыми указателями (Fat Pointers)</strong>.</p>
<p>Из чего состоят эти 16 байт?</p>
<ol>
<li><strong>Первые 8 байт:</strong> адрес самих данных (где лежит структура в куче или на стеке).</li>
<li><strong>Вторые 8 байт:</strong> адрес таблицы методов (<code>vtable</code>) для конкретного типа.</li>
</ol>
<div style="background-color:white; padding: 20px">
<pre class="not-prose mermaid">
graph TD
    subgraph FatPointer["Толстый указатель (16 байт)"]
        direction LR
        DataPtr["1. Адрес данных (8 байт)"]
        VtablePtr["2. Адрес vtable (8 байт)"]
    end

    DataMemory["<b>Данные в памяти</b><br/>──────────────────────<br/>NetworkTask<br/><i>(структура объекта)</i>"]
    VtableMemory["<b>Таблица методов (vtable)</b><br/>──────────────────────<br/>vtable для NetworkTask<br/>• Инструкция очистки (Drop)<br/>• Размер типа в байтах (size)<br/>• Выравнивание в памяти (align)<br/>• Указатель на метод execute()"]

    DataPtr --> DataMemory
    VtablePtr --> VtableMemory

    style FatPointer fill:#f4f4f6,stroke:#666,stroke-width:2px,color:#000
    style DataPtr fill:#fff,stroke:#333,color:#000
    style VtablePtr fill:#fff,stroke:#333,color:#000
    style DataMemory fill:#e8f4f8,stroke:#0288d1,stroke-width:2px,color:#000
    style VtableMemory fill:#efebe9,stroke:#5d4037,stroke-width:2px,color:#000
</pre>

</div>

<h3 class="relative group">Таблица методов (vtable) на практике
    <div id="таблица-методов-vtable-на-практике" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%82%d0%b0%d0%b1%d0%bb%d0%b8%d1%86%d0%b0-%d0%bc%d0%b5%d1%82%d0%be%d0%b4%d0%be%d0%b2-vtable-%d0%bd%d0%b0-%d0%bf%d1%80%d0%b0%d0%ba%d1%82%d0%b8%d0%ba%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Для каждого типа, реализующего трейт, компилятор при сборке создает небольшую статическую таблицу <code>vtable</code>. В ней хранятся:</p>
<ul>
<li>Адрес функции очистки ресурсов (<code>Drop</code>).</li>
<li>Размер объекта и требования к выравниванию в памяти (нужно, чтобы <code>Box</code> знал, сколько байт вернуть ОС при удалении).</li>
<li>Адреса всех методов, объявленных в трейте.</li>
</ul>
<p>Когда мы вызываем метод у трейт-объекта:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">task</span>: <span class="nb">Box</span><span class="o">&lt;</span><span class="k">dyn</span><span class="w"> </span><span class="n">Task</span><span class="o">&gt;</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="nb">Box</span>::<span class="n">new</span><span class="p">(</span><span class="n">NetworkTask</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="n">task</span><span class="p">.</span><span class="n">execute</span><span class="p">();</span></span></span></code></pre></div></div>
<p>Происходит следующее:</p>
<ol>
<li>Программа считывает второй адрес из толстого указателя и переходит к таблице <code>vtable</code>.</li>
<li>Из нужной строчки таблицы берется адрес функции <code>NetworkTask::execute</code>.</li>
<li>Функция вызывается, а в качестве первого аргумента <code>&amp;self</code> передается адрес данных из первой половины толстого указателя.</li>
</ol>
<hr>

<h2 class="relative group">4. Объединение трейтов и почему нельзя написать <code>dyn (Fly + Swim)</code>
    <div id="4-объединение-трейтов-и-почему-нельзя-написать-dyn-fly--swim" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#4-%d0%be%d0%b1%d1%8a%d0%b5%d0%b4%d0%b8%d0%bd%d0%b5%d0%bd%d0%b8%d0%b5-%d1%82%d1%80%d0%b5%d0%b9%d1%82%d0%be%d0%b2-%d0%b8-%d0%bf%d0%be%d1%87%d0%b5%d0%bc%d1%83-%d0%bd%d0%b5%d0%bb%d1%8c%d0%b7%d1%8f-%d0%bd%d0%b0%d0%bf%d0%b8%d1%81%d0%b0%d1%82%d1%8c-dyn-fly--swim" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>В Rust нельзя напрямую создать трейт-объект из двух разных трейтов:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="c1">// Ошибка компиляции!
</span></span></span><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">handle</span><span class="p">(</span><span class="n">obj</span>: <span class="kp">&amp;</span><span class="p">(</span><span class="k">dyn</span><span class="w"> </span><span class="n">Fly</span><span class="w"> </span><span class="o">+</span><span class="w"> </span><span class="n">Swim</span><span class="p">))</span><span class="w"> </span><span class="p">{}</span></span></span></code></pre></div></div>
<p>Причина проста: для этого компилятору потребовался бы «трехмерный» указатель (адрес данных + vtable1 + vtable2), что создавало бы путаницу и лишние расходы памяти.</p>

<h3 class="relative group">Как решить эту задачу в коде?
    <div id="как-решить-эту-задачу-в-коде" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%ba%d0%b0%d0%ba-%d1%80%d0%b5%d1%88%d0%b8%d1%82%d1%8c-%d1%8d%d1%82%d1%83-%d0%b7%d0%b0%d0%b4%d0%b0%d1%87%d1%83-%d0%b2-%d0%ba%d0%be%d0%b4%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Мы создаем один обобщённый трейт, который наследует нужные трейты. Затем пишем для него реализацию (Blanket Implementation):</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">trait</span><span class="w"> </span><span class="n">Fly</span><span class="w"> </span><span class="p">{</span><span class="w"> 
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">fn</span> <span class="nf">fly</span><span class="p">(</span><span class="o">&amp;</span><span class="bp">self</span><span class="p">)</span><span class="w"> </span>-&gt; <span class="nb">String</span><span class="p">;</span><span class="w"> 
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">trait</span><span class="w"> </span><span class="n">Swim</span><span class="w"> </span><span class="p">{</span><span class="w"> 
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">fn</span> <span class="nf">swim</span><span class="p">(</span><span class="o">&amp;</span><span class="bp">self</span><span class="p">)</span><span class="w"> </span>-&gt; <span class="nb">String</span><span class="p">;</span><span class="w"> 
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="c1">// Объединяем требования в один трейт
</span></span></span><span class="line"><span class="cl"><span class="k">trait</span><span class="w"> </span><span class="n">Duck</span>: <span class="nc">Fly</span><span class="w"> </span><span class="o">+</span><span class="w"> </span><span class="n">Swim</span><span class="w"> </span><span class="p">{}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="c1">// Автоматически реализуем Duck для любого типа, у которого есть и Fly, и Swim
</span></span></span><span class="line"><span class="cl"><span class="k">impl</span><span class="o">&lt;</span><span class="n">T</span>: <span class="nc">Fly</span><span class="w"> </span><span class="o">+</span><span class="w"> </span><span class="n">Swim</span><span class="o">&gt;</span><span class="w"> </span><span class="n">Duck</span><span class="w"> </span><span class="k">for</span><span class="w"> </span><span class="n">T</span><span class="w"> </span><span class="p">{}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="c1">// Теперь dyn Duck абсолютно легален и занимает стандартные 16 байт!
</span></span></span><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">handle_duck</span><span class="p">(</span><span class="n">duck</span>: <span class="kp">&amp;</span><span class="nc">dyn</span><span class="w"> </span><span class="n">Duck</span><span class="p">)</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Утка: </span><span class="si">{}</span><span class="s">, </span><span class="si">{}</span><span class="s">&#34;</span><span class="p">,</span><span class="w"> </span><span class="n">duck</span><span class="p">.</span><span class="n">fly</span><span class="p">(),</span><span class="w"> </span><span class="n">duck</span><span class="p">.</span><span class="n">swim</span><span class="p">());</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<hr>

<h2 class="relative group">5. Динамическая совместимость (Object Safety)
    <div id="5-динамическая-совместимость-object-safety" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#5-%d0%b4%d0%b8%d0%bd%d0%b0%d0%bc%d0%b8%d1%87%d0%b5%d1%81%d0%ba%d0%b0%d1%8f-%d1%81%d0%be%d0%b2%d0%bc%d0%b5%d1%81%d1%82%d0%b8%d0%bc%d0%be%d1%81%d1%82%d1%8c-object-safety" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Не любой трейт можно превратить в трейт-объект <code>dyn Trait</code>. Чтобы трейт поддерживал динамический полиморфизм, он должен удовлетворять правилам <strong>динамической совместимости (Object Safety)</strong>.</p>
<p>Суть этих правил легко понять, если вспомнить, как работает <code>vtable</code>: в таблицу можно записать только те методы, которые имеют конкретный фиксированный адрес и принимают ссылку на данные.</p>

<h3 class="relative group">Почему некоторые методы ломают Object Safety?
    <div id="почему-некоторые-методы-ломают-object-safety" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d0%be%d1%87%d0%b5%d0%bc%d1%83-%d0%bd%d0%b5%d0%ba%d0%be%d1%82%d0%be%d1%80%d1%8b%d0%b5-%d0%bc%d0%b5%d1%82%d0%be%d0%b4%d1%8b-%d0%bb%d0%be%d0%bc%d0%b0%d1%8e%d1%82-object-safety" aria-label="Якорь">#</a>
    </span>
    
</h3>
<ol>
<li><strong>Метод принимает <code>self</code> по значению (<code>fn run(self)</code>):</strong> Передача по значению требует положить объект на стек, но размер <code>dyn Trait</code> не известен во время сборки.</li>
<li><strong>Метод использует дженерики (<code>fn process&lt;T&gt;(&amp;self, val: T)</code>):</strong> Компилятор генерирует отдельную функцию под каждый <code>T</code>. Поместить бесконечное число вариантов функции в фиксированную таблицу <code>vtable</code> невозможно.</li>
<li><strong>Метод не принимает <code>self</code> (<code>fn new() -&gt; Self</code>):</strong> Если у метода нет аргумента <code>self</code>, программу нельзя направить в vtable конкретного экземпляра - у нас просто нет объекта, из которого можно прочитать адрес vtable.</li>
<li><strong>Метод возвращает или принимает тип <code>Self</code>:</strong> В рантайме за <code>dyn Trait</code> могут скрываться совершенно разные структуры, и Rust не сможет гарантировать безопасность типов при их сравнении или передаче.</li>
</ol>

<h3 class="relative group">Практическое решение с помощью <code>where Self: Sized</code>
    <div id="практическое-решение-с-помощью-where-self-sized" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%b0%d0%ba%d1%82%d0%b8%d1%87%d0%b5%d1%81%d0%ba%d0%be%d0%b5-%d1%80%d0%b5%d1%88%d0%b5%d0%bd%d0%b8%d0%b5-%d1%81-%d0%bf%d0%be%d0%bc%d0%be%d1%89%d1%8c%d1%8e-where-self-sized" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Если вам нужны конструкторы или дженерик-методы при обычной работе с типом, но вы все равно хотите использовать трейт через <code>dyn Trait</code>, добавьте к «проблемным» методам ограничение <code>where Self: Sized</code>:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">trait</span><span class="w"> </span><span class="n">SafeTrait</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">fn</span> <span class="nf">name</span><span class="p">(</span><span class="o">&amp;</span><span class="bp">self</span><span class="p">)</span><span class="w"> </span>-&gt; <span class="nb">String</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="c1">// Этот метод использует Self по значению и сломал бы Object Safety.
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="c1">// Ограничение where Self: Sized исключает его из vtable!
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">fn</span> <span class="nf">copy_by_val</span><span class="p">(</span><span class="bp">self</span><span class="p">)</span><span class="w"> </span>-&gt; <span class="nc">Self</span><span class="w"> </span><span class="k">where</span><span class="w"> </span><span class="bp">Self</span>: <span class="nb">Sized</span> <span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="bp">self</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p>Благодаря <code>where Self: Sized</code> компилятор понимает: этот метод будет доступен только при статической типизации, а из таблицы <code>vtable</code> для <code>dyn SafeTrait</code> его можно просто исключить. Трейт остается динамически совместимым!</p>
<p>Давайте посмотрим, как все эти механизмы работают на практике в интерактивных слайдах:</p>
### 💻 Трейт-объекты и динамический полиморфизм


#### Шаг 1


```rust
trait Task {
    fn execute(&self) -> String;
}

struct NetworkTask;
impl Task for NetworkTask {
    fn execute(&self) -> String { "Сеть".to_string() }
}

struct DbTask;
impl Task for DbTask {
    fn execute(&self) -> String { "База данных".to_string() }
}

fn main() {
    // Вектор содержит разные типы данных под общим трейт-объектом
    let tasks: Vec<Box<dyn Task>> = vec![
        Box::new(NetworkTask),
        Box::new(DbTask),
    ];
    for t in tasks {
        println!("Выполняю задачу: {}", t.execute());
    }
}
```


**Динамический полиморфизм и dyn Trait**
- Трейт-объект `dyn Task` позволяет работать с гетерогенными коллекциями, где типы известны только в рантайме.
- Для этого объекты обязательно помещаются за указатель (например, `Box<dyn Task>` или `&dyn Task`).
- Определение вызываемого метода происходит во время выполнения через таблицу виртуальных методов (vtable).


#### Шаг 2


```rust
// По умолчанию все дженерики имеют ограничение T: Sized.
// Мы отключаем его с помощью ?Sized, чтобы функция могла принимать слайсы или dyn Trait.
fn print_debug<T: ?Sized + std::fmt::Debug>(val: &T) {
    println!("Значение: {:?}", val);
}

fn main() {
    let s: &str = "Привет, мир!"; // str - это DST (Dynamically Sized Type)
    print_debug(s);
}
```


**Неявный bound Sized и ?Sized**
- Большинство типов имеют размер, известный при компиляции (Sized), и могут создаваться на стеке.
- Динамические типы (DST), такие как `str`, `[T]` или `dyn Trait`, не имеют фиксированного размера.
- Синтаксис `?Sized` отменяет неявное требование фиксированного размера, позволяя передавать DST по ссылке.


#### Шаг 3


```rust
trait Fly {
    fn fly(&self) -> String;
}

trait Swim {
    fn swim(&self) -> String;
}

// Rust запрещает &(dyn Fly + Swim) напрямую.
// Создаем общий трейт:
trait Duck: Fly + Swim {}

// И пишем Blanket-реализацию для всех подходящих типов:
impl<T: Fly + Swim> Duck for T {}

struct Mallard;
impl Fly for Mallard {
    fn fly(&self) -> String { "Летит".to_string() }
}
impl Swim for Mallard {
    fn swim(&self) -> String { "Плывет".to_string() }
}

fn handle_duck(duck: &dyn Duck) {
    println!("Утка: {}, {}", duck.fly(), duck.swim());
}

fn main() {
    let d = Mallard;
    handle_duck(&d);
}
```


**Двойные трейт-объекты**
- В Rust нельзя напрямую составить трейт-объект из нескольких независимых трейтов: `dyn Trait1 + Trait2` не скомпилируется.
- Для обхода создается собирающий трейт `Duck`, наследующий от нужных супертрейтов.
- Blanket implementation автоматически делает любой подходящий тип наследником `Duck`.


#### Шаг 4


```rust
// Трейт содержит метод `copy_by_val`, который возвращает Self по значению.
// Это ломает Object Safety (динамическую совместимость).
// Но мы изолируем его с помощью where Self: Sized!
trait SafeTrait {
    fn name(&self) -> String;
    
    // Этот метод не будет включен в vtable и не сломает dyn SafeTrait
    fn copy_by_val(self) -> Self where Self: Sized {
        self
    }
}

struct Item;
impl SafeTrait for Item {
    fn name(&self) -> String { "Предмет".to_string() }
}

fn print_name(obj: &dyn SafeTrait) {
    println!("Имя: {}", obj.name());
    // obj.copy_by_val(); // Ошибка! Метод недоступен для трейт-объекта.
}

fn main() {
    let item = Item;
    print_name(&item);
}
```


**Динамическая совместимость (Object Safety)**
- Трейт-объект можно создать только для Object Safe трейтов (нет Self по значению, нет дженерик-методов и т.д.).
- Конструкция `where Self: Sized` позволяет исключить проблемные методы из vtable.
- Трейт остается динамически совместимым, а «опасные» методы можно вызывать только у статических типов.


<hr>

<h2 class="relative group">Проверь свои знания!
    <div id="проверь-свои-знания" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%be%d0%b2%d0%b5%d1%80%d1%8c-%d1%81%d0%b2%d0%be%d0%b8-%d0%b7%d0%bd%d0%b0%d0%bd%d0%b8%d1%8f" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Пройдите короткий тест, чтобы проверить, насколько хорошо вы разобрались в устройстве трейт-объектов и динамического полиморфизма в Rust.</p>
### 📝 Трейт-объекты и динамический полиморфизм

**1. В чем отличие DST (Dynamically Sized Types) от обычных Sized типов?**

- [ ] DST типы могут располагаться только на стеке, а Sized типы - в куче
- [x] DST имеют размер, неизвестный на этапе компиляции. Их запрещено размещать на стеке напрямую или передавать по значению; работа с ними идет через указатели
- [ ] Для DST типов компилятор генерирует отдельные копии функций на этапе мономорфизации

> 💡 *Размер DST типов (таких как `str`, `[T]` или `dyn Trait`) вычисляется только в рантайме, поэтому компилятор не может выделить под них фиксированный стек-фрейм. Они всегда используются за указателями.*


**2. Из чего состоит «толстый указатель» (Fat Pointer) на трейт-объект в Rust?**

- [x] Из указателя на данные объекта в памяти и указателя на таблицу виртуальных функций (vtable)
- [ ] Из указателя на данные и длины объекта (количества элементов)
- [ ] Из указателя на vtable и количества методов в трейте

> 💡 *Толстый указатель на трейт-объект занимает 16 байт (два машинных слова на 64-битной архитектуре) и содержит адрес самого объекта в куче или на стеке плюс адрес статической таблицы vtable.*


**3. Какая информация НЕ хранится в таблице vtable для трейт-объекта?**

- [ ] Указатель на деструктор (Drop) для очистки памяти
- [ ] Размер и требования к выравниванию конкретного типа в памяти
- [x] Строковые имена методов для динамического поиска по имени в рантайме
- [ ] Адреса скомпилированных методов, реализующих трейт

> 💡 *В vtable нет имен методов. Вызовы функций разрешаются по фиксированным числовым смещениям (индексам в vtable), рассчитанным еще на этапе компиляции, что обеспечивает высокую скорость работы.*


**4. Какое свойство трейта нарушает его динамическую совместимость (Object Safety) и запрещает создавать dyn Trait?**

- [ ] Наличие методов, принимающих изменяемую ссылку `&mut self`
- [ ] Наличие ассоциированных типов
- [x] Наличие статических методов (без аргумента `self`), у которых отсутствует ограничение `where Self: Sized`

> 💡 *Функции без `self` не могут быть вызваны у трейт-объекта в рантайме, так как без экземпляра объекта компилятор не может получить vtable. Если такой метод не изолирован через `where Self: Sized`, трейт становится несовместим с dyn.*


**5. Как компилятор узнает, сколько байт памяти нужно освободить в аллокаторе при drop объекта Box<dyn Trait>?**

- [ ] Размер вычисляется путем сканирования байтов памяти кучи
- [ ] Аллокатор сам отслеживает размер всех выделений без участия Rust
- [x] Компилятор считывает размер (size) и выравнивание (align) исходного типа из таблицы vtable объекта

> 💡 *В vtable трейт-объекта записываются метаданные исходного типа (size и align). При уничтожении `Box<dyn Trait>` рантайм берет эти значения из vtable и передает деаллокатору `__rust_dealloc(ptr, size, align)`.*




