# Условные конструкции и ветвление в Rust

> Изучите основы ветвления в Rust: блоки кода, выражения if-else, сопоставление шаблонов через if let и цепочки условий let-chains.

- **URL:** https://eagle2000.space/rust/rust-conditionals/
- **Date:** 2026-07-31

- **Tags:** rust, dev

---


<h2 class="relative group">Введение. Блок кода (Code Block) и выражения
    <div id="введение-блок-кода-code-block-и-выражения" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d0%b2%d0%b5%d0%b4%d0%b5%d0%bd%d0%b8%d0%b5-%d0%b1%d0%bb%d0%be%d0%ba-%d0%ba%d0%be%d0%b4%d0%b0-code-block-%d0%b8-%d0%b2%d1%8b%d1%80%d0%b0%d0%b6%d0%b5%d0%bd%d0%b8%d1%8f" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Блок кода в Rust является фундаментальной синтаксической единицей. Он не просто группирует инструкции, но и сам по себе является выражением, возвращающим значение.</p>
<ul>
<li><strong>Синтаксис:</strong> Ограничивается фигурными скобками <code>{ ... }</code>. Внутри блока может содержаться набор инструкций, завершающихся точкой с запятой <code>;</code>.</li>
<li><strong>Возвращаемое значение:</strong>
<ul>
<li>Если последняя строка блока является выражением без точки с запятой в конце, её результат становится возвращаемым значением всего блока кода.</li>
<li>Если последняя строка завершается точкой с запятой <code>;</code> (или блок кода пуст), блок возвращает тип <strong><code>()</code></strong> (пустой кортеж / unit-тип).</li>
</ul>
</li>
</ul>
<p>Давайте изучим блоки кода, синтаксис <code>if-else</code> и сопоставления на интерактивных слайдах:</p>
### 💻 Ветвление и блоки кода


#### Шаг 1


```rust
fn main() {
    // Блок кода вычисляется в значение последнего выражения
    let y = {
        let x = 1;
        let z = 2;
        x + z // Нет точки с запятой в конце -> возвращает 3
    };

    // Блок кода завершающийся точкой с запятой возвращает ()
    let u = {
        let x = 10;
        println!("x = {}", x);
    };

    println!("y = {}, u = {:?}", y, u);
}
```


**Блоки кода (Code Blocks)**
- Блок кода ограничивается фигурными скобками `{}`.
- Если последнее выражение блока не заканчивается `;`, его результат возвращается из блока.
- Блок кода, оканчивающийся на `;`, или пустой блок возвращает unit-тип `()`.


#### Шаг 2


```rust
fn main() {
    let x = 150;

    // В Rust if-else является выражением и может возвращать значение
    let y = if x > 100 {
        let val = 2024;
        val
    } else {
        let val = 2025;
        val
    };

    println!("y = {}", y);

    // ОШИБКА КОМПИЛЯЦИИ: типы веток if-else должны быть одинаковыми!
    // let z = if x > 100 { 2024 } else { "2025" };
}
```


**Условные выражения if-else**
- Круглые скобки вокруг условия ставить не нужно (например, `if x > 100` вместо `if (x > 100)`).
- Условие обязано возвращать строго логический тип `bool`.
- Ветви `if` и `else` обязаны возвращать значения одного и того же типа. В Rust нет классического тернарного оператора, его роль играет `if-else`.


#### Шаг 3


```rust
fn main() {
    let some_val: Option<i32> = Some(150);

    // Использование if let для простого сопоставления с шаблоном
    if let Some(x) = some_val {
        println!("Совпало! x = {}", x);
    }

    let another_val: Option<i32> = Some(50);

    // Начиная с Rust 1.88 поддерживаются цепочки if let с && (let_chains)
    if let Some(x) = some_val && let Some(y) = another_val && x > y {
        println!("Обе переменные извлечены, и x ({}) > y ({})", x, y);
    }
}
```


**Конструкция if let и цепочки условий**
- `if let` — лаконичный аналог `match` для проверки соответствия конкретному шаблону.
- Переменная, связанная в шаблоне, видна только внутри блока `if let`.
- В современных версиях Rust (начиная с 1.88) можно комбинировать несколько `let` и логических выражений через `&&`.


<hr>

<h2 class="relative group">Условные конструкции и ветвление
    <div id="условные-конструкции-и-ветвление" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%83%d1%81%d0%bb%d0%be%d0%b2%d0%bd%d1%8b%d0%b5-%d0%ba%d0%be%d0%bd%d1%81%d1%82%d1%80%d1%83%d0%ba%d1%86%d0%b8%d0%b8-%d0%b8-%d0%b2%d0%b5%d1%82%d0%b2%d0%bb%d0%b5%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>

<h3 class="relative group">Базовое выражение <code>if</code> и <code>else</code>
    <div id="базовое-выражение-if-и-else" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b1%d0%b0%d0%b7%d0%be%d0%b2%d0%be%d0%b5-%d0%b2%d1%8b%d1%80%d0%b0%d0%b6%d0%b5%d0%bd%d0%b8%d0%b5-if-%d0%b8-else" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Конструкция <code>if</code> проверяет истинность условия и выполняет соответствующую ветку кода.</p>
<ul>
<li><strong>Правила синтаксиса:</strong>
<ul>
<li>Выражение условия <strong>не нужно</strong> оборачивать в круглые скобки <code>( )</code>. Использование круглых скобок вокруг самого выражения условия (например, <code>if (a &gt; 5)</code>) считается нарушением стиля.</li>
<li>Результат условного выражения строго обязан иметь тип <strong><code>bool</code></strong>. Использование целых чисел (как в C/C++) приведет к ошибке компиляции.</li>
</ul>
</li>
</ul>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="c1">// ОШИБКА КОМПИЛЯЦИИ: count имеет тип отличный от bool
</span></span></span><span class="line"><span class="cl"><span class="k">if</span><span class="w"> </span><span class="n">count</span><span class="w"> </span><span class="p">{</span><span class="w"> </span><span class="o">..</span><span class="p">.</span><span class="w"> </span><span class="p">}</span><span class="w"> 
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="c1">// КОРРЕКТНО: выражения возвращают bool
</span></span></span><span class="line"><span class="cl"><span class="k">if</span><span class="w"> </span><span class="n">count</span><span class="w"> </span><span class="o">&gt;</span><span class="w"> </span><span class="mi">0</span><span class="w"> </span><span class="p">{</span><span class="w"> </span><span class="o">..</span><span class="p">.</span><span class="w"> </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">if</span><span class="w"> </span><span class="n">count</span><span class="w"> </span><span class="o">!=</span><span class="w"> </span><span class="mi">0</span><span class="w"> </span><span class="p">{</span><span class="w"> </span><span class="o">..</span><span class="p">.</span><span class="w"> </span><span class="p">}</span></span></span></code></pre></div></div>

<h3 class="relative group">Операторы сравнения и логические операторы
    <div id="операторы-сравнения-и-логические-операторы" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%be%d0%bf%d0%b5%d1%80%d0%b0%d1%82%d0%be%d1%80%d1%8b-%d1%81%d1%80%d0%b0%d0%b2%d0%bd%d0%b5%d0%bd%d0%b8%d1%8f-%d0%b8-%d0%bb%d0%be%d0%b3%d0%b8%d1%87%d0%b5%d1%81%d0%ba%d0%b8%d0%b5-%d0%be%d0%bf%d0%b5%d1%80%d0%b0%d1%82%d0%be%d1%80%d1%8b" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Используются для формирования условных выражений:</p>
<ul>
<li><code>&gt;</code> (больше), <code>&gt;=</code> (больше либо равно)</li>
<li><code>&lt;</code> (меньше), <code>&lt;=</code> (меньше либо равно)</li>
<li><code>==</code> (равенство), <code>!=</code> (неравенство)</li>
<li><code>&amp;&amp;</code> (логическое И), <code>||</code> (логическое ИЛИ)</li>
</ul>

<h3 class="relative group"><code>if</code> как выражение (Assigning from <code>if</code>)
    <div id="if-как-выражение-assigning-from-if" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#if-%d0%ba%d0%b0%d0%ba-%d0%b2%d1%8b%d1%80%d0%b0%d0%b6%d0%b5%d0%bd%d0%b8%d0%b5-assigning-from-if" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>В Rust <code>if-else</code> является выражением, а значит, может возвращать значение, которое можно присвоить переменной.</p>
<ul>
<li><strong>Критическое правило типов:</strong> Все ветви конструкции <code>if-else</code> при использовании в качестве выражения <strong>обязаны возвращать один и тот же тип данных</strong>.</li>
<li>В Rust нет классического тернарного оператора <code>? :</code>. Вместо него используется <code>if-else</code>.</li>
</ul>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="c1">// Корректно: обе ветки возвращают i32
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">y</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="k">if</span><span class="w"> </span><span class="n">x</span><span class="w"> </span><span class="o">&gt;</span><span class="w"> </span><span class="mi">100</span><span class="w"> </span><span class="p">{</span><span class="w"> </span><span class="mi">2024</span><span class="w"> </span><span class="p">}</span><span class="w"> </span><span class="k">else</span><span class="w"> </span><span class="p">{</span><span class="w"> </span><span class="mi">2025</span><span class="w"> </span><span class="p">};</span><span class="w"> 
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="c1">// ОШИБКА КОМПИЛЯЦИИ (Несовпадение типов: i32 и &amp;str):
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">y</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="k">if</span><span class="w"> </span><span class="n">x</span><span class="w"> </span><span class="o">&gt;</span><span class="w"> </span><span class="mi">100</span><span class="w"> </span><span class="p">{</span><span class="w"> </span><span class="mi">2024</span><span class="w"> </span><span class="p">}</span><span class="w"> </span><span class="k">else</span><span class="w"> </span><span class="p">{</span><span class="w"> </span><span class="s">&#34;2025&#34;</span><span class="w"> </span><span class="p">};</span><span class="w"> 
</span></span></span></code></pre></div></div>
<p>Если ветка <code>else</code> отсутствует, то выражение <code>if</code> обязано возвращать unit-тип <code>()</code>, так как если условие не выполнится, результатом по умолчанию будет <code>()</code>:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">u</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="k">if</span><span class="w"> </span><span class="n">x</span><span class="w"> </span><span class="o">&gt;</span><span class="w"> </span><span class="mi">100</span><span class="w"> </span><span class="p">{</span><span class="w"> </span><span class="n">foo</span><span class="p">()</span><span class="w"> </span><span class="p">};</span><span class="w"> </span><span class="c1">// Допустимо, только если foo() возвращает ()
</span></span></span></code></pre></div></div>
<hr>

<h2 class="relative group">Конструкция <code>if let</code> и сопоставление с шаблоном
    <div id="конструкция-if-let-и-сопоставление-с-шаблоном" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%ba%d0%be%d0%bd%d1%81%d1%82%d1%80%d1%83%d0%ba%d1%86%d0%b8%d1%8f-if-let-%d0%b8-%d1%81%d0%be%d0%bf%d0%be%d1%81%d1%82%d0%b0%d0%b2%d0%bb%d0%b5%d0%bd%d0%b8%d0%b5-%d1%81-%d1%88%d0%b0%d0%b1%d0%bb%d0%be%d0%bd%d0%be%d0%bc" aria-label="Якорь">#</a>
    </span>
    
</h2>

<h3 class="relative group">Базовый <code>if let</code>
    <div id="базовый-if-let" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b1%d0%b0%d0%b7%d0%be%d0%b2%d1%8b%d0%b9-if-let" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Используется как лаконичная альтернатива <code>match</code>, когда нас интересует только один конкретный шаблон.</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="c1">// Для Option
</span></span></span><span class="line"><span class="cl"><span class="k">if</span><span class="w"> </span><span class="kd">let</span><span class="w"> </span><span class="nb">Some</span><span class="p">(</span><span class="n">x</span><span class="p">)</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">foo_opt</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">bar</span><span class="p">(</span><span class="n">x</span><span class="p">);</span><span class="w"> </span><span class="c1">// x доступен только внутри этого блока
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w"> </span><span class="k">else</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="c1">// Выполняется, если результат None
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>

<h3 class="relative group">Цепочки <code>if let</code> (let-chains)
    <div id="цепочки-if-let-let-chains" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%86%d0%b5%d0%bf%d0%be%d1%87%d0%ba%d0%b8-if-let-let-chains" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Начиная с версии <strong>Rust 1.88</strong>, поддерживается синтаксис объединения нескольких <code>let</code>-шаблонов и логических условий через оператор <code>&amp;&amp;</code> в одном выражении <code>if</code>.</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="c1">// Сопоставление с шаблоном + дополнительное ограничение на переменную x
</span></span></span><span class="line"><span class="cl"><span class="k">if</span><span class="w"> </span><span class="kd">let</span><span class="w"> </span><span class="nb">Some</span><span class="p">(</span><span class="n">x</span><span class="p">)</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">foo</span><span class="p">()</span><span class="w"> </span><span class="o">&amp;&amp;</span><span class="w"> </span><span class="n">x</span><span class="w"> </span><span class="o">&gt;</span><span class="w"> </span><span class="mi">100</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="fm">todo!</span><span class="p">()</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="c1">// Множественные сопоставления с шаблонами в одном условии
</span></span></span><span class="line"><span class="cl"><span class="k">if</span><span class="w"> </span><span class="kd">let</span><span class="w"> </span><span class="nb">Some</span><span class="p">(</span><span class="n">x</span><span class="p">)</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">foo</span><span class="p">()</span><span class="w"> </span><span class="o">&amp;&amp;</span><span class="w"> </span><span class="kd">let</span><span class="w"> </span><span class="nb">Some</span><span class="p">(</span><span class="n">y</span><span class="p">)</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">bar</span><span class="p">()</span><span class="w"> </span><span class="o">&amp;&amp;</span><span class="w"> </span><span class="n">x</span><span class="w"> </span><span class="o">&gt;</span><span class="w"> </span><span class="n">y</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="fm">todo!</span><span class="p">()</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<hr>

<h2 class="relative group">Проверь свои знания!
    <div id="проверь-свои-знания" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%be%d0%b2%d0%b5%d1%80%d1%8c-%d1%81%d0%b2%d0%be%d0%b8-%d0%b7%d0%bd%d0%b0%d0%bd%d0%b8%d1%8f" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Пройдите короткий интерактивный тест, чтобы закрепить понимание блоков кода и ветвлений в Rust.</p>
### 📝 Условные конструкции в Rust

**1. Что возвращает блок кода `{ let x = 5; x * 2; }`?**

- [ ] 10
- [ ] 5
- [x] Пустой кортеж `()` (unit-тип)
- [ ] Ошибку компиляции из-за отсутствия явного return

> 💡 *Блок кода оканчивается выражением с точкой с запятой `;` (`x * 2;`), поэтому он вычисляется в единичный тип `()`.*


**2. Какое из следующих условий в конструкции `if` вызовет ошибку компиляции?**

- [ ] if x != 0 { ... }
- [x] if x { ... } (где x равен 1 типа i32)
- [ ] if x > 5 && y < 10 { ... }
- [ ] if true { ... }

> 💡 *В Rust условие `if` обязано иметь строго тип `bool`. В отличие от C/C++, целые числа (например, 1) автоматически в логическое значение не приводятся и вызывают ошибку компиляции.*


**3. Какое критическое требование предъявляется к веткам `if-else`, когда они используются как выражение?**

- [ ] Они обязаны возвращать целочисленный тип
- [ ] Они должны быть заключены в круглые скобки
- [x] Обе ветки (if и else) обязаны возвращать значение одного и того же типа
- [ ] Ветка else должна отсутствовать

> 💡 *Поскольку `if-else` является выражением, возвращаемые значения всех ветвей обязаны принадлежать к одному типу, чтобы тип результирующей переменной был однозначно известен компилятору.*


**4. Что возвращает выражение `let u = if x > 100 { foo() };` если foo() возвращает i32 и x равен 50?**

- [ ] 0
- [ ] Option::None
- [x] Ошибку компиляции (типы не совпадают)
- [ ] i32

> 💡 *Если ветка `else` отсутствует, результатом всего выражения `if` по умолчанию в случае ложного условия является `()`. Поскольку ветка `if` пытается вернуть `i32` (через `foo()`), компилятор сообщает об ошибке несоответствия типов `i32` и `()`.*


**5. Какое логическое выражение объединяет несколько шаблонов `let` и условий в цепочке `if let`?**

- [ ] || (ИЛИ)
- [x] && (И)
- [ ] , (запятая)
- [ ] and

> 💡 *В цепочках `if let` (let-chains), доступных с Rust 1.88, сопоставления шаблонов и логические условия связываются через оператор конъюнкции `&&`.*




