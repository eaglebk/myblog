# Области видимости и время жизни переменных в Rust

> Подробный разбор областей видимости (scopes) в Rust: блоки кода, передача владения, автоматическое освобождение ресурсов и предотвращение ошибок.

- **URL:** https://eagle2000.space/rust/rust-scope/
- **Date:** 2025-05-16

- **Tags:** rust, dev

---


<h2 class="relative group">Блоки и скоупы
    <div id="блоки-и-скоупы" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b1%d0%bb%d0%be%d0%ba%d0%b8-%d0%b8-%d1%81%d0%ba%d0%be%d1%83%d0%bf%d1%8b" aria-label="Якорь">#</a>
    </span>
    
</h2>
<div class="lead text-neutral-500 dark:text-neutral-400 !mb-9 text-xl">
  Подключим воображение чтобы лучше понять эту тему.
</div>

<p>Итак, существует переменная со своей силой. Теперь представьте себе область видимости как некую территорию на которой эта переменная имеет силу.
В Rust такая территория называется скоупом (Scope). В этой статье мы поговорим о том, что такое скоуп/блоки и как это всё работает.</p>
<p>Блок (Block) - это область между открытыми и закрытыми фигурными скобками. Каждый раз, когда вы пишете фигурные скобки, например после названия функции:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">hello</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w"> </span><span class="n"> `[1]`
</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">   </span><span class="n"> `[2]`
</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p>вы создаете новый блок  *(строка 1)*
 . Все, что находится внутри этого блока   *(строка 2)*
 считается его областью видимости.</p>
<p><strong>Пример:</strong></p>
<p>Создадим переменную для хранения значения срабатывания датчика движения <em>от 0 до 100%</em></p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">main</span><span class="p">(){</span><span class="w"> </span><span class="n"> `[3]`
</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="n">move_sensor</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="mi">10</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p>Теперь зададим себе вопрос: в какой области эта переменная доступна? Ответом  будет: область видимости переменной move_sensor - главная функция  *(строка 3)*
 <code>(main)</code>.</p>
<p>Стало чуть понятнее? Хорошо тогда движемся дальше.</p>

<h3 class="relative group">Вложенные скоупы
    <div id="вложенные-скоупы" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d0%bb%d0%be%d0%b6%d0%b5%d0%bd%d0%bd%d1%8b%d0%b5-%d1%81%d0%ba%d0%be%d1%83%d0%bf%d1%8b" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Что если мы хотим создать новый скоуп внутри функции <code>main</code>? Мы можем сделать это используя фигурные скобки:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">main</span><span class="p">(){</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="kd">let</span><span class="w"> </span><span class="n">move_sensor</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="mi">10</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">        </span><span class="n"> `[4]`
</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="p">}</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p>Мы создали новый блок - вложенная область видимости или вложенный контекст  *(строка 4)*
 если сказать иначе.</p>

<h3 class="relative group">Взаимодействие между скоупами
    <div id="взаимодействие-между-скоупами" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d0%b7%d0%b0%d0%b8%d0%bc%d0%be%d0%b4%d0%b5%d0%b9%d1%81%d1%82%d0%b2%d0%b8%d0%b5-%d0%bc%d0%b5%d0%b6%d0%b4%d1%83-%d1%81%d0%ba%d0%be%d1%83%d0%bf%d0%b0%d0%bc%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Как я уже упоминал в прошлой статье <a href="../rust-shadowing/" >про затенение</a>, внутренний скоуп может захватывать переменные из внешнего скоупа. Но в обратную сторону это не сработает. Вы не сможете объявить во внутреннем скоупе переменную и использовать её вне его границ.</p>
### 💻 Области видимости и скоупы в Rust


#### Шаг 1


```rust
// ?compile_fail
fn main() {
    {
        let light_sensor = 20;
        println!("Внутри блока (light): {}", light_sensor);
    }
    // ❌ Ошибка компиляции: light_sensor больше не существует!
    println!("Вне блока (light): {}", light_sensor);
}
```


**Выход за границы области видимости**
- Переменная `light_sensor` объявлена внутри вложенного блока `{}`.
- За границами этого блока переменная уничтожается (освобождается память).
- Обращение к ней вне блока приводит к ошибке компиляции E0425.


#### Шаг 2


```rust
fn main() {
    let move_sensor = 10;
    {
        // Внутренний скоуп видит внешние переменные
        println!("Внутри блока (move): {}", move_sensor);
        let inner_sensor = 5;
        println!("Сумма в блоке: {}", move_sensor + inner_sensor);
    }
    println!("Вне блока (move): {}", move_sensor);
}
```


**Доступ к внешнему контексту**
- Вложенный блок имеет доступ к переменным внешнего скоупа (`move_sensor`).
- В обратную сторону (вынести `inner_sensor` наружу) это правило не работает.


<p>При попытке использовать <code>light_sensor</code> за пределами фигурных скобок (как показано на Шаге 1) компилятор выдает ошибку <code>cannot find value 'light_sensor' in this scope</code>, потому что время жизни этой переменной закончилось на закрывающей фигурной скобке.</p>

<h2 class="relative group">Вывод
    <div id="вывод" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d1%8b%d0%b2%d0%be%d0%b4" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Вложенный скоуп можно представить как некий портал в который вы можете что-то принести, но вынести из него ничего нельзя. ✨</p>
<p>Важно запомнить, что:</p>
<ul>
<li>Переменные существуют внутри скоупа, в котором они объявлены.</li>
<li>Внутренний скоуп может захватывать переменные из внешнего скоупа.</li>
<li>Вы не можете использовать переменную вне ее скоупа.</li>
</ul>
<p>Теперь вы знаете как правильно &ldquo;готовить&rdquo; области видимости в Rust! 😉</p>
<hr>
### 📝 Тест на знание областей видимости (Scope) в Rust

**1. Что происходит со значением при выходе управляющей переменной из области видимости (scope)?**

- [ ] Вызывается макрос panic!()
- [x] Автоматически вызывается деструктор Drop и освобождается память
- [ ] Переменная сохраняется в глобальной памяти static
- [ ] Память очищает Garbage Collector

> 💡 *В Rust вызов Drop происходит строго детерминированно в момент закрывающей фигурной скобки `}`.*




