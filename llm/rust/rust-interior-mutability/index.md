# Внутренняя мутабельность в Rust: паттерн использования Cell, RefCell

> Разбираемся с паттерном внутренней мутабельности (Interior Mutability) в Rust: философия работы, контейнеры Cell и RefCell, отличие от унаследованной мутабельности.

- **URL:** https://eagle2000.space/rust/rust-interior-mutability/
- **Date:** 2026-09-09

- **Tags:** rust, dev, interior-mutability, cell, refcell, mutex

---


<h2 class="relative group">Введение
    <div id="введение" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d0%b2%d0%b5%d0%b4%d0%b5%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Обычно borrow checker действует жестко: объявляем переменную без <code>mut</code> — и компилятор ни за что не даст изменять её поля по иммутабельной ссылке <code>&amp;T</code>. Это стандартная <strong>унаследованная мутабельность (Inherited Mutability)</strong>.</p>
<p>Но на практике регулярно возникают ситуации, когда компоненту нужно сохранить закэшированный ответ, записать ошибку в журнал или обновить внутренний идентификатор, имея на руках только неизменяемую ссылку <code>&amp;self</code>.</p>
<p>Для таких случаев в стандартной библиотеке Rust есть паттерн <strong>Внутренней мутабельности (Interior Mutability)</strong>. В этой статье разберем, как устроены <code>Cell</code> и <code>RefCell</code>, где они применяются на практике и чем отличаются от других контейнеров.</p>
<hr>

<h2 class="relative group">1. Контейнер Cell<T>: Мутабельность без ссылок
    <div id="1-контейнер-cell-мутабельность-без-ссылок" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#1-%d0%ba%d0%be%d0%bd%d1%82%d0%b5%d0%b9%d0%bd%d0%b5%d1%80-cell-%d0%bc%d1%83%d1%82%d0%b0%d0%b1%d0%b5%d0%bb%d1%8c%d0%bd%d0%be%d1%81%d1%82%d1%8c-%d0%b1%d0%b5%d0%b7-%d1%81%d1%81%d1%8b%d0%bb%d0%be%d0%ba" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Самый простой инструмент внутренней мутабельности — тип <strong><code>std::cell::Cell&lt;T&gt;</code></strong>.</p>
<p>Особенность <code>Cell&lt;T&gt;</code> в том, что он <strong>никогда не выдает прямых ссылок</strong> <code>&amp;T</code> или <code>&amp;mut T</code> на то значение, которое лежит внутри него.</p>
<p>Вместо ссылок <code>Cell</code> работает путем цельного чтения, записи или перемещения данных:</p>
<ul>
<li><code>get()</code> — возвращает копию внутреннего значения (требует, чтобы <code>T: Copy</code>).</li>
<li><code>set(val)</code> — полностью заменяет внутреннее значение на <code>val</code>.</li>
<li><code>replace(val)</code> — помещает <code>val</code> внутрь контейнера и возвращает ранее хранившееся там значение.</li>
<li><code>take()</code> — извлекает значение, оставляя на его месте <code>Default::default()</code>.</li>
</ul>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">use</span><span class="w"> </span><span class="n">std</span>::<span class="n">cell</span>::<span class="n">Cell</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">struct</span> <span class="nc">Counter</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">count</span>: <span class="nc">Cell</span><span class="o">&lt;</span><span class="kt">u32</span><span class="o">&gt;</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">main</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="n">c</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">Counter</span><span class="w"> </span><span class="p">{</span><span class="w"> </span><span class="n">count</span>: <span class="nc">Cell</span>::<span class="n">new</span><span class="p">(</span><span class="mi">0</span><span class="p">)</span><span class="w"> </span><span class="p">};</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="c1">// Переменная &#39;c&#39; НЕ мутабельна! Но мы спокойно меняем count:
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">c</span><span class="p">.</span><span class="n">count</span><span class="p">.</span><span class="n">set</span><span class="p">(</span><span class="n">c</span><span class="p">.</span><span class="n">count</span><span class="p">.</span><span class="n">get</span><span class="p">()</span><span class="w"> </span><span class="o">+</span><span class="w"> </span><span class="mi">1</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Текущий счетчик: </span><span class="si">{}</span><span class="s">&#34;</span><span class="p">,</span><span class="w"> </span><span class="n">c</span><span class="p">.</span><span class="n">count</span><span class="p">.</span><span class="n">get</span><span class="p">());</span><span class="w"> </span><span class="c1">// Выведет 1
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<div class="admonition relative overflow-hidden rounded-lg border-l-4 my-3 px-4 py-3 shadow-sm" data-type="note">
      <div class="flex items-center gap-2 font-semibold text-inherit">
        <div class="flex shrink-0 h-5 w-5 items-center justify-center text-lg"><span class="relative block icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path fill="currentColor" d="M256 0C114.6 0 0 114.6 0 256s114.6 256 256 256s256-114.6 256-256S397.4 0 256 0zM256 128c17.67 0 32 14.33 32 32c0 17.67-14.33 32-32 32S224 177.7 224 160C224 142.3 238.3 128 256 128zM296 384h-80C202.8 384 192 373.3 192 360s10.75-24 24-24h16v-64H224c-13.25 0-24-10.75-24-24S210.8 224 224 224h32c13.25 0 24 10.75 24 24v88h16c13.25 0 24 10.75 24 24S309.3 384 296 384z"/></svg>
</span></div>
        <div class="grow">
          Примечание
        </div>
      </div><div class="admonition-content mt-3 text-base leading-relaxed text-inherit"><p>Запомните: если тип <code>T</code> не реализует типаж <code>Copy</code> (например, <code>String</code> или <code>Vec</code>), метод <code>.get()</code> не скомпилируется. Однако со сложными типами без <code>Copy</code> в <code>Cell</code> по-прежнему можно работать через <code>.set()</code>, <code>.replace()</code> и <code>.take()</code>.</p></div></div><div class="admonition relative overflow-hidden rounded-lg border-l-4 my-3 px-4 py-3 shadow-sm" data-type="tip">
      <div class="flex items-center gap-2 font-semibold text-inherit">
        <div class="flex shrink-0 h-5 w-5 items-center justify-center text-lg"><span class="relative block icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512"><path fill="currentColor" d="M112.1 454.3c0 6.297 1.816 12.44 5.284 17.69l17.14 25.69c5.25 7.875 17.17 14.28 26.64 14.28h61.67c9.438 0 21.36-6.401 26.61-14.28l17.08-25.68c2.938-4.438 5.348-12.37 5.348-17.7L272 415.1h-160L112.1 454.3zM191.4 .0132C89.44 .3257 16 82.97 16 175.1c0 44.38 16.44 84.84 43.56 115.8c16.53 18.84 42.34 58.23 52.22 91.45c.0313 .25 .0938 .5166 .125 .7823h160.2c.0313-.2656 .0938-.5166 .125-.7823c9.875-33.22 35.69-72.61 52.22-91.45C351.6 260.8 368 220.4 368 175.1C368 78.61 288.9-.2837 191.4 .0132zM192 96.01c-44.13 0-80 35.89-80 79.1C112 184.8 104.8 192 96 192S80 184.8 80 176c0-61.76 50.25-111.1 112-111.1c8.844 0 16 7.159 16 16S200.8 96.01 192 96.01z"/></svg>
</span></div>
        <div class="grow">
          Совет
        </div>
      </div><div class="admonition-content mt-3 text-base leading-relaxed text-inherit"><p>Поскольку <code>Cell&lt;T&gt;</code> не выдает ссылок на внутреннее содержимое, ему <strong>не нужно отслеживать счётчик заимствований</strong>. В нем полностью отсутствуют накладные расходы рантайма (Zero-cost abstraction) и риск паники! Это идеальный выбор для флагов, счетчиков и <code>Copy</code>-типов.</p></div></div><hr>

<h2 class="relative group">2. Контейнер RefCell<T>: Динамический Borrow Checker
    <div id="2-контейнер-refcell-динамический-borrow-checker" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#2-%d0%ba%d0%be%d0%bd%d1%82%d0%b5%d0%b9%d0%bd%d0%b5%d1%80-refcell-%d0%b4%d0%b8%d0%bd%d0%b0%d0%bc%d0%b8%d1%87%d0%b5%d1%81%d0%ba%d0%b8%d0%b9-borrow-checker" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Если внутри контейнера лежит сложная структура (например, <code>Vec&lt;String&gt;</code>, <code>HashMap</code> или пользовательский объект), скопировать ее полностью через <code>Cell::get()</code> не получится.</p>
<p>Для сложных типов используется <strong><code>std::cell::RefCell&lt;T&gt;</code></strong>.</p>
<p><code>RefCell&lt;T&gt;</code> позволяет запрашивать ссылки на внутреннее содержимое:</p>
<ul>
<li><code>.borrow()</code> — возвращает обертку <code>Ref&lt;T&gt;</code>, работающую как обычная иммутабельная ссылка <code>&amp;T</code>.</li>
<li><code>.borrow_mut()</code> — возвращает обертку <code>RefMut&lt;T&gt;</code>, работающую как мутабельная ссылка <code>&amp;mut T</code>.</li>
<li><code>.try_borrow()</code> / <code>.try_borrow_mut()</code> — безопасные варианты, возвращающие <code>Result</code>. Они позволяют проверить заимствование и избежать вызова паники при конфликте ссылок.</li>
</ul>

<h3 class="relative group">Проверки заимствования в рантайме
    <div id="проверки-заимствования-в-рантайме" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%be%d0%b2%d0%b5%d1%80%d0%ba%d0%b8-%d0%b7%d0%b0%d0%b8%d0%bc%d1%81%d1%82%d0%b2%d0%be%d0%b2%d0%b0%d0%bd%d0%b8%d1%8f-%d0%b2-%d1%80%d0%b0%d0%bd%d1%82%d0%b0%d0%b9%d0%bc%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Обычный компилятор Rust проверяет правило: <em>«или много <code>&amp;</code>, или ровно один <code>&amp;mut</code>»</em> во время компиляции.</p>
<p><code>RefCell</code> переносит этот счетчик заимствований на этап <strong>выполнения программы (runtime)</strong>.</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">use</span><span class="w"> </span><span class="n">std</span>::<span class="n">cell</span>::<span class="n">RefCell</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">main</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="n">data</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">RefCell</span>::<span class="n">new</span><span class="p">(</span><span class="fm">vec!</span><span class="p">[</span><span class="mi">1</span><span class="p">,</span><span class="w"> </span><span class="mi">2</span><span class="p">,</span><span class="w"> </span><span class="mi">3</span><span class="p">]);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="k">mut</span><span class="w"> </span><span class="n">b1</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">data</span><span class="p">.</span><span class="n">borrow_mut</span><span class="p">();</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">b1</span><span class="p">.</span><span class="n">push</span><span class="p">(</span><span class="mi">4</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="c1">// ВНИМАНИЕ: b1 еще активен! Попытка сделать второй borrow_mut вызовет ПАНИКУ:
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="c1">// let mut b2 = data.borrow_mut(); // ALREADY BORROWED PANIC!
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p>Благодаря идиоме RAII, смарт-указатели <code>Ref</code> и <code>RefMut</code> в своём деструкторе <code>Drop</code> автоматически уменьшают счётчик заимствований <code>RefCell</code> при выходе из области видимости. Поэтому если вы явно вызовите <code>drop(b1)</code>, следующий вызов <code>data.borrow_mut()</code> отработает без ошибок.</p>
<div class="admonition relative overflow-hidden rounded-lg border-l-4 my-3 px-4 py-3 shadow-sm" data-type="caution">
      <div class="flex items-center gap-2 font-semibold text-inherit">
        <div class="flex shrink-0 h-5 w-5 items-center justify-center text-lg"><span class="relative block icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512">
<path fill="currentColor"  d="M159.3 5.4c7.8-7.3 19.9-7.2 27.7 .1c27.6 25.9 53.5 53.8 77.7 84c11-14.4 23.5-30.1 37-42.9c7.9-7.4 20.1-7.4 28 .1c34.6 33 63.9 76.6 84.5 118c20.3 40.8 33.8 82.5 33.8 111.9C448 404.2 348.2 512 224 512C98.4 512 0 404.1 0 276.5c0-38.4 17.8-85.3 45.4-131.7C73.3 97.7 112.7 48.6 159.3 5.4zM225.7 416c25.3 0 47.7-7 68.8-21c42.1-29.4 53.4-88.2 28.1-134.4c-2.8-5.6-5.6-11.2-9.8-16.8l-50.6 58.8s-81.4-103.6-87.1-110.6C133.1 243.8 112 273.2 112 306.8C112 375.4 162.6 416 225.7 416z"/></svg></span></div>
        <div class="grow">
          Предостережение
        </div>
      </div><div class="admonition-content mt-3 text-base leading-relaxed text-inherit"><p>Помните: <code>RefCell</code> не отменяет правила заимствования Rust, он лишь откладывает их проверку до рантайма. Ошибки проектирования приведут не к ошибке компиляции, а к падению программы!</p></div></div><hr>

<h2 class="relative group">3. Где это пригодится в реальном коде
    <div id="3-где-это-пригодится-в-реальном-коде" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#3-%d0%b3%d0%b4%d0%b5-%d1%8d%d1%82%d0%be-%d0%bf%d1%80%d0%b8%d0%b3%d0%be%d0%b4%d0%b8%d1%82%d1%81%d1%8f-%d0%b2-%d1%80%d0%b5%d0%b0%d0%bb%d1%8c%d0%bd%d0%be%d0%bc-%d0%ba%d0%be%d0%b4%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Теперь, когда мы знаем, как работают <code>Cell</code> и <code>RefCell</code>, разберем, зачем менять данные через неизменяемую ссылку <code>&amp;self</code>, если можно просто прописать <code>&amp;mut self</code>.</p>
<p>В реальном коде сделать <code>&amp;mut self</code> получается не всегда. Вот три частых сценария:</p>

<h3 class="relative group">1. Счётчик кликов у кнопки
    <div id="1-счётчик-кликов-у-кнопки" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#1-%d1%81%d1%87%d1%91%d1%82%d1%87%d0%b8%d0%ba-%d0%ba%d0%bb%d0%b8%d0%ba%d0%be%d0%b2-%d1%83-%d0%ba%d0%bd%d0%be%d0%bf%d0%ba%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Возьмём обычный UI-компонент кнопки. Попробуем обновить счётчик кликов в методе <code>&amp;self</code>:</p>
### 💻 Сравнение: Ошибка компиляции vs Решение с Cell


#### Шаг 1


```rust
// ?compile_fail

struct Button {
    label: String,
    click_count: u32,
}

impl Button {
    fn on_click(&self) {
        println!("Кнопка '{}' нажата!", self.label);
        self.click_count += 1;
    }
}

fn main() {
    let btn = Button {
        label: String::from("Отправить"),
        click_count: 0,
    };
    btn.on_click();
}
```


**Без Cell: Ошибка компиляции**
- Компилятор запрещает изменять поле `click_count` по неизменяемой ссылке `&self`.
- Сделать метод `&mut self` в UI-компонентах часто нельзя, так как кнопка доступна одновременно из разных частей приложения.


#### Шаг 2


```rust
use std::cell::Cell;

struct Button {
    label: String,
    click_count: Cell<u32>,
}

impl Button {
    fn on_click(&self) {
        println!("Кнопка '{}' нажата!", self.label);
        let count = self.click_count.get();
        self.click_count.set(count + 1);
    }
}

fn main() {
    let btn = Button {
        label: String::from("Отправить"),
        click_count: Cell::new(0),
    };

    btn.on_click();
    btn.on_click();
    println!("Всего кликов по кнопке: {}", btn.click_count.get());
}
```


**Решение с Cell: Код компилируется**
- Орачиваем счетчик в `Cell<u32>`.
- Метод `on_click(&self)` успешно обновляет значение через `.set()`, сохраняя ссылку `&self`.


<p>В UI-библиотеках методы обработки событий работают через ссылки <code>&amp;self</code>, так как кнопка вызывается из разных частей приложения. <code>Cell</code> дает возможность менять внутренний счётчик кликов прямо по <code>&amp;self</code> без пробрасывания <code>&amp;mut self</code> по всей цепочке вызовов.</p>

<h3 class="relative group">2. Накопление истории логов в сервисе
    <div id="2-накопление-истории-логов-в-сервисе" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#2-%d0%bd%d0%b0%d0%ba%d0%be%d0%bf%d0%bb%d0%b5%d0%bd%d0%b8%d0%b5-%d0%b8%d1%81%d1%82%d0%be%d1%80%d0%b8%d0%b8-%d0%bb%d0%be%d0%b3%d0%be%d0%b2-%d0%b2-%d1%81%d0%b5%d1%80%d0%b2%d0%b8%d1%81%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Ещё один кейс — общая структура <code>App</code>, к которой компоненты обращаются по ссылке <code>&amp;App</code> для чтения настроек. С <code>RefCell</code> любая функция может на лету записать предупреждение в общий массив логов:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">struct</span> <span class="nc">App</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">name</span>: <span class="nb">String</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="c1">// Список логов прямо внутри приложения
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">logs</span>: <span class="nc">RefCell</span><span class="o">&lt;</span><span class="nb">Vec</span><span class="o">&lt;</span><span class="nb">String</span><span class="o">&gt;&gt;</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">impl</span><span class="w"> </span><span class="n">App</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">fn</span> <span class="nf">log_error</span><span class="p">(</span><span class="o">&amp;</span><span class="bp">self</span><span class="p">,</span><span class="w"> </span><span class="n">msg</span>: <span class="kp">&amp;</span><span class="kt">str</span><span class="p">)</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="c1">// Добавляем запись в список, имея только &amp;self
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="bp">self</span><span class="p">.</span><span class="n">logs</span><span class="p">.</span><span class="n">borrow_mut</span><span class="p">().</span><span class="n">push</span><span class="p">(</span><span class="n">msg</span><span class="p">.</span><span class="n">to_string</span><span class="p">());</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>

<h3 class="relative group">3. Флаг состояния «уже выполнено»
    <div id="3-флаг-состояния-уже-выполнено" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#3-%d1%84%d0%bb%d0%b0%d0%b3-%d1%81%d0%be%d1%81%d1%82%d0%be%d1%8f%d0%bd%d0%b8%d1%8f-%d1%83%d0%b6%d0%b5-%d0%b2%d1%8b%d0%bf%d0%be%d0%bb%d0%bd%d0%b5%d0%bd%d0%be" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>То же самое с флагами инициализации в фоновом плеере или загрузчике:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">struct</span> <span class="nc">Player</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">is_playing</span>: <span class="nc">Cell</span><span class="o">&lt;</span><span class="kt">bool</span><span class="o">&gt;</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">impl</span><span class="w"> </span><span class="n">Player</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">fn</span> <span class="nf">play</span><span class="p">(</span><span class="o">&amp;</span><span class="bp">self</span><span class="p">)</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="k">if</span><span class="w"> </span><span class="o">!</span><span class="bp">self</span><span class="p">.</span><span class="n">is_playing</span><span class="p">.</span><span class="n">get</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">            </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Включаем воспроизведение...&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">            </span><span class="bp">self</span><span class="p">.</span><span class="n">is_playing</span><span class="p">.</span><span class="n">set</span><span class="p">(</span><span class="kc">true</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<hr>

<h2 class="relative group">4. Ограничение по многопоточности
    <div id="4-ограничение-по-многопоточности" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#4-%d0%be%d0%b3%d1%80%d0%b0%d0%bd%d0%b8%d1%87%d0%b5%d0%bd%d0%b8%d0%b5-%d0%bf%d0%be-%d0%bc%d0%bd%d0%be%d0%b3%d0%be%d0%bf%d0%be%d1%82%d0%be%d1%87%d0%bd%d0%be%d1%81%d1%82%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Важный нюанс: контейнеры <code>Cell</code> и <code>RefCell</code> предназначены <strong>строго для однопоточного кода</strong>.</p>
<p>Оба типа не реализуют маркерный типаж <code>Sync</code>, поэтому передать ссылку на них в другой поток не выйдет — компилятор заблокирует такую попытку еще при сборке.</p>
<p>Подробный разбор многопоточной внутренней мутабельности (<code>Mutex</code>, <code>RwLock</code>), а также типажей <code>Send</code> и <code>Sync</code> выйдет в отдельной статье.</p>
<hr>
<table>
	<thead>
			<tr>
					<th style="text-align: left">Тип</th>
					<th style="text-align: center">Выдает ссылки?</th>
					<th style="text-align: center">Проверка заимствования</th>
					<th style="text-align: center">Требования к типам / Трейтам</th>
					<th style="text-align: left">Накладные расходы</th>
			</tr>
	</thead>
	<tbody>
			<tr>
					<td style="text-align: left"><strong><code>Cell&lt;T&gt;</code></strong></td>
					<td style="text-align: center">Нет (только get/set)</td>
					<td style="text-align: center">Не требуется</td>
					<td style="text-align: center">Требует <code>T: Copy</code> для <code>.get()</code></td>
					<td style="text-align: left">Нулевые (Zero-cost)</td>
			</tr>
			<tr>
					<td style="text-align: left"><strong><code>RefCell&lt;T&gt;</code></strong></td>
					<td style="text-align: center">Да (<code>Ref</code> / <code>RefMut</code>)</td>
					<td style="text-align: center">Runtime (Паника при нарушениях)</td>
					<td style="text-align: center">Работает с любым <code>T</code></td>
					<td style="text-align: left">Небольшой счетчик в рантайме</td>
			</tr>
	</tbody>
</table>
<p>Давайте изучим работу этих типов на практических слайдах:</p>
### 💻 Сравнение работы Cell<T> и RefCell<T>


#### Шаг 1


```rust
use std::cell::Cell;

struct Logger {
    log_count: Cell<usize>,
}

impl Logger {
    fn new() -> Self {
        Logger {
            log_count: Cell::new(0),
        }
    }

    // Обратите внимание: метод принимает &self (неизменяемую ссылку!), но меняет log_count!
    fn log(&self, message: &str) {
        println!("[LOG]: {message}");
        let current = self.log_count.get();
        self.log_count.set(current + 1);
    }
}

fn main() {
    let logger = Logger::new();

    logger.log("Первое сообщение системы");
    logger.log("Второе сообщение системы");

    println!("Всего записей в логе: {}", logger.log_count.get());

    // Метод replace заменяет значение и возвращает старое
    let old_val = logger.log_count.replace(100);
    println!("Старый счетчик: {old_val}, новый счетчик: {}", logger.log_count.get());
}
```


**Внутренняя мутабельность с Cell<T>**
- `Cell<T>` позволяет изменять значение внутри объекта, имея только неизменяемую ссылку `&self`.
- Запись `set()` и чтение `get()` работают путем копирования или перемещения значения без создания ссылок `&T`.
- Идеально подходит для простых типов (`Copy`) или флагов и счетчиков.


#### Шаг 2


```rust
use std::cell::RefCell;

struct UserCache {
    cache: RefCell<Vec<String>>,
}

impl UserCache {
    fn new() -> Self {
        UserCache {
            cache: RefCell::new(Vec::new()),
        }
    }

    fn add_user(&self, name: String) {
        // Запрашиваем изменяемое заимствование во время выполнения
        self.cache.borrow_mut().push(name);
    }

    fn print_users(&self) {
        // Запрашиваем неизменяемое заимствование
        let users = self.cache.borrow();
        println!("Пользователи в кэше: {:?}", *users);
    }
}

fn main() {
    let cache = UserCache::new();

    cache.add_user("Алиса".to_string());
    cache.add_user("Борис".to_string());

    cache.print_users();

    // Проверка размера кэша
    println!("Количество пользователей: {}", cache.cache.borrow().len());
}
```


**Динамическое заимствование с RefCell<T>**
- `RefCell<T>` позволяет изменять сложные структуры в куче через `borrow_mut()` по ссылкам `&self`.
- В отличие от `Cell`, `RefCell` выдает временные ссылки `Ref` и `RefMut`.
- Проверки заимствований переносятся на runtime. Попытка открыть два `borrow_mut()` одновременно вызовет панику.


<hr>

<h2 class="relative group">Проверь свои знания!
    <div id="проверь-свои-знания" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%be%d0%b2%d0%b5%d1%80%d1%8c-%d1%81%d0%b2%d0%be%d0%b8-%d0%b7%d0%bd%d0%b0%d0%bd%d0%b8%d1%8f" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Пройдите короткий интерактивный тест по теме внутренней мутабельности:</p>
### 📝 Внутренняя мутабельность в Rust

**1. В чём заключается суть паттерна внутренней мутабельности (Interior Mutability)?**

- [ ] Автоматическое копирование всех переменных в кучу
- [x] Изменение внутренних данных структуры через неизменяемую ссылку &self
- [ ] Полный отказ от проверки типов во время компиляции
- [ ] Увеличение размера стека для сложных объектов

> 💡 *Паттерн внутренней мутабельности позволяет изменять состояние объекта по иммутабельной ссылке (`&self`), когда это требуется по архитектуре.*


**2. Какой из методов `Cell<T>` забирает внутреннее значение, оставляя на его месте `T::default()`?**

- [x] Cell::take()
- [ ] Cell::get()
- [ ] Cell::replace()
- [ ] Cell::drop()

> 💡 *Метод `take()` извлекает значение из `Cell`, замещая его дефолтным для данного типа (`Default::default()`).*


**3. В чём ключевое отличие `RefCell<T>` от `Cell<T>`?**

- [ ] Cell компилируется в unsafe код без проверок
- [x] RefCell выдаёт временные умные ссылки Ref/RefMut на внутренний объект любой сложности, а Cell копирует или перемещает значение целиком
- [ ] RefCell не работает со структурами данных
- [ ] Cell можно использовать только с вектором строк

> 💡 *`Cell<T>` не выдает ссылок и работает с цельной заменой/копированием данных, в то время как `RefCell<T>` выдает временные умные ссылки `Ref<T>` и `RefMut<T>`.*


**4. Что произойдёт во время выполнения, если вызвать `.borrow_mut()` на `RefCell`, когда первое изменяемое заимствование ещё активно?**

- [ ] Компилятор выдаст ошибку при сборке
- [ ] Поток заблокируется до освобождения ссылки
- [x] Программа запаникует во время работы (Already borrowed panic)
- [ ] Данные автоматически продублируются в памяти

> 💡 *`RefCell` переносит проверки правил заимствования в runtime. Если при открытом `borrow_mut()` запросить ещё одно изменяемое заимствование, программа запаникует.*


**5. Какое требование предъявляется к типу `T` для вызова метода `.get()` у `Cell<T>`?**

- [x] Тип T должен реализовывать типаж Copy
- [ ] Тип T должен быть обёрнут в Option
- [ ] Тип T должен реализовывать типаж Display
- [ ] Тип T должен храниться только в куче

> 💡 *Метод `Cell::get()` возвращает копию содержимого, поэтому он доступен только для типов, реализующих типаж `Copy`.*


**6. В какой ситуации `Cell<u32>` предпочтительнее `RefCell<u32>`?**

- [ ] Когда нужно заимствовать элементы по длинным ссылкам
- [x] Когда требуется счетчик или флаг с нулевым оверхедом и гарантированным отсутствием паник в рантайме
- [ ] Когда данные нужно передавать между потоками
- [ ] Никогда, RefCell всегда эффективнее

> 💡 *Для скалярных `Copy`-типов (например, `u32` или `bool`) `Cell` предпочтительнее: у него нет рантайм-счётчиков заимствования, и он не может паниковать.*


**7. Что возвращает метод `.borrow_mut()` у контейнера `RefCell<T>`?**

- [ ] Обычную сырую ссылку &mut T
- [x] Умную обёртку RefMut<T>, которая уменьшает счётчик заимствований при своём уничтожении (Drop)
- [ ] Указатель *mut T
- [ ] Новый экземпляр RefCell<T>

> 💡 *Метод `borrow_mut()` возвращает RAII-обертку `RefMut<T>`. При выходе из области видимости её метод `drop()` автоматичеcки освобождает заимствование.*


**8. Как безопасно попытаться получить заимствование в `RefCell<T>` без риска вызвать панику программы?**

- [x] Использовать методы try_borrow() и try_borrow_mut(), возвращающие Result
- [ ] Обернуть код в блок unsafe
- [ ] Вызвать метод std::mem::forget
- [ ] Использовать макрос panic_catch!

> 💡 *Методы `try_borrow()` и `try_borrow_mut()` возвращают `Result`, позволяя обработать возможный конфликт заимствований без краша программы.*


**9. Какой метод `Cell<T>` помещает новое значение внутрь контейнера и возвращает ранее хранившееся значение?**

- [ ] Cell::set()
- [ ] Cell::get()
- [x] Cell::replace()
- [ ] Cell::take()

> 💡 *Метод `replace(new_val)` сохраняет `new_val` в контейнер и возвращает старое значение, находившееся там.*


**10. Какая ошибка произойдет, если запросить `.borrow_mut()`, пока держится активная неизменяемая ссылка через `.borrow()` у `RefCell`?**

- [ ] Компилятор заблокирует сборку
- [x] Паника во время работы: нарушено правило о невозможности совмещения & и &mut
- [ ] Программа молча проигнорирует запрос
- [ ] Старая ссылка автоматически аннулируется

> 💡 *Даже в рантайме правило Rust о заимствованиях строжайше соблюдается: нельзя взять `borrow_mut()`, пока активен хотя бы один `borrow()`.*


**11. Какова величина накладных расходов у `Cell<T>` по сравнению с обычным полем структуры?**

- [ ] Значительный оверхед из-за мьютексов
- [x] Нулевая (Zero-cost) — в скомпилированном коде Cell представляет собой простое значение в памяти
- [ ] Постоянный выделение памяти в куче
- [ ] Замедление на каждый вызов на 50%

> 💡 *`Cell<T>` является прозрачной обёрткой без дополнительных полей рантайма. Его накладные расходы равны нулю (Zero-cost abstraction).*


**12. Что происходит со счётчиком заимствований `RefCell`, когда объект `Ref` или `RefMut` выходит из области видимости?**

- [ ] Счётчик обнуляется мгновенно в 0
- [x] Автоматически вызывается drop(), который уменьшает соответствующий счётчик заимствований
- [ ] Счётчик никогда не уменьшается до перезапуска программы
- [ ] Счётчик увеличивается на 1

> 💡 *Благодаря идиоме RAII обёртки `Ref` и `RefMut` в своём деструкторе `Drop` уменьшают счётчик заимствований `RefCell`.*


**13. Можно ли использовать `Cell<String>` со сложным типом `String`, у которого нет типажа `Copy`?**

- [x] Да, но метод .get() будет недоступен; работать можно через set(), replace() и take()
- [ ] Нет, Cell работает строго с типами f32 и u32
- [ ] Да, но только в unsafe блоках
- [ ] Нет, компилятор запрещает создавать Cell<String>

> 💡 *`Cell<String>` можно создать и менять через `set()` / `take()`, но вызвать `.get()` нельзя, так как `String` не реализует `Copy`.*


**14. Почему последовательный вызов b1 = data.borrow_mut(); drop(b1); b2 = data.borrow_mut(); НЕ вызовет панику?**

- [x] Первое заимствование b1 завершилось и было освобождено до вызова b2
- [ ] RefCell разрешает сколько угодно мутабельных ссылок
- [ ] drop() отключает проверку заимствований
- [ ] Компилятор объединяет два заимствования в одно

> 💡 *Если предыдущая ссылка `RefMut` завершила свой жизненный цикл (вышел `drop`), счётчик заимствований равен 0 и можно успешно взять новое заимствование.*


**15. Для какого сценария `RefCell<Vec<String>>` подходит идеально?**

- [ ] Для многопоточного сервера с 100 потоками
- [ ] Для вычисления числовых констант в comptime
- [x] Для структуры сервиса, передаваемой по &self, которой нужно накапливать логи в вектор
- [ ] Для создания глобальных неизменяемых констант

> 💡 *`RefCell<Vec<String>>` позволяет накапливать строки в динамический вектор по методам `&self` в однопоточных сервисах.*


**16. Что будет, если вызвать `Cell::set(new_val)` у `Cell<u32>`?**

- [x] Старое значение будет перезаписано новым `new_val`
- [ ] Метод вернёт ошибку Result
- [ ] Значение добавится в конец очереди
- [ ] Программа заблокирует вызовы к структуре

> 💡 *Метод `set(val)` полностью заменяет содержимое `Cell` на новое значение.*


**17. Какое базовое правило заимствований Rust переносит `RefCell` на runtime?**

- [x] Либо одна мутабельная ссылка &mut T, либо произвольное количество неизменяемых ссылок &T
- [ ] Переменные должны быть заглавными буквами
- [ ] Все структуры должны быть Copy
- [ ] Переменные из стека переносятся в кучу

> 💡 *`RefCell` строго соблюдает фундаментальное правило Rust: в один момент времени может существовать либо ровно одна `&mut T`, либо сколько угодно `&T`.*




