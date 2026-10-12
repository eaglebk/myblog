# Паттерн Строитель (Builder) в Rust: гибкое создание объектов

> Как применить паттерн проектирования Builder в языке Rust? Гайд по пошаговому конструированию сложных структур с валидацией полей и обработкой ошибок.

- **URL:** https://eagle2000.space/rust/rust-builder/
- **Date:** 2025-02-09

- **Tags:** rust, dev

---


<h2 class="relative group">Зачем нужны паттерны проектирования?
    <div id="зачем-нужны-паттерны-проектирования" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b7%d0%b0%d1%87%d0%b5%d0%bc-%d0%bd%d1%83%d0%b6%d0%bd%d1%8b-%d0%bf%d0%b0%d1%82%d1%82%d0%b5%d1%80%d0%bd%d1%8b-%d0%bf%d1%80%d0%be%d0%b5%d0%ba%d1%82%d0%b8%d1%80%d0%be%d0%b2%d0%b0%d0%bd%d0%b8%d1%8f" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Паттерны проектирования — это проверенные временем решения типичных задач, которые возникают при разработке программного обеспечения. Они не являются строгими правилами, а скорее рекомендациями, которые помогают структурировать код, делая его более читаемым, поддерживаемым и расширяемым.</p>
<p>Можно писать код и без паттернов, но вы скажете себе тысячу раз спасибо если научитесь их применять.
Паттерны помогают:</p>
<ul>
<li>Упростить поддержку кода: Чёткая структура делает код понятным для других разработчиков.</li>
<li>Ускорить разработку: Не нужно изобретать велосипед — можно использовать готовые решения.</li>
<li>Сделать код гибким: Паттерны позволяют легко вносить изменения, не ломая существующую логику.</li>
</ul>
<p>Один из самых известных источников по паттернам — книга &ldquo;Банды четырёх&rdquo; (<a href="https://ru.wikipedia.org/wiki/Design_Patterns"  target="_blank" rel="noreferrer">Design Patterns: Elements of Reusable Object-Oriented Software</a>). Однако когда я последний раз открывал её, примеры были приведены на C++ и Smalltalk. Согласитесь, что Smalltalk не самый актуальный язык на сегодня. Поэтому советую искать актуальные примеры на современных языках, таких как Rust, <a href="https://github.com/scottt2/design-patterns-in-dart/blob/master/builder/builder.dart"  target="_blank" rel="noreferrer">Dart</a>, <a href="https://github.com/faif/python-patterns/blob/master/patterns/creational/builder.py"  target="_blank" rel="noreferrer">Python</a> или <a href="https://github.com/tmrts/go-patterns/blob/master/creational/builder.md"  target="_blank" rel="noreferrer">Go</a>, чтобы лучше понимать, как применять паттерны в реальных проектах.</p>

<h2 class="relative group">Паттерн &ldquo;Строитель&rdquo; в Rust
    <div id="паттерн-строитель-в-rust" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d0%b0%d1%82%d1%82%d0%b5%d1%80%d0%bd-%d1%81%d1%82%d1%80%d0%be%d0%b8%d1%82%d0%b5%d0%bb%d1%8c-%d0%b2-rust" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Паттерн &ldquo;Строитель&rdquo; (Builder) — это порождающий шаблон проектирования, который позволяет пошагово создавать сложные объекты. Он особенно полезен, когда объект имеет множество параметров, часть из которых может быть необязательной. В Rust, где строгая типизация и безопасность памяти являются ключевыми принципами, этот паттерн помогает писать чистый и поддерживаемый код.</p>

<h2 class="relative group">Пример из жизни: заказ бургера
    <div id="пример-из-жизни-заказ-бургера" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%b8%d0%bc%d0%b5%d1%80-%d0%b8%d0%b7-%d0%b6%d0%b8%d0%b7%d0%bd%d0%b8-%d0%b7%d0%b0%d0%ba%d0%b0%d0%b7-%d0%b1%d1%83%d1%80%d0%b3%d0%b5%d1%80%d0%b0" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Представьте, что вы заказываете бургер в маке (или как говорят в профессиональных кругах &ldquo;макич&rdquo;). Вы можете выбрать:</p>
<ul>
<li>Булочку (обязательно)</li>
<li>Котлету (обязательно)</li>
<li>Сыр (опционально)</li>
<li>Салат (опционально)</li>
<li>Соусы (опционально)</li>
</ul>
<p>Паттерн &ldquo;Строитель&rdquo; идеально подходит для моделирования такого сценария.</p>
<hr>

<h2 class="relative group">Реализация на Rust
    <div id="реализация-на-rust" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%80%d0%b5%d0%b0%d0%bb%d0%b8%d0%b7%d0%b0%d1%86%d0%b8%d1%8f-%d0%bd%d0%b0-rust" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>В Rust паттерн &ldquo;Строитель&rdquo; можно реализовать без создания отдельной структуры для строителя. Вместо этого мы добавляем методы конфигурации прямо к основной структуре.</p>
### 💻 Паттерн Строитель в Rust



```rust
#[derive(Debug)]
struct Burger {
    bun: String,
    patty: String,
    cheese: Option<String>,
    salad: bool,
    sauce: Option<String>,
}

impl Burger {
    // Создаем новую реализацию с обязательными параметрами
    fn new(bun: String, patty: String) -> Self {
        Burger {
            bun,
            patty,
            cheese: None,
            salad: false,
            sauce: None,
        }
    }

    // Добавляем сыр
    fn add_cheese(mut self, cheese: String) -> Self {
        self.cheese = Some(cheese);
        self
    }

    // Добавляем салат
    fn add_salad(mut self) -> Self {
        self.salad = true;
        self
    }

    // Добавляем соус
    fn add_sauce(mut self, sauce: String) -> Self {
        self.sauce = Some(sauce);
        self
    }

    // Валидация и финализация объекта
    fn build(self) -> Result<Self, String> {
        if self.bun.is_empty() || self.patty.is_empty() {
            return Err("Булочка и котлета обязательны!".to_string());
        }
        Ok(self)
    }
}

fn main() {
    let burger = Burger::new("Булочка с кунжутом".to_string(), "Котлета из говядины".to_string())
        .add_cheese("Чеддер".to_string())
        .add_salad()
        .add_sauce("Барбекю".to_string())
        .build();

    match burger {
        Ok(burger) => println!("Ваш бургер: {:?}", burger),
        Err(e) => println!("Ошибка: {}", e),
    }
}
```


**Реализация паттерна "Строитель" (Builder)**
- Конструируем сложный объект `Burger` пошагово с помощью цепочки вызовов.
- Метод `build()` выполняет валидацию и возвращает `Result`.



<h3 class="relative group">Как это работает?
    <div id="как-это-работает" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%ba%d0%b0%d0%ba-%d1%8d%d1%82%d0%be-%d1%80%d0%b0%d0%b1%d0%be%d1%82%d0%b0%d0%b5%d1%82" aria-label="Якорь">#</a>
    </span>
    
</h3>
<ol>
<li>Методы конфигурации:
Каждый метод (add_cheese, add_salad, add_sauce) возвращает self, что позволяет использовать цепочку вызовов.</li>
<li>Валидация в build:
Метод build проверяет, что обязательные поля (bun и patty) заполнены. Если всё в порядке, он возвращает Ok(Self), иначе — ошибку.</li>
<li>Иммутабельность:
Каждый метод создает новый экземпляр структуры с обновленными значениями, что соответствует философии Rust.</li>
</ol>

<h2 class="relative group">Преимущества паттерна &ldquo;Строитель&rdquo;
    <div id="преимущества-паттерна-строитель" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%b5%d0%b8%d0%bc%d1%83%d1%89%d0%b5%d1%81%d1%82%d0%b2%d0%b0-%d0%bf%d0%b0%d1%82%d1%82%d0%b5%d1%80%d0%bd%d0%b0-%d1%81%d1%82%d1%80%d0%be%d0%b8%d1%82%d0%b5%d0%bb%d1%8c" aria-label="Якорь">#</a>
    </span>
    
</h2>
<ol>
<li>Гибкость:
Вы можете добавлять только те параметры, которые нужны, оставляя остальные по умолчанию.</li>
<li>Валидация:
Метод build гарантирует, что объект будет создан только с корректными данными.</li>
<li>Читаемость:
Цепочка вызовов методов делает код интуитивно понятным.</li>
<li>Подход похож на лего:
Мы конструируем объект из кубиков.</li>
</ol>
<hr>

<h2 class="relative group">Когда стоит использовать?
    <div id="когда-стоит-использовать" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%ba%d0%be%d0%b3%d0%b4%d0%b0-%d1%81%d1%82%d0%be%d0%b8%d1%82-%d0%b8%d1%81%d0%bf%d0%be%d0%bb%d1%8c%d0%b7%d0%be%d0%b2%d0%b0%d1%82%d1%8c" aria-label="Якорь">#</a>
    </span>
    
</h2>
<ul>
<li>Когда объект имеет много параметров, часть из которых необязательна.</li>
<li>Когда важно обеспечить валидацию данных перед созданием объекта.</li>
<li>Когда вы хотите сделать код гибким и легко расширяемым.</li>
</ul>
<hr>

<h2 class="relative group">Заключение
    <div id="заключение" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b7%d0%b0%d0%ba%d0%bb%d1%8e%d1%87%d0%b5%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Паттерн &ldquo;Строитель&rdquo; (Builder) — это ваш помощник в проектировании объектов. Конечно это не волшебная таблетка, это всего лишь ваш инструмент. В Rust его можно реализовать как с использованием отдельной структуры, так и без неё. Выбор зависит от сложности задачи и ваших предпочтений.</p>
<p>Паттерны проектирования, такие как &ldquo;Строитель&rdquo; (Builder), помогают писать код, который легче поддерживать и расширять. И хотя можно обойтись и без них, их использование делает разработку более предсказуемой и менее стрессовой. Попробуйте этот паттерн в своих проектах, чтобы убедиться в его эффективности! 🚀</p>

