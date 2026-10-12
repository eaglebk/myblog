# Процедурные макросы в Rust: derive, attribute-like и function-like

> Глубокое руководство по процедурным макросам в Rust: три вида макросов (derive, attribute-like, function-like), устройство TokenStream, работа с syn и quote.

- **URL:** https://eagle2000.space/rust/rust-procedural-macros/
- **Date:** 2026-08-09

- **Tags:** rust, dev, procedural-macros, proc-macro, syn, quote

---
## Введение

В отличие от декларативных макросов (`macro_rules!`), которые работают по принципу паттерн-матчинга токенов, **процедурные макросы (Procedural Macros)** в Rust являются полноценными функциями на языке Rust.

Процедурный макрос принимает на вход поток токенов **`proc_macro::TokenStream`**, выполняет произвольную логику трансформации кода на этапе компиляции и возвращает новый `TokenStream`, который компилятор встраивает в итоговую сборку.

Именно благодаря процедурным макросам функционируют популярнейшие библиотеки экосистемы: `serde` (`#[derive(Serialize, Deserialize)]`), `tokio` (`#[tokio::main]`), `actix-web` (`#[get("/api")]`) и `sqlx` (`sql!("SELECT...")`).

В этой статье мы подробно разберем три типа процедурных макросов, устройство `proc-macro` крейтов и роль связки библиотек `syn` и `quote`.

---

## 1. Три категории процедурных макросов

Стандарт языка Rust выделяет три разновидности процедурных макросов:

### 1. Пользовательский Derive (Custom Derive)
Вызывается через атрибут `#[derive(MyTrait)]` над структурами или перечислениями.
* **Назначение:** Автоматически генерирует реализацию типажа (`impl MyTrait for Struct`).
* **Особенность:** Не может изменять существующий код структуры, он лишь **добавляет** новый код рядом.

### 2. Атрибутные макросы (Attribute-like)
Создают пользовательские атрибуты вида `#[route(GET, "/users")]` или `#[tokio::main]`.
* **Назначение:** Применяются к функциям, структурам или модулям.
* **Особенность:** Принимают в качестве аргументов как свойства атрибута, так и декорируемый элемент, способны **полностью переписать** или заменить его.

### 3. Функциональные макросы (Function-like)
Вызываются как обычные макросы `my_macro!(...)`.
* **Назначение:** Принимают произвольный поток токенов внутри скобок.
* **Особенность:** Часто используются для валидации запросов к базе данных, регулярных выражений или парсинга сторонних форматов прямо во время компиляции.

Попробуйте запустить интерактивную эмуляцию работы разных типов макросов:


### 💻 Концепция Custom Derive и Атрибутов


#### Шаг 1

```rust
// Эмуляция поведения Custom Derive #[derive(Describe)]
trait Describe {
    fn describe(&self) -> String;
}

struct UserProfile {
    username: String,
    age: u32,
    is_active: bool,
}

// Процедурный макрос derive генерирует следующую реализацию типажа во время компиляции:
impl Describe for UserProfile {
    fn describe(&self) -> String {
        format!(
            "Структура UserProfile [username: {}, age: {}, is_active: {}]",
            self.username, self.age, self.is_active
        )
    }
}

fn main() {
    let user = UserProfile {
        username: "Alex".to_string(),
        age: 28,
        is_active: true,
    };

    println!("{}", user.describe());
}
```

**1. Идея Custom Derive макросов**
- Атрибут `#[derive(MyTrait)]` анализирует AST структуры или перечисления на этапе компиляции.
- Он генерирует новую реализацию типажа (`impl MyTrait for Struct`), НЕ изменяя исходный код структуры.


#### Шаг 2

```rust
// Эмуляция работы Attribute-like макроса #[route(GET, "/api/users")]
struct RouteHandler {
    method: &'static str,
    path: &'static str,
    handler_name: &'static str,
}

fn register_route(method: &'static str, path: &'static str, name: &'static str) -> RouteHandler {
    RouteHandler { method, path, handler_name: name }
}

fn main() {
    // В асинхронных фреймворках (Actix, Axum) атрибут #[get("/api/users")]
    // превращает функцию в объект обработчика с зарегистрированным роутом:
    let route = register_route("GET", "/api/users", "get_users_handler");
    
    println!("Зарегистрирован эндпоинт: {} {} -> {}", route.method, route.path, route.handler_name);
}
```

**2. Идея Attribute-like макросов**
- Атрибутные макросы `#[my_attribute]` могут изменять саму функцию или структуру, к которой они применены.
- Принимают аргументы атрибута и полностью переписывают синтаксическое дерево декорируемого элемента.


---

## 2. Анатомия Proc-Macro Крейта

В силу того, что процедурный макрос исполняется в процессе работы самого компилятора `rustc`, он подлежит абсолютно строгому архитектурному ограничению:

> [!IMPORTANT]
> Процедурные макросы должны быть вынесены в **отдельный Cargo-крейт**, в файле `Cargo.toml` которого явно прописан флаг `proc-macro = true`:
>
> ```toml
> [lib]
> proc-macro = true
> ```

Компилятор сначала собирает этот отдельный крейт в динамическую библиотеку (`.so` / `.dylib` / `.dll`), затем загружает её в память своего процесса и передает ей токены из вашего основного проекта.

---

## 3. Инструментарий: syn, quote и TokenStream

Ручной парсинг сырых токенов `proc_macro::TokenStream` — задача крайне сложная. Поэтому 99% процедурных макросов в мире Rust пишутся с использованием двух фундаментальных библиотек:

1. **`syn`** — библиотека-парсер. Она превращает сырые токены Rust в типизированное абстрактное синтаксическое дерево (**AST**).
   ```rust
   use syn::{parse_macro_input, DeriveInput};

   #[proc_macro_derive(MyTrait)]
   pub fn my_derive(input: TokenStream) -> TokenStream {
       let ast = parse_macro_input!(input as DeriveInput);
       // Теперь ast содержит struct_name, имена полей и их типы!
   }
   ```

2. **`quote`** — библиотека квази-цитирования. Она позволяет генерировать новый код Rust прямо в фигурных скобках с подстановкой переменных `#ast_node`:
   ```rust
   use quote::quote;

   let expanded = quote! {
       impl MyTrait for #name {
           fn hello() { println!("Привет от структуры!"); }
       }
   };
   expanded.into()
   ```

Изучите рабочий процесс разбора AST и генерации кода в интерактивных слайдах:


### 💻 Связка syn + quote и генерация кода


#### Шаг 1

```rust
// Эмуляция разбора AST через syn и генерации кода через quote
struct FieldInfo {
    name: &'static str,
    ty: &'static str,
}

struct ParsedStruct {
    struct_name: &'static str,
    fields: Vec<FieldInfo>,
}

fn simulate_syn_parse() -> ParsedStruct {
    // Библиотека syn превращает сырой TokenStream в типизированную AST-структуру DeriveInput
    ParsedStruct {
        struct_name: "Config",
        fields: vec![
            FieldInfo { name: "db_url", ty: "String" },
            FieldInfo { name: "max_connections", ty: "u32" },
        ],
    }
}

fn simulate_quote_generate(ast: &ParsedStruct) -> String {
    // Библиотека quote! { ... } генерирует конечный код на основе разобранного AST
    let mut code = format!("impl Default for {} {{\n  fn default() -> Self {{\n    Self {{\n", ast.struct_name);
    for field in &ast.fields {
        code.push_str(&format!("      {}: Default::default(),\n", field.name));
    }
    code.push_str("    }\n  }\n}");
    code
}

fn main() {
    let ast = simulate_syn_parse();
    let generated_code = simulate_quote_generate(&ast);

    println!("--- Сгенерированный код через syn & quote ---");
    println!("{generated_code}");
}
```

**1. Связка syn и quote**
- `syn::parse::<DeriveInput>(tokens)` парсит исходные лексемы кода в выверенное AST-дерево.
- `quote! { ... }` генерирует готовые токены Rust для вставки в итоговую бинарную сборку.


#### Шаг 2

```rust
// Эмуляция работы Function-like макроса sql_query!("SELECT * FROM users WHERE id = $1")
struct SqlQuery {
    raw_query: &'static str,
    params_count: usize,
}

impl SqlQuery {
    fn new(query: &'static str) -> Self {
        let count = query.matches('$').count();
        SqlQuery { raw_query: query, params_count: count }
    }
}

fn main() {
    // В SQL-библиотеках (например, sqlx!) процедурный макрос sql!("SELECT...")
    // проверяет валидность синтаксиса базы данных на этапе КОМПИЛЯЦИИ:
    let q = SqlQuery::new("SELECT name, email FROM users WHERE id = $1 AND active = $2");

    println!("Проверенный запрос: '{}'", q.raw_query);
    println!("Ожидается параметров на вход: {}", q.params_count);
}
```

**2. Идея Function-like макросов**
- Вызываются как `my_macro!(...)` и принимают произвольный поток токенов в скобках.
- Способны выполнять статическую валидацию запросов к БД, регулярных выражений или конфигураций прямо при сборке.


---

## Проверь свои знания!

Пройдите короткий интерактивный тест по процедурным макросам:


### 📝 Процедурные макросы в Rust


**1. В чем главное фундаментальное отличие процедурных макросов от декларативных (`macro_rules!`)?**

- [ ] Процедурные макросы — это функции на Rust, которые принимают TokenStream и возвращают модифицированный TokenStream при компиляции
- [ ] Процедурные макросы работают во время выполнения программы (runtime)
- [ ] Процедурные макросы используют исключительно текстовую замену C-препроцессора

**2. Почему процедурные макросы необходимо выносить в отдельный Cargo-крейт с параметром `proc-macro = true`?**

- [ ] Компилятору rustc необходимо сначала скомпилировать макрос в динамическую библиотеку и запустить ее для сборки основного проекта
- [ ] Чтобы код макроса не попал в итоговый бинарный файл пользователя
- [ ] Для возможности вызова макросов из языков C и Python

**3. Какую роль выполняет библиотека `syn` при создании процедурных макросов?**

- [ ] Разбирает сырой поток токенов TokenStream в удобное типизированное синтаксическое дерево (AST)
- [ ] Отвечает за генерацию ассемблерного кода
- [ ] Синхронизирует потоки выполнения

**4. Для чего используется макрос `quote! { ... }` из библиотеки `quote`?**

- [ ] Для удобной квази-цитатной генерации нового кода и превращения элементов AST обратно в TokenStream
- [ ] Для экранирования кавычек в строках
- [ ] Для замера времени компиляции

