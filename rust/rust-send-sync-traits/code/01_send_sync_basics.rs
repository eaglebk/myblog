use std::thread;

// Шаг 1: Автоматические маркерные типажи Send и Sync
struct SafeData {
    value: String,
}

// Структура SafeData автоматически реализует Send и Sync,
// так как поле String реализует Send и Sync.

fn main() {
    let data = SafeData { value: String::from("Привет из главного потока!") };

    // Ключевое слово move передаёт владение `data` внутрь замыкания потока.
    // Функция thread::spawn требует, чтобы замыкание и его данные реализовывали Send + 'static.
    let handle = thread::spawn(move || {
        println!("Данные внутри фонового потока: {}", data.value);
    });

    handle.join().unwrap();
}

// ! **1. Философия Send и Sync**
// ! - `Send` означает, что владение объектом типа `T` можно безопасно передать в другой поток.
// ! - `Sync` означает, что иммутабельные ссылки `&T` можно безопасно разделять между несколькими потоками одновременно (`T: Sync` ⟺ `&T: Send`).
// ! - Оба типажа являются маркерными (Auto Traits) и вычитаются компилятором автоматически.

// ---

// ?hidden:start
use std::thread;
// ?hidden:end

// Шаг 2: Ручная реализация Send / Sync для пользовательских оберток над сырыми указателями
struct MyRawWrapper<T> {
    ptr: *mut T,
}

// Сырые указатели (*mut T) по умолчанию НЕ реализуют Send и Sync.
// Если мы гарантируем потокобезопасность инвариантов вручную, мы используем unsafe impl:
unsafe impl<T: Send> Send for MyRawWrapper<T> {}
unsafe impl<T: Sync> Sync for MyRawWrapper<T> {}

fn main() {
    let mut val = 100i32;
    let wrapper = MyRawWrapper { ptr: &mut val as *mut i32 };

    let handle = thread::spawn(move || {
        println!("Сырой указатель успешно передан в фоновый поток через unsafe impl Send");
    });

    handle.join().unwrap();
}

// ! **2. Ручная реализация unsafe impl Send / Sync**
// ! - Сырые указатели `*const T` и `*mut T` считаются `!Send` и `!Sync`, так как компилятор не может проверить правила заимствований.
// ! - Обёртки над ними требуют явной инструкции `unsafe impl Send` только после тщательного аудита потокобезопасности.
