# BufReader в Rust: ускоряем ввод-вывод

> BufReader в Rust - подробный разбор. Как ускорить чтение файлов, работать с вводом-выводом, использовать BufRead и Read. Примеры построчного чтения и оптимизации IO.

- **URL:** https://eagle2000.space/rust/bufreader/
- **Date:** 2025-08-26

- **Tags:** rust, dev

---

<p>В этой статье разберём зачем нужен <code>BufReader</code>, как он работает и какие есть способы читать данные из файлов. Мы сравним разные подходы и посмотрим сколько они занимают времени.</p>
<blockquote><p>🎙️ К статье прилагается подкаст-версия - лёгкий и образный рассказ с аналогиями. Включайте, если хотите услышать объяснение на слух.
<figure >
    <audio controls preload="metadata">
      <source src="/audio/Rust_details_1.mp3" type="audio/mpeg">
    </audio>
    <figcaption>Подкаст Rust в деталях. Выпуск 1</figcaption>
</figure></p>
</blockquote>
<h2 class="relative group">Трейты и чтение данных
    <div id="трейты-и-чтение-данных" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%82%d1%80%d0%b5%d0%b9%d1%82%d1%8b-%d0%b8-%d1%87%d1%82%d0%b5%d0%bd%d0%b8%d0%b5-%d0%b4%d0%b0%d0%bd%d0%bd%d1%8b%d1%85" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>В Rust <strong>трейты</strong> описывают общее поведение. Если тип реализует трейт - он &ldquo;умеет&rdquo; делать определённые действия.</p>
<p>Например, трейт <code>Read</code> говорит: &ldquo;этот объект можно читать&rdquo;. Под него попадают файлы (<code>File</code>), сетевые соединения (<code>TcpStream</code>), стандартный ввод (<code>stdin</code>).</p>
<p>Главный метод:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">fn</span> <span class="nf">read</span><span class="p">(</span><span class="o">&amp;</span><span class="k">mut</span><span class="w"> </span><span class="bp">self</span><span class="p">,</span><span class="w"> </span><span class="n">buf</span>: <span class="kp">&amp;</span><span class="nc">mut</span><span class="w"> </span><span class="p">[</span><span class="kt">u8</span><span class="p">])</span><span class="w"> </span>-&gt; <span class="nb">Result</span><span class="o">&lt;</span><span class="kt">usize</span><span class="o">&gt;</span><span class="p">;</span></span></span></code></pre></div></div>
<p>Он записывает доступные байты в буфер и возвращает число реально прочитанных. Чтобы загрузить все данные - <code>.read()</code> вызывают многократно.</p>

<h2 class="relative group">Системные вызовы и их цена
    <div id="системные-вызовы-и-их-цена" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%81%d0%b8%d1%81%d1%82%d0%b5%d0%bc%d0%bd%d1%8b%d0%b5-%d0%b2%d1%8b%d0%b7%d0%be%d0%b2%d1%8b-%d0%b8-%d0%b8%d1%85-%d1%86%d0%b5%d0%bd%d0%b0" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Когда мы читаем файл или сокет через <code>Read</code>, каждый вызов <code>.read()</code> почти всегда приводит к системному вызову <code>read(2)</code> (на Linux/Unix) или его аналогу в других ОС.</p>
<p>⚙️ Разберём по шагам что же происходит когда в коде вызвается этот метод:</p>
<ol>
<li>Программа вызывает метод .read().</li>
<li>Процессор переключается из пользовательского режима <a href="https://ru.wikipedia.org/wiki/User-mode_Linux"  target="_blank" rel="noreferrer">(user mode)</a> в режим ядра <a href="https://ru.wikipedia.org/wiki/%D0%A0%D0%B5%D0%B6%D0%B8%D0%BC_%D1%8F%D0%B4%D1%80%D0%B0"  target="_blank" rel="noreferrer">(kernel mode)</a>.</li>
<li>ОС выполняет реальную операцию чтения: обращается к диску, сети или памяти.</li>
<li>Управление возвращается в пользовательское приложение.</li>
</ol>
<p>Каждый такой переход - это сотни инструкций процессора. Поэтому чтение по 1 байту с диска будет катастрофически медленным.</p>

<h2 class="relative group">BufReader
    <div id="bufreader" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#bufreader" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Чтобы уменьшить число системных вызовов существует <strong>BufReader</strong>.</p>
<p>Он один раз считывает крупный блок (по умолчанию 8 КБ) во внутренний буфер, а дальше выдаёт данные уже из памяти. Так вместо тысячи вызовов вы получаете десятки.</p>
### 💻 BufReader: чтение всего файла


#### Шаг 1


```rust
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
```


**Разберем подробнее:**
1. Функция `ensure_file` проверяет наличие файла `hello.txt`. 
Если его нет — создаём и записываем туда строку "Знакомство с BufReader!".
2. В `main` открываем файл и оборачиваем его в `BufReader`. 
Буферизованное чтение делает работу эффективной.
3. С помощью `read_to_string` считываем всё содержимое в строку.
4. Результат печатаем в консоль.
Это самый простой пример: мы просто читаем файл целиком.
Но ключевая идея уже видна - `BufReader` используется как оболочка
вокруг файла, которая делает операции ввода-вывода более эффективными.


#### Шаг 2


```rust
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
```


**Разберем подробнее что же тут происходит.**
_Мы читаем лог-файл с помощью `BufReader`:_
1. Функция `ensure_file` имитирует создание файла с логом приложения. Если файла `app.log` нет — создаём его и записываем несколько строк с метками времени, уровнями логов (`INFO`, `WARN`, `ERROR`) и сообщениями.
2. В `main` открываем файл и оборачиваем его в `BufReader` чтобы читать эффективно.
3. С помощью `reader.lines()` проходим построчно в цикле:
- считаем общее количество строк
- ищем вхождения слова `"ERROR"` и увеличиваем отдельный счётчик ошибок
- печатаем строку с префиксом `>>`
4. После цикла выводим статистику: сколько строк всего и сколько среди них сообщений об ошибках.
Такой приём реально полезен: BufReader позволяет работать даже с огромными логами построчно,
без необходимости загружать их целиком в память.



<h2 class="relative group">BufRead против Read
    <div id="bufread-против-read" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#bufread-%d0%bf%d1%80%d0%be%d1%82%d0%b8%d0%b2-read" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>BufReader реализует трейт <code>BufRead</code>, который расширяет возможности <code>Read</code>.</p>
<p>Зачем нужен отдельный трейт?
Потому что некоторые операции становятся эффективными только при наличии внутреннего буфера.</p>
<p>Например, метод <code>.lines</code>() возвращает итератор по строкам:</p>
### 💻 BufReader: построчное чтение



```rust
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
```


**Разберем подробнее:**
1. BufReader читает файл большими блоками и хранит их в своём буфере.
2. Трейт BufRead добавляет методы, которые работают не с файловым дескриптором напрямую, а с этим буфером в памяти.
3. Метод .lines() ищет символ перевода строки \n в буфере и возвращает готовые строки.
Если бы у нас был только Read, то для поиска конца строки пришлось бы каждый раз вызывать .read() → это означало бы больше системных вызовов и ненужных копирований.



<h3 class="relative group">Другие полезные методы BufRead:
    <div id="другие-полезные-методы-bufread" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b4%d1%80%d1%83%d0%b3%d0%b8%d0%b5-%d0%bf%d0%be%d0%bb%d0%b5%d0%b7%d0%bd%d1%8b%d0%b5-%d0%bc%d0%b5%d1%82%d0%be%d0%b4%d1%8b-bufread" aria-label="Якорь">#</a>
    </span>
    
</h3>
<ul>
<li><code>.read_line(&amp;mut String)</code> — читает одну строку целиком в переданную строку.</li>
<li><code>.split(u8)</code> — возвращает итератор, разделяющий поток по заданному байту.</li>
<li><code>.fill_buf()</code> — даёт прямой доступ к текущему содержимому буфера (без копирования).</li>
<li><code>.consume(n)</code> — указывает сколько байт &ldquo;съесть&rdquo; из буфера после обработки.</li>
</ul>

<h2 class="relative group">Четыре способа читать файл построчно
    <div id="четыре-способа-читать-файл-построчно" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%87%d0%b5%d1%82%d1%8b%d1%80%d0%b5-%d1%81%d0%bf%d0%be%d1%81%d0%be%d0%b1%d0%b0-%d1%87%d0%b8%d1%82%d0%b0%d1%82%d1%8c-%d1%84%d0%b0%d0%b9%d0%bb-%d0%bf%d0%be%d1%81%d1%82%d1%80%d0%be%d1%87%d0%bd%d0%be" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Чтобы увидеть разницу, сравним четыре подхода.</p>
### 💻 Примеры для 4 подходов


#### Шаг 1


```rust
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
```


1. **Небуферизованное чтение по одному символу**
❌ Очень медленно {{< badge >}} ~123 миллисекунды {{< /badge >}}
Здесь каждая операция read идёт напрямую к ОС.
Удобно для понимания, но не для реальной работы.


#### Шаг 2


```rust
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
```


2. **BufReader + .lines()**
⚡ Гораздо быстрее {{< badge >}} ~500µs {{< /badge >}}
Это стандартный способ, удобный и быстрый.


#### Шаг 3


```rust
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
```


3. *BufReader + переиспользование строки*
⚡ Ещё быстрее {{< badge >}} ~360µs {{< /badge >}}
read_line записывает прямо в существующую строку.
Нет лишних аллокаций памяти, работает эффективнее.


#### Шаг 4


```rust
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
```


*4. Еще один вариант*
⚡ {{< badge >}} ~270µs {{< /badge >}}
Всё содержимое читается за раз, а потом разбивается на строки. 
Время выполнения у вас может отличаться от представленного здесь.
Минус: выделяется объём памяти равный размеру файла.



<h2 class="relative group">Когда BufReader не нужен
    <div id="когда-bufreader-не-нужен" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%ba%d0%be%d0%b3%d0%b4%d0%b0-bufreader-%d0%bd%d0%b5-%d0%bd%d1%83%d0%b6%d0%b5%d0%bd" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Хотя <code>BufReader</code> полезен в большинстве сценариев, есть ситуации, когда он не только не ускорит программу, но и будет лишним.</p>

<h3 class="relative group">1. Данные уже в памяти
    <div id="1-данные-уже-в-памяти" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#1-%d0%b4%d0%b0%d0%bd%d0%bd%d1%8b%d0%b5-%d1%83%d0%b6%d0%b5-%d0%b2-%d0%bf%d0%b0%d0%bc%d1%8f%d1%82%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Если у вас есть <code>Vec&lt;u8&gt;</code>, <code>String</code> или срез <code>&amp;[u8]</code>, никакого дополнительного системного вызова не происходит. Данные уже находятся в оперативной памяти, а значит никакой выгоды от дополнительного буфера не будет.</p>

<h3 class="relative group">2. Чтение крупными блоками
    <div id="2-чтение-крупными-блоками" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#2-%d1%87%d1%82%d0%b5%d0%bd%d0%b8%d0%b5-%d0%ba%d1%80%d1%83%d0%bf%d0%bd%d1%8b%d0%bc%d0%b8-%d0%b1%d0%bb%d0%be%d0%ba%d0%b0%d0%bc%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Если вы сразу читаете данные большими кусками (десятки или сотни килобайт), то накладные расходы на системные вызовы становятся несущественными.</p>

<h3 class="relative group">3. Ограниченная память
    <div id="3-ограниченная-память" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#3-%d0%be%d0%b3%d1%80%d0%b0%d0%bd%d0%b8%d1%87%d0%b5%d0%bd%d0%bd%d0%b0%d1%8f-%d0%bf%d0%b0%d0%bc%d1%8f%d1%82%d1%8c" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Если у вас тысячи одновременных соединений (например, работаете с сервером), каждый <code>BufReader</code> создаёт внутренний буфер <code>(по умолчанию 8 КБ)</code>. При тысячах соединений это может означать десятки мегабайт памяти, которые не всегда оправданы.</p>
<p>В асинхронном коде часто используют другие подходы: читают ровно столько - сколько пришло в сокете и сами управляют буферами чтобы экономить память.</p>
<hr>

<h2 class="relative group">Вывод
    <div id="вывод" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d1%8b%d0%b2%d0%be%d0%b4" aria-label="Якорь">#</a>
    </span>
    
</h2>
<ul>
<li><code>Read</code> — низкоуровневый контракт для чтения потоков байтов.</li>
<li>Каждый вызов <code>.read()</code> - это системный вызов и он дорогой.</li>
<li><code>BufReader</code> сокращает число системных вызовов и делает чтение в разы быстрее.</li>
</ul>
<p>На практике BufReader с повторным использованием строки даёт лучший баланс между скоростью и памятью.</p>

