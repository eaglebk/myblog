// ?hidden:start
use std::fs::{self, OpenOptions};
use std::io::{self, Write, Read};
use std::path::Path;
use std::fs::File;


/// Создаём файл big.txt, если его ещё нет.
/// 100_0 строк с текстом и номерами.
fn ensure_file() -> io::Result<()> {
    if Path::new("big.txt").exists() {
        return Ok(());
    }
    println!("Создается файл big.txt...");
    let mut f = OpenOptions::new().create(true).write(true).open("big.txt")?;
    for i in 0..100_0 {
        writeln!(f, "Строка номер {i:06} — пример содержимого файла для BufReader")?;
    }
    println!("Файл создан!");
    Ok(())
}
// ?hidden:end
fn main() -> io::Result<()> {
    ensure_file()?; // создаём файл

    let start = std::time::Instant::now();
    let mut file = File::open("big.txt")?;
    let mut buffer = [0; 1];
    let mut line = String::new();

    while file.read(&mut buffer)? > 0 {
        let ch = buffer[0] as char;
        if ch == '\n' {
            line.clear();
        } else {
            line.push(ch);
        }
    }
    let end = std::time::Instant::now();
    println!("Время выполнения: {:?}", end - start);

    Ok(())
}

// ! 1. **Небуферизованное чтение по одному символу**
// ! 
// ! ❌ Очень медленно {{< badge >}} ~123 миллисекунды {{< /badge >}}
// ! 
// ! Здесь каждая операция read идёт напрямую к ОС.
// ! Удобно для понимания, но не для реальной работы.

// ---
// ?hidden:start
use std::fs::{self, OpenOptions};
use std::io::{self, Write, BufRead, BufReader};
use std::path::Path;
use std::fs::File;

/// Создаём файл big.txt, если его ещё нет.
/// 100_0 строк с текстом и номерами.
fn ensure_file() -> io::Result<()> {
    if Path::new("big.txt").exists() {
        return Ok(());
    }
    println!("Создается файл big.txt...");
    let mut f = OpenOptions::new().create(true).write(true).open("big.txt")?;
    for i in 0..100_0 {
        writeln!(f, "Строка номер {i:06}")?;
    }
    println!("Файл создан!");
    Ok(())
}
// ?hidden:end
fn main() -> io::Result<()> {
    ensure_file()?; // создаём файл
    let start = std::time::Instant::now();
    let mut lines_count = 0;

    let file = File::open("big.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        lines_count += 1;
    }

    let end = std::time::Instant::now();
    println!("Время выполнения: {:?}", end - start);

    Ok(())
}

// ! 2. **BufReader + .lines()**
// ! 
// ! ⚡ Гораздо быстрее {{< badge >}} ~500µs {{< /badge >}}
// ! 
// ! Это стандартный способ, удобный и быстрый.


// ---
// ?hidden:start
use std::fs::{self, OpenOptions};
use std::io::{self, Write, BufRead, BufReader};
use std::path::Path;
use std::fs::File;



/// Создаём файл big.txt, если его ещё нет.
/// 100_0 строк с текстом и номерами.
fn ensure_file() -> io::Result<()> {
    if Path::new("big.txt").exists() {
        return Ok(());
    }
    println!("Создается файл big.txt...");
    let mut f = OpenOptions::new().create(true).write(true).open("big.txt")?;
    for i in 0..100_0 {
        writeln!(f, "Строка номер {i:06}")?;
    }
    println!("Файл создан!");
    Ok(())
}
// ?hidden:end
fn main() -> io::Result<()> {
    ensure_file()?; // создаём файл

    let start = std::time::Instant::now();
    let mut lines_count = 0;
    let file = File::open("big.txt")?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    while reader.read_line(&mut line)? > 0 {
        lines_count += 1;
        line.clear(); // строка очищается, но не выделяется заново
    }

    let end = std::time::Instant::now();
    println!("Время выполнения: {:?}", end - start);

    Ok(())
}

// ! 3. *BufReader + переиспользование строки*
// ! 
// ! ⚡ Ещё быстрее {{< badge >}} ~360µs {{< /badge >}}
// ! read_line записывает прямо в существующую строку.
// ! Нет лишних аллокаций памяти, работает эффективнее.

// ---
// ?hidden:start
use std::fs::{self, OpenOptions};
use std::io::{self, Write, Read};
use std::path::Path;
use std::fs::File;

/// Создаём файл big.txt, если его ещё нет.
/// 100_0 строк с текстом и номерами.
fn ensure_file() -> io::Result<()> {
    if Path::new("big.txt").exists() {
        return Ok(());
    }
    println!("Создается файл big.txt...");
    let mut f = OpenOptions::new().create(true).write(true).open("big.txt")?;
    for i in 0..100_0 {
        writeln!(f, "Строка номер {i:06}")?;
    }
    println!("Файл создан!");
    Ok(())
}
// ?hidden:end
fn main() -> io::Result<()> {

    ensure_file()?; // создаём файл
    let start = std::time::Instant::now();

    let mut lines_count = 0;
    let mut file = File::open("big.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    for line in contents.lines() {
        lines_count += 1;
    }

    let end = std::time::Instant::now();
    println!("Время выполнения: {:?}", end - start);

    Ok(())
}

// ! *4. Еще один вариант*
// ! 
// ! ⚡ {{< badge >}} ~270µs {{< /badge >}}
// ! Всё содержимое читается за раз, а потом разбивается на строки. 
// ! 
// ! Время выполнения у вас может отличаться от представленного здесь.
// ! 
// ! Минус: выделяется объём памяти равный размеру файла.