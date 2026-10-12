# Затенение переменных (Shadowing) в Rust на реальных примерах

> Особенности и преимущества затенения переменных (Shadowing) в языке Rust: переиспользование имен, изменение типов данных и лучшие практики.

- **URL:** https://eagle2000.space/rust/rust-shadowing/
- **Date:** 2025-05-14

- **Tags:** rust, dev

---


<h2 class="relative group">Затенение (variable shadowing)
    <div id="затенение-variable-shadowing" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b7%d0%b0%d1%82%d0%b5%d0%bd%d0%b5%d0%bd%d0%b8%d0%b5-variable-shadowing" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Затенение переменных - это концепция Rust, которая позволяет повторно объявить переменную с тем же именем, используя ключевое слово <code>let</code>. При этом оригинальная переменная заменяется (затеняется) новой, что позволяет изменять тип или состояние данных без создания временных переменных (например, <code>temp_str</code> или <code>temp_float</code>).</p>

<h2 class="relative group">Пример использования затенения
    <div id="пример-использования-затенения" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%b8%d0%bc%d0%b5%d1%80-%d0%b8%d1%81%d0%bf%d0%be%d0%bb%d1%8c%d0%b7%d0%be%d0%b2%d0%b0%d0%bd%d0%b8%d1%8f-%d0%b7%d0%b0%d1%82%d0%b5%d0%bd%d0%b5%d0%bd%d0%b8%d1%8f" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Один из наиболее распространенных примеров, где это может потребоваться - это последовательная трансформация данных и работа с локальными областями видимости (например, обработка данных с температурных датчиков или сенсоров яркости в умном доме).</p>
### 💻 Затенение переменных в Rust


#### Шаг 1


```rust
fn main() {
    let temperature = "25.5";
    let temperature: f64 = temperature.parse().unwrap();
    let mut temperature = temperature * 9.0 / 5.0 + 32.0; // переводим в Фаренгейт

    temperature = temperature + 1.0;
    println!("Температура: {}", temperature);
}
```


**Последовательная трансформация данных**
- Затенение позволяет повторно использовать имя переменной `temperature`.
- Мы изменили тип данных со строки `&str` на число `f64`.
- В конце мы сделали её изменяемой с помощью `let mut`.


#### Шаг 2


```rust
fn main() {
    let brightness = "50";
    {
        let brightness: u8 = brightness.parse().unwrap();
        println!("Внутри блока: {}%", brightness);
        let brightness = brightness + 20;
        println!("Внутри блока (увеличено): {}%", brightness);
    }
    println!("Вне блока (оригинал): {}", brightness);
}
```


**Затенение во вложенной области видимости (Scope)**
- Внутри блока `{}` мы затеняем внешнюю переменную `brightness` новым типом и значением.
- После выхода из блока временные переменные удаляются из стека.
- Внешняя переменная восстанавливает своё оригинальное значение `"50"`.



<h3 class="relative group">Описание работы примера:
    <div id="описание-работы-примера" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%be%d0%bf%d0%b8%d1%81%d0%b0%d0%bd%d0%b8%d0%b5-%d1%80%d0%b0%d0%b1%d0%be%d1%82%d1%8b-%d0%bf%d1%80%d0%b8%d0%bc%d0%b5%d1%80%d0%b0" aria-label="Якорь">#</a>
    </span>
    
</h3>
<ol>
<li>
<p><strong>Шаг 1. Последовательная трансформация данных</strong>:</p>
<ul>
<li>Сначала мы объявили переменную <code>temperature</code> со строковым значением <code>&quot;25.5&quot;</code>.</li>
<li>Затем с помощью ключевого слова <code>let</code> затенили её новой переменной типа <code>f64</code>, распарсив строку.</li>
<li>Далее мы перевели значение в Фаренгейты, а затем переопределили её как изменяемую (<code>mut</code>), прибавив единицу. Это позволяет повторно использовать понятное имя переменной без придумывания временных названий.</li>
</ul>
</li>
<li>
<p><strong>Шаг 2. Затенение в рамках области видимости (scope)</strong>:</p>
<ul>
<li>Мы объявили внешнюю переменную <code>brightness</code> со строковым значением <code>&quot;50&quot;</code>.</li>
<li>Внутри блока <code>{}</code> мы объявили локальную переменную <code>brightness</code> с типом <code>u8</code> и числовым значением <code>50</code>.</li>
<li>Внутри блока мы затенили её ещё раз, увеличив на <code>20</code>.</li>
<li>После выхода из блока все временные локальные переменные очищаются из стека, и внешняя переменная восстанавливает своё оригинальное значение <code>&quot;50&quot;</code>.</li>
</ul>
</li>
</ol>
<p>Надеюсь, эта статья помогла вам понять концепцию затенения переменных в Rust и избавит вас от придумывания новых имен для переменных наподобие <code>upd_temperature</code>.</p>

