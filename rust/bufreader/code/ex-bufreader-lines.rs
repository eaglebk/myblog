use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufRead, Write};

// Создадим файл
fn ensure_file() -> io::Result<()> {
    if !std::path::Path::new("data.txt").exists() {
        let mut f = OpenOptions::new().create(true).write(true).open("data.txt")?;
        writeln!(f, "Первая строка")?;
        writeln!(f, "Вторая строка")?;
        writeln!(f, "Третья строка")?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    ensure_file()?;

    let file = File::open("data.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}

// ! **Разберем подробнее:**
// ! 1. BufReader читает файл большими блоками и хранит их в своём буфере.
// ! 2. Трейт BufRead добавляет методы, которые работают не с файловым дескриптором напрямую, а с этим буфером в памяти.
// ! 3. Метод .lines() ищет символ перевода строки \n в буфере и возвращает готовые строки.
// ! 
// ! Если бы у нас был только Read, то для поиска конца строки пришлось бы каждый раз вызывать .read() → это означало бы больше системных вызовов и ненужных копирований.
