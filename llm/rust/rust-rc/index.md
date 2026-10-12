# Rc в Rust: Разделённое владение, циклические ссылки и Weak<T>

> Узнайте, как работает Rc в Rust: счётчик ссылок, разделяемое владение между структурами, связка с RefCell, проблема циклических ссылок (Rc cycles) и её решение через Weak<T>.

- **URL:** https://eagle2000.space/rust/rust-rc/
- **Date:** 2025-06-03

- **Tags:** rust, dev, rc, weak, refcell, middle

---


<h2 class="relative group">Введение
    <div id="введение" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d0%b2%d0%b5%d0%b4%d0%b5%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Представьте ситуацию: у вас есть данные в куче (<code>heap</code>), и вы хотите, чтобы несколько независимых объектов могли одновременно ими пользоваться. В обычных языках программирования ссылки раздаются направо и налево, но в Rust строгая система владения требует четкого понимания: кто является единственным владельцем ресурса?</p>
<p>Если владельцев должно быть несколько одновременно (в пределах одного потока), на сцену выходит умный указатель <strong><code>Rc&lt;T&gt;</code></strong> (Reference Counted).</p>
<hr>

<h2 class="relative group">1. Что такое Rc и как работает strong_count
    <div id="1-что-такое-rc-и-как-работает-strong_count" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#1-%d1%87%d1%82%d0%be-%d1%82%d0%b0%d0%ba%d0%be%d0%b5-rc-%d0%b8-%d0%ba%d0%b0%d0%ba-%d1%80%d0%b0%d0%b1%d0%be%d1%82%d0%b0%d0%b5%d1%82-strong_count" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p><code>Rc&lt;T&gt;</code> — это управляющий указатель с подсчетом ссылок для <strong>однопоточных</strong> сценариев.</p>
<p>Каждый раз, когда вы делаете <code>Rc::clone(&amp;a)</code>, вы не копируете сами данные внутри кучи. Вместо этого вы просто увеличиваете служебный счетчик сильных ссылок — <code>strong_count</code> — на 1. А когда очередная переменная выходит из области видимости, <code>drop()</code> уменьшает <code>strong_count</code> на 1.</p>
<p>Как только <code>strong_count</code> становится равен 0, память кучи автоматически очищается!</p>
<div class="admonition relative overflow-hidden rounded-lg border-l-4 my-3 px-4 py-3 shadow-sm" data-type="note">
      <div class="flex items-center gap-2 font-semibold text-inherit">
        <div class="flex shrink-0 h-5 w-5 items-center justify-center text-lg"><span class="relative block icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path fill="currentColor" d="M256 0C114.6 0 0 114.6 0 256s114.6 256 256 256s256-114.6 256-256S397.4 0 256 0zM256 128c17.67 0 32 14.33 32 32c0 17.67-14.33 32-32 32S224 177.7 224 160C224 142.3 238.3 128 256 128zM296 384h-80C202.8 384 192 373.3 192 360s10.75-24 24-24h16v-64H224c-13.25 0-24-10.75-24-24S210.8 224 224 224h32c13.25 0 24 10.75 24 24v88h16c13.25 0 24 10.75 24 24S309.3 384 296 384z"/></svg>
</span></div>
        <div class="grow">
          Примечание
        </div>
      </div><div class="admonition-content mt-3 text-base leading-relaxed text-inherit"><p>Почему бы не использовать обычную ссылку <code>&amp;T</code>?
<code>&amp;T</code> — это временное заимствование. Ссылка жестко ограничена временем жизни (<code>lifetimes</code>), её нельзя просто так сохранить в структуре без явной аннотации <code>'a</code>. <code>Rc&lt;T&gt;</code> решает эту проблему, давая независимое разделяемое владение.</p></div></div>
<h3 class="relative group">Пример разделения данных между структурами
    <div id="пример-разделения-данных-между-структурами" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%b8%d0%bc%d0%b5%d1%80-%d1%80%d0%b0%d0%b7%d0%b4%d0%b5%d0%bb%d0%b5%d0%bd%d0%b8%d1%8f-%d0%b4%d0%b0%d0%bd%d0%bd%d1%8b%d1%85-%d0%bc%d0%b5%d0%b6%d0%b4%d1%83-%d1%81%d1%82%d1%80%d1%83%d0%ba%d1%82%d1%83%d1%80%d0%b0%d0%bc%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Представьте, что у нас есть описание города <strong>City</strong>, которое одновременно используется в справочнике туриста и в отчете транспортной компании:</p>
### 💻 Пример c городами и историей


#### Шаг 1


```rust
// ?compile_fail
#[derive(Debug)]
struct City {
    name: String,
    history: String,
}

#[derive(Debug)]
struct CitySummary {
    headline: String,
    description: String,
}

fn main() {
    let history = String::from("Город основан в 1574 году");

    let city = City {
        name: String::from("Уфа"),
        history: history,
    };

    let summary = CitySummary {
        headline: String::from("Уфа"),
        description: history, // ❌ перемещено из-под city
    };

    println!("{:?}, {:?}", city, summary);
}
```


**1. Ошибка: попытка поделиться строкой напрямую между структурами**  
- Мы либо копируем строку, либо нарушаем правила владения, переместив значение. ⚠️ _**Данные не могут быть одновременно в двух структурах**_.


#### Шаг 2


```rust
use std::rc::Rc;

#[derive(Debug)]
struct City {
    name: Rc<String>,
    history: Rc<String>,
}

#[derive(Debug)]
struct CitySummary {
    headline: Rc<String>,
    description: Rc<String>,
}

fn main() {
    let name = Rc::new(String::from("Уфа"));
    let history = Rc::new(String::from("Основан в 1574 году"));

    let city = City {
        name: Rc::clone(&name),
        history: Rc::clone(&history),
    };

    let summary = CitySummary {
        headline: Rc::clone(&name),
        description: Rc::clone(&history),
    };

    println!("Город: {:?} Сводка: {:?}", city, summary);
    println!("Ссылок на описание: {}", Rc::strong_count(&history));
}
```


**2. Решение: использование Rc для разделения строк**
- С Rc<T> можно безопасно использовать одни и те же данные в нескольких структурах. Владение разделено, а данные освобождаются когда последний владелец исчезает.


<hr>

<h2 class="relative group">2. Связка Rc + RefCell для внутренней мутабельности
    <div id="2-связка-rc--refcell-для-внутренней-мутабельности" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#2-%d1%81%d0%b2%d1%8f%d0%b7%d0%ba%d0%b0-rc--refcell-%d0%b4%d0%bb%d1%8f-%d0%b2%d0%bd%d1%83%d1%82%d1%80%d0%b5%d0%bd%d0%bd%d0%b5%d0%b9-%d0%bc%d1%83%d1%82%d0%b0%d0%b1%d0%b5%d0%bb%d1%8c%d0%bd%d0%be%d1%81%d1%82%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Сам по себе <code>Rc&lt;T&gt;</code> дает только <strong>иммутабельный</strong> доступ к имеющимся данным. Если вы попробуете изменить значение внутри <code>Rc</code>, компилятор Rust выдаст ошибку.</p>
<p>Что делать, если несколько объектов должны не только читать, но и динамически менять общее состояние?
Ответ — обернуть данные в <strong><code>RefCell&lt;T&gt;</code></strong>, получив комбинацию <code>Rc&lt;RefCell&lt;T&gt;&gt;</code>.</p>

<h3 class="relative group">Ошибки и паника в рантайме
    <div id="ошибки-и-паника-в-рантайме" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%be%d1%88%d0%b8%d0%b1%d0%ba%d0%b8-%d0%b8-%d0%bf%d0%b0%d0%bd%d0%b8%d0%ba%d0%b0-%d0%b2-%d1%80%d0%b0%d0%bd%d1%82%d0%b0%d0%b9%d0%bc%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p><code>RefCell</code> переносит проверки правил заимствования (borrow checker) с этапа компиляции на этап выполнения программы (runtime).
Если вы случайно попытаетесь взять две изменяемые ссылки <code>borrow_mut()</code> одновременно на один и тот же <code>RefCell</code>, программа аварийно завершится с паникой!</p>
### 💻 Примеры работы с RefCell


#### Шаг 1


```rust
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let users = Rc::new(RefCell::new(vec!["Алиса".to_string()]));

    let session1 = Rc::clone(&users);
    let session2 = Rc::clone(&users);

    session1.borrow_mut().push("Макс".to_string());
    session2.borrow_mut().push("Петр".to_string());

    println!("Онлайн: {:?}", users.borrow());
}
```


**1. Общий список пользователей в чате**
- Мы моделируем общее состояние `users`, доступное из разных частей приложения. RefCell позволяет менять данные при множественных владельцах.


#### Шаг 2


```rust
// ?should_panic
#[allow(unused_variables)]
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let shared = Rc::new(RefCell::new(vec![1, 2, 3]));

    let _first = shared.borrow_mut();
    let _second = shared.borrow_mut(); // ❌ паника: уже есть активный borrow_mut
}
```


**2. Ошибка: двойной borrow_mut при вложенных вызовах**
- `RefCell` проверяет заимствование во время выполнения. Нельзя взять borrow_mut дважды одновременно.


#### Шаг 3


```rust
#[allow(unused_variables)]
use std::rc::Rc;
use std::cell::RefCell;
fn main() {
    let shared = Rc::new(RefCell::new(vec![1, 2, 3]));

    {
        let mut data = shared.borrow_mut();
        data.push(4);
    }

    let again = shared.borrow_mut();
    println!("{:?}", again);
}
```


**3. Исправление: ограничиваем область действия borrow**
- Вложите `borrow_mut` в отдельный блок — это освободит заимствование до следующего вызова. ✅


<hr>

<h2 class="relative group">3. Проблема циклических ссылок (Rc cycles)
    <div id="3-проблема-циклических-ссылок-rc-cycles" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#3-%d0%bf%d1%80%d0%be%d0%b1%d0%bb%d0%b5%d0%bc%d0%b0-%d1%86%d0%b8%d0%ba%d0%bb%d0%b8%d1%87%d0%b5%d1%81%d0%ba%d0%b8%d1%85-%d1%81%d1%81%d1%8b%d0%bb%d0%be%d0%ba-rc-cycles" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Хотя <code>Rc</code> в связке с <code>RefCell</code> дает огромную гибкость, здесь подстерегает опасная коварность: <strong>утечка памяти из-за циклических ссылок</strong>.</p>
<p>Если структура А содержит <code>Rc</code> узел на структуру Б, а структура Б содержит <code>Rc</code> узел обратно на структуру А, мы получаем замкнутый цикл:</p>
<ol>
<li>Узел А имеет <code>strong_count == 1</code> (из-за ссылки от Б).</li>
<li>Узел Б имеет <code>strong_count == 1</code> (из-за ссылки от А).</li>
<li>Когда внешние переменные <code>a</code> и <code>b</code> выходят из функции, счетчики у обоих объектов уменьшаются лишь до 1, но <strong>никогда не становятся равны 0</strong>!</li>
</ol>
<p>В результате деструктор <code>Drop</code> не вызовется ни у одного из узлов, и память в куче останется висеть «навсегда».</p>
<hr>

<h2 class="relative group">4. Решение через Weak<T> (downgrade и upgrade)
    <div id="4-решение-через-weak-downgrade-и-upgrade" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#4-%d1%80%d0%b5%d1%88%d0%b5%d0%bd%d0%b8%d0%b5-%d1%87%d0%b5%d1%80%d0%b5%d0%b7-weak-downgrade-%d0%b8-upgrade" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Чтобы разорвать замкнутый круг циклических ссылок, Rust предоставляет ослабленные ссылки — <strong><code>Weak&lt;T&gt;</code></strong>.</p>
<ul>
<li><strong>Сильная ссылка (<code>Rc&lt;T&gt;</code>)</strong> — вы выражаете прямое <em>владение</em> объектом. Объект существует до тех пор, пока <code>strong_count &gt; 0</code>.</li>
<li><strong>Слабая ссылка (<code>Weak&lt;T&gt;</code>)</strong> — вы выражаете <em>наблюдение</em> за объектом без владения. Создается вызовом <code>Rc::downgrade(&amp;rc)</code>.</li>
</ul>
<p><code>Weak&lt;T&gt;</code> увеличивает служебный счетчик <code>weak_count</code>, но <strong>не влияет на <code>strong_count</code></strong>. Это означает, что главный объект в куче спокойно очистится при обнулении <code>strong_count</code>, даже если на него до сих пор смотрят слабые ссылки!</p>

<h3 class="relative group">Как работать со ссылками Weak?
    <div id="как-работать-со-ссылками-weak" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%ba%d0%b0%d0%ba-%d1%80%d0%b0%d0%b1%d0%be%d1%82%d0%b0%d1%82%d1%8c-%d1%81%d0%be-%d1%81%d1%81%d1%8b%d0%bb%d0%ba%d0%b0%d0%bc%d0%b8-weak" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Так как объект, на который указывает <code>Weak&lt;T&gt;</code>, мог быть уже очищен в памяти, вы не можете разыменовать <code>Weak</code> напрямую. Для этого используется метод <strong><code>.upgrade()</code></strong>:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">weak_ref</span>: <span class="nc">Weak</span><span class="o">&lt;</span><span class="n">Gadget</span><span class="o">&gt;</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">Rc</span>::<span class="n">downgrade</span><span class="p">(</span><span class="o">&amp;</span><span class="n">gadget</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="c1">// Пытаемся временно превратить слабую ссылку обратно в сильную:
</span></span></span><span class="line"><span class="cl"><span class="k">if</span><span class="w"> </span><span class="kd">let</span><span class="w"> </span><span class="nb">Some</span><span class="p">(</span><span class="n">strong_ref</span><span class="p">)</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">weak_ref</span><span class="p">.</span><span class="n">upgrade</span><span class="p">()</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Объект еще жив: </span><span class="si">{}</span><span class="s">&#34;</span><span class="p">,</span><span class="w"> </span><span class="n">strong_ref</span><span class="p">.</span><span class="n">id</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span><span class="w"> </span><span class="k">else</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Объект уже деаллоцирован!&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<p>Давайте посмотрим на проблему циклов и их решение через <code>Weak</code> на интерактивных слайдах:</p>
### 💻 Циклы ссылок и безопасный Weak


#### Шаг 1


```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Default)]
struct Node {
    name: String,
    next: Option<Rc<RefCell<Node>>>,
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Деструктор узла '{}' вызван!", self.name);
    }
}

fn main() {
    println!("--- Демонстрация сильных ссылок ---");
    let first = Rc::new(RefCell::new(Node {
        name: "Узел А".to_string(),
        next: None,
    }));
    
    let second = Rc::new(RefCell::new(Node {
        name: "Узел Б".to_string(),
        next: None,
    }));

    println!("Сильные ссылки 'first': {}", Rc::strong_count(&first));
    println!("Сильные ссылки 'second': {}", Rc::strong_count(&second));
    
    // Связываем узел А -> узел Б
    first.borrow_mut().next = Some(Rc::clone(&second));
    println!("После связывания 'second' strong_count: {}", Rc::strong_count(&second));
}
```


**Подсчет сильных ссылок Rc**
- `Rc::strong_count(&a)` позволяет узнать количество владельцев объекта в куче.
- Каждый `Rc::clone` увеличивает `strong_count` на 1, а при выходе переменной из scope — уменьшает на 1.
- Если узел А владеет узлом Б через `Rc`, счетчик ссылок Б становится равен 2.


#### Шаг 2


```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Gadget {
    id: String,
    owner: Option<Weak<Owner>>, // Ослабленная ссылка на владельца предотвращает циклический вызов Drop!
}

struct Owner {
    name: String,
    gadgets: RefCell<Vec<Rc<RefCell<Gadget>>>>,
}

impl Drop for Gadget {
    fn drop(&mut self) {
        println!("Устройство '{}' удалено из памяти", self.id);
    }
}

impl Drop for Owner {
    fn drop(&mut self) {
        println!("Владелец '{}' удален из памяти", self.name);
    }
}

fn main() {
    println!("--- Безопасная связь через Weak ---");
    
    let owner = Rc::new(Owner {
        name: "Алексей".to_string(),
        gadgets: RefCell::new(Vec::new()),
    });

    let gadget = Rc::new(RefCell::new(Gadget {
        id: "Смартфон".to_string(),
        owner: Some(Rc::downgrade(&owner)), // Создаем ослабленную ссылку Weak<Owner>
    }));

    owner.gadgets.borrow_mut().push(Rc::clone(&gadget));

    println!("Сильные ссылки владельца: {}", Rc::strong_count(&owner));
    println!("Слабые ссылки владельца: {}", Rc::weak_count(&owner));

    // Для обращения к владельцу из устройства используем upgrade():
    if let Some(ref weak_owner) = gadget.borrow().owner {
        if let Some(upgraded_owner) = weak_owner.upgrade() {
            println!("Устройство принадлежит владельцу: {}", upgraded_owner.name);
        }
    }
}
```


**Решение циклов через Weak ссылки (downgrade и upgrade)**
- `Rc::downgrade(&rc)` создает слабую ссылку `Weak<T>`, увеличивая `weak_count`, но НЕ `strong_count`.
- `Weak<T>` не препятствует деаллокации объекта, когда все сильные ссылки выходят из области видимости.
- Для доступа к данным `Weak::upgrade()` возвращает `Option<Rc<T>>`. Если объект уже удален, вернется `None`.


<hr>

<h2 class="relative group">Где использовать Rc и Weak
    <div id="где-использовать-rc-и-weak" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b3%d0%b4%d0%b5-%d0%b8%d1%81%d0%bf%d0%be%d0%bb%d1%8c%d0%b7%d0%be%d0%b2%d0%b0%d1%82%d1%8c-rc-%d0%b8-weak" aria-label="Якорь">#</a>
    </span>
    
</h2>
<ul>
<li><strong>Деревья и Графы:</strong> Узлы хранят сильные ссылки <code>Rc</code> на дочерние элементы, а дочерние элементы хранят слабые ссылки <code>Weak</code> на родительский узел (Parent pointers).</li>
<li><strong>Кэширование:</strong> Кэш хранит <code>Weak</code> ссылки на объекты. Если объект больше никому в программе не нужен, он автоматически выгружается из памяти.</li>
<li><strong>Событийно-ориентированные системы:</strong> Подписчики на события хранят <code>Weak</code> ссылки на слушателей.</li>
</ul>
<hr>

<h2 class="relative group">Проверь свои знания!
    <div id="проверь-свои-знания" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%be%d0%b2%d0%b5%d1%80%d1%8c-%d1%81%d0%b2%d0%be%d0%b8-%d0%b7%d0%bd%d0%b0%d0%bd%d0%b8%d1%8f" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Пройдите короткий тест, чтобы закрепить знания по работе с <code>Rc</code>, <code>RefCell</code>, цикличным ссылкам и типажу <code>Weak&lt;T&gt;</code>.</p>
### 📝 Разделяемое владение и Weak

**1. Что происходит со счетчиками ссылок при вызове `Rc::downgrade(&rc_ptr)`?**

- [ ] strong_count увеличивается на 1, а weak_count не меняется
- [x] strong_count не меняется, а weak_count увеличивается на 1
- [ ] strong_count уменьшается на 1, а объект перемещается на стек

> 💡 *`Rc::downgrade` создает слабую ссылку `Weak<T>`, которая не дает владения. Увеличивается только `weak_count`, поэтому объект может быть деаллоцирован при обнулении `strong_count`.*


**2. К чему приводит создание взаимных сильных ссылок `Rc` между двумя объектами?**

- [ ] К ошибке компиляции borrow checker
- [x] К циклу ссылок (Rc cycle) и скрытой утечке памяти, так как strong_count обоих объектов никогда не станет равен 0
- [ ] К автоматической конвертации типа в Arc

> 💡 *Если два объекта содержат сильные `Rc` ссылки друг на друга, их `strong_count` остается равным минимум 1 даже после выхода их внешних переменных из scope, блокируя вызов `Drop`.*


**3. Как получить безопасный доступ к данным, находящимся под слабой ссылкой `Weak<T>`?**

- [ ] Разыменовать ее напрямую оператором `*weak_ref`
- [x] Вызвать метод `weak_ref.upgrade()`, который возвращает `Option<Rc<T>>`
- [ ] Принудительно вызвать метод `.unwrap()`

> 💡 *Так как объект под `Weak<T>` уже мог быть удален, единственный безопасный доступ выполняется через `.upgrade()`, который проверяет `strong_count > 0` и возвращает `Some(Rc<T>)` или `None`.*


**4. Когда освобождается память под объект `T`, обернутый в `Rc<T>`?**

- [ ] Только когда и strong_count, и weak_count одновременно станут равны 0
- [x] Как только `strong_count` становится равен 0 (даже если `weak_count` больше 0)
- [ ] При закрытии приложения

> 💡 *Память под само значение `T` освобождается сразу при `strong_count == 0`. Однако память под служебные счетчики освобождается только после обнуления `weak_count`.*



<hr>

  



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
    
    >Не забывайте, что <code>Rc</code> и <code>Weak</code> предназначены для однопоточного кода. ❗️
Если вам нужно делиться данными между несколькими параллельными потоками — для этого используется потокобезопасная пара <code>Arc&lt;T&gt;</code> и <strong><code>std::sync::Weak&lt;T&gt;</code></strong>, подробный разбор которой вы найдете в <a href="../rust-arc/" >отдельной статье про Arc</a>!</span>
</div>


