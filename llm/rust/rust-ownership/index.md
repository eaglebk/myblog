# Владение (Ownership) в Rust

> Владение (Ownership) в Rust — это метод контроля использования памяти, который помогает избежать ошибок таких как недопустимый доступ к данным или гонки данных.

- **URL:** https://eagle2000.space/rust/rust-ownership/
- **Date:** 2025-05-31

- **Tags:** rust, dev

---


<h2 class="relative group">Введение
    <div id="введение" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d0%b2%d0%b5%d0%b4%d0%b5%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>В Rust реализована концепция владения (ownership) определяющая правила работы с данными. Эти правила помогают избежать ошибок при управлении памятью.</p>
<p>В языке Rust каждое значение имеет единственного владельца. Владелец управляет временем жизни данных: когда он выходит из области видимости, память автоматически освобождается. Передача данных между переменными может происходить через копирование <code>copy</code> (для простых типов) или перемещение <code>move</code> (для сложных).</p>

<h3 class="relative group">Аналогия владения из обычной жизни
    <div id="аналогия-владения-из-обычной-жизни" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b0%d0%bd%d0%b0%d0%bb%d0%be%d0%b3%d0%b8%d1%8f-%d0%b2%d0%bb%d0%b0%d0%b4%d0%b5%d0%bd%d0%b8%d1%8f-%d0%b8%d0%b7-%d0%be%d0%b1%d1%8b%d1%87%d0%bd%d0%be%d0%b9-%d0%b6%d0%b8%d0%b7%d0%bd%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Представьте, что у каждой коробки с данными есть один владелец. Хозяйн коробки может владеть содержимым пока он находится в определенном помещении. Когда владелец покидает помещение и не возвращается за ней (выходит за фигурные скобки), коробка выбрасывается арендодателем (строгим borrow checker-ом). Если вы хотите передать коробку другому, старый владелец теряет к ней доступ (происходит перемещение <code>move</code>). Чтобы у другого человека появилась копия этой коробки - её нужно обязательно склонировать или создать дубликат.</p>

<h3 class="relative group">Пример
    <div id="пример" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%b8%d0%bc%d0%b5%d1%80" aria-label="Якорь">#</a>
    </span>
    
</h3>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">s1</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="nb">String</span>::<span class="n">from</span><span class="p">(</span><span class="s">&#34;hello&#34;</span><span class="p">);</span><span class="w"> </span><span class="n"> `[1]`
</span><span class="w"> </span><span class="c1">// s1 — владелец строки
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">s2</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">s1</span><span class="p">;</span><span class="w"> </span><span class="n"> `[2]`
</span><span class="w">                    </span><span class="c1">// данные перемещаются в s2, s1 больше не валидна
</span></span></span><span class="line"><span class="cl"><span class="c1">// println!(&#34;{}&#34;, s1);  `[3]`
          // Ошибка: попытка использовать перемещённое значение
</span></span></span></code></pre></div></div>
<ul>
<li>В данном примере у строки hello появился владелец - переменная <code>s1</code>  *(строка 1)*
</li>
<li>Затем владение передается переменной <code>s2</code>  *(строка 2)*
</li>
<li>Все! Больше переменная s1 ничем не владеет и получить доступ к строке hello не получится.</li>
<li>Если попробовать вывести  *(строка 3)*
 значение переменной <code>s1</code> то мы получим ошибку.</li>
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
    
    ><p>Вспомним аналогию с коробками:</p>
<p>Владелец с именем <code>s1</code> 👱‍♂️ коробки со значением строки <strong>hello</strong> 🔶 решил продать эту коробку. Он разместил объявление на онлайн площадках. Через некоторое время к нему подъехал покупатель с именем <code>s2</code> 👩 и приобрел коробку а также значение строки 🔶 в ней.</p>
<p>👱‍♂️🔶 &ndash;&gt; ➡️ 👩 &mdash;-&gt;  👩🔶</p>
<p>Больше коробка никак не связана со старым владельцем 👱‍♂️ 🚫 🔶. Он только может при желании выкупить ее у нового хозяина коробки <code>s2</code> 👩🔶.</p>
<p><code>s2</code> 👩🔶 волен распоряжаться ей как угодно: передать во временное хранение, продать, напечатать книгу с содержимым этой коробки.</p>
<p>Примерно так и рассуждает компилятор языка rust когда обрабатывает исходный код приложения.</p>
</span>
</div>


<h2 class="relative group">Основные понятия
    <div id="основные-понятия" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%be%d1%81%d0%bd%d0%be%d0%b2%d0%bd%d1%8b%d0%b5-%d0%bf%d0%be%d0%bd%d1%8f%d1%82%d0%b8%d1%8f" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Теперь закрепим основные понятия и определения:</p>
<ol>
<li><strong>Владелец</strong> (Owner) — это переменная, которая управляет данными (например: <code>let x = 5;</code>).</li>
<li><strong>Заимствование</strong> (Borrowing) — временное использование данных без передачи владения (например: <code>&amp;x</code>).</li>
<li><strong>Правила владения:</strong>
<ul>
<li>У данных может быть <strong>только один владелец</strong> в каждый момент времени.</li>
<li>Можно <strong>либо</strong> изменять данные, <strong>либо</strong> читать их, но не одновременно.</li>
<li>Данные автоматически освобождаются, когда их владелец выходит из области видимости.</li>
</ul>
</li>
<li><strong>Borrow checker</strong> (механизим проверяющий заимствования)
<ul>
<li>Это &ldquo;строгий охранник/арендодатель&rdquo; в компиляторе языка Rust. Он следит за соблюдением правил владения.</li>
</ul>
</li>
</ol>
<p>В его задачи входит:</p>
<ul>
<li>Предотвращение ошибок
<ul>
<li>Использование данных после их освобождения - проблема висячих указателей (dangling pointers).</li>
<li>Одновременное чтение и изменение данных - гонка данных (data races).</li>
<li>Утечки памяти.</li>
</ul>
</li>
</ul>
<p>Он работает <strong>на этапе компиляции</strong>, поэтому ошибки обнаруживаются до запуска программы.</p>

<h2 class="relative group">Стек (Stack) и куча (Heap)
    <div id="стек-stack-и-куча-heap" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%81%d1%82%d0%b5%d0%ba-stack-%d0%b8-%d0%ba%d1%83%d1%87%d0%b0-heap" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Для разработки на rust важно понимать устройство работы с памятью.</p>
<p>Выделяется 2 типа памяти: <em>cтек</em> и <em>куча</em>:</p>

<h3 class="relative group">О стеке (Stack)
    <div id="о-стеке-stack" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%be-%d1%81%d1%82%d0%b5%d0%ba%d0%b5-stack" aria-label="Якорь">#</a>
    </span>
    
</h3>
<ul>
<li>
<p><strong>Как работает:</strong></p>
<ul>
<li>Это быстрая и упорядоченная память. В стеке хранятся локальные переменные.</li>
<li>Работает по принципу &ldquo;последний вошёл — первый вышел&rdquo; (LIFO). <em><strong>Представьте стопку тарелок: кладёте по одной наверх и когда понадобилась -&gt; достаете её сверху.</strong></em></li>
<li>Размер данных на стеке известен <strong>на этапе компиляции</strong> (в стеке хранятся простые типы: числа, булевы значения&hellip;).</li>
</ul>
</li>
<li>
<p>Пример:</p>
</li>
</ul>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">main</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="n">x</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="mi">42</span><span class="p">;</span><span class="w"> </span><span class="c1">// `x` хранится на стеке
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="n">y</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="mi">10</span><span class="p">;</span><span class="w"> </span><span class="c1">// `y` тоже на стеке
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w"> </span><span class="n"> `[4]`
</span></span></span></code></pre></div></div>
<p>после выхода из блока  *(строка 4)*
 <code>x</code> и <code>y</code> автоматически удаляются</p>
<p><em>Когда функция завершается, её стековые данные мгновенно очищаются без дополнительных усилий.</em></p>

<h3 class="relative group">О куче (Heap)
    <div id="о-куче-heap" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%be-%d0%ba%d1%83%d1%87%d0%b5-heap" aria-label="Якорь">#</a>
    </span>
    
</h3>
<ul>
<li>
<p><strong>Как работает:</strong></p>
<ul>
<li>Это динамическая память, где хранятся данные, размер которых <strong>неизвестен заранее</strong> (например: строки, векторы, структуры&hellip;).</li>
<li>Выделение и освобождение памяти требует работы (аллокация и деаллокация).</li>
<li>В Rust для работы с кучей используются Box, Vec, String и другие умные указатели.</li>
<li><strong>Указатель</strong> на данные лежит в стеке, а сами данные — в куче.</li>
</ul>
</li>
<li>
<p>Пример:</p>
</li>
</ul>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">main</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="n">s</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="nb">String</span>::<span class="n">from</span><span class="p">(</span><span class="s">&#34;Hello&#34;</span><span class="p">);</span><span class="w"> </span><span class="c1">// данные строки – в куче, указатель – в стеке
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w"> </span><span class="n"> `[5]`
</span></span></span></code></pre></div></div>
<p><code>s</code> выходит из области видимости  *(строка 5)*
 память автоматически освобождается</p>
<p><em>Rust сам вызывает деструктор для освобождения памяти.</em></p>

<h2 class="relative group">Различия между стеком и кучей
    <div id="различия-между-стеком-и-кучей" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%80%d0%b0%d0%b7%d0%bb%d0%b8%d1%87%d0%b8%d1%8f-%d0%bc%d0%b5%d0%b6%d0%b4%d1%83-%d1%81%d1%82%d0%b5%d0%ba%d0%be%d0%bc-%d0%b8-%d0%ba%d1%83%d1%87%d0%b5%d0%b9" aria-label="Якорь">#</a>
    </span>
    
</h2>
<table>
	<thead>
			<tr>
					<th></th>
					<th>Стек</th>
					<th>Куча</th>
			</tr>
	</thead>
	<tbody>
			<tr>
					<td>Скорость</td>
					<td>Быстро (аллокация и доступ)</td>
					<td>Медленнее (аллокация дороже и сложнее)</td>
			</tr>
			<tr>
					<td>Размер</td>
					<td>Фиксированный размер данных</td>
					<td>Динамический размер</td>
			</tr>
			<tr>
					<td>Управление</td>
					<td>Автоматическое управление</td>
					<td>Нужны указатели (Rust сам следит ними)</td>
			</tr>
			<tr>
					<td>Доступ</td>
					<td>Локальность (L1/L2 кеш CPU)</td>
					<td>Доступ из любой части программы</td>
			</tr>
	</tbody>
</table>

<h2 class="relative group">Зачем такая сложность?
    <div id="зачем-такая-сложность" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b7%d0%b0%d1%87%d0%b5%d0%bc-%d1%82%d0%b0%d0%ba%d0%b0%d1%8f-%d1%81%d0%bb%d0%be%d0%b6%d0%bd%d0%be%d1%81%d1%82%d1%8c" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Rust <strong>строго контролирует доступ</strong> к куче через правила владения и заимствования.</p>
<p>Благодаря этому нет утечек памяти: когда что-то покидает стек - куча автоматически очищается.</p>
<p>В rust не нужно вручную вызывать <code>free/delete</code> (как в C++) или ждать сборщика мусора (как в Golang, Java).</p>
<p>На сегодня это все!</p>
<p>Чтобы не перегружать сразу большим количеством информации о новой концепции, я решил разбить тему владения на несколько статей.</p>
<div id="typeit-ae2b3399e22cfe5f8e21bee944ba02cf"></div>

<script>
    document.addEventListener("DOMContentLoaded", function () {
      var strings = ["","Продолжение следует...",""];
      
      new TypeIt("#typeit-ae2b3399e22cfe5f8e21bee944ba02cf", {
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


