fn main() {
    let temperature = "25.5";
    let temperature: f64 = temperature.parse().unwrap();
    let mut temperature = temperature * 9.0 / 5.0 + 32.0; // переводим в Фаренгейт

    temperature = temperature + 1.0;
    println!("Температура: {}", temperature);
}
// ! **Последовательная трансформация данных**
// ! - Затенение позволяет повторно использовать имя переменной `temperature`.
// ! - Мы изменили тип данных со строки `&str` на число `f64`.
// ! - В конце мы сделали её изменяемой с помощью `let mut`.

// ---

fn main() {
    let brightness = "50";
    {
        let brightness: u8 = brightness.parse().unwrap();
        println!("Внутри блока: {}%", brightness);
        let brightness = brightness + 20;
        println!("Внутри блока (увеличено): {}%", brightness);
    }
    println!("Вне блока (оригинал): {}", brightness);
}
// ! **Затенение во вложенной области видимости (Scope)**
// ! - Внутри блока `{}` мы затеняем внешнюю переменную `brightness` новым типом и значением.
// ! - После выхода из блока временные переменные удаляются из стека.
// ! - Внешняя переменная восстанавливает своё оригинальное значение `"50"`.
