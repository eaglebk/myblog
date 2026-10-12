# Циклы и диапазоны в Rust: loop, while и for на практике

> Пошаговый гайд по циклам в Rust: синтаксис loop, while, for, работа с диапазонами range, итераторами и досрочным выходом break/continue с примерами.

- **URL:** https://eagle2000.space/rust/rust-loops/
- **Date:** 2026-07-31

- **Tags:** rust, dev, Rust-basic

---


<h2 class="relative group">Введение
    <div id="введение" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d0%b2%d0%b5%d0%b4%d0%b5%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>В языке Rust есть три основных вида циклов: <code>while</code>, <code>loop</code> и <code>for</code>. Для управления итерациями используются стандартные операторы:</p>
<ul>
<li><strong><code>break</code></strong> — немедленно прерывает выполнение цикла.</li>
<li><strong><code>continue</code></strong> — пропускает оставшуюся часть тела цикла и переходит к следующей итерации.</li>
</ul>
<p>Давайте детально рассмотрим все виды циклов, способы обхода коллекций и работу с метками на интерактивных слайдах:</p>
### 💻 Циклы в Rust


#### Шаг 1


```rust
fn main() {
    let mut n = 0;
    // Цикл while
    while n < 3 {
        println!("while: n = {}", n);
        n += 1;
    }

    // Цикл loop с возвратом значения через break
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 5 {
            break counter * 2; // Возвращает 10 из цикла loop
        }
    };
    println!("Результат loop: {}", result);

    // Симуляция do-while через loop
    let mut x = 0;
    loop {
        x += 1;
        println!("do-while: x = {}", x);
        if x >= 3 {
            break;
        }
    }
}
```


**Циклы while, loop и возврат значений**
- `while` выполняет тело, пока истинно логическое условие.
- `loop` представляет собой бесконечный цикл.
- Выражение `loop` может возвращать значение наружу: для этого нужно указать его после оператора `break`.
- Отсутствующий в Rust цикл `do-while` можно сымитировать проверкой условия выхода в конце бесконечного `loop`.


#### Шаг 2


```rust
fn main() {
    let arr = [10, 20, 30];

    // Итерация по ссылкам (&arr не потребляет коллекцию)
    for val in &arr {
        println!("Ссылка: {}", val);
    }

    // Итерация с получением индекса через .enumerate()
    for (idx, val) in arr.into_iter().enumerate() {
        println!("Индекс {}: элемент {}", idx, val);
    }

    let mut mut_arr = [1, 2, 3];
    // Изменяемая итерация для модификации элементов
    for val in &mut mut_arr {
        *val *= 10;
    }
    println!("Измененный массив: {:?}", mut_arr);
}
```


**Диапазоны, цикл for и итерация**
- Цикл `for` итерируется по объектам, реализующим типаж `IntoIterator`.
- `for item in &arr` выполняет неизменяемое заимствование (итерируется по ссылкам `&T`).
- `for item in &mut arr` позволяет изменять элементы на месте.
- Метод `.enumerate()` возвращает кортеж `(index, item)` для одновременного доступа к счетчику и элементу.


#### Шаг 3


```rust
fn main() {
    // Метки циклов позволяют управлять внешними циклами из вложенных
    'outer: for i in 1..=3 {
        for j in 1..=3 {
            if i * j > 4 {
                println!("Выход из внешнего цикла при i={}, j={}", i, j);
                break 'outer;
            }
        }
    }

    // Метки именованных блоков кода
    let val = 'tag: {
        let x = 10;
        if x > 5 {
            break 'tag 100; // Досрочный выход из блока с возвратом значения 100
        }
        x
    };
    println!("Значение из блока: {}", val);
}
```


**Метки циклов и блоков (Labels)**
- Метки начинаются с апострофа (например, `'outer`) и позволяют управлять внешними циклами с помощью `break` или `continue`.
- Обычные блоки кода `{}` также могут иметь метки. Из них можно досрочно выйти с помощью `break 'tag` и вернуть значение.
- Использование `break` без метки внутри обычного блока кода (не цикла) вызовет ошибку компиляции.


<hr>

<h2 class="relative group">Виды циклов в Rust
    <div id="виды-циклов-в-rust" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d0%b8%d0%b4%d1%8b-%d1%86%d0%b8%d0%ba%d0%bb%d0%be%d0%b2-%d0%b2-rust" aria-label="Якорь">#</a>
    </span>
    
</h2>

<h3 class="relative group">1. Цикл <code>while</code>
    <div id="1-цикл-while" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#1-%d1%86%d0%b8%d0%ba%d0%bb-while" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Выполняет тело цикла до тех пор, пока условие истинно (<code>true</code>).</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="k">mut</span><span class="w"> </span><span class="n">n</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="mi">0</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">while</span><span class="w"> </span><span class="n">n</span><span class="w"> </span><span class="o">&lt;</span><span class="w"> </span><span class="mi">10</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">foo</span><span class="p">(</span><span class="n">n</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">n</span><span class="w"> </span><span class="o">+=</span><span class="w"> </span><span class="mi">1</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>

<h3 class="relative group">2. Цикл <code>while let</code>
    <div id="2-цикл-while-let" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#2-%d1%86%d0%b8%d0%ba%d0%bb-while-let" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Повторяет выполнение цикла, пока выражение успешно сопоставляется с шаблоном (например, для разбора <code>Option</code> или <code>Result</code>).</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="c1">// Цикл работает, пока foo() возвращает Some(x)
</span></span></span><span class="line"><span class="cl"><span class="k">while</span><span class="w"> </span><span class="kd">let</span><span class="w"> </span><span class="nb">Some</span><span class="p">(</span><span class="n">x</span><span class="p">)</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">foo</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">bar</span><span class="p">(</span><span class="n">x</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>

<h3 class="relative group">3. Цикл <code>loop</code> и возврат значений
    <div id="3-цикл-loop-и-возврат-значений" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#3-%d1%86%d0%b8%d0%ba%d0%bb-loop-%d0%b8-%d0%b2%d0%be%d0%b7%d0%b2%d1%80%d0%b0%d1%82-%d0%b7%d0%bd%d0%b0%d1%87%d0%b5%d0%bd%d0%b8%d0%b9" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Бесконечный цикл, который выполняется до тех пор, пока не будет встречен явный оператор <code>break</code>.</p>
<ul>
<li><strong>Использование как выражения:</strong> <code>loop</code> может возвращать значение через оператор <code>break</code>. Это значение можно присвоить переменной.</li>
</ul>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">event</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="k">loop</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="n">event</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">get_event</span><span class="p">();</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">if</span><span class="w"> </span><span class="kd">let</span><span class="w"> </span><span class="nb">Some</span><span class="p">(</span><span class="n">e</span><span class="p">)</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">event</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="k">break</span><span class="w"> </span><span class="n">e</span><span class="p">;</span><span class="w"> </span><span class="c1">// Возвращает значение e из цикла loop
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">};</span></span></span></code></pre></div></div>

<h3 class="relative group">4. Реализация цикла <code>do ... while</code>
    <div id="4-реализация-цикла-do--while" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#4-%d1%80%d0%b5%d0%b0%d0%bb%d0%b8%d0%b7%d0%b0%d1%86%d0%b8%d1%8f-%d1%86%d0%b8%d0%ba%d0%bb%d0%b0-do--while" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>В Rust нет встроенного ключевого слова <code>do-while</code>. Его поведение можно сымитировать двумя способами:</p>
<ol>
<li><strong>Через бесконечный <code>loop</code> и проверку условия в конце:</strong></li>
</ol>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">loop</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="n">x</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">foo</span><span class="p">();</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">if</span><span class="w"> </span><span class="n">x</span><span class="w"> </span><span class="o">==</span><span class="w"> </span><span class="mi">0</span><span class="w"> </span><span class="p">{</span><span class="w"> </span><span class="k">break</span><span class="p">;</span><span class="w"> </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<ol start="2">
<li><strong>Используя блок кода в качестве условия цикла <code>while</code>:</strong></li>
</ol>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">while</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="n">x</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">foo</span><span class="p">();</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">x</span><span class="w"> </span><span class="o">!=</span><span class="w"> </span><span class="mi">0</span><span class="w"> </span><span class="c1">// Условие продолжения цикла
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w"> </span><span class="p">{}</span><span class="w"> </span><span class="c1">// Пустое тело цикла
</span></span></span></code></pre></div></div>
<hr>

<h2 class="relative group">Диапазоны (Ranges) и цикл <code>for</code>
    <div id="диапазоны-ranges-и-цикл-for" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b4%d0%b8%d0%b0%d0%bf%d0%b0%d0%b7%d0%be%d0%bd%d1%8b-ranges-%d0%b8-%d1%86%d0%b8%d0%ba%d0%bb-for" aria-label="Якорь">#</a>
    </span>
    
</h2>

<h3 class="relative group">Типы диапазонов (Range Types)
    <div id="типы-диапазонов-range-types" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%82%d0%b8%d0%bf%d1%8b-%d0%b4%d0%b8%d0%b0%d0%bf%d0%b0%d0%b7%d0%be%d0%bd%d0%be%d0%b2-range-types" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Диапазоны представляют собой структуры из модуля <code>std::ops</code>. Нижняя граница (если она указана) <strong>всегда включается</strong> в диапазон.</p>
<table>
	<thead>
			<tr>
					<th>Синтаксис</th>
					<th>Описание типа</th>
					<th>Включение границ</th>
					<th>Внутреннее представление</th>
			</tr>
	</thead>
	<tbody>
			<tr>
					<td><code>..</code></td>
					<td>Неограниченный диапазон</td>
					<td>Нет границ</td>
					<td><code>RangeFull</code></td>
			</tr>
			<tr>
					<td><code>..10</code></td>
					<td>Ограниченный сверху диапазон</td>
					<td>Исключая верхнюю границу (0-9)</td>
					<td><code>RangeTo</code></td>
			</tr>
			<tr>
					<td><code>..=10</code></td>
					<td>Ограниченный сверху диапазон, включительно</td>
					<td>Включая верхнюю границу (0-10)</td>
					<td><code>RangeToInclusive</code></td>
			</tr>
			<tr>
					<td><code>0..</code></td>
					<td>Ограниченный снизу диапазон</td>
					<td>От 0 до бесконечности</td>
					<td><code>RangeFrom</code></td>
			</tr>
			<tr>
					<td><code>0..10</code></td>
					<td>Ограниченный диапазон</td>
					<td>Включая 0, исключая 10 (0-9)</td>
					<td><code>Range</code></td>
			</tr>
			<tr>
					<td><code>0..=10</code></td>
					<td>Ограниченный диапазон, включительно</td>
					<td>Включая обе границы 0 и 10 (0-10)</td>
					<td><code>RangeInclusive</code></td>
			</tr>
	</tbody>
</table>
<p><em>Пример:</em> Цикл <code>for n in 1..10</code> выполнится ровно <strong>9 раз</strong> (для значений от 1 до 9 включительно).</p>

<h3 class="relative group">Анатомия цикла <code>for</code>
    <div id="анатомия-цикла-for" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b0%d0%bd%d0%b0%d1%82%d0%be%d0%bc%d0%b8%d1%8f-%d1%86%d0%b8%d0%ba%d0%bb%d0%b0-for" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Цикл <code>for</code> используется для итерации по элементам коллекции или диапазона.</p>
<ul>
<li><strong>Требование к типу:</strong> Выражение справа от <code>in</code> обязано реализовывать типаж <strong><code>std::iter::IntoIterator</code></strong>.</li>
</ul>
<p>В зависимости от необходимости владения данными или ссылками, применяются разные подходы:</p>
<ol>
<li><strong>Потребление коллекции (Move):</strong> <code>for item in arr { ... }</code> (неявно вызывает <code>arr.into_iter()</code>).</li>
<li><strong>Итерация по разделяемым ссылкам (<code>&amp;T</code>):</strong> <code>for item in &amp;arr { ... }</code> (неявно вызывает <code>(&amp;arr).into_iter()</code>).</li>
<li><strong>Итерация по изменяемым ссылкам (<code>&amp;mut T</code>):</strong> <code>for item in &amp;mut arr { ... }</code> (неявно вызывает <code>(&amp;mut arr).into_iter()</code>).</li>
</ol>

<h3 class="relative group">Получение индекса при итерации (<code>.enumerate()</code>)
    <div id="получение-индекса-при-итерации-enumerate" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d0%be%d0%bb%d1%83%d1%87%d0%b5%d0%bd%d0%b8%d0%b5-%d0%b8%d0%bd%d0%b4%d0%b5%d0%ba%d1%81%d0%b0-%d0%bf%d1%80%d0%b8-%d0%b8%d1%82%d0%b5%d1%80%d0%b0%d1%86%d0%b8%d0%b8-enumerate" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Для одновременного получения индекса элемента и самого элемента используется метод <code>.enumerate()</code>, который возвращает кортеж вида <code>(index, item)</code>.</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">arr</span>: <span class="p">[</span><span class="kt">u8</span><span class="p">;</span><span class="w"> </span><span class="mi">3</span><span class="p">]</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="p">[</span><span class="mi">10</span><span class="p">,</span><span class="w"> </span><span class="mi">20</span><span class="p">,</span><span class="w"> </span><span class="mi">30</span><span class="p">];</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">for</span><span class="w"> </span><span class="p">(</span><span class="n">idx</span><span class="p">,</span><span class="w"> </span><span class="n">item</span><span class="p">)</span><span class="w"> </span><span class="k">in</span><span class="w"> </span><span class="n">arr</span><span class="p">.</span><span class="n">into_iter</span><span class="p">().</span><span class="n">enumerate</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;</span><span class="si">{idx}</span><span class="s">: </span><span class="si">{item}</span><span class="s">&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<hr>

<h2 class="relative group">Метки циклов и блоков (Labels)
    <div id="метки-циклов-и-блоков-labels" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bc%d0%b5%d1%82%d0%ba%d0%b8-%d1%86%d0%b8%d0%ba%d0%bb%d0%be%d0%b2-%d0%b8-%d0%b1%d0%bb%d0%be%d0%ba%d0%be%d0%b2-labels" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Метки позволяют управлять выполнением конкретного внешнего цикла или блока изнутри вложенных структур. Названия меток всегда начинаются с одиночной кавычки (апострофа), например: <code>'label</code>.</p>

<h3 class="relative group">Метки в циклах
    <div id="метки-в-циклах" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bc%d0%b5%d1%82%d0%ba%d0%b8-%d0%b2-%d1%86%d0%b8%d0%ba%d0%bb%d0%b0%d1%85" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Применяются для выхода из внешнего цикла при нахождении во вложенном.</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="na">&#39;outer</span>: <span class="nc">loop</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="na">&#39;inner</span>: <span class="nc">loop</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="k">if</span><span class="w"> </span><span class="n">need_exit</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">            </span><span class="k">break</span><span class="w"> </span><span class="nl">&#39;outer</span><span class="p">;</span><span class="w"> </span><span class="c1">// Полный выход из внешнего цикла
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>

<h3 class="relative group">Метки в именованных блоках кода
    <div id="метки-в-именованных-блоках-кода" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bc%d0%b5%d1%82%d0%ba%d0%b8-%d0%b2-%d0%b8%d0%bc%d0%b5%d0%bd%d0%be%d0%b2%d0%b0%d0%bd%d0%bd%d1%8b%d1%85-%d0%b1%d0%bb%d0%be%d0%ba%d0%b0%d1%85-%d0%ba%d0%be%d0%b4%d0%b0" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Обычные блоки кода <code>{ ... }</code> также могут иметь метки. Это позволяет досрочно прерывать выполнение текущего блока и возвращать значение во внешнюю переменную с помощью <code>break</code>.</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">y</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="na">&#39;tag</span>: <span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="n">x</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">foo</span><span class="p">();</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">if</span><span class="w"> </span><span class="n">x</span><span class="w"> </span><span class="o">&lt;</span><span class="w"> </span><span class="mi">0</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="k">break</span><span class="w"> </span><span class="nl">&#39;tag</span><span class="w"> </span><span class="mi">0</span><span class="p">;</span><span class="w"> </span><span class="c1">// Досрочный выход из блока с возвратом значения 0
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">x</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">};</span></span></span></code></pre></div></div>
<blockquote><p><strong>Ограничение компилятора:</strong> Оператор <code>break</code> без указания явной метки внутри обычного обособленного блока кода (не являющегося циклом) вызовет <strong>ошибку компиляции</strong>.</p>
</blockquote><hr>

<h2 class="relative group">Проверь свои знания!
    <div id="проверь-свои-знания" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%be%d0%b2%d0%b5%d1%80%d1%8c-%d1%81%d0%b2%d0%be%d0%b8-%d0%b7%d0%bd%d0%b0%d0%bd%d0%b8%d1%8f" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Пройдите короткий интерактивный тест, чтобы закрепить понимание циклов в Rust.</p>
### 📝 Циклы в Rust

**1. Каким образом бесконечный цикл `loop` может возвращать значение наружу?**

- [ ] С помощью ключевого слова `return`
- [x] Указав значение после оператора `break` (например, `break value;`)
- [ ] Цикл loop не умеет возвращать значения
- [ ] Просто оставив выражение без точки с запятой в конце тела цикла

> 💡 *В Rust оператор `break` в цикле `loop` может принимать выражение, результат которого будет возвращен как значение всего цикла `loop` (например, `let x = loop { break 42; };`).*


**2. Сколько раз выполнится тело цикла `for i in 1..5 { ... }`?**

- [ ] 5 раз (для 1, 2, 3, 4, 5)
- [x] 4 раза (для 1, 2, 3, 4)
- [ ] 6 раз (для 0, 1, 2, 3, 4, 5)
- [ ] Ни разу, так как синтаксис некорректен

> 💡 *Диапазон вида `1..5` (Range) является исключающим для верхней границы. Он включает значения 1, 2, 3, 4, поэтому цикл выполнится ровно 4 раза.*


**3. Какой типаж (trait) обязано реализовывать выражение после ключевого слова `in` в цикле `for`?**

- [ ] std::iter::Iterator
- [x] std::iter::IntoIterator
- [ ] std::clone::Clone
- [ ] std::ops::Index

> 💡 *Для работы цикла `for` выражение справа от `in` должно реализовывать типаж `std::iter::IntoIterator`, который позволяет преобразовать объект в итератор.*


**4. Для чего используется метод `.enumerate()` при итерации?**

- [ ] Для сортировки элементов по возрастанию
- [x] Для получения пар вида (индекс, элемент) на каждой итерации
- [ ] Для фильтрации дубликатов в коллекции
- [ ] Для преобразования всех элементов к строковому типу

> 💡 *Метод `.enumerate()` оборачивает итератор и на каждом шаге выдает кортеж `(index, item)`, содержащий порядковый номер итерации (начиная с 0) и сам элемент.*


**5. Какое из утверждений о метках (labels) в Rust является неверным?**

- [ ] Названия меток всегда начинаются с одиночной кавычки (например, `'outer`)
- [ ] Метки можно ставить как на циклы, так и на обычные блоки кода `{}`
- [ ] Из именованного блока кода `{}` можно досрочно выйти с помощью `break 'label`
- [x] Оператор `break` без указания метки можно использовать для выхода из обычного обособленного блока кода `{}`

> 💡 *Использование оператора `break` без указания метки внутри обычного блока кода `{ ... }` (который не является циклом) приведет к ошибке компиляции.*




