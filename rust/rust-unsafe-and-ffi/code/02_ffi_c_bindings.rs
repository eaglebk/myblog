use std::os::raw::c_char;

// Шаг 1: Объявление внешних C-функций через unsafe extern "C"
unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    // Вызов функции abs из системной библиотеки C (libc)
    let negative_number = -42;
    let absolute_val = unsafe { abs(negative_number) };

    println!("Результат работы C-функции abs({negative_number}): {absolute_val}");
}

// ! **1. Вызов C-функций через unsafe extern "C"**
// ! - Блок `unsafe extern "C"` объявляет имена и сигнатуры функций сторонней C-библиотеки.
// ! - Любой вызов внешней FFI-функции считается компилятором Rust небезопасным (`unsafe`), так как Rust не может проверить контракт C-кода.

// ---

// ?hidden:start
use std::ffi::CStr;
use std::os::raw::c_char;
// ?hidden:end

// Шаг 2: Безопасная работа с C-строками через CStr
fn main() {
    // Си-строка завершается нулевым байтом (null-terminated '\0')
    let c_string_bytes = b"Hello from C!\0";
    
    let c_str = unsafe {
        CStr::from_ptr(c_string_bytes.as_ptr() as *const c_char)
    };

    if let Ok(rust_str) = c_str.to_str() {
        println!("Успешно конвертировано в Rust-строку: '{rust_str}'");
    }
}

// ! **2. Конвертация строк при взаимодействии с C**
// ! - В C строки завершаются нулевым байтом `\0`, а в Rust хранят явную длину.
// ! - Типы `CStr` и `CString` из `std::ffi` служат безопасным мостом для передачи текстовых данных между Rust и C.
