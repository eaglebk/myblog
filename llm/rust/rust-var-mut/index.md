# Изменяемые (mut) и неизменяемые переменные в Rust: иммутабельность по умолчанию

> Почему переменные в Rust неизменяемы по умолчанию и как работает ключевое слово mut. Разбираем гарантии безопасности памяти и приращение производительности.

- **URL:** https://eagle2000.space/rust/rust-var-mut/
- **Date:** 2025-05-06

- **Tags:** rust, dev

---


<h2 class="relative group">Введение
    <div id="введение" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d0%b2%d0%b5%d0%b4%d0%b5%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>В Rust все переменные по умолчанию иммутабельные (Immutable), то есть они не могут изменяться. Мутабельные переменные, напротив, имеют способность изменяться (Mutable).</p>

<h2 class="relative group">Обещание себе
    <div id="обещание-себе" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%be%d0%b1%d0%b5%d1%89%d0%b0%d0%bd%d0%b8%d0%b5-%d1%81%d0%b5%d0%b1%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Давайте рассмотрим пример. Предположим, вы решили вставать в 6 утра каждый день и ложиться в 22:00. Для этого вы создаете две переменные. Вы хотите вывести значения этих переменных в консоль, а затем изменить их.</p>
### 💻 Изменяемые и неизменяемые переменные в Rust


#### Шаг 1


```rust
// ?compile_fail
fn main() {
    let get_up = 6;
    let bedtime = 22;
    println!("Я планирую вставать в {} часов и ложиться спать в {}", get_up, bedtime);

    get_up = 7; // ❌ Ошибка компиляции: get_up неизменяема
    bedtime = 23;
}
```


**Попытка изменить неизменяемую переменную**
- В Rust переменные по умолчанию неизменяемы (immutable).
- Этот код приведёт к ошибке компиляции E0384.


#### Шаг 2


```rust
fn main() {
    let mut get_up = 6;
    let mut bedtime = 22;
    println!("Я планирую вставать в {} часов и ложиться спать в {}", get_up, bedtime);

    get_up = 7; // ✅ Корректно: переменные помечены как `mut`
    bedtime = 23;

    println!("Я планирую вставать в {} часов и ложиться спать в {}", get_up, bedtime);
}
```


**Использование ключевого слова mut**
- Чтобы сделать переменную изменяемой, добавьте `mut` после `let`.
- Теперь значения переменных можно безопасно переопределять.



<h2 class="relative group">Анализ ошибки
    <div id="анализ-ошибки" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b0%d0%bd%d0%b0%d0%bb%d0%b8%d0%b7-%d0%be%d1%88%d0%b8%d0%b1%d0%ba%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Компилятор сообщает, что вы пытаетесь присвоить новое значение неизменяемой переменной (<code>cannot assign twice to immutable variable</code>). В Rust переменные по умолчанию неизменяемые (иммутабельные).</p>

<h2 class="relative group">Исправление ошибки
    <div id="исправление-ошибки" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b8%d1%81%d0%bf%d1%80%d0%b0%d0%b2%d0%bb%d0%b5%d0%bd%d0%b8%d0%b5-%d0%be%d1%88%d0%b8%d0%b1%d0%ba%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Rust предлагает решение: сделать вашу переменную мутабельной, добавив ключевое слово <code>mut</code> после <code>let</code>. Как вы видите на втором шаге интерактивного примера выше, после добавления <code>mut</code> программа успешно выполняется и обновляет значения в соответствии с вашим новым графиком.</p>

<h2 class="relative group">Итоги
    <div id="итоги" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b8%d1%82%d0%be%d0%b3%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Rust — язык со строгими правилами. Он требует от разработчика определяться с тем, будут ли его переменные изменяемыми или нет. По умолчанию все переменные объявленные при помощи ключевого слова <code>let</code> являются иммутабельными (неизменяемыми). Чтобы сделать переменную изменяемой, нужно добавить ключевое слово <code>mut</code> после <code>let</code>. Это помогает писать более безопасный и предсказуемый код (создатели языка Rust нас именно в этом стараются убедить).</p>

