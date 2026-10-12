# Трейты в Rust: от первых шагов до понятного мономорфного кода

> Понятный гайд по трейтам в Rust: от базоваго синтаксиса trait и impl до методов по умолчанию, дженериков, синтаксиса impl Trait и нулевой стоимости мономорфизации.

- **URL:** https://eagle2000.space/rust/rust-traits/
- **Date:** 2026-08-07

- **Tags:** rust, dev, traits, generics, middle

---


<h2 class="relative group">Введение: Зачем нужны трейты?
    <div id="введение-зачем-нужны-трейты" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d0%b2%d0%b5%d0%b4%d0%b5%d0%bd%d0%b8%d0%b5-%d0%b7%d0%b0%d1%87%d0%b5%d0%bc-%d0%bd%d1%83%d0%b6%d0%bd%d1%8b-%d1%82%d1%80%d0%b5%d0%b9%d1%82%d1%8b" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Допустим, в приложении для фитнеса есть разные упражнения: <code>Pushups</code> и <code>Plank</code>. Оба умеют считать потраченные калории, но делают это по-разному:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">struct</span> <span class="nc">Pushups</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">reps</span>: <span class="kt">u32</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">struct</span> <span class="nc">Plank</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">duration_sec</span>: <span class="kt">u32</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">impl</span><span class="w"> </span><span class="n">Pushups</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">fn</span> <span class="nf">calories_burned</span><span class="p">(</span><span class="o">&amp;</span><span class="bp">self</span><span class="p">)</span><span class="w"> </span>-&gt; <span class="kt">u32</span> <span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="bp">self</span><span class="p">.</span><span class="n">reps</span><span class="w"> </span><span class="o">*</span><span class="w"> </span><span class="mi">1</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">impl</span><span class="w"> </span><span class="n">Plank</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">fn</span> <span class="nf">calories_burned</span><span class="p">(</span><span class="o">&amp;</span><span class="bp">self</span><span class="p">)</span><span class="w"> </span>-&gt; <span class="kt">u32</span> <span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="bp">self</span><span class="p">.</span><span class="n">duration_sec</span><span class="w"> </span><span class="o">*</span><span class="w"> </span><span class="mi">2</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p>Теперь возникает задача — написать одну общую функцию подсчёта суммарных калорий для любого упражнения. Без абстракций нам пришлось бы дублировать код функции под каждый тип.</p>
<hr>

<h3 class="relative group">Как Rust решает эту проблему
    <div id="как-rust-решает-эту-проблему" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%ba%d0%b0%d0%ba-rust-%d1%80%d0%b5%d1%88%d0%b0%d0%b5%d1%82-%d1%8d%d1%82%d1%83-%d0%bf%d1%80%d0%be%d0%b1%d0%bb%d0%b5%d0%bc%d1%83" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Во многих языках подобную задачу решают с помощью обобщенных типов или шаблонов. Однако без явных ограничений ошибки часто проявляются уже внутри шаблонного кода и сопровождаются длинными сообщениями компилятора.</p>
<p><strong>Rust решает проблему с помощью Трейтов (Traits)</strong> — явных контрактов поведения.</p>
<p>Сначала объявляем контракт <code>Workout</code>:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">pub</span><span class="w"> </span><span class="k">trait</span><span class="w"> </span><span class="n">Workout</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">fn</span> <span class="nf">calories_burned</span><span class="p">(</span><span class="o">&amp;</span><span class="bp">self</span><span class="p">)</span><span class="w"> </span>-&gt; <span class="kt">u32</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p>Затем заявляем, что наши структуры реализуют этот трейт:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">impl</span><span class="w"> </span><span class="n">Workout</span><span class="w"> </span><span class="k">for</span><span class="w"> </span><span class="n">Pushups</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">fn</span> <span class="nf">calories_burned</span><span class="p">(</span><span class="o">&amp;</span><span class="bp">self</span><span class="p">)</span><span class="w"> </span>-&gt; <span class="kt">u32</span> <span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="bp">self</span><span class="p">.</span><span class="n">reps</span><span class="w"> </span><span class="o">*</span><span class="w"> </span><span class="mi">1</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">impl</span><span class="w"> </span><span class="n">Workout</span><span class="w"> </span><span class="k">for</span><span class="w"> </span><span class="n">Plank</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">fn</span> <span class="nf">calories_burned</span><span class="p">(</span><span class="o">&amp;</span><span class="bp">self</span><span class="p">)</span><span class="w"> </span>-&gt; <span class="kt">u32</span> <span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="bp">self</span><span class="p">.</span><span class="n">duration_sec</span><span class="w"> </span><span class="o">*</span><span class="w"> </span><span class="mi">2</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p>Теперь мы можем написать <strong>одну единую функцию</strong> с ограничением (Trait Bound <code>T: Workout</code>):</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">total_calories</span><span class="o">&lt;</span><span class="n">T</span>: <span class="nc">Workout</span><span class="o">&gt;</span><span class="p">(</span><span class="n">exercises</span>: <span class="kp">&amp;</span><span class="p">[</span><span class="n">T</span><span class="p">])</span><span class="w"> </span>-&gt; <span class="kt">u32</span> <span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">exercises</span><span class="p">.</span><span class="n">iter</span><span class="p">().</span><span class="n">map</span><span class="p">(</span><span class="o">|</span><span class="n">e</span><span class="o">|</span><span class="w"> </span><span class="n">e</span><span class="p">.</span><span class="n">calories_burned</span><span class="p">()).</span><span class="n">sum</span><span class="p">()</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>

<h3 class="relative group">Что происходит под капотом?
    <div id="что-происходит-под-капотом" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%87%d1%82%d0%be-%d0%bf%d1%80%d0%be%d0%b8%d1%81%d1%85%d0%be%d0%b4%d0%b8%d1%82-%d0%bf%d0%be%d0%b4-%d0%ba%d0%b0%d0%bf%d0%be%d1%82%d0%be%d0%bc" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Компилятор не вызывает один универсальный метод для всех типов. Вместо этого он генерирует отдельную специализированную версию функции для каждого конкретного типа на этапе сборки (<strong>мономорфизация</strong>). В результате вы получаете удобство и абсолютную гибкость абстракций с производительностью чистого C (Zero-cost abstractions).</p>
<p>Попробуйте пошаговый интерактивный пример ниже: на первом шаге вы увидите ошибку компилятора при попытке вызвать метод у неотграниченного типа <code>T</code>, а на втором — её решение с помощью трейта <code>Workout</code>:</p>
### 💻 Пример: Трейты на практике (Фитнес-тренировки)


#### Шаг 1


```rust
// ?compile_fail
struct Pushups {
    reps: u32,
}

impl Pushups {
    fn calories_burned(&self) -> u32 {
        self.reps * 1
    }
}

// Попытка написать обобщенную функцию БЕЗ ограничения трейтом T
fn total_calories<T>(exercises: &[T]) -> u32 {
    exercises.iter().map(|e| e.calories_burned()).sum()
}

fn main() {
    let pushups = vec![Pushups { reps: 20 }, Pushups { reps: 30 }];
    let total = total_calories(&pushups);
    println!("Всего калорий: {}", total);
}
```


**Шаг 1: Ошибка компиляции без ограничения трейта**
- Попытка вызвать метод `calories_burned()` у обобщенного типа `T` без объявления контракта.
- Компилятор не знает, есть ли у типа `T` такой метод, и блокирует сборку.


#### Шаг 2


```rust
pub trait Workout {
    fn calories_burned(&self) -> u32;
}

struct Pushups {
    reps: u32,
}

struct Plank {
    duration_sec: u32,
}

impl Workout for Pushups {
    fn calories_burned(&self) -> u32 {
        self.reps * 1
    }
}

impl Workout for Plank {
    fn calories_burned(&self) -> u32 {
        self.duration_sec * 2
    }
}

// Единая обобщенная функция с ограничением Trait Bound (T: Workout)
fn total_calories<T: Workout>(exercises: &[T]) -> u32 {
    exercises.iter().map(|e| e.calories_burned()).sum()
}

fn main() {
    let pushups = vec![Pushups { reps: 20 }, Pushups { reps: 30 }];
    let planks = vec![Plank { duration_sec: 60 }, Plank { duration_sec: 45 }];

    println!("Калории от отжиманий: {}", total_calories(&pushups));
    println!("Калории от планки: {}", total_calories(&planks));
}
```


**Шаг 2: Решение с явным трейтом Workout**
- Трейт `Workout` декларирует обязательный контракт: наличие метода `calories_burned`.
- Структуры `Pushups` и `Plank` реализуют типаж через `impl Workout for ...`.
- Функция `total_calories` принимает любой тип `T: Workout`, а компилятор генерирует оптимизированный код под каждый тип без задержек в рантайме.


<hr>

<h2 class="relative group">1. Что такое трейты и базовый синтаксис
    <div id="1-что-такое-трейты-и-базовый-синтаксис" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#1-%d1%87%d1%82%d0%be-%d1%82%d0%b0%d0%ba%d0%be%d0%b5-%d1%82%d1%80%d0%b5%d0%b9%d1%82%d1%8b-%d0%b8-%d0%b1%d0%b0%d0%b7%d0%be%d0%b2%d1%8b%d0%b9-%d1%81%d0%b8%d0%bd%d1%82%d0%b0%d0%ba%d1%81%d0%b8%d1%81" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p><strong>Трейт (Trait)</strong> — это описание набора методов, которые должен уметь выполнять тип данных. Сам по себе трейт не хранит данные, он лишь декларирует поведение.</p>
<p>Давайте объявим простой трейт <code>Say</code> и реализуем его для структуры <code>Dog</code>. По соглашению Rust, имена трейтов и структур пишутся в <code>CamelCase</code>.</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="c1">// Объявляем контракт: кто угодно может «сказать» что-то
</span></span></span><span class="line"><span class="cl"><span class="k">pub</span><span class="w"> </span><span class="k">trait</span><span class="w"> </span><span class="n">Say</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">fn</span> <span class="nf">say</span><span class="p">(</span><span class="o">&amp;</span><span class="bp">self</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">struct</span> <span class="nc">Dog</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="c1">// Реализуем контракт для нашей собаки
</span></span></span><span class="line"><span class="cl"><span class="k">impl</span><span class="w"> </span><span class="n">Say</span><span class="w"> </span><span class="k">for</span><span class="w"> </span><span class="n">Dog</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">fn</span> <span class="nf">say</span><span class="p">(</span><span class="o">&amp;</span><span class="bp">self</span><span class="p">)</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Woof!&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p>Теперь, если у нас есть экземпляр <code>Dog</code>, мы можем вызвать этот метод через точку:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">dog</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">Dog</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="n">dog</span><span class="p">.</span><span class="n">say</span><span class="p">();</span><span class="w"> </span><span class="c1">// Выведет: Woof!
</span></span></span></code></pre></div></div>
<div class="admonition relative overflow-hidden rounded-lg border-l-4 my-3 px-4 py-3 shadow-sm" data-type="important">
      <div class="flex items-center gap-2 font-semibold text-inherit">
        <div class="flex shrink-0 h-5 w-5 items-center justify-center text-lg"><span class="relative block icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 576 512"><path fill="currentColor" d="M287.9 0C297.1 0 305.5 5.25 309.5 13.52L378.1 154.8L531.4 177.5C540.4 178.8 547.8 185.1 550.7 193.7C553.5 202.4 551.2 211.9 544.8 218.2L433.6 328.4L459.9 483.9C461.4 492.9 457.7 502.1 450.2 507.4C442.8 512.7 432.1 513.4 424.9 509.1L287.9 435.9L150.1 509.1C142.9 513.4 133.1 512.7 125.6 507.4C118.2 502.1 114.5 492.9 115.1 483.9L142.2 328.4L31.11 218.2C24.65 211.9 22.36 202.4 25.2 193.7C28.03 185.1 35.5 178.8 44.49 177.5L197.7 154.8L266.3 13.52C270.4 5.249 278.7 0 287.9 0L287.9 0zM287.9 78.95L235.4 187.2C231.9 194.3 225.1 199.3 217.3 200.5L98.98 217.9L184.9 303C190.4 308.5 192.9 316.4 191.6 324.1L171.4 443.7L276.6 387.5C283.7 383.7 292.2 383.7 299.2 387.5L404.4 443.7L384.2 324.1C382.9 316.4 385.5 308.5 391 303L476.9 217.9L358.6 200.5C350.7 199.3 343.9 194.3 340.5 187.2L287.9 78.95z"/></svg></span></div>
        <div class="grow">
          Важно
        </div>
      </div><div class="admonition-content mt-3 text-base leading-relaxed text-inherit"><p><strong>Ограничение видимости:</strong> На практике не забудьте импортировать трейт! Чтобы вызвать метод трейта для какого-то типа, сам трейт должен быть импортирован в текущую область видимости с помощью ключевого слова <code>use</code> (например, <code>use crate::Say;</code>). Без этого компилятор выдаст ошибку, даже если тип реализует данный трейт.</p></div></div><hr>

<h2 class="relative group">2. Анатомия элементов трейта
    <div id="2-анатомия-элементов-трейта" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#2-%d0%b0%d0%bd%d0%b0%d1%82%d0%be%d0%bc%d0%b8%d1%8f-%d1%8d%d0%bb%d0%b5%d0%bc%d0%b5%d0%bd%d1%82%d0%be%d0%b2-%d1%82%d1%80%d0%b5%d0%b9%d1%82%d0%b0" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Внутри трейта могут находиться не только обычные методы экземпляра.</p>

<h3 class="relative group">Методы и ассоциированные функции
    <div id="методы-и-ассоциированные-функции" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bc%d0%b5%d1%82%d0%be%d0%b4%d1%8b-%d0%b8-%d0%b0%d1%81%d1%81%d0%be%d1%86%d0%b8%d0%b8%d1%80%d0%be%d0%b2%d0%b0%d0%bd%d0%bd%d1%8b%d0%b5-%d1%84%d1%83%d0%bd%d0%ba%d1%86%d0%b8%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h3>
<ul>
<li><strong>Методы экземпляра:</strong> принимают первым аргументом <code>self</code>, <code>&amp;self</code> или <code>&amp;mut self</code>. Вызываются через точку от объекта (<code>dog.say()</code>).</li>
<li><strong>Ассоциированные функции:</strong> не принимают <code>self</code> первым аргументом. Это своего рода «статические методы» типа, которые вызываются через двоеточие (<code>Dog::species()</code>).</li>
</ul>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">pub</span><span class="w"> </span><span class="k">trait</span><span class="w"> </span><span class="n">Animal</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">fn</span> <span class="nf">name</span><span class="p">(</span><span class="o">&amp;</span><span class="bp">self</span><span class="p">)</span><span class="w"> </span>-&gt; <span class="kp">&amp;</span><span class="kt">str</span><span class="p">;</span><span class="w">        </span><span class="c1">// Метод экземпляра
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">fn</span> <span class="nf">species</span><span class="p">()</span><span class="w"> </span>-&gt; <span class="kp">&amp;</span><span class="nb">&#39;static</span> <span class="kt">str</span><span class="p">;</span><span class="w"> </span><span class="c1">// Ассоциированная функция без self
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">struct</span> <span class="nc">Dog</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">impl</span><span class="w"> </span><span class="n">Animal</span><span class="w"> </span><span class="k">for</span><span class="w"> </span><span class="n">Dog</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">fn</span> <span class="nf">name</span><span class="p">(</span><span class="o">&amp;</span><span class="bp">self</span><span class="p">)</span><span class="w"> </span>-&gt; <span class="kp">&amp;</span><span class="kt">str</span> <span class="p">{</span><span class="w"> </span><span class="s">&#34;Барбос&#34;</span><span class="w"> </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">fn</span> <span class="nf">species</span><span class="p">()</span><span class="w"> </span>-&gt; <span class="kp">&amp;</span><span class="nb">&#39;static</span> <span class="kt">str</span> <span class="p">{</span><span class="w"> </span><span class="s">&#34;Canis lupus&#34;</span><span class="w"> </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="c1">// Вызов ассоциированной функции:
</span></span></span><span class="line"><span class="cl"><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Вид: </span><span class="si">{}</span><span class="s">&#34;</span><span class="p">,</span><span class="w"> </span><span class="n">Dog</span>::<span class="n">species</span><span class="p">());</span></span></span></code></pre></div></div>

<h3 class="relative group">Реализации по умолчанию (Default Implementations)
    <div id="реализации-по-умолчанию-default-implementations" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%80%d0%b5%d0%b0%d0%bb%d0%b8%d0%b7%d0%b0%d1%86%d0%b8%d0%b8-%d0%bf%d0%be-%d1%83%d0%bc%d0%be%d0%bb%d1%87%d0%b0%d0%bd%d0%b8%d1%8e-default-implementations" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Мы можем написать тело метода прямо в объявлении трейта. Тогда типы, реализующие его, могут просто использовать дефолтное поведение или переопределить его при необходимости:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">pub</span><span class="w"> </span><span class="k">trait</span><span class="w"> </span><span class="n">Greet</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">fn</span> <span class="nf">hello</span><span class="p">(</span><span class="o">&amp;</span><span class="bp">self</span><span class="p">)</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Привет!&#34;</span><span class="p">);</span><span class="w"> </span><span class="c1">// Реализация по умолчанию
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">struct</span> <span class="nc">Student</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">impl</span><span class="w"> </span><span class="n">Greet</span><span class="w"> </span><span class="k">for</span><span class="w"> </span><span class="n">Student</span><span class="w"> </span><span class="p">{}</span><span class="w"> </span><span class="c1">// Оставляем тело пустым — используется дефолтное поведение
</span></span></span></code></pre></div></div>

<h3 class="relative group">Ассоциированные константы
    <div id="ассоциированные-константы" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b0%d1%81%d1%81%d0%be%d1%86%d0%b8%d0%b8%d1%80%d0%be%d0%b2%d0%b0%d0%bd%d0%bd%d1%8b%d0%b5-%d0%ba%d0%be%d0%bd%d1%81%d1%82%d0%b0%d0%bd%d1%82%d1%8b" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Трейты могут объявлять константы, значения которых задаются при реализации для конкретного типа. Их имена пишутся в <code>UPPER_SNAKE_CASE</code>:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">pub</span><span class="w"> </span><span class="k">trait</span><span class="w"> </span><span class="n">HasId</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">const</span><span class="w"> </span><span class="no">ID</span>: <span class="kt">usize</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">struct</span> <span class="nc">User</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">impl</span><span class="w"> </span><span class="n">HasId</span><span class="w"> </span><span class="k">for</span><span class="w"> </span><span class="n">User</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">const</span><span class="w"> </span><span class="no">ID</span>: <span class="kt">usize</span> <span class="o">=</span><span class="w"> </span><span class="mi">42</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<hr>

<h2 class="relative group">3. Маркерные трейты: Copy и Clone
    <div id="3-маркерные-трейты-copy-и-clone" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#3-%d0%bc%d0%b0%d1%80%d0%ba%d0%b5%d1%80%d0%bd%d1%8b%d0%b5-%d1%82%d1%80%d0%b5%d0%b9%d1%82%d1%8b-copy-%d0%b8-clone" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>В Rust есть два фундаментальных трейта, которые управляют тем, как данные копируются в памяти:</p>
<table>
	<thead>
			<tr>
					<th>Трейт</th>
					<th>Тип трейта</th>
					<th>Механизм действия</th>
					<th>Вызов в коде</th>
			</tr>
	</thead>
	<tbody>
			<tr>
					<td><strong><code>Clone</code></strong></td>
					<td>Обычный явный</td>
					<td>Создает глубокую копию объекта (может выделять память в куче и выполнять произвольный код)</td>
					<td>Требует явного вызова метода <code>.clone()</code></td>
			</tr>
			<tr>
					<td><strong><code>Copy</code></strong></td>
					<td>Маркерный авто-трейт</td>
					<td>Сигнализирует компилятору о возможности побитового копирования объекта (<code>memcpy</code>). Меняет семантику перемещения (Move) на копирование</td>
					<td>Происходит автоматически при присваивании или передаче в функцию</td>
			</tr>
	</tbody>
</table>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">a</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="nb">String</span>::<span class="n">from</span><span class="p">(</span><span class="s">&#34;abc&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">b</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">a</span><span class="p">;</span><span class="w"> </span><span class="c1">// Move (владение перешло к b, &#39;a&#39; больше недоступна)
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">x</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="mi">5</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">y</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">x</span><span class="p">;</span><span class="w"> </span><span class="c1">// Copy (значение x неявно скопировано, обе переменные доступны)
</span></span></span></code></pre></div></div>
<p>Трейт <code>Copy</code> является наследником <code>Clone</code> (<code>trait Copy: Clone {}</code>). Вы не можете сделать тип <code>Copy</code>, не реализовав для него <code>Clone</code>.</p>
<ul>
<li>Числовые типы (<code>i32</code>, <code>f64</code>), <code>bool</code>, <code>char</code> реализуют <code>Copy</code> изначально.</li>
<li>Для пользовательских структур мы можем автоматически вывести их с помощью макроса: <code>#[derive(Copy, Clone)]</code>.</li>
</ul>
<hr>

<h2 class="relative group">4. Дженерики, Trait Bounds и <code>impl Trait</code>
    <div id="4-дженерики-trait-bounds-и-impl-trait" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#4-%d0%b4%d0%b6%d0%b5%d0%bd%d0%b5%d1%80%d0%b8%d0%ba%d0%b8-trait-bounds-%d0%b8-impl-trait" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p><strong>Дженерики (обобщенные типы)</strong> позволяют писать структуры и функции, работающие с любым типом данных.</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">struct</span> <span class="nc">Point</span><span class="o">&lt;</span><span class="n">T</span><span class="o">&gt;</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">x</span>: <span class="nc">T</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">y</span>: <span class="nc">T</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>

<h3 class="relative group">Ограничения типажей (Trait Bounds)
    <div id="ограничения-типажей-trait-bounds" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%be%d0%b3%d1%80%d0%b0%d0%bd%d0%b8%d1%87%d0%b5%d0%bd%d0%b8%d1%8f-%d1%82%d0%b8%d0%bf%d0%b0%d0%b6%d0%b5%d0%b9-trait-bounds" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Если мы хотим производить операции над обобщенным типом <code>T</code>, мы накладываем ограничения (Trait Bounds):</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">impl</span><span class="o">&lt;</span><span class="n">T</span><span class="o">&gt;</span><span class="w"> </span><span class="n">Point</span><span class="o">&lt;</span><span class="n">T</span><span class="o">&gt;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">where</span><span class="w"> 
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="n">T</span>: <span class="nc">std</span>::<span class="n">ops</span>::<span class="n">Add</span><span class="o">&lt;</span><span class="n">Output</span><span class="o">=</span><span class="n">T</span><span class="o">&gt;</span><span class="w"> </span><span class="c1">// Ограничиваем тип T контрактом сложения
</span></span></span><span class="line"><span class="cl"><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="k">fn</span> <span class="nf">add</span><span class="p">(</span><span class="bp">self</span><span class="p">,</span><span class="w"> </span><span class="n">other</span>: <span class="nc">Self</span><span class="p">)</span><span class="w"> </span>-&gt; <span class="nc">Self</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="bp">Self</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">            </span><span class="n">x</span>: <span class="nc">self</span><span class="p">.</span><span class="n">x</span><span class="w"> </span><span class="o">+</span><span class="w"> </span><span class="n">other</span><span class="p">.</span><span class="n">x</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">            </span><span class="n">y</span>: <span class="nc">self</span><span class="p">.</span><span class="n">y</span><span class="w"> </span><span class="o">+</span><span class="w"> </span><span class="n">other</span><span class="p">.</span><span class="n">y</span><span class="p">,</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>

<h3 class="relative group">Сокращенная форма <code>impl Trait</code> в аргументах функций
    <div id="сокращенная-форма-impl-trait-в-аргументах-функций" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%81%d0%be%d0%ba%d1%80%d0%b0%d1%89%d0%b5%d0%bd%d0%bd%d0%b0%d1%8f-%d1%84%d0%be%d1%80%d0%bc%d0%b0-impl-trait-%d0%b2-%d0%b0%d1%80%d0%b3%d1%83%d0%bc%d0%b5%d0%bd%d1%82%d0%b0%d1%85-%d1%84%d1%83%d0%bd%d0%ba%d1%86%d0%b8%d0%b9" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Для большинства обычных функций в Rust вместо явного <code>fn foo&lt;T: Trait&gt;(x: T)</code> используют более лаконичный синтаксис — <code>impl Trait</code>:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">print_workout</span><span class="p">(</span><span class="n">w</span>: <span class="nc">impl</span><span class="w"> </span><span class="n">Workout</span><span class="p">)</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Калории: </span><span class="si">{}</span><span class="s">&#34;</span><span class="p">,</span><span class="w"> </span><span class="n">w</span><span class="p">.</span><span class="n">calories_burned</span><span class="p">());</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p>Обе записи принципиально эквивалентны: компилятор точно так же превратит <code>impl Trait</code> в обобщенную функцию на этапе сборки.</p>
<hr>

<h2 class="relative group">5. Глубокий механизм мономорфизации
    <div id="5-глубокий-механизм-мономорфизации" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#5-%d0%b3%d0%bb%d1%83%d0%b1%d0%be%d0%ba%d0%b8%d0%b9-%d0%bc%d0%b5%d1%85%d0%b0%d0%bd%d0%b8%d0%b7%d0%bc-%d0%bc%d0%be%d0%bd%d0%be%d0%bc%d0%be%d1%80%d1%84%d0%b8%d0%b7%d0%b0%d1%86%d0%b8%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>В отличие от языков вроде Java или C#, где дженерики стираются в рантайме, или Python, где всё проверяется на лету, Rust использует <strong>мономорфизацию</strong>.</p>
<p>Когда компилятор видит обобщенный код, он находит все конкретные типы, с которыми вызывалась функция, и во время сборки генерирует физические копии этой функции под каждый тип.</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="c1">// Исходный обобщенный код
</span></span></span><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">foo</span><span class="o">&lt;</span><span class="n">T</span><span class="o">&gt;</span><span class="p">(</span><span class="n">x</span>: <span class="nc">T</span><span class="p">)</span><span class="w"> </span><span class="p">{</span><span class="w"> </span><span class="o">..</span><span class="p">.</span><span class="w"> </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="n">foo</span><span class="p">(</span><span class="mi">5</span><span class="p">);</span><span class="w">       </span><span class="c1">// Вызов с i32
</span></span></span><span class="line"><span class="cl"><span class="n">foo</span><span class="p">(</span><span class="s">&#34;Hello&#34;</span><span class="p">);</span><span class="w"> </span><span class="c1">// Вызов с &amp;str
</span></span></span></code></pre></div></div>
<p>Под капотом компилятор концептуально превратит это в две обычные функции (конкретные имена в бинарнике будут другими — это лишь концептуальная иллюстрация):</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">foo_i32</span><span class="p">(</span><span class="n">x</span>: <span class="kt">i32</span><span class="p">)</span><span class="w"> </span><span class="p">{</span><span class="w"> </span><span class="o">..</span><span class="p">.</span><span class="w"> </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">foo_str</span><span class="p">(</span><span class="n">x</span>: <span class="kp">&amp;</span><span class="kt">str</span><span class="p">)</span><span class="w"> </span><span class="p">{</span><span class="w"> </span><span class="o">..</span><span class="p">.</span><span class="w"> </span><span class="p">}</span></span></span></code></pre></div></div>

<h4 class="relative group">Плюсы мономорфизации:
    <div id="плюсы-мономорфизации" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d0%bb%d1%8e%d1%81%d1%8b-%d0%bc%d0%be%d0%bd%d0%be%d0%bc%d0%be%d1%80%d1%84%d0%b8%d0%b7%d0%b0%d1%86%d0%b8%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h4>
<ul>
<li><strong>Нулевые издержки (Zero-cost abstraction):</strong> Программа работает максимально быстро, так как типы известны заранее, нет динамических проверок и лишней адресации в памяти.</li>
<li><strong>Агрессивный инлайнинг:</strong> Компилятор может легко встраивать код коротких функций прямо в место вызова.</li>
</ul>

<h4 class="relative group">Минусы мономорфизации:
    <div id="минусы-мономорфизации" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bc%d0%b8%d0%bd%d1%83%d1%81%d1%8b-%d0%bc%d0%be%d0%bd%d0%be%d0%bc%d0%be%d1%80%d1%84%d0%b8%d0%b7%d0%b0%d1%86%d0%b8%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h4>
<ul>
<li><strong>Замедление компиляции:</strong> Компилятору приходится собирать и оптимизировать кучу сгенерированных дубликатов функций.</li>
<li><strong>Раздувание бинарника (Code Bloat):</strong> Сгенерированный машинный код увеличивается в объеме.</li>
</ul>
<hr>
<p>Давайте посмотрим на работу трейтов и статического полиморфизма в действии в интерактивных слайдах ниже:</p>
### 💻 Трейты и статический полиморфизм


#### Шаг 1


```rust
pub trait Say {
    fn say(&self) -> String;
}

struct Dog;

impl Say for Dog {
    fn say(&self) -> String {
        "Woof!".to_string()
    }
}

fn main() {
    let dog = Dog;
    println!("Собака говорит: {}", dog.say());
}
```


**Объявление и реализация трейта**
- Трейт `Say` описывает контракт: любой тип, реализующий его, должен уметь говорить.
- Структура `Dog` реализует метод `say` из трейта `Say`.
- После реализации метод становится доступен у структуры через точку.


#### Шаг 2


```rust
pub trait HasId {
    const ID: usize;
    type Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> where Self: Sized;
}

struct MyStruct;

impl HasId for MyStruct {
    const ID: usize = 10;
    type Err = std::convert::Infallible;
    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(MyStruct)
    }
}

fn main() {
    println!("ID структуры: {}", MyStruct::ID);
    let _s = MyStruct::from_str("test").unwrap();
}
```


**Ассоциированные элементы**
- Константы (`const ID`) и типы (`type Err`) объявляются внутри трейта и уточняются в имплементациях.
- Ассоциированный тип выступает в роли плейсхолдера типа, который конкретизируется при реализации.
- Это избавляет от необходимости плодить дженерики во всех сигнатурах методов трейта.


#### Шаг 3


```rust
use std::fmt;
// Мы не можем реализовать Display для Vec<i32> напрямую, так как оба типа чужие.
// Но мы можем обернуть Vec в наш собственный кортежный тип (Newtype)!
struct MyVec(Vec<i32>);

impl fmt::Display for MyVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Мой вектор с {} элементами", self.0.len())
    }
}

fn main() {
    let my_vec = MyVec(vec![100, 200, 300]);
    println!("{}", my_vec);
}
```


**Паттерн Newtype (Обертка)**
- Правило сиротства (Orphan Rules) запрещает реализовывать чужой трейт для чужого типа.
- Создав структуру-обертку `MyVec`, мы делаем тип локальным.
- Теперь мы можем реализовать для нее любой внешний трейт (например, `Display`).


#### Шаг 4


```rust
// Трейт, который мы будем использовать как ограничение
trait Loud {
    fn loud_say(&self) -> String;
}

// Обобщенная структура
struct Point<T> {
    x: T,
    y: T,
}

// Blanket implementation: реализуем Loud для любого Point<T>, если T реализует Loud
impl<T: Loud> Loud for Point<T> {
    fn loud_say(&self) -> String {
        format!("X: {}, Y: {}", self.x.loud_say(), self.y.loud_say())
    }
}

struct Dog;
impl Loud for Dog {
    fn loud_say(&self) -> String {
        "WOOF!".to_string()
    }
}

fn main() {
    let p = Point { x: Dog, y: Dog };
    println!("Точка кричит: {}", p.loud_say());
}
```


**Дженерики и Blanket-реализации**
- Ограничения типа (Trait Bounds) `T: Loud` гарантируют, что тип `T` поддерживает нужные операции.
- Blanket implementation позволяет реализовать трейт сразу для целого семейства типов, подходящих под условия.
- Компилятор производит мономорфизацию обобщенного кода, генерируя отдельные функции под каждый тип.


#### Шаг 5


```rust
use std::mem::size_of;
trait A {
    fn get(&self) -> &str;
}

trait B {
    fn get(&self) -> &str;
}

struct S;

impl A for S {
    fn get(&self) -> &str { "реализация A" }
}

impl B for S {
    fn get(&self) -> &str { "реализация B" }
}

fn main() {
    let s = S;
    // s.get(); // Ошибка компиляции: имя метода get неоднозначно!
    
    // Полный синтаксис (Fully Qualified Syntax) решает конфликт:
    let from_a = <S as A>::get(&s);
    let from_b = <S as B>::get(&s); 
    
    // Turbofish для явного указания типа функции:
    let float_size = size_of::<f64>();
    
    println!("A: {}, B: {}, размер f64: {}", from_a, from_b, float_size);
}
```


**Turbofish и Полный синтаксис**
- Turbofish `::<>` подсказывает типы компилятору там, где он не может вывести их автоматически.
- Полностью квалифицированное имя `<S as A>::get` разрешает коллизии, когда один тип реализует несколько трейтов с одинаковыми именами методов.


<hr>

<h2 class="relative group">Проверь свои знания!
    <div id="проверь-свои-знания" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%be%d0%b2%d0%b5%d1%80%d1%8c-%d1%81%d0%b2%d0%be%d0%b8-%d0%b7%d0%bd%d0%b0%d0%bd%d0%b8%d1%8f" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Пройдите короткий тест, чтобы закрепить понимание трейтов и статического полиморфизма в Rust.</p>
### 📝 Трейты и статический полиморфизм

**1. Зачем нужны трейты в Rust?**

- [ ] Для динамической аллокации памяти в куче
- [x] Для описания контрактов поведения типов и реализации статического полиморфизма без дублирования кода
- [ ] Для автоматического освобождения ресурсов без трейта Drop
- [ ] Для скрытия приватных полей структуры

> 💡 *Трейты декларируют набор методов (контракт поведения), который могут реализовывать разные типы данных, обеспечивая гибкий и безопасный полиморфизм.*


**2. В чём главное различие между ассоциированной функцией и методом трейта?**

- [x] Ассоциированная функция не принимает параметр self (или &self) и вызывается через Type::func(), а метод принимает self и вызывается у объекта через точку
- [ ] Методы нельзя переопределять, а ассоциированные функции можно
- [ ] Ассоциированные функции могут возвращать только численные типы

> 💡 *Ассоциированные функции не принимают аргумент self (выступая аналогом статических методов) и вызываются через двоеточие (Dog::species()), тогда как методы принимают self и вызываются у экземпляра (dog.say()).*


**3. В чем ключевое различие между трейтами Copy и Clone?**

- [ ] Clone выполняется автоматически при присваивании, а Copy требует вызова метода
- [x] Copy выполняет неявное побитовое копирование (memcpy) на уровне компилятора и меняет семантику Move, а Clone создает копию объекта через явный вызов метода `.clone()`
- [ ] Copy работает только в куче, а Clone — на стеке

> 💡 *Copy — маркерный трейт для автоматического побитового копирования данных при присваивании. Clone — обычный трейт для явного вызова `.clone()` с произвольной логикой копирования.*


**4. Как синтаксис `impl Trait` в аргументах функции соотносится с Trait Bounds `<T: Trait>`?**

- [ ] impl Trait переводит функцию на динамический полиморфизм через vtable
- [x] impl Trait — это удобная короткая форма Trait Bound: компилятор точно так же превратит её в обобщенную функцию и выполнит мономорфизацию
- [ ] impl Trait запрещает передавать структуры, выведенные через derive

> 💡 *В аргументах функций `fn foo(x: impl Trait)` является удобной сокращенной формой записи `fn foo<T: Trait>(x: T)`. Оба варианта используют статический полиморфизм и мономорфизацию.*


**5. Что такое мономорфизация дженериков?**

- [ ] Приведение типов к динамическим трейт-объектам во время работы программы
- [ ] Вызов виртуальных функций через таблицу vtable
- [x] Генерация компилятором отдельных физических копий кода под каждый конкретный тип, с которым вызывается обобщенный код, на этапе сборки

> 💡 *Мономорфизация — это процесс дублирования кода компилятором с подстановкой реальных типов вместо дженерика. Это дает производительность с нулевой накладной стоимостью (Zero-cost abstraction).*




