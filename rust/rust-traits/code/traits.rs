pub trait Say {
    fn say(&self) -> String;
}

struct Dog;

impl Say for Dog {
    fn say(&self) -> String {
        "Woof!".to_string()
    }
}

fn main() {
    let dog = Dog;
    println!("Собака говорит: {}", dog.say());
}

// ! **Объявление и реализация трейта**
// ! - Трейт `Say` описывает контракт: любой тип, реализующий его, должен уметь говорить.
// ! - Структура `Dog` реализует метод `say` из трейта `Say`.
// ! - После реализации метод становится доступен у структуры через точку.

// ---

pub trait HasId {
    const ID: usize;
    type Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> where Self: Sized;
}

struct MyStruct;

impl HasId for MyStruct {
    const ID: usize = 10;
    type Err = std::convert::Infallible;
    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(MyStruct)
    }
}

fn main() {
    println!("ID структуры: {}", MyStruct::ID);
    let _s = MyStruct::from_str("test").unwrap();
}

// ! **Ассоциированные элементы**
// ! - Константы (`const ID`) и типы (`type Err`) объявляются внутри трейта и уточняются в имплементациях.
// ! - Ассоциированный тип выступает в роли плейсхолдера типа, который конкретизируется при реализации.
// ! - Это избавляет от необходимости плодить дженерики во всех сигнатурах методов трейта.

// ---

// ?hidden:start
use std::fmt;
// ?hidden:end

// Мы не можем реализовать Display для Vec<i32> напрямую, так как оба типа чужие.
// Но мы можем обернуть Vec в наш собственный кортежный тип (Newtype)!
struct MyVec(Vec<i32>);

impl fmt::Display for MyVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Мой вектор с {} элементами", self.0.len())
    }
}

fn main() {
    let my_vec = MyVec(vec![100, 200, 300]);
    println!("{}", my_vec);
}

// ! **Паттерн Newtype (Обертка)**
// ! - Правило сиротства (Orphan Rules) запрещает реализовывать чужой трейт для чужого типа.
// ! - Создав структуру-обертку `MyVec`, мы делаем тип локальным.
// ! - Теперь мы можем реализовать для нее любой внешний трейт (например, `Display`).

// ---

// Трейт, который мы будем использовать как ограничение
trait Loud {
    fn loud_say(&self) -> String;
}

// Обобщенная структура
struct Point<T> {
    x: T,
    y: T,
}

// Blanket implementation: реализуем Loud для любого Point<T>, если T реализует Loud
impl<T: Loud> Loud for Point<T> {
    fn loud_say(&self) -> String {
        format!("X: {}, Y: {}", self.x.loud_say(), self.y.loud_say())
    }
}

struct Dog;
impl Loud for Dog {
    fn loud_say(&self) -> String {
        "WOOF!".to_string()
    }
}

fn main() {
    let p = Point { x: Dog, y: Dog };
    println!("Точка кричит: {}", p.loud_say());
}

// ! **Дженерики и Blanket-реализации**
// ! - Ограничения типа (Trait Bounds) `T: Loud` гарантируют, что тип `T` поддерживает нужные операции.
// ! - Blanket implementation позволяет реализовать трейт сразу для целого семейства типов, подходящих под условия.
// ! - Компилятор производит мономорфизацию обобщенного кода, генерируя отдельные функции под каждый тип.

// ---

// ?hidden:start
use std::mem::size_of;
// ?hidden:end

trait A {
    fn get(&self) -> &str;
}

trait B {
    fn get(&self) -> &str;
}

struct S;

impl A for S {
    fn get(&self) -> &str { "реализация A" }
}

impl B for S {
    fn get(&self) -> &str { "реализация B" }
}

fn main() {
    let s = S;
    // s.get(); // Ошибка компиляции: имя метода get неоднозначно!
    
    // Полный синтаксис (Fully Qualified Syntax) решает конфликт:
    let from_a = <S as A>::get(&s);
    let from_b = <S as B>::get(&s); 
    
    // Turbofish для явного указания типа функции:
    let float_size = size_of::<f64>();
    
    println!("A: {}, B: {}, размер f64: {}", from_a, from_b, float_size);
}

// ! **Turbofish и Полный синтаксис**
// ! - Turbofish `::<>` подсказывает типы компилятору там, где он не может вывести их автоматически.
// ! - Полностью квалифицированное имя `<S as A>::get` разрешает коллизии, когда один тип реализует несколько трейтов с одинаковыми именами методов.
