# Операторы let-else и пропагации ошибок в Rust

> Продвинутый контроль выполнения в Rust: пропагация ошибок и отсутствие значений с помощью оператора ?, безопасное извлечение с let-else и требования к расходящимся блокам.

- **URL:** https://eagle2000.space/rust/rust-flow-control/
- **Date:** 2026-08-02

- **Tags:** rust, dev

---


<h2 class="relative group">Введение
    <div id="введение" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d0%b2%d0%b5%d0%b4%d0%b5%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Написание выразительного кода требует лаконичных инструментов управления потоком выполнения. В Rust для борьбы с глубокой вложенностью кода (так называемым «callback hell» или «лестницей скобок») используются мощные операторы:</p>
<ul>
<li><strong>Оператор <code>?</code></strong> — для быстрой передачи ошибок и отсутствия значений наверх по стеку вызовов.</li>
<li><strong>Конструкция <code>let-else</code></strong> — для безопасного развертывания опровержимых шаблонов с гарантированным расходящимся выходом в случае неудачи.</li>
</ul>
<p>Давайте изучим работу этих операторов на интерактивных примерах кода:</p>
### 💻 Операторы let-else и ?


#### Шаг 1


```rust
fn find_number(option: Option<i32>) -> Option<i32> {
    // Оператор ? возвращает значение внутри Some(x), либо делает досрочный возврат None
    let num = option?;
    Some(num * 2)
}

fn parse_and_double(s: &str) -> Result<i32, std::num::ParseIntError> {
    // Оператор ? возвращает i32, либо немедленно пробрасывает ошибку Err вверх
    let val = s.parse::<i32>()?;
    Ok(val * 2)
}

fn main() {
    let result_opt = find_number(Some(21));
    let none_opt = find_number(None);

    let result_res = parse_and_double("10");
    let err_res = parse_and_double("abc");

    println!("opt success: {:?}, opt none: {:?}", result_opt, none_opt);
    println!("res success: {:?}, res err: {:?}", result_res, err_res);
}
```


**Оператор ? для пропагации ошибок**
- Оператор `?` используется для быстрого извлечения значений из `Option` и `Result`.
- Для `Option`: если результат `Some(x)`, извлекает `x`; если `None`, делает досрочный возврат `None` из функции.
- Для `Result`: если результат `Ok(x)`, извлекает `x`; если `Err(e)`, делает досрочный возврат `Err(e.into())`.


#### Шаг 2


```rust
fn handle_event(event: Option<String>) {
    // let-else извлекает значение из Some, либо выполняет расходящийся блок else
    let Some(name) = event else {
        println!("Событие не содержит имени, завершаем функцию.");
        return; // Обязательный расходящийся выход
    };

    println!("Обработка события: {}", name);
}

fn main() {
    handle_event(Some("Click".to_string()));
    handle_event(None);

    // Пример let-else в цикле с continue
    for i in 0..5 {
        let opt = if i % 2 == 0 { Some(i) } else { None };
        
        let Some(val) = opt else {
            continue; // Пропускаем нечетные числа
        };
        println!("Четное число: {}", val);
    }
}
```


**Конструкция let-else (Diverging let)**
- `let-else` позволяет сопоставить опровержимый шаблон (например, `Some(x)`) и связать переменную в текущей области видимости.
- Если шаблон не совпал, выполняется блок `else`, который обязан быть **расходящимся** (diverging).
- Расходящийся блок `else` не имеет права просто вернуть управление; он обязан завершиться с помощью `return`, `break`, `continue` или `panic!()`.


<hr>

<h2 class="relative group">Оператор <code>?</code> (Error &amp; Option Propagation)
    <div id="оператор--error--option-propagation" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%be%d0%bf%d0%b5%d1%80%d0%b0%d1%82%d0%be%d1%80--error--option-propagation" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Оператор <code>?</code> используется для быстрой пропагации (передачи наверх) ошибок или отсутствующих значений. Он разворачивает успешный результат, а в случае неудачи — осуществляет немедленный досрочный возврат из текущей функции.</p>

<h3 class="relative group">1. Работа с типом <code>Option</code>
    <div id="1-работа-с-типом-option" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#1-%d1%80%d0%b0%d0%b1%d0%be%d1%82%d0%b0-%d1%81-%d1%82%d0%b8%d0%bf%d0%be%d0%bc-option" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Если результатом выражения является <code>Some(r)</code>, оператор <code>?</code> извлекает и возвращает <code>r</code>. Если результат <code>None</code>, происходит досрочный выход из функции с возвратом <code>None</code>.</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="c1">// Эквивалентные конструкции:
</span></span></span><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">foo</span><span class="p">()</span><span class="w"> </span>-&gt; <span class="nb">Option</span><span class="o">&lt;</span><span class="kt">u32</span><span class="o">&gt;</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="n">value</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">bar</span><span class="p">()</span><span class="o">?</span><span class="p">;</span><span class="w"> </span><span class="c1">// Использование оператора ?
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="nb">Some</span><span class="p">(</span><span class="n">value</span><span class="p">)</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="c1">// Что происходит под капотом:
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">value</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="k">match</span><span class="w"> </span><span class="n">bar</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="nb">Some</span><span class="p">(</span><span class="n">r</span><span class="p">)</span><span class="w"> </span><span class="o">=&gt;</span><span class="w"> </span><span class="n">r</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="nb">None</span><span class="w"> </span><span class="o">=&gt;</span><span class="w"> </span><span class="k">return</span><span class="w"> </span><span class="nb">None</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">};</span></span></span></code></pre></div></div>

<h3 class="relative group">2. Работа с типом <code>Result</code>
    <div id="2-работа-с-типом-result" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#2-%d1%80%d0%b0%d0%b1%d0%be%d1%82%d0%b0-%d1%81-%d1%82%d0%b8%d0%bf%d0%be%d0%bc-result" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Если результатом является <code>Ok(r)</code>, возвращается <code>r</code>. Если <code>Err(err)</code>, происходит досрочный возврат из функции значения <code>Err(err.into())</code> (с автоматическим приведением типов ошибок через типаж <code>From/Into</code>).</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="c1">// Эквивалентные конструкции:
</span></span></span><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">foo</span><span class="p">()</span><span class="w"> </span>-&gt; <span class="nb">Result</span><span class="o">&lt;</span><span class="kt">u32</span><span class="p">,</span><span class="w"> </span><span class="n">Error</span><span class="o">&gt;</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="n">value</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">bar</span><span class="p">()</span><span class="o">?</span><span class="p">;</span><span class="w"> </span><span class="c1">// Использование оператора ?
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="nb">Ok</span><span class="p">(</span><span class="n">value</span><span class="p">)</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="c1">// Что происходит под капотом:
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">value</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="k">match</span><span class="w"> </span><span class="n">bar</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="nb">Ok</span><span class="p">(</span><span class="n">r</span><span class="p">)</span><span class="w"> </span><span class="o">=&gt;</span><span class="w"> </span><span class="n">r</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="nb">Err</span><span class="p">(</span><span class="n">err</span><span class="p">)</span><span class="w"> </span><span class="o">=&gt;</span><span class="w"> </span><span class="k">return</span><span class="w"> </span><span class="nb">Err</span><span class="p">(</span><span class="n">err</span><span class="p">.</span><span class="n">into</span><span class="p">()),</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">};</span></span></span></code></pre></div></div>
<hr>

<h2 class="relative group">Конструкция <code>let - else</code> (Diverging <code>let</code>)
    <div id="конструкция-let---else-diverging-let" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%ba%d0%be%d0%bd%d1%81%d1%82%d1%80%d1%83%d0%ba%d1%86%d0%b8%d1%8f-let---else-diverging-let" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Синтаксис <code>let-else</code> позволяет связать переменную по шаблону в случае успеха, либо выполнить расходящийся (diverging) блок кода в случае неудачи.</p>
<ul>
<li><strong>Синтаксис:</strong> <code>let pattern = expression else { ... };</code>. Обратите внимание на обязательное наличие точки с запятой <code>;</code> после закрывающей фигурной скобки.</li>
<li><strong>Критическое правило блока <code>else</code>:</strong> Блок <code>else</code> обязан быть <strong>расходящимся</strong> (diverging), то есть он <strong>не имеет права</strong> возвращать управление в текущую область видимости обычным путем. Он обязан завершаться операторами:
<ul>
<li><code>return</code> (выход из функции)</li>
<li><code>break</code> (выход из цикла)</li>
<li><code>continue</code> (переход на следующую итерацию цикла)</li>
<li><code>panic!()</code> (завершение потока с ошибкой)</li>
</ul>
</li>
</ul>

<h3 class="relative group">Различие шаблонов
    <div id="различие-шаблонов" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%80%d0%b0%d0%b7%d0%bb%d0%b8%d1%87%d0%b8%d0%b5-%d1%88%d0%b0%d0%b1%d0%bb%d0%be%d0%bd%d0%be%d0%b2" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Обычный <code>let</code> работает только с неопровержимыми (irrefutable) шаблонами, которые всегда совпадают (например, кортежи, ссылки или простые переменные). Для опровержимых (refutable) шаблонов, таких как <code>Some(x)</code>, необходим <code>let-else</code> или <code>if let</code>.</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="c1">// Примеры неопровержимых шаблонов для классического let:
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">a</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="mi">1</span><span class="w"> </span><span class="o">+</span><span class="w"> </span><span class="mi">2</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="p">(</span><span class="n">a</span><span class="p">,</span><span class="w"> </span><span class="n">b</span><span class="p">)</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">some_tuple</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="c1">// Использование let-else для опровержимых шаблонов:
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="nb">Some</span><span class="p">(</span><span class="n">x</span><span class="p">)</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">some_option</span><span class="w"> </span><span class="k">else</span><span class="w"> </span><span class="p">{</span><span class="w"> </span><span class="k">return</span><span class="p">;</span><span class="w"> </span><span class="p">};</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="nb">Some</span><span class="p">(</span><span class="n">x</span><span class="p">)</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">some_option</span><span class="w"> </span><span class="k">else</span><span class="w"> </span><span class="p">{</span><span class="w"> </span><span class="k">break</span><span class="p">;</span><span class="w"> </span><span class="p">};</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="nb">Some</span><span class="p">(</span><span class="n">x</span><span class="p">)</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">some_option</span><span class="w"> </span><span class="k">else</span><span class="w"> </span><span class="p">{</span><span class="w"> </span><span class="k">continue</span><span class="p">;</span><span class="w"> </span><span class="p">};</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="nb">Some</span><span class="p">(</span><span class="n">x</span><span class="p">)</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">some_option</span><span class="w"> </span><span class="k">else</span><span class="w"> </span><span class="p">{</span><span class="w"> </span><span class="fm">panic!</span><span class="p">();</span><span class="w"> </span><span class="p">};</span></span></span></code></pre></div></div>

<h3 class="relative group">Практический пример обработки потока событий в цикле
    <div id="практический-пример-обработки-потока-событий-в-цикле" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%b0%d0%ba%d1%82%d0%b8%d1%87%d0%b5%d1%81%d0%ba%d0%b8%d0%b9-%d0%bf%d1%80%d0%b8%d0%bc%d0%b5%d1%80-%d0%be%d0%b1%d1%80%d0%b0%d0%b1%d0%be%d1%82%d0%ba%d0%b8-%d0%bf%d0%be%d1%82%d0%be%d0%ba%d0%b0-%d1%81%d0%be%d0%b1%d1%8b%d1%82%d0%b8%d0%b9-%d0%b2-%d1%86%d0%b8%d0%ba%d0%bb%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Использование <code>let-else</code> значительно уменьшает уровень вложенности кода по сравнению с <code>match</code> или <code>if let</code>:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">loop</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="c1">// Если get_event() вернул Err, прерываем цикл
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="nb">Ok</span><span class="p">(</span><span class="n">event</span><span class="p">)</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">get_event</span><span class="p">()</span><span class="w"> </span><span class="k">else</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="k">break</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">};</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="c1">// Если event равен None, переходим к следующей итерации
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="nb">Some</span><span class="p">(</span><span class="n">event</span><span class="p">)</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">event</span><span class="w"> </span><span class="k">else</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="k">continue</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">};</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="c1">// Переменная event гарантированно развернута и валидна в этой точке
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">handle_event</span><span class="p">(</span><span class="n">event</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<hr>

<h2 class="relative group">Проверь свои знания!
    <div id="проверь-свои-знания" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%be%d0%b2%d0%b5%d1%80%d1%8c-%d1%81%d0%b2%d0%be%d0%b8-%d0%b7%d0%bd%d0%b0%d0%bd%d0%b8%d1%8f" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Пройдите короткий интерактивный тест, чтобы закрепить понимание let-else и ?.</p>
### 📝  let-else и оператор ? в Rust

**1. Что происходит под капотом при использовании оператора `?` на типе `Result<T, E>` в случае возврата `Err(err)`?**

- [ ] Программа паникует и завершает работу
- [x] Функция немедленно прекращает выполнение и возвращает `Err(err.into())` на уровень выше
- [ ] Ошибка просто игнорируется, и возвращается значение по умолчанию
- [ ] Возвращается значение `Ok(err)`

> 💡 *Оператор `?` разворачивает значение `Ok(x)`, а в случае `Err(err)` немедленно прерывает выполнение текущей функции и делает досрочный возврат `Err(err.into())`.*


**2. Какое критическое требование предъявляется к блоку `else` в конструкции `let-else`?**

- [ ] Он обязан возвращать значение того же типа, что и при успешном сопоставлении
- [ ] Он должен быть пустым
- [x] Он обязан быть расходящимся (diverging) — то есть не возвращать управление в текущую область видимости обычным путем
- [ ] Он должен состоять только из одного макроса panic!

> 💡 *Блок `else` в конструкции `let-else` обязан завершаться расходящимся выражением, то есть передавать поток управления в другое место (через `return`, `break`, `continue`, `panic!()` или возврат типа `!`).*


**3. Какие ключевые слова управления потоком разрешены в конце блока `else` в `let-else` внутри тела цикла?**

- [ ] Только return
- [x] Любые операторы, включая return, break, continue или вызов panic!()
- [ ] Никакие, блок должен возвращать unit-тип `()`
- [ ] Только break

> 💡 *Внутри циклов блок `else` конструкции `let-else` может использовать `break` для выхода из цикла, `continue` для перехода к следующей итерации, `return` для выхода из функции или `panic!()`.*


**4. В чём основное преимущество `let-else` по сравнению с `if let` при успешном извлечении значения?**

- [x] Связанные переменные становятся доступными во внешней области видимости, что уменьшает вложенность кода (избегается «лестница скобок»)
- [ ] Связанные переменные видны только внутри блока `else`
- [ ] let-else работает быстрее, так как не производит никаких проверок шаблона
- [ ] let-else можно использовать с неопровержимыми шаблонами

> 💡 *В отличие от `if let`, где извлеченные переменные доступны только внутри фигурных скобок ветки `if`, конструкция `let-else` принудительно прерывает выполнение в случае ошибки, делая извлеченные переменные доступными во всей внешней области видимости ниже по коду.*


**5. Какая пунктуация обязательна в самом конце выражения `let pattern = expr else { ... }`?**

- [x] Точка с запятой `;`
- [ ] Двоеточие `:`
- [ ] Запятая `,`
- [ ] Никаких знаков препинания не требуется

> 💡 *Синтаксис `let-else` является инструкцией объявления переменной `let`, поэтому после закрывающей фигурной скобки блока `else` обязательно должна стоять точка с запятой `;`.*




