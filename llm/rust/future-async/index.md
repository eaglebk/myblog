# Rust async и Tokio: В ожидании будущего

> История экипажа Vectoria — наглядное объяснение синхронного и асинхронного кода, разницы с параллельностью, и практики Tokio: spawn, spawn_blocking, join!, join_all, JoinHandle, Handle, cooperative multitasking.

- **URL:** https://eagle2000.space/rust/future-async/
- **Date:** 2025-09-18

- **Tags:** rust, dev

---


<h2 class="relative group">Вступление
    <div id="вступление" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d1%81%d1%82%d1%83%d0%bf%d0%bb%d0%b5%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Иногда программа &ldquo;замирает&rdquo; в ожидании задач. Язык Rust, чтобы справиться с ними и избежать простоя, для нас приготовил три разные типа исполнения: <strong>синхронность</strong>, <strong>асинхронность</strong> и <strong>параллельность</strong>. Чтобы разобраться с ними, мы снова отправимся на борт <em>«Vectoria»</em>, где находчивый капитан Нова и робот RUST-Y учатся работать с <strong>Tokio</strong> - рантаймом, который умеет переключать тысячи лёгких задач без простоя.</p>

  



<div
  
    class="flex px-4 py-3 rounded-md shadow bg-primary-100 dark:bg-primary-900"
  
  >
  <span
    
      class="text-primary-400 pe-3 flex items-center"
    
    >
    <span class="relative block icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path fill="currentColor" d="M256 32C114.6 32 .0272 125.1 .0272 240c0 49.63 21.35 94.98 56.97 130.7c-12.5 50.37-54.27 95.27-54.77 95.77c-2.25 2.25-2.875 5.734-1.5 8.734C1.979 478.2 4.75 480 8 480c66.25 0 115.1-31.76 140.6-51.39C181.2 440.9 217.6 448 256 448c141.4 0 255.1-93.13 255.1-208S397.4 32 256 32z"/></svg>
</span>
  </span>

  <span
    
      class="dark:text-neutral-300"
    
    >В конце вас ждёт мини-квиз по Tokio и async: 18 вопросов (разбитые на группы по 8 вопросов) - от основ до практики с кодом по темам: <code>join!</code>, <code>join_all</code>, <code>spawn</code>, <code>JoinSet</code> и другим которые вы встретите в этой статье. Поэтому совет - читайте внимательно, это пригодится при прохождении квиза.</span>
</div>

<p>Готовы? Открываем новую главу.</p>
<hr>

<h2 class="relative group">Пролог. Врата
    <div id="пролог-врата" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%be%d0%bb%d0%be%d0%b3-%d0%b2%d1%80%d0%b0%d1%82%d0%b0" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Корабль <em>&ldquo;Vectoria&rdquo;</em> медленно дрейфовал у древних космических врат. Белёсые дуги уходили в пустоту, будто застывшие волны. На мостике царила напряжённая тишина.</p>
<p>RUST-Y, бортовой робот, переминался на месте: его сенсоры мигали тревожно.<br>
Наконец он собрался с духом и заговорил, голос дрогнул:</p>
<p>— &ldquo;Капитан&rdquo;, разрешите обратиться и задать вопрос… Я не понимаю разницы между <strong>синхронным</strong> и <strong>асинхронным</strong> кодом. И ещё - все упоминают <strong>Tokio</strong>. Что это? И чем же асинхронность отличается от параллельности?</p>
<p>Экипаж оживился. Такие вопросы всегда означали, что сейчас будет короткое введени в новую тему - и обычно за этим следовало приключение.</p>
<p>Капитан Нова активировал голографический проектор. В воздухе появились три сцены:</p>
<ul>
<li><strong>Синхронный вызов:</strong> команда корабля отправляет запрос на сервер и застывает в ожидании ответа. Экипаж ждёт, пока сервер думает и ответит.</li>
<li><strong>Асинхронный вызов:</strong> запрос ушёл — но команда продолжает жить своей жизнью: прокладывает курс, регулирует питание, сканирует окрестности. Когда сервер ответит, задача &ldquo;сообщит о готовности&rdquo;.</li>
<li><strong>Параллельность:</strong> несколько членов экипажа работают одновременно - Зори у навигации, Хекс у связи, Арчи в лазарете. Это потоки ОС: тяжёлые, у каждого свой стек и ресурсы.</li>
</ul>

<figure>
      <img class="my-0 rounded-md" src="Frame%2027.jpg" alt="Sync vs async" />
  
  
  </figure>

  



<div
  
    class="flex px-4 py-3 rounded-md shadow bg-primary-100 dark:bg-primary-900"
  
  >
  <span
    
      class="text-primary-400 pe-3 flex items-center"
    
    >
    <span class="relative block icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path fill="currentColor" d="M256 32C114.6 32 .0272 125.1 .0272 240c0 49.63 21.35 94.98 56.97 130.7c-12.5 50.37-54.27 95.27-54.77 95.77c-2.25 2.25-2.875 5.734-1.5 8.734C1.979 478.2 4.75 480 8 480c66.25 0 115.1-31.76 140.6-51.39C181.2 440.9 217.6 448 256 448c141.4 0 255.1-93.13 255.1-208S397.4 32 256 32z"/></svg>
</span>
  </span>

  <span
    
      class="dark:text-neutral-300"
    
    >Tokio — это диспетчер. Он управляет тысячами лёгких задач (<code>Future</code>) поверх ограниченного числа потоков ОС. Каждая задача знает, когда уступить выполнение (<code>.await</code>). Именно так программа остаётся всегда живой и готовой к выполнению работы.</span>
</div>

<hr>

<h2 class="relative group">Часть 1. Когда корабль уснул
    <div id="часть-1-когда-корабль-уснул" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%87%d0%b0%d1%81%d1%82%d1%8c-1-%d0%ba%d0%be%d0%b3%d0%b4%d0%b0-%d0%ba%d0%be%d1%80%d0%b0%d0%b1%d0%bb%d1%8c-%d1%83%d1%81%d0%bd%d1%83%d0%bb" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Вдруг неожиданно раздался синал тревоги и лампа вспыхнула на мостике: красный свет, гул сирен. Сенсоры дальнего сканирования перестали отвечать.
Зори в отчаянии ударил ладонью по панели:</p>
<p>— &ldquo;Капитан! Навигация замерла. Даже связь упала!&rdquo;</p>
<p>RUST-Y в панике вывел код на экран:</p>
### 💻 Ошибка: thread::sleep



```rust
// ?ignore
fn query_sensor() -> u32 { 42 }
fn process(d: u32) -> u32 { d }

use std::{thread, time::Duration};
fn calibrate() -> u32 {
    let d = query_sensor();
    thread::sleep(Duration::from_secs(5)); 
    process(d)
}

fn main() {
    println!("[МОСТИК] Запуск калибровки...");
    let data = calibrate();
    println!("[МОСТИК] Калибровка завершена: {data}");
}
```


Проблема:
- thread::sleep блокирует ОС-поток целиком.
- Если так "усыпить" воркер рантайма, другие async-задачи не выполнятся.


<p>Нова нахмурился:</p>
<p>— &ldquo;Ты заставил весь корабль спать вместе с датчиком. <code>thread::sleep</code> блокирует поток. Это словно дежурный задремал на вахте. Пока он спит — системы корабля парализованы&rdquo;.</p>
<p>Доктор Арчи добавил:</p>
<p>— &ldquo;Это худшая сторона <strong>синхронного кода</strong>. В боевых условиях такие паузы смертельно опасны&rdquo;.</p>
<hr>

<h2 class="relative group">Часть 2. Потоки и их цена
    <div id="часть-2-потоки-и-их-цена" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%87%d0%b0%d1%81%d1%82%d1%8c-2-%d0%bf%d0%be%d1%82%d0%be%d0%ba%d0%b8-%d0%b8-%d0%b8%d1%85-%d1%86%d0%b5%d0%bd%d0%b0" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Инженер Спаркс пододвинулся к консоли:</p>
<p>— &ldquo;Предлагаю вынести тяжёлую работу в отдельный поток. Пусть остальное живёт!&rdquo;</p>
<p>На голограмме вспыхнул новый фрагмент:</p>
### 💻 Решение: spawn_blocking



```rust
// ?ignore
use std::time::Duration;
use std::thread;
use tokio::task;

fn query_sensor() -> u32 { 42 }
fn process(d: u32) -> u32 { d }

fn calibrate_blocking() -> u32 {
    let d = query_sensor();
    thread::sleep(Duration::from_secs(5)); // блокируем, но в отдельном потоке
    process(d)
}

#[tokio::main]
async fn main() {
    println!("[МОСТИК] Запуск калибровки (spawn_blocking)...");
    let data = task::spawn_blocking(|| calibrate_blocking())
        .await
        .expect("join error");
    println!("[МОСТИК] Калибровка завершена: {data}");
}
```


Комментарий
- spawn_blocking уводит блокирующую работу в пул блокирующих потоков.
- Это спасает рантайм, но каждый поток — требует дополнительную память; используйте осторожно.


<p>Системы ожили, связь вернулась. Экипаж облегчённо выдохнул.</p>
<p>Но капитан Нова покачал головой:</p>
<p>— &ldquo;Каждый поток операционной системы забирает мегабайты памяти. Это временное средство. Мы не можем плодить их тысячами&rdquo;.</p>
<hr>

<h2 class="relative group">Часть 3. Искусство уступать
    <div id="часть-3-искусство-уступать" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%87%d0%b0%d1%81%d1%82%d1%8c-3-%d0%b8%d1%81%d0%ba%d1%83%d1%81%d1%81%d1%82%d0%b2%d0%be-%d1%83%d1%81%d1%82%d1%83%d0%bf%d0%b0%d1%82%d1%8c" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Через час <em>Vectoria</em> готовилась к гиперпрыжку. Сенсоры снова были перегружены.<br>
Зори докладывал:</p>
<p>— &ldquo;Если будем ждать по пять секунд синхронно, нас размажет о врата!&rdquo;</p>
<p>Капитан спокойно включил новую голограмму:</p>
### 💻 Асинхронная калибровка



```rust
use tokio::time::{sleep, Duration};

async fn query_sensor_async() -> u32 {
    sleep(Duration::from_millis(50)).await;
    return 42
}

async fn process_async(d: u32) -> u32 { d }

async fn calibrate_async() -> u32 {
    let d = query_sensor_async().await;
    sleep(Duration::from_secs(1)).await; // уступаем планировщику
    process_async(d).await
}

#[tokio::main]
async fn main() {
    println!("[МОСТИК] Запуск калибровки (async)...");
    let data = calibrate_async().await;
    println!("[МОСТИК] Калибровка завершена: {data}");
}
```


Комментарий:
Асинхронные точки ожидания .await — место где задача уступает поток выполнения рантайму (cooperative).


<p>— &ldquo;Вот это правильный путь. Мы ждём, но не блокируем. Сенсор думает, а корабль продолжает подготовку. Это и есть сила <strong>асинхронности</strong>&rdquo;.</p>
<hr>

<h2 class="relative group">Часть 4. Задачи и обещания
    <div id="часть-4-задачи-и-обещания" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%87%d0%b0%d1%81%d1%82%d1%8c-4-%d0%b7%d0%b0%d0%b4%d0%b0%d1%87%d0%b8-%d0%b8-%d0%be%d0%b1%d0%b5%d1%89%d0%b0%d0%bd%d0%b8%d1%8f" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>RUST-Y вновь задал вопрос:</p>
<p>— &ldquo;А если функция не async? Как вернуть результат?&rdquo;</p>
<p>Доктор Арчи поправил очки:</p>
<p>— &ldquo;Тогда мы возвращаем <code>JoinHandle</code>. Это обещание, что задача выполнится&rdquo;.</p>
### 💻 JoinHandle



```rust
use tokio::task::{self, JoinHandle};

fn not_an_async_function() -> JoinHandle<()> {
    task::spawn(async {
        println!("[ЭКИПАЖ] Второе сообщение из async-задачи");
    })
}

#[tokio::main]
async fn main() {
    println!("[МОСТИК] Первое сообщение");
    let _ = not_an_async_function().await;
}
```


Комментарий:
Возвращаем JoinHandle из sync-функции и ждём её выше (await).


<p>Нова добавил:</p>
<p>— &ldquo;Но помни: если не дождаться <code>.await</code>, результат уйдёт в пустоту&rdquo;.</p>
<p>Экипаж понял: <code>JoinHandle</code> - это не просто объект, а инструмент управления задачей.</p>
<hr>

<h2 class="relative group">Часть 5. Мост между мирами
    <div id="часть-5-мост-между-мирами" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%87%d0%b0%d1%81%d1%82%d1%8c-5-%d0%bc%d0%be%d1%81%d1%82-%d0%bc%d0%b5%d0%b6%d0%b4%d1%83-%d0%bc%d0%b8%d1%80%d0%b0%d0%bc%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Корабль принял сигнал бедствия. Нужно было пропустить сигнал через модуль диагностики прямо из <strong>потока операционной системы</strong>.</p>
<p>Спаркс нахмурился:</p>
<p>— &ldquo;Наш код асинхронный (async). Как его связать с обычным потоком?&rdquo;</p>
<p>Капитан показал:</p>
### 💻 Handle — мост между потоками



```rust
use tokio::runtime::Handle;

fn not_an_async_function(handle: Handle) {
    handle.block_on(async {
        println!("[ЭКИПАЖ] Второе сообщение через block_on");
    })
}

#[tokio::main]
async fn main() {
    println!("[МОСТИК] Первое сообщение");
    let handle = Handle::current();
    std::thread::spawn(move || {
        not_an_async_function(handle);
    })
    .join()
    .unwrap();
}
```


Мост между потоком операционной системы и Tokio — Handle::block_on.


<p>— &ldquo;<code>Handle</code> позволяет ОС-потоку вызвать асинхонную-задачу. Но помните: поток тяжёлый, используем только в исключительных случаях&rdquo;.</p>
<hr>

<h2 class="relative group">Часть 6. Блокировка в сердце шторма
    <div id="часть-6-блокировка-в-сердце-шторма" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%87%d0%b0%d1%81%d1%82%d1%8c-6-%d0%b1%d0%bb%d0%be%d0%ba%d0%b8%d1%80%d0%be%d0%b2%d0%ba%d0%b0-%d0%b2-%d1%81%d0%b5%d1%80%d0%b4%d1%86%d0%b5-%d1%88%d1%82%d0%be%d1%80%d0%bc%d0%b0" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Начинался космический шторм, медлить больше было нельзя.</p>
<p>Во время шторма у врат RUST-Y снова вставил <code>thread::sleep</code> в async-код.<br>
Навигация застыла, индикаторы мигнули красным.</p>
<p>— &ldquo;Опять корабль уснул!&rdquo; — закричал Зори.</p>
<p>Нова строго посмотрел на код:</p>
### 💻 Ошибка: sleep внутри async


#### Шаг 1


```rust
use std::{thread, time::Duration};

async fn bad_sleep() {
    // Плохо: блокирует воркер рантайма
    thread::sleep(Duration::from_millis(200));
    println!("Проснулся (но всё это время рантайм простаивал)");
}

#[tokio::main]
async fn main() {
    let t1 = bad_sleep();
    let t2 = async {
        for i in 0..5 {
            println!("Параллельная задача {i}");
            tokio::task::yield_now().await;
        }
    };
    tokio::join!(t1, t2);
}
```


Итог: мало шансов увидеть чередование выполнения — поток занят thread::sleep.


#### Шаг 2


```rust
use tokio::time::{sleep, Duration};

async fn good_sleep() {
    // Хорошо: уступаем планировщику
    sleep(Duration::from_millis(200)).await;
    println!("Проснулся (планировщик выполнял другие задачи)");
}

#[tokio::main]
async fn main() {
    let t1 = good_sleep();
    let t2 = async {
        for i in 0..5 {
            println!("Параллельная задача {i}");
            tokio::task::yield_now().await;
        }
    };
    tokio::join!(t1, t2);
}
```


В async-контексте только `tokio::sleep`. Всё остальное — блокировка для сотен задач.


<p>Он тут же внес исправление (шаг 2). 👆🏻</p>
<p>Свет тревоги погас, корабль вернулся в строй.</p>
<hr>

<h2 class="relative group">Часть 7. Научиться делиться временем выполнения
    <div id="часть-7-научиться-делиться-временем-выполнения" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%87%d0%b0%d1%81%d1%82%d1%8c-7-%d0%bd%d0%b0%d1%83%d1%87%d0%b8%d1%82%d1%8c%d1%81%d1%8f-%d0%b4%d0%b5%d0%bb%d0%b8%d1%82%d1%8c%d1%81%d1%8f-%d0%b2%d1%80%d0%b5%d0%bc%d0%b5%d0%bd%d0%b5%d0%bc-%d0%b2%d1%8b%d0%bf%d0%be%d0%bb%d0%bd%d0%b5%d0%bd%d0%b8%d1%8f" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Команда выполнила тренировочный пробный манёвр и навигация сожрала все ресурсы процессора. Связь и сенсоры замерли.</p>
<p>Нова объяснил:</p>
### 💻 yield_now — уступаем место



```rust
use tokio::task;

async fn looper(name: &str) {
    for i in 0..3 {
        println!("{name}-{i}");
        task::yield_now().await;
    }
}

#[tokio::main]
async fn main() {
    tokio::join!(looper("A"), looper("B"));
}
```


Кооперативность: yield_now явно отдаёт квант выполнения другим задачам.


<p>— &ldquo;В async Rust Tokio многозадачность кооперативная. Задачи должны уступать сами.<br>
Без <code>.await</code> или <code>yield_now().await</code> — планировщик бессилен&rdquo;.</p>
<hr>

<h2 class="relative group">Часть 8. Экипаж действует как единое целое
    <div id="часть-8-экипаж-действует-как-единое-целое" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%87%d0%b0%d1%81%d1%82%d1%8c-8-%d1%8d%d0%ba%d0%b8%d0%bf%d0%b0%d0%b6-%d0%b4%d0%b5%d0%b9%d1%81%d1%82%d0%b2%d1%83%d0%b5%d1%82-%d0%ba%d0%b0%d0%ba-%d0%b5%d0%b4%d0%b8%d0%bd%d0%be%d0%b5-%d1%86%d0%b5%d0%bb%d0%be%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Перед прыжком нужно было:</p>
<ul>
<li>проверить реактор,</li>
<li>перезапустить связь,</li>
<li>откалибровать навигацию.</li>
</ul>
<p>Капитан показал решение:</p>
### 💻 Несколько задач: join!



```rust
use tokio::time::{sleep, Duration};

async fn job(id: u32) -> u32 {
    sleep(Duration::from_millis(60 * id as u64)).await;
    println!("Задача {id} завершена");
    id * 10
}

#[tokio::main]
async fn main() {
    let (a, b) = tokio::join!(job(1), job(2));
    println!("Результаты: {a}, {b}");
}
```


Для фиксированного набора задач используйте tokio::join!


<p>— &ldquo;А если задач десятки, сотни?&rdquo; — спросил Расти.</p>
### 💻 Коллекция задач: join_all



```rust
use tokio::time::{sleep, Duration};
use futures::future::join_all;
async fn work(i: u32) -> u32 {
    sleep(Duration::from_millis(50 * i as u64)).await;
    i * 2
}

#[tokio::main]
async fn main() {
    let futs = vec![work(1), work(2), work(3)];
    let results = join_all(futs).await;
    println!("Результаты: {:?}", results); // [2,4,6]
}
```


Используем join_all из пакета futures
- Он ждёт коллекцию Future и возвращает Vec результатов.



  



<div
  
    class="flex px-4 py-3 rounded-md shadow bg-primary-100 dark:bg-primary-900"
  
  >
  <span
    
      class="text-primary-400 pe-3 flex items-center"
    
    >
    <span class="relative block icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path fill="currentColor" d="M256 32C114.6 32 .0272 125.1 .0272 240c0 49.63 21.35 94.98 56.97 130.7c-12.5 50.37-54.27 95.27-54.77 95.77c-2.25 2.25-2.875 5.734-1.5 8.734C1.979 478.2 4.75 480 8 480c66.25 0 115.1-31.76 140.6-51.39C181.2 440.9 217.6 448 256 448c141.4 0 255.1-93.13 255.1-208S397.4 32 256 32z"/></svg>
</span>
  </span>

  <span
    
      class="dark:text-neutral-300"
    
    ><ul>
<li><code>tokio::join!</code> — фиксированный набор задач → кортеж.</li>
<li><code>futures::future::join_all</code> — коллекция задач → <code>Vec</code> результатов.</li>
</ul>
</span>
</div>

<hr>

<h2 class="relative group">Часть 9. Ремонтные дроны и фоновая работа
    <div id="часть-9-ремонтные-дроны-и-фоновая-работа" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%87%d0%b0%d1%81%d1%82%d1%8c-9-%d1%80%d0%b5%d0%bc%d0%be%d0%bd%d1%82%d0%bd%d1%8b%d0%b5-%d0%b4%d1%80%d0%be%d0%bd%d1%8b-%d0%b8-%d1%84%d0%be%d0%bd%d0%be%d0%b2%d0%b0%d1%8f-%d1%80%d0%b0%d0%b1%d0%be%d1%82%d0%b0" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>— &ldquo;Запустить ремонтных дронов!&rdquo; — приказал капитан.</p>
<p>RUST-Y замялся:<br>
— &ldquo;Как запустить задачу в фоне?&rdquo;</p>
<p>Нова вывел на экран код:</p>
<p>Хекс уточнил:<br>
— &ldquo;А если дронов много?&rdquo;</p>
<p>Нова вывел другой код на на экран:</p>
### 💻 JoinSet: задачи по мере завершения



```rust
use tokio::task::JoinSet;
use tokio::time::{sleep, Duration};

async fn unit(i: u32) -> u32 {
    sleep(Duration::from_millis(30 * i as u64)).await;
    i * 100
}

#[tokio::main]
async fn main() {
    let mut set = JoinSet::new();
    for i in 1..=5 {
        set.spawn(unit(i));
    }

    while let Some(res) = set.join_next().await {
        match res {
            Ok(v) => println!("Готов блок: {v}"),
            Err(e) => eprintln!("Ошибка задачи: {e}"),
        }
    }
    println!("JoinSet завершил все задачи");
}
```


JoinSet позволяет получать результаты по мере готовности (в любом порядке).


<hr>

<h2 class="relative group">Эпилог. Ожидающее будущее
    <div id="эпилог-ожидающее-будущее" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%8d%d0%bf%d0%b8%d0%bb%d0%be%d0%b3-%d0%be%d0%b6%d0%b8%d0%b4%d0%b0%d1%8e%d1%89%d0%b5%d0%b5-%d0%b1%d1%83%d0%b4%d1%83%d1%89%d0%b5%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Шторм утих. Сенсоры работали, все подсистемы были в порядке, дроны вернулись.</p>
<p>RUST-Y стоял на мостике:</p>
<p>— &ldquo;Спасибо, капитан. Теперь я понял: ошибки — это ступени развития. Async и Future — это жизнь без блокировок&rdquo;.</p>
<p>Нова улыбнулся:</p>
<p>— &ldquo;Ничего, мы понемногу учимся жить в ритме вселенной. А теперь готовы к новым испытаниям&rdquo;.</p>
<p>Врата активировались и <em>Vectoria</em> умчалась в даль скозь сияние портала.</p>
<div id="typeit-f5bece48635ee5a2911bec204958f93c"></div>

<script>
    document.addEventListener("DOMContentLoaded", function () {
      var strings = ["  ","Путь в многопоточность только начинается…  ",""];
      
      new TypeIt("#typeit-f5bece48635ee5a2911bec204958f93c", {
        strings: strings,
        speed:  50 ,
        lifeLike:  false ,
        startDelay:  0 ,
        breakLines:  true ,
        waitUntilVisible:  true ,
        loop:  false 
      }).go();
    });
</script>

<hr>

<h2 class="relative group">Приложение: шпаргалка капитана по ключевым приёмам
    <div id="приложение-шпаргалка-капитана-по-ключевым-приёмам" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%b8%d0%bb%d0%be%d0%b6%d0%b5%d0%bd%d0%b8%d0%b5-%d1%88%d0%bf%d0%b0%d1%80%d0%b3%d0%b0%d0%bb%d0%ba%d0%b0-%d0%ba%d0%b0%d0%bf%d0%b8%d1%82%d0%b0%d0%bd%d0%b0-%d0%bf%d0%be-%d0%ba%d0%bb%d1%8e%d1%87%d0%b5%d0%b2%d1%8b%d0%bc-%d0%bf%d1%80%d0%b8%d1%91%d0%bc%d0%b0%d0%bc" aria-label="Якорь">#</a>
    </span>
    
</h2>

  



<div
  
    class="flex px-4 py-3 rounded-md shadow bg-primary-100 dark:bg-primary-900"
  
  >
  <span
    
      class="text-primary-400 pe-3 flex items-center"
    
    >
    <span class="relative block icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path fill="currentColor" d="M256 32C114.6 32 .0272 125.1 .0272 240c0 49.63 21.35 94.98 56.97 130.7c-12.5 50.37-54.27 95.27-54.77 95.77c-2.25 2.25-2.875 5.734-1.5 8.734C1.979 478.2 4.75 480 8 480c66.25 0 115.1-31.76 140.6-51.39C181.2 440.9 217.6 448 256 448c141.4 0 255.1-93.13 255.1-208S397.4 32 256 32z"/></svg>
</span>
  </span>

  <span
    
      class="dark:text-neutral-300"
    
    ><h3 class="relative group">Шпаргалка капитана по асинхронности:
    <div id="шпаргалка-капитана-по-асинхронности" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%88%d0%bf%d0%b0%d1%80%d0%b3%d0%b0%d0%bb%d0%ba%d0%b0-%d0%ba%d0%b0%d0%bf%d0%b8%d1%82%d0%b0%d0%bd%d0%b0-%d0%bf%d0%be-%d0%b0%d1%81%d0%b8%d0%bd%d1%85%d1%80%d0%be%d0%bd%d0%bd%d0%be%d1%81%d1%82%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Чтобы уверенно пройти сквозь врата async-программирования и не завалить финальный тест, держи под рукой этот краткий бортовой журнал:</p>
<ul>
<li><strong>Не блокируйте воркеры:</strong> Забудьте про <code>thread::sleep</code> в async-функциях - всегда заменяйте его на асинхронный <code>tokio::time::sleep</code>.</li>
<li><strong>Тяжёлое CPU / блокирующее I/O:</strong> Если код нельзя переписать под async (например, старые библиотеки), оборачивайте его в <code>tokio::task::spawn_blocking</code>.</li>
<li><strong>Старт из sync-кода:</strong> Чтобы построить мост из обычного синхронного потока ОС в мир Tokio, используйте <code>tokio::runtime::Handle::block_on</code>.</li>
<li><strong>Параллельное ожидание:</strong> Используйте макрос <code>tokio::join!</code> для фиксированного набора разных задач и <code>futures::future::join_all</code> для динамических коллекций (<code>Vec</code>).</li>
<li><strong>Фоновая работа:</strong> Для запуска независимых фоновых процессов используйте <code>tokio::spawn</code> (помните, что он требует передачи владения через <code>move</code> и времени жизни <code>'static</code>).</li>
<li><strong>Пачки динамических задач:</strong> Если задач много, они создаются в цикле и результаты нужно обрабатывать по мере их готовности — используйте <code>tokio::task::JoinSet</code> в связке с <code>.join_next().await</code>.</li>
<li><strong>Кооперативность:</strong> Помните, что планировщик Tokio не может прервать задачу насильно. Она должна уступать место сама в точках <code>.await</code>. Если у вас долгий вычислительный цикл без точек ожидания — вставляйте внутрь <code>tokio::task::yield_now().await</code>.</li>
</ul></span>
</div>

<hr>

<h2 class="relative group">Финальный квиз 🚀
    <div id="финальный-квиз-" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%84%d0%b8%d0%bd%d0%b0%d0%bb%d1%8c%d0%bd%d1%8b%d0%b9-%d0%ba%d0%b2%d0%b8%d0%b7-" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>🛰️</p>
### 📝 Rust Async & Tokio

**1. Что выведет следующий код?**

- [ ] Привет, экипаж
- [x] Начало потом Привет, экипаж
- [ ] Привет, экипаж потом Начало
- [ ] Начало только

> 💡 *println!("Начало") выполняется сразу; результат задачи печатается после await.*


**2. Что делает tokio::task::spawn?**

- [ ] Всегда создаёт новый поток ОС
- [x] Запускает async-задачу под Tokio, возвращает JoinHandle
- [ ] Останавливает текущий поток до завершения
- [ ] Блокирует рантайм

> 💡 *spawn планирует future на исполнение внутри рантайма и отдаёт JoinHandle.*


**3. Какой результат/порядок завершения вероятен?**

- [ ] Задача 1, потом 2, потом 3
- [x] Порядок зависит от задержек; job(1) вероятно завершится первой
- [ ] Все три одновременно
- [ ] Ошибка компиляции

> 💡 *Время ожидания разное; join! запускает параллельно в рамках рантайма.*


**4. Чем отличается futures::future::join_all от tokio::join! ?**

- [ ] join_all работает только с двумя задачами
- [x] join_all принимает коллекцию Future и ждёт их все
- [ ] join! всегда быстрее
- [ ] Разницы никакой

> 💡 *join_all удобен для Vec<Future>; join! — для фиксированного списка.*


**5. Что напечатает код со spawn и ожиданием всех JoinHandle?**

- [x] Готово 1, Готово 2, Готово 3, затем Все задачи завершены (порядок готово не гарантирован)
- [ ] Все задачи завершены напечатается раньше
- [ ] Только Готово 3
- [ ] Ошибка

> 💡 *Ждём каждый JoinHandle; финальная строка — после всех.*


**6. Что такое JoinSet в Tokio?**

- [ ] Массив синхронных функций
- [x] Коллекция async-задач со spawn и извлечением результатов через join_next/join_all
- [ ] Функция блокирующего сна
- [ ] Особый тип потока ОС

> 💡 *JoinSet позволяет добавлять задачи и получать результаты по мере завершения.*


**7. Почему thread::sleep опасен внутри async?**

- [ ] Ничем не опасен
- [x] Блокирует воркер рантайма; другие задачи не выполняются в этом потоке
- [ ] Даёт небольшую задержку, но без влияния
- [ ] Вызывает ошибку компиляции

> 💡 *thread::sleep блокирует весь поток ОС. В рантайме там могли работать десятки задач.*


**8. Что выведет код со spawn двух печатателей и join! по JoinHandle?**

- [x] A, B (в произвольном порядке), затем Завершили
- [ ] Завершили до A и B
- [ ] Только A
- [ ] Ошибка

> 💡 *Обе задачи выполняются; порядок A/B не гарантирован; затем печатается финальная строка.*


**9. Что означает требование 'static для spawn?**

- [ ] Задача живёт вечно
- [x] Future не должен ссылаться на не-статические данные: используйте move или владение
- [ ] Задача запускается только один раз
- [ ] Код становится синхронным

> 💡 *spawn требует 'static, чтобы рантайм не держал висячих ссылок.*


**10. Что выведет код с futures::join_all?**

- [x] [2,4,6]
- [ ] [6,4,2]
- [ ] [1,2,3]
- [ ] Ошибка

> 💡 *join_all возвращает Vec результатов в порядке входа.*


**11. Что напечатает этот код без await на JoinHandle?**

- [ ] Фоновая задача → через секунду; потом Сразу после spawn
- [x] Сразу после spawn напечатается сразу, фоновая задача — позже или может не успеть
- [ ] Только Фоновая задача
- [ ] Ошибка

> 💡 *Без await рантайм может завершить main раньше, чем фон успеет всё вывести.*


**12. Какой порядок вывода вероятен при worker_threads=1?**

- [x] Fast затем Slow
- [ ] Slow затем Fast
- [ ] Одновременно
- [ ] Нет гарантии

> 💡 *Обе задачи в одном воркере; меньшая задержка завершится раньше.*


**13. Что делает join_all (futures)?**

- [x] Ждёт завершения коллекции Future и возвращает Vec результатов
- [ ] Запускает задачи по очереди
- [ ] Делает код синхронным
- [ ] Работает только с async-std

> 💡 *Классический комбинирующий future.*


**14. Верное объяснение async vs parallel?**

- [ ] async — это потоки ОС; parallel — ожидание
- [x] async — кооперативная многозадачность; parallel — одновременное выполнение в нескольких потоках
- [ ] async всегда быстрее parallel
- [ ] Разницы нет

> 💡 *Асинхронность — про ожидания без простоя; параллельность — про одновременность.*


**15. Что напечатает цикл с yield_now?**

- [ ] A-0, A-1, A-2, затем B-0, B-1, B-2
- [x] Чередование A-0, B-0, A-1, B-1, ...
- [ ] Только A
- [ ] Только B

> 💡 *yield_now даёт шанс другой задаче — получаем чередование.*


**16. Что значит кооперативная многозадачность в Rust async?**

- [ ] Планировщик вытесняет задачу в любой момент
- [x] Задача уступает в .await или yield_now().await
- [ ] Всегда один поток
- [ ] Только для I/O

> 💡 *Без явного ожидания переключения не будет.*


**17. Как ведёт себя JoinSet::join_next()?**

- [ ] Возвращает в порядке launch
- [x] Возвращает по мере завершения задач (любой порядок)
- [ ] Ждёт только первую задачу
- [ ] Работает только с spawn_blocking

> 💡 *join_next извлекает следующий готовый результат.*


**18. Какое ключевое слово делает функцию асинхронной?**

- [ ] await
- [x] async
- [ ] spawn
- [ ] yield

> 💡 *async fn возвращает Future; внутри можно использовать .await.*




