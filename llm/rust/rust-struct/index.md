# Создание и использование структур в Rust: Полное руководство

> Практическое руководство по структурам в Rust: классические структуры, кортежные структуры, структуры-маркеры, управление видимостью полей и синтаксис инициализации.

- **URL:** https://eagle2000.space/rust/rust-struct/
- **Date:** 2025-02-04

- **Tags:** rust, dev

---


<h2 class="relative group">Введение. Что такое структура
    <div id="введение-что-такое-структура" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d0%b2%d0%b5%d0%b4%d0%b5%d0%bd%d0%b8%d0%b5-%d1%87%d1%82%d0%be-%d1%82%d0%b0%d0%ba%d0%be%d0%b5-%d1%81%d1%82%d1%80%d1%83%d0%ba%d1%82%d1%83%d1%80%d0%b0" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Структура (или struct) в Rust — это пользовательский тип данных, который позволяет группировать связанные данные вместе. Структуры похожи на классы в других языках программирования, но в Rust (как говорит документация) они более гибкие и мощные, так как язык предоставляет различные возможности для работы с ними. Кто бы сомневался 😁 При этом видов структур существует несколько.</p>
<p>В этой главе мы подробно рассмотрим как классические C-структуры, так и структуры-кортежи, юнит-структуры, а также способы работы с ними.</p>

<h2 class="relative group">Шаг 1. Проектирование структуры
    <div id="шаг-1-проектирование-структуры" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%88%d0%b0%d0%b3-1-%d0%bf%d1%80%d0%be%d0%b5%d0%ba%d1%82%d0%b8%d1%80%d0%be%d0%b2%d0%b0%d0%bd%d0%b8%d0%b5-%d1%81%d1%82%d1%80%d1%83%d0%ba%d1%82%d1%83%d1%80%d1%8b" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Прежде чем писать код, важно спроектировать и описать, что мы хотим получить. Давайте возьмем нашу любимую тему по системам для управления умным домом. В нашей системе есть хаб (центральное устройство) и несколько сенсоров, которые подключены к этому хабу.</p>

<h3 class="relative group">Диаграмма классов
    <div id="диаграмма-классов" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b4%d0%b8%d0%b0%d0%b3%d1%80%d0%b0%d0%bc%d0%bc%d0%b0-%d0%ba%d0%bb%d0%b0%d1%81%d1%81%d0%be%d0%b2" aria-label="Якорь">#</a>
    </span>
    
</h3>
<div style="background-color:white; padding: 20px">
<pre class="not-prose mermaid">
classDiagram
    class HUB {
        +String name
        +String id
        +String location
        +Vec~Sensor~ sensors
    }

    class Sensor {
        +String name
        +String id
        +String type
        +f32 value
        +String unit
        +bool isActive
    }

    HUB "1" --> "n" Sensor : хаб хранит сенсоры
</pre>

</div>

<h3 class="relative group">Описание структур и полей:
    <div id="описание-структур-и-полей" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%be%d0%bf%d0%b8%d1%81%d0%b0%d0%bd%d0%b8%d0%b5-%d1%81%d1%82%d1%80%d1%83%d0%ba%d1%82%d1%83%d1%80-%d0%b8-%d0%bf%d0%be%d0%bb%d0%b5%d0%b9" aria-label="Якорь">#</a>
    </span>
    
</h3>
<ul>
<li><strong>Hub</strong>:
<ul>
<li><code>name: String</code> — имя хаба.</li>
<li><code>id: String</code> — уникальный идентификатор хаба.</li>
<li><code>location: String</code> — местоположение хаба.</li>
<li><code>sensors: Vec&lt;Sensor&gt;</code> — список сенсоров, подключенных к хабу.</li>
</ul>
</li>
<li><strong>Sensor</strong>:
<ul>
<li><code>name: String</code> — имя сенсора.</li>
<li><code>id: String</code> — уникальный идентификатор сенсора.</li>
<li><code>sensor_type: String</code> — тип сенсора (например, &ldquo;температура&rdquo;, &ldquo;влажность&rdquo; и т.д.).</li>
<li><code>value: f32</code> — текущее значение, измеренное сенсором.</li>
<li><code>unit: String</code> — единица измерения (например, &ldquo;°C&rdquo;, &ldquo;%&rdquo; и т.д.).</li>
<li><code>is_active: bool</code> — статус сенсора (активен/неактивен).</li>
</ul>
</li>
</ul>

<h2 class="relative group">Шаг 2. Разновидности структур и синтаксис инициализации
    <div id="шаг-2-разновидности-структур-и-синтаксис-инициализации" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%88%d0%b0%d0%b3-2-%d1%80%d0%b0%d0%b7%d0%bd%d0%be%d0%b2%d0%b8%d0%b4%d0%bd%d0%be%d1%81%d1%82%d0%b8-%d1%81%d1%82%d1%80%d1%83%d0%ba%d1%82%d1%83%d1%80-%d0%b8-%d1%81%d0%b8%d0%bd%d1%82%d0%b0%d0%ba%d1%81%d0%b8%d1%81-%d0%b8%d0%bd%d0%b8%d1%86%d0%b8%d0%b0%d0%bb%d0%b8%d0%b7%d0%b0%d1%86%d0%b8%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Rust поддерживает три основных вида структур:</p>
<ol>
<li><strong>Структуры с именованными полями</strong> (классические C-структуры).</li>
<li><strong>Структуры-кортежи</strong> (Tuple Structs).</li>
<li><strong>Юнит-структуры</strong> (Unit-like Structs).</li>
</ol>
<p>Давайте изучим все виды структур, сокращенную инициализацию, синтаксис перехода (<code>..</code>) и правила видимости полей на интерактивных слайдах ниже:</p>
### 💻 Структуры в Rust


#### Шаг 1


```rust
#[allow(dead_code)]
#[derive(Debug)]
struct Hub {
    name: String,
    id: String,
    location: String,
    sensors: Vec<Sensor>,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Sensor {
    name: String,
    id: String,
    sensor_type: String,
    value: f32,
    unit: String,
    is_active: bool,
}

fn main() {
    let hub = Hub {
        name: "Main Hub".to_string(),
        id: String::from("HUB001"),
        location: "Room 101".to_string(),
        sensors: Vec::new(), // пустой список сенсоров
    };

    println!("{:#?}", hub);
}
```


**Объявление и базовая инициализация структур**
- Структуры определяются с помощью ключевого слова `struct`.
- Атрибут `#[derive(Debug)]` позволяет выводить структуру в отладочном формате с помощью `{:#?}`.


#### Шаг 2


```rust
#[allow(dead_code)]
#[derive(Debug)]
struct Hub {
    name: String,
    id: String,
    location: String,
    sensors: Vec<Sensor>,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Sensor {
    name: String,
    id: String,
    sensor_type: String,
    value: f32,
    unit: String,
    is_active: bool,
}

fn main() {
    let sensor1 = Sensor {
        name: "Temperature Sensor".to_string(),
        id: "TEMP001".to_string(),
        sensor_type: "Temperature".to_string(),
        value: 25.3,
        unit: "°C".to_string(),
        is_active: true,
    };

    let sensor2 = Sensor {
        name: "Humidity Sensor".to_string(),
        id: "HUM001".to_string(),
        sensor_type: "Humidity".to_string(),
        value: 60.5,
        unit: "%".to_string(),
        is_active: true,
    };

    let hub = Hub {
        name: "Main Hub".to_string(),
        id: String::from("HUB001"),
        location: "Room 101".to_string(),
        sensors: vec![sensor1, sensor2],
    };

    println!("{:#?}", hub);
}
```


**Структуры со вложенными коллекциями**
- Мы создали экземпляры `Sensor` и передали их в поле `sensors` структуры `Hub`.
- Макрос `vec!` используется для удобной инициализации вектора `Vec<Sensor>`.


#### Шаг 3


```rust
#[derive(Debug, Default)]
struct Foo {
    a: u32,
    b: u8,
}

fn main() {
    let a = 42;
    let b = 13;

    // Сокращенный синтаксис инициализации (field init shorthand)
    // Имена переменных совпадают с именами полей
    let x = Foo { a, b };

    // Синтаксис перехода (struct update syntax ..)
    // Копирование (перенос) значений оставшихся полей из другой структуры
    let y = Foo { a: 50, ..x };

    // Инициализация структуры с дефолтными значениями, кроме поля a
    let z = Foo { a: 100, ..Default::default() };

    println!("x: {:?}", x);
    println!("y: {:?}", y);
    println!("z: {:?}", z);
}
```


**Сокращенный синтаксис и синтаксис перехода**
- Если имена переменных совпадают с именами полей, можно писать `a` вместо `a: a` (field init shorthand).
- Синтаксис `..` позволяет скопировать остальные поля из другого экземпляра той же структуры.
- Типаж `Default` позволяет инициализировать поля по умолчанию с помощью `..Default::default()`.


#### Шаг 4


```rust
// Объявление структуры-кортежа
#[derive(Debug)]
struct Bar(u32, bool);

fn main() {
    // Создание экземпляра
    let x = Bar(42, true);

    // Доступ к полям по порядковому индексу
    let number = x.0;
    let flag = x.1;

    println!("x: {:?}", x);
    println!("Поля: number = {}, flag = {}", number, flag);

    // Неявное представление конструктора структуры-кортежа как функции
    let constructor: fn(u32, bool) -> Bar = Bar;
    let y = constructor(100, false);
    println!("y: {:?}", y);
}
```


**Структуры-кортежи (Tuple Structs)**
- Поля в структурах-кортежах не имеют названий, а определяются только своими типами.
- Доступ к полям осуществляется по порядковому индексу через точку: `x.0`, `x.1`.
- Имя структуры-кортежа выступает в роли функции-конструктора (например, `Bar` имеет тип `fn(u32, bool) -> Bar`).


#### Шаг 5


```rust
// Объявление юнит-структуры
#[derive(Debug)]
struct Baz;

fn main() {
    // Корректное создание (без использования скобок)
    let x = Baz;

    println!("x: {:?}", x);
}
```


**Юнит-структуры (Unit-like Structs)**
- Объявляются вообще без использования скобок.
- Создаются также без использования скобок: `let x = Baz;`.
- Обычно используются как маркеры или для реализации типажей на них без хранения данных.


#### Шаг 6


```rust
// Структура с полями разного уровня видимости
#[allow(dead_code)]
pub struct VisibleStruct {
    pub a: u32,          // Публичное поле (доступно везде)
    pub(crate) b: u8,    // Доступно только внутри текущего крейта
    c: u64,              // Приватное поле (доступно только внутри текущего модуля)
}

fn main() {
    let x = VisibleStruct { a: 1, b: 2, c: 3 };
    println!("a = {}, b = {}", x.a, x.b);
    // Доступ к x.c разрешен, так как main находится в том же модуле
    println!("c = {}", x.c);
}
```


**Управление видимостью полей**
- Поля структур приватны по умолчанию.
- Модификатор `pub` делает поле видимым извне текущего модуля и крейта.
- Модификатор `pub(crate)` делает поле видимым только внутри текущего крейта.



<h3 class="relative group">Детали реализации и шаги:
    <div id="детали-реализации-и-шаги" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b4%d0%b5%d1%82%d0%b0%d0%bb%d0%b8-%d1%80%d0%b5%d0%b0%d0%bb%d0%b8%d0%b7%d0%b0%d1%86%d0%b8%d0%b8-%d0%b8-%d1%88%d0%b0%d0%b3%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h3>
<ol>
<li>
<p><strong>Шаг 1. Определение и базовая инициализация</strong>:</p>
<ul>
<li>Мы определили классические структуры с именованными полями <code>Hub</code> и <code>Sensor</code>.</li>
<li>Инициализировали экземпляр структуры <code>Hub</code> с пустым вектором <code>sensors</code> (<code>Vec::new()</code>).</li>
</ul>
</li>
<li>
<p><strong>Шаг 2. Структуры со вложенными коллекциями</strong>:</p>
<ul>
<li>Мы создали два сенсора и проинициализировали структуру <code>Hub</code>, передав сенсоры во вложенный массив с помощью макроса <code>vec!</code>.</li>
</ul>
</li>
<li>
<p><strong>Шаг 3. Сокращенный синтаксис и синтаксис перехода</strong>:</p>
<ul>
<li><strong>Field Init Shorthand:</strong> Если имена переменных совпадают с именами полей структуры, мы можем просто указать имя переменной (например, <code>let x = Foo { a, b };</code>).</li>
<li><strong>Struct Update Syntax:</strong> С помощью <code>..</code> мы можем скопировать или перенести значения оставшихся полей из другого экземпляра структуры (например, <code>let y = Foo { a: 50, ..x };</code>).</li>
</ul>
</li>
<li>
<p><strong>Шаг 4. Структуры-кортежи (Tuple Structs)</strong>:</p>
<ul>
<li>Именованный тип, поля которого не имеют названий, а определяются только типами в круглых скобках: <code>struct Bar(u32, bool);</code>.</li>
<li>Доступ к полям осуществляется по порядковому индексу: <code>x.0</code>, <code>x.1</code>.</li>
<li>Тип структуры-кортежа является функцией-конструктором: <code>let constructor: fn(u32, bool) -&gt; Bar = Bar;</code>.</li>
<li>Часто используется для паттерна <strong>Newtype</strong> (создание безопасной обертки вокруг одного типа).</li>
</ul>
</li>
<li>
<p><strong>Шаг 5. Юнит-структуры (Unit-like Structs)</strong>:</p>
<ul>
<li>Объявляются с помощью ключевого слова <code>struct</code> вообще без использования скобок: <code>struct Baz;</code>.</li>
<li>Создаются аналогично без скобок: <code>let x = Baz;</code>.</li>
</ul>
</li>
<li>
<p><strong>Шаг 6. Видимость и управление доступом к полям</strong>:</p>
<ul>
<li>По умолчанию все поля приватны (доступны только внутри текущего модуля).</li>
<li>Модификаторы видимости управляют доступом:
<ul>
<li><code>pub</code> — поле публично везде.</li>
<li><code>pub(crate)</code> — поле видимо только внутри текущего крейта.</li>
<li><code>pub(super)</code> — поле видимо только родительскому модулю.</li>
</ul>
</li>
</ul>
</li>
</ol>
<blockquote><p><strong>Примечание по видимости:</strong> Даже если поле структуры помечено как <code>pub</code>, сама структура должна быть доступна (публична) внешнему коду, чтобы поля можно было прочитать.</p>
</blockquote><hr>

<h2 class="relative group">Проверь свои знания!
    <div id="проверь-свои-знания" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%be%d0%b2%d0%b5%d1%80%d1%8c-%d1%81%d0%b2%d0%be%d0%b8-%d0%b7%d0%bd%d0%b0%d0%bd%d0%b8%d1%8f" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Пройдите короткий интерактивный тест, чтобы закрепить понимание структур в Rust.</p>
### 📝 Структуры в Rust

**1. Что из нижеперечисленного позволяет использовать сокращенную инициализацию полей (field init shorthand)?**

- [ ] Если все поля структуры имеют одинаковый тип
- [x] Если имена переменных совпадают с именами полей структуры
- [ ] Если структура является юнит-структурой
- [ ] Если все поля структуры помечены как публичные (pub)

> 💡 *Сокращенная инициализация доступна, когда имена локальных переменных в области видимости точно совпадают с названиями полей инициализируемой структуры (например, `Foo { a, b }` вместо `Foo { a: a, b: b }`).*


**2. Какое ключевое выражение используется для обновления полей структуры на основе другого экземпляра того же типа?**

- [x] ..instance
- [ ] copy instance
- [ ] extends instance
- [ ] using instance

> 💡 *Синтаксис перехода (struct update syntax) использует две точки: `..instance` в конце инициализации, чтобы скопировать/перенести значения оставшихся полей из другого экземпляра.*


**3. Как осуществляется доступ к полям в структуре-кортеже (Tuple Struct)?**

- [ ] По именам полей, заданным при объявлении
- [x] По порядковому индексу через точку (например, `x.0`, `x.1`)
- [ ] С помощью квадратных скобок (например, `x[0]`)
- [ ] Через вызов метода `.get(index)`

> 💡 *В структурах-кортежах поля не имеют названий. Доступ к ним осуществляется по порядковому номеру элемента (индексу) через точку, начиная с нуля.*


**4. Какое из утверждений верно для юнит-структур (Unit-like Structs)?**

- [ ] Они всегда занимают ровно 1 байт в памяти
- [x] Они объявляются и инициализируются вообще без использования скобок
- [ ] Для их инициализации обязательно вызывать `Baz()` с круглыми скобками
- [ ] Они не могут реализовывать никакие типажи (traits)

> 💡 *Юнит-структуры объявляются и создаются без каких-либо скобок: `struct Baz; let x = Baz;`. Они обычно служат маркерами и не хранят данные, занимая 0 байт в памяти.*


**5. Что делает модификатор видимости `pub(crate)` перед полем структуры?**

- [ ] Делает поле видимым только внутри текущего файла исходного кода
- [ ] Ограничивает видимость поля пределами текущего модуля
- [x] Делает поле видимым и доступным исключительно в пределах текущего крейта (crate)
- [ ] Разрешает изменять поле только из внешних пакетов

> 💡 *Модификатор `pub(crate)` делает поле публичным в пределах всего текущего крейта, но скрывает его от внешних зависимостей.*




