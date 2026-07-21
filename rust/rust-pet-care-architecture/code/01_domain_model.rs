// Шаг 1: Доменная модель и полиморфизм через Trait Objects (dyn PetAction)
trait PetAction {
    fn name(&self) -> &str;
    fn perform_care(&self) -> String;
}

struct Cat {
    name: String,
}

impl PetAction for Cat {
    fn name(&self) -> &str { &self.name }
    fn perform_care(&self) -> String {
        format!("Кот {}: выдать порцию сухого корма и запустить автопоилку", self.name)
    }
}

struct Dog {
    name: String,
}

impl PetAction for Dog {
    fn name(&self) -> &str { &self.name }
    fn perform_care(&self) -> String {
        format!("Собака {}: выдать влажный корм и открыть умную дверцу в сад", self.name)
    }
}

fn main() {
    // Гетерогенная коллекция питомцев через Box<dyn PetAction>
    let pets: Vec<Box<dyn PetAction>> = vec![
        Box::new(Cat { name: "Барсик".to_string() }),
        Box::new(Dog { name: "Рекс".to_string() }),
    ];

    println!("--- Автоматический уход за питомцами ---");
    for pet in &pets {
        println!("{}", pet.perform_care());
    }
}

// ! **1. Доменная модель и Trait Objects**
// ! - Типаж `PetAction` описывает общий контракт поведения для всех питомцев.
// ! - Обертка `Box<dyn PetAction>` позволяет хранить разные типы животных в одном векторе.

// ---

// Шаг 2: Декларативный макрос регистратора питомцев pet_profile!
macro_rules! pet_profile {
    ( $name:expr, $species:expr, $weight:expr ) => {
        {
            format!("Карточка питомца: Кличка '{}' | Вид: {} | Вес: {} кг", $name, $species, $weight)
        }
    };
}

fn main() {
    let cat_info = pet_profile!("Люцифер", "Кот", 4.5);
    let parrot_info = pet_profile!("Кеша", "Попугай", 0.3);

    println!("{cat_info}");
    println!("{parrot_info}");
}

// ! **2. Макрос быстрого создания профилей питомца**
// ! - Макрос `pet_profile!` избавляет от дублирования кода при создании структуры метаданных.
