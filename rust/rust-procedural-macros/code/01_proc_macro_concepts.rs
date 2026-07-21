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

// ! **1. Идея Custom Derive макросов**
// ! - Атрибут `#[derive(MyTrait)]` анализирует AST структуры или перечисления на этапе компиляции.
// ! - Он генерирует новую реализацию типажа (`impl MyTrait for Struct`), НЕ изменяя исходный код структуры.

// ---

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

// ! **2. Идея Attribute-like макросов**
// ! - Атрибутные макросы `#[my_attribute]` могут изменять саму функцию или структуру, к которой они применены.
// ! - Принимают аргументы атрибута и полностью переписывают синтаксическое дерево декорируемого элемента.
