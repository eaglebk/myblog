# Система типов в Rust: почему компилятор — ваш лучший друг

> Простыми словами о системе типов в Rust. Разбираем статическую и строгую типизацию, а также смотрим, как проектировать логику через типы (Type-Driven Development), чтобы избегать глупых багов.

- **URL:** https://eagle2000.space/rust/rust-type-system/
- **Date:** 2026-08-01

- **Tags:** rust, dev, theory

---


<h2 class="relative group">Введение
    <div id="введение" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d0%b2%d0%b5%d0%b4%d0%b5%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>В мире программирования типы данных — это не просто скучные ярлыки, которые говорят компьютеру, сколько памяти выделить под переменную. Система типов определяет то, как мы пишем код, как думаем о логике нашей программы и насколько спокойно мы спим по ночам, не боясь внезапных падений в продакшене.</p>
<p>В этой статье мы простыми словами разберем, как устроена система типов, чем Rust отличается от других языков и как крутая фича под названием <strong>Type-Driven Development</strong> (проектирование через типы) помогает нам переложить заботу о логических ошибках на плечи компилятора.</p>
<hr>

<h2 class="relative group">1. Как языки работают с типами?
    <div id="1-как-языки-работают-с-типами" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#1-%d0%ba%d0%b0%d0%ba-%d1%8f%d0%b7%d1%8b%d0%ba%d0%b8-%d1%80%d0%b0%d0%b1%d0%be%d1%82%d0%b0%d1%8e%d1%82-%d1%81-%d1%82%d0%b8%d0%bf%d0%b0%d0%bc%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Все системы типов в языках программирования можно разложить по двум осям: <strong>когда происходит проверка</strong> и <strong>насколько строгие правила</strong>.</p>
<div style="background-color:white; padding: 20px">
<pre class="not-prose mermaid">
graph TD
    A[Система типов] --> B[Когда проверяем?]
    A --> C[Насколько строгие правила?]
    B --> B1[При компиляции <br>Статическая]
    B --> B2[Во время работы <br>Динамическая]
    C --> C1[Никакой магии <br>Строгая / Сильная]
    C --> C2[Авто-преобразования <br>Слабая]
    
    style B1 fill:#f9f,stroke:#333,stroke-width:2px,color:#000
    style C1 fill:#f9f,stroke:#333,stroke-width:2px,color:#000
</pre>

</div>

<h3 class="relative group">Проверка типов: до запуска или во время работы?
    <div id="проверка-типов-до-запуска-или-во-время-работы" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%be%d0%b2%d0%b5%d1%80%d0%ba%d0%b0-%d1%82%d0%b8%d0%bf%d0%be%d0%b2-%d0%b4%d0%be-%d0%b7%d0%b0%d0%bf%d1%83%d1%81%d0%ba%d0%b0-%d0%b8%d0%bb%d0%b8-%d0%b2%d0%be-%d0%b2%d1%80%d0%b5%d0%bc%d1%8f-%d1%80%d0%b0%d0%b1%d0%be%d1%82%d1%8b" aria-label="Якорь">#</a>
    </span>
    
</h3>
<ul>
<li>
<p><strong>Статическая типизация (проверка ДО запуска):</strong></p>
<ul>
<li><strong>Как это работает:</strong> Компилятор проверяет типы всех переменных и аргументов функций еще на этапе сборки. Если вы попытаетесь передать строку туда, где ожидается число, программа просто не соберется.</li>
<li><strong>Где используется:</strong> Rust, Go, C++, C#, Java, Haskell.</li>
<li><strong>Плюсы:</strong> Ошибки ловятся моментально на вашем компьютере, а не у пользователя. Плюс программа работает быстрее, ведь в процессе работы не нужно тратить время на проверку типов.</li>
</ul>
</li>
<li>
<p><strong>Динамическая типизация (проверка ВО ВРЕМЯ работы):</strong></p>
<ul>
<li><strong>Как это работает:</strong> Переменным все равно, что хранить. Типы проверяются на лету, прямо во время выполнения программы.</li>
<li><strong>Где используется:</strong> Python, JavaScript, Ruby, PHP.</li>
<li><strong>Плюсы:</strong> Код пишется очень быстро и гибко, не нужно описывать типы.</li>
<li><strong>Минусы:</strong> Легко пропустить опечатку или передать не тот объект, и программа упадет посреди ночи у реального клиента.</li>
</ul>
</li>
<li>
<p><strong>Гибриды:</strong>
Некоторые динамические языки добавляют подсказки типов (<em>type hints</em>), как в Python. Они помогают редактору кода подсвечивать ошибки, но сам интерпретатор Python во время работы их игнорирует. В языке Julia типы проверяются динамически, но оптимизируются для высокой скорости.</p>
</li>
</ul>

<h3 class="relative group">Правила игры: строгие или мягкие?
    <div id="правила-игры-строгие-или-мягкие" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%b0%d0%b2%d0%b8%d0%bb%d0%b0-%d0%b8%d0%b3%d1%80%d1%8b-%d1%81%d1%82%d1%80%d0%be%d0%b3%d0%b8%d0%b5-%d0%b8%d0%bb%d0%b8-%d0%bc%d1%8f%d0%b3%d0%ba%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h3>
<ul>
<li>
<p><strong>Строгая (сильная) типизация:</strong></p>
<ul>
<li><strong>Суть:</strong> Язык не делает никаких поблажек и неявных преобразований. Если вам нужно сложить целое число <code>42</code> и вещественное <code>3.14</code>, вам придется явно сказать компилятору привести их к общему знаменателю. Никакого «само рассосется».</li>
<li><strong>Где используется:</strong> Rust, Python.</li>
</ul>
</li>
<li>
<p><strong>Слабая типизация:</strong></p>
<ul>
<li><strong>Суть:</strong> Язык пытается «помочь» и неявно преобразует типы на ходу. Например, в JavaScript сложение строки <code>&quot;5&quot;</code> и числа <code>10</code> даст строку <code>&quot;510&quot;</code>, а в языке C компилятор может молча привести указатель к числу.</li>
<li><strong>Где используется:</strong> JavaScript, C.</li>
</ul>
</li>
</ul>
<div class="admonition relative overflow-hidden rounded-lg border-l-4 my-3 px-4 py-3 shadow-sm" data-type="note">
      <div class="flex items-center gap-2 font-semibold text-inherit">
        <div class="flex shrink-0 h-5 w-5 items-center justify-center text-lg"><span class="relative block icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path fill="currentColor" d="M256 0C114.6 0 0 114.6 0 256s114.6 256 256 256s256-114.6 256-256S397.4 0 256 0zM256 128c17.67 0 32 14.33 32 32c0 17.67-14.33 32-32 32S224 177.7 224 160C224 142.3 238.3 128 256 128zM296 384h-80C202.8 384 192 373.3 192 360s10.75-24 24-24h16v-64H224c-13.25 0-24-10.75-24-24S210.8 224 224 224h32c13.25 0 24 10.75 24 24v88h16c13.25 0 24 10.75 24 24S309.3 384 296 384z"/></svg>
</span></div>
        <div class="grow">
          Примечание
        </div>
      </div><div class="admonition-content mt-3 text-base leading-relaxed text-inherit"><p>Разделение на строгую и слабую типизацию — это не четкое «да» или «нет», а целый спектр. Но Rust находится на самом строгом его краю: здесь компилятор бьет по рукам за любые попытки смешать несовместимые типы без вашего явного указания.</p></div></div><hr>

<h2 class="relative group">2. Проектирование через типы (Type-Driven Development)
    <div id="2-проектирование-через-типы-type-driven-development" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#2-%d0%bf%d1%80%d0%be%d0%b5%d0%ba%d1%82%d0%b8%d1%80%d0%be%d0%b2%d0%b0%d0%bd%d0%b8%d0%b5-%d1%87%d0%b5%d1%80%d0%b5%d0%b7-%d1%82%d0%b8%d0%bf%d1%8b-type-driven-development" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Благодаря строгой статической типизации в Rust, компилятор работает как автоматический контролер качества. Мы можем спроектировать наши типы данных так, чтобы некорректное состояние программы было <strong>невозможно написать физически</strong>. Это называется принципом <em>Make illegal states unrepresentable</em>.</p>

<h3 class="relative group">Пример из жизни: Этапы заказа
    <div id="пример-из-жизни-этапы-заказа" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%b8%d0%bc%d0%b5%d1%80-%d0%b8%d0%b7-%d0%b6%d0%b8%d0%b7%d0%bd%d0%b8-%d1%8d%d1%82%d0%b0%d0%bf%d1%8b-%d0%b7%d0%b0%d0%ba%d0%b0%d0%b7%d0%b0" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Представьте, что мы пишем логику интернет-магазина. Мы не хотим, чтобы программа случайно отправила клиенту заказ, который еще не оплачен.</p>
<p>В других языках мы бы писали проверки:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">if</span><span class="w"> </span><span class="n">order</span><span class="p">.</span><span class="n">status</span><span class="w"> </span><span class="o">==</span><span class="w"> </span><span class="n">Status</span>::<span class="n">Paid</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">ship</span><span class="p">(</span><span class="n">order</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w"> </span><span class="k">else</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="c1">// Что делать, если статус не тот? Ошибка в рантайме?
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p>В Rust мы можем пойти другим путем — создать отдельные типы для каждого состояния заказа:</p>
<ol>
<li><code>NewOrder</code> (Новый заказ)</li>
<li><code>PaidOrder</code> (Оплаченный заказ)</li>
<li><code>ShippedOrder</code> (Отправленный заказ)</li>
</ol>
<p>Мы пишем функцию <code>ship(order: PaidOrder) -&gt; ShippedOrder</code>. Эта функция физически не примет <code>NewOrder</code>! Попытка передать туда новый неоплаченный заказ вызовет ошибку компиляции. Нам не нужно писать юнит-тесты на эту логику — компилятор сам гарантирует, что шаги процесса нельзя перепутать.</p>
<p>Давайте посмотрим, как это выглядит в коде, на интерактивных слайдах ниже:</p>
### 💻 Система типов и Type-Driven Development


#### Шаг 1


```rust
// ?compile_fail
fn main() {
    let integer: i32 = 42;
    let float: f64 = 3.14;
    
    // Ошибка! Типы не соответствуют друг другу.
    let result = integer + float; 
}
```


**Строгая типизация: Неявное приведение**
- В Rust отсутствует автоматическое (неявное) приведение типов.
- Попытка сложить целое число `i32` и вещественное `f64` приведет к ошибке компилятора E0308.
- Это предотвращает случайные потери точности в вычислениях.


#### Шаг 2


```rust
fn main() {
    let integer: i32 = 42;
    let float: f64 = 3.14;
    
    let result = (integer as f64) + float;
    println!("Результат сложения: {}", result);
}
```


**Строгая типизация: Явное приведение**
- Для безопасного приведения типов используется оператор `as`.
- Мы явно приводим `i32` к `f64` перед выполнением сложения.
- Теперь компилятор уверен в наших намерениях, и код успешно выполняется.


#### Шаг 3


```rust
struct NewOrder {
    id: u64,
}

struct ShippedOrder {
    id: u64,
    tracking_number: String,
}

impl NewOrder {
    fn new(id: u64) -> Self {
        NewOrder { id }
    }
    
    fn ship(self, tracking: &str) -> ShippedOrder {
        ShippedOrder {
            id: self.id,
            tracking_number: tracking.to_string(),
        }
    }
}

impl ShippedOrder {
    fn deliver(self) {
        println!("Заказ {} успешно доставлен с трек-номером {}!", self.id, self.tracking_number);
    }
}
fn main() {
    let order = NewOrder::new(1024);
    
    // 1. Мы не можем вызвать order.deliver() напрямую — компилятор выдаст ошибку!
    // 2. Сначала мы должны перевести заказ в состояние отправленного (ShippedOrder):
    let shipped = order.ship("RU-777-XYZ");
    
    // 3. Теперь мы можем безопасно вызвать доставку:
    shipped.deliver();
}
```


**Type-Driven Development: Безопасное состояние**
- Мы кодируем правила логики программы в типах `NewOrder` и `ShippedOrder`.
- Метод `ship` определен только для `NewOrder`, возвращая `ShippedOrder`.
- Метод `deliver` определен только для `ShippedOrder`.
- Ошибиться невозможно: компилятор не позволит вызвать доставку для нового неотправленного заказа.


<hr>

<h2 class="relative group">Проверь свои знания!
    <div id="проверь-свои-знания" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%be%d0%b2%d0%b5%d1%80%d1%8c-%d1%81%d0%b2%d0%be%d0%b8-%d0%b7%d0%bd%d0%b0%d0%bd%d0%b8%d1%8f" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Пройдите короткий интерактивный тест, чтобы закрепить понимание классификации систем типов и Type-Driven Development.</p>
### 📝 Система типов в Rust

**1. К какой категории по времени проверки типов относится Rust?**

- [ ] Динамическая система типов
- [x] Статическая система типов
- [ ] Опциональная (гибридная) система типов

> 💡 *В Rust все проверки типов происходят на этапе компиляции исходного кода, что относит его к статической системе типов.*


**2. Разрешены ли в Rust неявные (автоматические) преобразования типов?**

- [ ] Да, для любых совместимых числовых типов
- [ ] Да, но только при сложении целых чисел и чисел с плавающей точкой
- [x] Нет, неявные преобразования запрещены; типы необходимо приводить явно

> 💡 *Rust обладает строгой (сильной) типизацией, запрещающей неявные преобразования. Для изменения типа необходимо использовать явное приведение (например, через оператор `as`).*


**3. В чем заключается суть концепции Type-Driven Development (разработки через типы)?**

- [ ] В обязательном написании тестов до написания кода
- [x] В кодировании бизнес-логики и инвариантов непосредственно в структурах и типах данных для проверки компилятором
- [ ] В автоматической генерации кода на основе текстового описания типов

> 💡 *Разработка через типы позволяет описать правила бизнес-логики (например, недопустимые переходы состояний) на уровне типов, чтобы компилятор отсекал некорректный код на этапе сборки.*


**4. Какое утверждение о границе между сильной и слабой типизацией верно?**

- [ ] Это четкое бинарное деление: язык либо на 100% строгий, либо на 100% слабый
- [x] Это непрерывный спектр, зависящий от количества допускаемых языком неявных преобразований
- [ ] Сила типизации определяется исключительно скоростью работы программы

> 💡 *Граница между сильной и слабой типизацией — это спектр. Различные языки допускают разную степень неявных преобразований в различных контекстах.*




