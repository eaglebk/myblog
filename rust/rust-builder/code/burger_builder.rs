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
// ! **Реализация паттерна "Строитель" (Builder)**
// ! - Конструируем сложный объект `Burger` пошагово с помощью цепочки вызовов.
// ! - Метод `build()` выполняет валидацию и возвращает `Result`.
