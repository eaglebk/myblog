# Переменные в Rust: объявление, связывание (binding) и область видимости

> Как объявлять переменные в Rust с помощью let, работать со скрытием (shadowing) и соблюдать соглашение snake_case. Понятные примеры для начинающих.

- **URL:** https://eagle2000.space/rust/rust-var/
- **Date:** 2025-05-03

- **Tags:** rust, dev

---

<p>Переменная в программировании представляет собой область памяти с присвоенным именем, предназначенную для хранения данных. В языке Rust объявление переменной осуществляется с помощью ключевого слова <code>let</code>, после которого указывается имя переменной.</p>

<h2 class="relative group">Именование переменных
    <div id="именование-переменных" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b8%d0%bc%d0%b5%d0%bd%d0%be%d0%b2%d0%b0%d0%bd%d0%b8%d0%b5-%d0%bf%d0%b5%d1%80%d0%b5%d0%bc%d0%b5%d0%bd%d0%bd%d1%8b%d1%85" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>В Rust принято использовать стиль <code>snake_case</code> для именования переменных. Это означает, что имя переменной пишется в нижнем регистре, а слова разделяются символом подчеркивания (_). Например, <code>my_variable</code>.</p>

<h2 class="relative group">Объявление и присвоение переменных
    <div id="объявление-и-присвоение-переменных" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%be%d0%b1%d1%8a%d1%8f%d0%b2%d0%bb%d0%b5%d0%bd%d0%b8%d0%b5-%d0%b8-%d0%bf%d1%80%d0%b8%d1%81%d0%b2%d0%be%d0%b5%d0%bd%d0%b8%d0%b5-%d0%bf%d0%b5%d1%80%d0%b5%d0%bc%d0%b5%d0%bd%d0%bd%d1%8b%d1%85" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>При объявлении переменной вы можете указать ее тип, например, <code>i32</code>, или доверить компилятору вывести тип самостоятельно на основе присвоенного значения.</p>
### 💻 Объявление и присвоение переменных в Rust


#### Шаг 1


```rust
fn main() {
    let sensors = 20;
    println!("Количество датчиков: {}", sensors);
}
```


**Объявление переменной**
- Ключевое слово `let` объявляет новую переменную.
- Компилятор автоматически выводит тип как `i32` на основе значения `20`.


#### Шаг 2


```rust
fn main() {
    let ultra = 10;
    let temp = ultra;
    println!("Значение temp: {}", temp);
}
```


**Присвоение значения другой переменной**
- Мы можем скопировать значение из одной переменной в другую.
- Теперь `temp` хранит копию значения `10`.


#### Шаг 3


```rust
fn main() {
    let ultra = 10;
    let temp = 5;
    let sensors = ultra + temp;
    println!("Сумма датчиков (sensors): {}", sensors);
}
```


**Присвоение результата выражения**
- Переменной можно присвоить результат математического выражения.
- Здесь `sensors` принимает результат сложения `10 + 5 = 15`.



  



<div
  
    class="flex px-4 py-3 rounded-md shadow bg-primary-100 dark:bg-primary-900"
  
  >
  <span
    
      class="text-primary-400 pe-3 flex items-center"
    
    >
    <span class="relative block icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path fill="currentColor" d="M256 32C114.6 32 .0272 125.1 .0272 240c0 49.63 21.35 94.98 56.97 130.7c-12.5 50.37-54.27 95.27-54.77 95.77c-2.25 2.25-2.875 5.734-1.5 8.734C1.979 478.2 4.75 480 8 480c66.25 0 115.1-31.76 140.6-51.39C181.2 440.9 217.6 448 256 448c141.4 0 255.1-93.13 255.1-208S397.4 32 256 32z"/></svg>
</span>
  </span>

  <span
    
      class="dark:text-neutral-300"
    
    ><strong>Интересный момент:</strong> Rust не любит, когда вы объявляете переменную, но не используете её. Go кстати тоже не любит.
Но в Rust можно перед названием переменной поставить _ и тогда компилятор проигнорирует такую переменную. Пример:
<code>let _ignore_me = 42; </code></span>
</div>


<h2 class="relative group">Изменяемость переменных
    <div id="изменяемость-переменных" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b8%d0%b7%d0%bc%d0%b5%d0%bd%d1%8f%d0%b5%d0%bc%d0%be%d1%81%d1%82%d1%8c-%d0%bf%d0%b5%d1%80%d0%b5%d0%bc%d0%b5%d0%bd%d0%bd%d1%8b%d1%85" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>По умолчанию переменные в Rust являются неизменяемыми (<code>immutable</code>). Это означает, что после объявления переменной ее значение не может быть изменено. Если вам необходимо изменить значение переменной, вы можете объявить ее как изменяемую (<code>mutable</code>), используя ключевое слово <code>mut</code>:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="k">mut</span><span class="w"> </span><span class="n">sensors</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="mi">20</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="n">sensors</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="mi">30</span><span class="p">;</span><span class="w"> </span><span class="c1">// Теперь sensors имеет значение 30
</span></span></span></code></pre></div></div>
<p>В этом примере мы объявляем <code>sensors</code> как изменяемую переменную и затем изменяем ее значение на <code>30</code>.</p>

