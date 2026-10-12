# Создание и использование констант (const) и static в Rust

> Разница между const и static в языке Rust: как, зачем и когда использовать константы? Практический гайд с понятными примерами кода без лишней теории.

- **URL:** https://eagle2000.space/rust/rust-const/
- **Date:** 2025-05-24

- **Tags:** rust, dev

---


<h2 class="relative group">Введение
    <div id="введение" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d0%b2%d0%b5%d0%b4%d0%b5%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Константы <code>const</code> — один из базовых инструментов в Rust который как и переменные позволяет сохранять какие-либо данные в памяти. На первый взгляд они могут показаться похожими на переменные. Но есть несколько ключевых отличий, которые делают их полезными в определённых ситуациях. Давайте разберёмся, как их использовать, зачем они нужны и какие подводные камни могут нам встретиться.</p>

<h2 class="relative group">Отличие констант от переменных
    <div id="отличие-констант-от-переменных" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%be%d1%82%d0%bb%d0%b8%d1%87%d0%b8%d0%b5-%d0%ba%d0%be%d0%bd%d1%81%d1%82%d0%b0%d0%bd%d1%82-%d0%be%d1%82-%d0%bf%d0%b5%d1%80%d0%b5%d0%bc%d0%b5%d0%bd%d0%bd%d1%8b%d1%85" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Начнём с главного: константы <strong>неизменяемы</strong>. В отличие от <code>let</code>, которое хоть и может создавать неизменяемые переменные <code>let x = 5</code> но их всегда можно перезаписать через затенение <code>let x = 6</code></p>
<p>С константами такой трюк не пройдёт: значение констант фиксировано в момент компиляции и не может быть изменено.</p>
<p>Два важных момента:</p>
<ol>
<li><strong>Область видимости</strong>
<ul>
<li>Обычные переменные живут только внутри своего блока <code>{}</code> <em>скоупа</em></li>
<li>Константы можно объявлять на уровне файла. Они будут доступны на протяжении всего кода в файле</li>
</ul>
</li>
<li><strong>Когда вычисляется значение</strong>
<ul>
<li>Переменные могут &ldquo;принимать&rdquo; значения во время выполнения <code>let user_input = read_line()</code></li>
<li>Константы должны быть известны при компиляции - нельзя присвоить им результат вычислений в рантайме</li>
</ul>
</li>
</ol>

<h2 class="relative group">Как объявить константу?
    <div id="как-объявить-константу" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%ba%d0%b0%d0%ba-%d0%be%d0%b1%d1%8a%d1%8f%d0%b2%d0%b8%d1%82%d1%8c-%d0%ba%d0%be%d0%bd%d1%81%d1%82%d0%b0%d0%bd%d1%82%d1%83" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Синтаксис объявления констант чрезвычайно прост, но имеет важные отличия от переменных.</p>
### 💻 Работа с константами в Rust


#### Шаг 1


```rust
const MAX_CONNECTIONS: u32 = 100;
const SECONDS_IN_HOUR: u64 = 60 * 60; // Компилятор вычислит это выражение заранее!

fn main() {
    println!("Max: {}, Seconds: {}", MAX_CONNECTIONS, SECONDS_IN_HOUR);
}
```


**Объявление констант**
- Константы `const` объявляются с явным указанием типа.
- Значение константы вычисляется на этапе компиляции.


#### Шаг 2


```rust
// ?compile_fail
const FAST_LANG: &str = "Rust";

fn main() {
    println!("The {} language", FAST_LANG);
    let FAST_LANG = "Dart"; // ❌ Ошибка компиляции
}
```


**Константы нельзя переопределять/затенять**
- Затенение `let` запрещено для констант.
- Компилятор выдаст ошибку, защищая код от нежелательного переопределения глобальных значений.



  



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
    
    ><p>🔥 Важные правила:</p>
<ol>
<li>Старайтесь придерживаться именования <code>SCREAMING_SNAKE_CASE</code> (заглавные буквы и подчёркивание).</li>
<li>Нельзя изменять значение константы.</li>
<li>Нельзя затенять константу с помощью <code>let</code>.</li>
<li>Нужно обязательно указать тип после двоеточия (компилятор не выводит типы констант автоматически).</li>
</ol></span>
</div>

<p>Зачем же так строго?
Компилятор Rust следует принципу &ldquo;явное лучше неявного&rdquo;. Если бы константы можно было затенять - код становился бы менее предсказуемым и проще было бы допустить ошибку.</p>

<h2 class="relative group">Где используют константы?
    <div id="где-используют-константы" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b3%d0%b4%d0%b5-%d0%b8%d1%81%d0%bf%d0%be%d0%bb%d1%8c%d0%b7%d1%83%d1%8e%d1%82-%d0%ba%d0%be%d0%bd%d1%81%d1%82%d0%b0%d0%bd%d1%82%d1%8b" aria-label="Якорь">#</a>
    </span>
    
</h2>

<h3 class="relative group">Знаменитые &ldquo;магические числа&rdquo;
    <div id="знаменитые-магические-числа" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b7%d0%bd%d0%b0%d0%bc%d0%b5%d0%bd%d0%b8%d1%82%d1%8b%d0%b5-%d0%bc%d0%b0%d0%b3%d0%b8%d1%87%d0%b5%d1%81%d0%ba%d0%b8%d0%b5-%d1%87%d0%b8%d1%81%d0%bb%d0%b0" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Вместо:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">if</span><span class="w"> </span><span class="n">users</span><span class="p">.</span><span class="n">len</span><span class="p">()</span><span class="w"> </span><span class="o">&gt;</span><span class="w"> </span><span class="mi">100</span><span class="w"> </span><span class="p">{</span><span class="w"> </span><span class="o">..</span><span class="p">.</span><span class="w"> </span><span class="p">}</span></span></span></code></pre></div></div>
<p>Лучше написать:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">const</span><span class="w"> </span><span class="no">MAX_USERS</span>: <span class="kt">usize</span> <span class="o">=</span><span class="w"> </span><span class="mi">100</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="k">if</span><span class="w"> </span><span class="n">users</span><span class="p">.</span><span class="n">len</span><span class="p">()</span><span class="w"> </span><span class="o">&gt;</span><span class="w"> </span><span class="no">MAX_USERS</span><span class="w"> </span><span class="p">{</span><span class="w"> </span><span class="o">..</span><span class="p">.</span><span class="w"> </span><span class="p">}</span></span></span></code></pre></div></div>
<hr>

<h3 class="relative group">Глобальные настройки
    <div id="глобальные-настройки" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b3%d0%bb%d0%be%d0%b1%d0%b0%d0%bb%d1%8c%d0%bd%d1%8b%d0%b5-%d0%bd%d0%b0%d1%81%d1%82%d1%80%d0%be%d0%b9%d0%ba%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Например, лимиты или таймауты, коды ошибок:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">const</span><span class="w"> </span><span class="no">REQUEST_TIMEOUT</span>: <span class="kt">u64</span> <span class="o">=</span><span class="w"> </span><span class="mi">5000</span><span class="p">;</span><span class="w"> </span><span class="c1">// 5 секунд
</span></span></span></code></pre></div></div>
<hr>

<h3 class="relative group">Константные вычисления
    <div id="константные-вычисления" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%ba%d0%be%d0%bd%d1%81%d1%82%d0%b0%d0%bd%d1%82%d0%bd%d1%8b%d0%b5-%d0%b2%d1%8b%d1%87%d0%b8%d1%81%d0%bb%d0%b5%d0%bd%d0%b8%d1%8f" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Rust позволяет делать простые вычисления в const:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">const</span><span class="w"> </span><span class="no">SECONDS_IN_HOUR</span>: <span class="kt">u64</span> <span class="o">=</span><span class="w"> </span><span class="mi">60</span><span class="w"> </span><span class="o">*</span><span class="w"> </span><span class="mi">60</span><span class="p">;</span><span class="w"> </span><span class="c1">// Компилятор вычислит это заранее
</span></span></span></code></pre></div></div>
<hr>

<h2 class="relative group">⚠️ Что нельзя делать с константами
    <div id="-что-нельзя-делать-с-константами" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#-%d1%87%d1%82%d0%be-%d0%bd%d0%b5%d0%bb%d1%8c%d0%b7%d1%8f-%d0%b4%d0%b5%d0%bb%d0%b0%d1%82%d1%8c-%d1%81-%d0%ba%d0%be%d0%bd%d1%81%d1%82%d0%b0%d0%bd%d1%82%d0%b0%d0%bc%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h2>
<ol>
<li>Нельзя брать значение из рантайма <em>например из пользовательского ввода</em></li>
</ol>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">user_input</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">get_user_input</span><span class="p">();</span><span class="w">  
</span></span></span><span class="line"><span class="cl"><span class="k">const</span><span class="w"> </span><span class="no">INPUT</span>: <span class="kp">&amp;</span><span class="kt">str</span> <span class="o">=</span><span class="w"> </span><span class="n">user_input</span><span class="p">;</span><span class="w"> </span><span class="c1">// Ошибка!
</span></span></span></code></pre></div></div>
<p>Для таких случаев лучше использовать обычные переменные.</p>
<ol start="2">
<li>Нельзя изменять элементы внутри массива/коллекции</li>
</ol>
<p>Если массив или коллекция объявлены как const их содержимое неизменяемо:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">const</span><span class="w"> </span><span class="no">NUMBERS</span>: <span class="p">[</span><span class="kt">i32</span><span class="p">;</span><span class="w"> </span><span class="mi">3</span><span class="p">]</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="p">[</span><span class="mi">1</span><span class="p">,</span><span class="w"> </span><span class="mi">2</span><span class="p">,</span><span class="w"> </span><span class="mi">3</span><span class="p">];</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="no">NUMBERS</span><span class="p">[</span><span class="mi">0</span><span class="p">]</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="mi">5</span><span class="p">;</span><span class="w"> </span><span class="c1">// Ошибка!
</span></span></span></code></pre></div></div>
<hr>

<h2 class="relative group">Итоги
    <div id="итоги" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b8%d1%82%d0%be%d0%b3%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Константы в Rust отлично подходят:</p>
<ul>
<li>Для неизменяемых значений известных заранее</li>
<li>Для замены &ldquo;магических чисел&rdquo; в коде</li>
<li>Для глобальных параметров которые не меняются</li>
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
    
    ><p>Не забывайте что:</p>
<ul>
<li>Нельзя изменять или затенять константы</li>
<li>Значение должно вычисляться на этапе компиляции</li>
<li><code>SCREAMING_SNAKE_CASE</code>- правило наименования для констант</li>
</ul></span>
</div>

<p>На последок совет: если не можете придумать название константы, подумайте к какой предметной области относится ее значение, а также читайте чужой код. Лучше если это будут исходники больших документированных проектов. Так вы получите насмотренность и зафиксируете для себя правильные наименования.</p>

