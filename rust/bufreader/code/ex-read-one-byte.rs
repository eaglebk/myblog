use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};

fn ensure_file() -> io::Result<()> {
    if !std::path::Path::new("data.txt").exists() {
        let mut f = OpenOptions::new().create(true).write(true).open("data.txt")?;
        for i in 0..100 {
            writeln!(f, "Строка {}", i)?;
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    ensure_file()?;

    let mut file = File::open("data.txt")?;
    let mut buffer = Vec::new();

    loop {
        let mut byte = [0u8; 1]; // читаем ровно один байт
        match file.read_exact(&mut byte) {
            Ok(_) => buffer.push(byte[0]),
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
            Err(e) => return Err(e),
        }
    }

    println!("Прочитано {} байт", buffer.len());
    Ok(())
}
