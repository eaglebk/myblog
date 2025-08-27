use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufRead, Write};

fn ensure_file() -> io::Result<()> {
    if !std::path::Path::new("data.txt").exists() {
        let mut f = OpenOptions::new().create(true).write(true).open("data.txt")?;
        writeln!(f, "alpha")?;
        writeln!(f, "beta")?;
        writeln!(f, "gamma")?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    ensure_file()?;

    let file = File::open("data.txt")?;
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    while reader.read_line(&mut line)? > 0 {
        print!("{}", line);
        line.clear(); // очищаем строку, чтобы переиспользовать её
    }

    Ok(())
}
