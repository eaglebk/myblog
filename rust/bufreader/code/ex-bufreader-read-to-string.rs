// ?hidden:start
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, Read, Write};
use std::path::Path;

fn ensure_file() -> io::Result<()> {
    if !Path::new("hello.txt").exists() {
        let mut f = OpenOptions::new().create(true).write(true).open("hello.txt")?;
        writeln!(f, "Знакомство с BufReader!")?;
    }
    Ok(())
}
// ?hidden:end
fn main() -> io::Result<()> {
    // Проверяем, что файл существует (создаём при необходимости)
    ensure_file()?;
    
    // Открываем файл
    let file = File::open("hello.txt")?;
    
    // Оборачиваем его в BufReader
    let mut reader = BufReader::new(file);

    // Читаем содержимое в строку
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    println!("Содержимое файла: {}", contents);

    Ok(())
}

// ! **Разберем подробнее:**
// ! 1. Функция `ensure_file` проверяет наличие файла `hello.txt`. 
// !    Если его нет — создаём и записываем туда строку "Знакомство с BufReader!".
// ! 2. В `main` открываем файл и оборачиваем его в `BufReader`. 
// !    Буферизованное чтение делает работу эффективной.
// ! 3. С помощью `read_to_string` считываем всё содержимое в строку.
// ! 4. Результат печатаем в консоль.
// ! 
// ! Это самый простой пример: мы просто читаем файл целиком.
// ! Но ключевая идея уже видна - `BufReader` используется как оболочка
// ! вокруг файла, которая делает операции ввода-вывода более эффективными.


// ---

use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufRead, Write};
use std::path::Path;


fn ensure_file() -> io::Result<()> {
    if !Path::new("app.log").exists() {
        let mut f = OpenOptions::new().create(true).write(true).open("app.log")?;
        writeln!(f, "[2025-08-27 10:00:01] INFO  — Приложение запущено")?;
        writeln!(f, "[2025-08-27 10:00:03] WARN  — Соединение нестабильно")?;
        writeln!(f, "[2025-08-27 10:00:05] ERROR — Ошибка чтения конфигурации")?;
        writeln!(f, "[2025-08-27 10:00:07] INFO  — Перезапуск сервиса")?;
    }
    Ok(())
}


fn main() -> io::Result<()> {
    ensure_file()?;

    let file = File::open("app.log")?;
    let reader = BufReader::new(file);

    let mut line_count = 0;
    let mut error_count = 0;

    for line in reader.lines() {
        let line = line?;
        line_count += 1;
        if line.contains("ERROR") {
            error_count += 1;
        }
        println!(">> {}", line);
    }

    println!("\nВ логе {} строки. Из них {} с ошибками.", line_count, error_count);

    Ok(())
}

// ! **Разберем подробнее что же тут происходит.**
// ! _Мы читаем лог-файл с помощью `BufReader`:_
// ! 1. Функция `ensure_file` имитирует создание файла с логом приложения. Если файла `app.log` нет — создаём его и записываем несколько строк с метками времени, уровнями логов (`INFO`, `WARN`, `ERROR`) и сообщениями.
// ! 2. В `main` открываем файл и оборачиваем его в `BufReader` чтобы читать эффективно.
// ! 3. С помощью `reader.lines()` проходим построчно в цикле:
// !    - считаем общее количество строк
// !    - ищем вхождения слова `"ERROR"` и увеличиваем отдельный счётчик ошибок
// !    - печатаем строку с префиксом `>>`
// ! 4. После цикла выводим статистику: сколько строк всего и сколько среди них сообщений об ошибках.
// ! 
// ! Такой приём реально полезен: BufReader позволяет работать даже с огромными логами построчно,
// ! без необходимости загружать их целиком в память.
