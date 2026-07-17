fn call_with_five(f: fn(i32) -> i32) {
    println!("Вызов fn pointer: {}", f(5));
}

fn main() {
    // 1. Указатель на обычную функцию
    fn add_one(x: i32) -> i32 { x + 1 }
    call_with_five(add_one);
    
    // 2. Замыкание с захватом внешнего окружения
    let coefficient = 10;
    let closure = |x| x * coefficient;
    
    // call_with_five(closure); // Ошибка! Замыкание с состоянием не приводится к fn pointer.
    println!("Вызов замыкания: {}", closure(5));
}

// ! **Указатели на функции и замыкания**
// ! - Указатель на функцию `fn` является "тонким" указателем: он хранит только адрес машинного кода.
// ! - Замыкание (анонимная функция) может захватывать внешние переменные и нести с собой состояние.
// ! - Замыкания получают уникальный анонимный тип, генерируемый компилятором.

// ---

fn call_fn<F: Fn()>(f: F) { f(); }
fn call_fn_mut<F: FnMut()>(mut f: F) { f(); }
fn call_fn_once<F: FnOnce()>(f: F) { f(); }

fn main() {
    let mut greeting = "Hello".to_string();
    
    // Fn: только читает окружение
    let closure_fn = || println!("Fn: {}", greeting);
    call_fn(closure_fn);
    
    // FnMut: изменяет окружение
    let closure_mut = || {
        greeting.push_str(" World");
        println!("FnMut: {}", greeting);
    };
    call_fn_mut(closure_mut);
    
    // FnOnce: поглощает (перемещает) владение greeting
    let closure_once = move || {
        let consumed = greeting;
        println!("FnOnce: {}", consumed);
    };
    call_fn_once(closure_once);
    // call_fn_once(closure_once); // Ошибка! Переменная closure_once уже израсходована.
}

// ! **Трейты Fn, FnMut и FnOnce**
// ! - `Fn`: захват по разделяемой ссылке `&self`. Допускает вызовы в нескольких потоках параллельно.
// ! - `FnMut`: захват по эксклюзивной ссылке `&mut self`. Позволяет модифицировать состояние.
// ! - `FnOnce`: поглощает состояние. Может быть вызвано ровно один раз.

// ---

fn get_counter(mut start: u32) -> impl FnMut() -> u32 {
    move || {
        start += 1;
        start
    }
}

fn main() {
    let mut next = get_counter(100);
    println!("Счетчик: {}", next());
    println!("Счетчик: {}", next());
}

// ! **Возврат замыканий из функций**
// ! - Так как типы замыканий анонимны, мы используем синтаксис `impl Trait`.
// ! - Мы обязаны использовать ключевое слово `move`, чтобы передать владение захваченными переменными.
// ! - Иначе возвращаемое замыкание ссылалось бы на локальные переменные функции, которые уничтожаются при выходе.

// ---

// ?hidden:start
fn process_borrowed<F>(f: F)
where
    F: for<'a> Fn(&'a str),
{
    let local_string = String::from("Локальные данные");
    f(&local_string); // Ссылка живет только внутри функции
}
// ?hidden:end

fn main() {
    // Замыкание принимает ссылку на строку, время жизни которой определено внутри process_borrowed
    let print_len = |s: &str| println!("Длина строки: {}", s.len());
    process_borrowed(print_len);
}
