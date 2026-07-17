// ?compile_fail
fn main() {
    let integer: i32 = 42;
    let float: f64 = 3.14;
    
    // Ошибка! Типы не соответствуют друг другу.
    let result = integer + float; 
}

// ! **Строгая типизация: Неявное приведение**
// ! - В Rust отсутствует автоматическое (неявное) приведение типов.
// ! - Попытка сложить целое число `i32` и вещественное `f64` приведет к ошибке компилятора E0308.
// ! - Это предотвращает случайные потери точности в вычислениях.

// ---

fn main() {
    let integer: i32 = 42;
    let float: f64 = 3.14;
    
    let result = (integer as f64) + float;
    println!("Результат сложения: {}", result);
}

// ! **Строгая типизация: Явное приведение**
// ! - Для безопасного приведения типов используется оператор `as`.
// ! - Мы явно приводим `i32` к `f64` перед выполнением сложения.
// ! - Теперь компилятор уверен в наших намерениях, и код успешно выполняется.

// ---

// ?hidden:start
struct NewOrder {
    id: u64,
}

struct ShippedOrder {
    id: u64,
    tracking_number: String,
}

impl NewOrder {
    fn new(id: u64) -> Self {
        NewOrder { id }
    }
    
    fn ship(self, tracking: &str) -> ShippedOrder {
        ShippedOrder {
            id: self.id,
            tracking_number: tracking.to_string(),
        }
    }
}

impl ShippedOrder {
    fn deliver(self) {
        println!("Заказ {} успешно доставлен с трек-номером {}!", self.id, self.tracking_number);
    }
}
// ?hidden:end

fn main() {
    let order = NewOrder::new(1024);
    
    // 1. Мы не можем вызвать order.deliver() напрямую — компилятор выдаст ошибку!
    // 2. Сначала мы должны перевести заказ в состояние отправленного (ShippedOrder):
    let shipped = order.ship("RU-777-XYZ");
    
    // 3. Теперь мы можем безопасно вызвать доставку:
    shipped.deliver();
}

// ! **Type-Driven Development: Безопасное состояние**
// ! - Мы кодируем правила логики программы в типах `NewOrder` и `ShippedOrder`.
// ! - Метод `ship` определен только для `NewOrder`, возвращая `ShippedOrder`.
// ! - Метод `deliver` определен только для `ShippedOrder`.
// ! - Ошибиться невозможно: компилятор не позволит вызвать доставку для нового неотправленного заказа.
