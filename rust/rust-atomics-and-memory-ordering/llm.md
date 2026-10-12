# Атомарные операции и Memory Ordering в Rust: от Ordering::Relaxed до Acquire/Release барьеров

> Глубокое погружение в атомарные типы std::sync::atomic (AtomicBool, AtomicUsize, AtomicPtr), операции fetch_add, compare_exchange и барьеры памяти Memory Ordering (Relaxed, Acquire, Release, SeqCst).

- **URL:** https://eagle2000.space/rust/rust-atomics-and-memory-ordering/
- **Date:** 2026-08-11

- **Tags:** rust, dev, atomics, memory-ordering, relaxed, acquire-release, concurrency, lock-free

---
## Введение

При написании многопоточных систем обычные блокировки (`Mutex` и `RwLock`) обеспечивают полную безопасность за счет приостановки потоков операционной системой. Однако контекстные переключения процессора и системные вызовы ядер ОС несут ощутимые накладные расходы.

В высоконагруженных lock-free алгоритмах, высокочастотных сетевых сервисах и ядрах СУБД для изменения данных используют **Атомарные операции (Atomics)**.

Атомарная операция выполняется процессором как **одна неделимая инструкция**, которую не может прервать или увидеть в промежуточном состоянии ни один другой поток.

---

## 1. Атомарные типы (`std::sync::atomic`)

Модуль `std::sync::atomic` предоставляют скалярные типы с поддержкой аппаратной атомарности: `AtomicBool`, `AtomicUsize`, `AtomicI32`, `AtomicPtr<T>`.

Ключевая особенность атомарных типов в Rust — **Внутренняя мутабельность (Interior Mutability)** на уровне железа. Вы можете безопасно изменять атомарные данные по обычным иммутабельным ссылкам `&self` из нескольких потоков одновременно.

Базовые методы атомиков:
* **`load` / `store`**: атомарное чтение и запись.
* **`fetch_add` / `fetch_sub`**: атомарный инкремент/декремент (возвращают предыдущее значение).
* **`swap`**: атомарный обмен значениями.
* **`compare_exchange`**: атомарная операция Compare-And-Swap (CAS).

Ниже представлен пошаговый пример работы атомарного счетчика и флага останова:


### 💻 Атомарные типы и счетчик без мьютекса


#### Шаг 1

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

// Шаг 1: Атомарный счетчик AtomicUsize без мьютекса
fn main() {
    // AtomicUsize изменяется по разделяемой иммутабельной ссылке (&self) без блокировки Mutex!
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                // fetch_add атомарно прибавляет 1 и возвращает предыдущее значение
                counter_clone.fetch_add(1, Ordering::Relaxed);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Итоговое значение атомарного счетчика: {}", counter.load(Ordering::Relaxed));
}
```

**1. Атомарные скалярные примитивы**
- Типы `AtomicUsize`, `AtomicBool`, `AtomicI32` позволяют изменять значения по иммутабельной ссылке `&self` (Interior Mutability на уровне железа).
- Операции `fetch_add`, `fetch_sub`, `store` и `load` выполняются одной непрерывной процессорной инструкцией.


#### Шаг 2

```rust
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

// Шаг 2: Флаг останова фонового потока на основе AtomicBool
fn main() {
    let running_flag = Arc::new(AtomicBool::new(true));
    let flag_clone = Arc::clone(&running_flag);

    let handle = thread::spawn(move || {
        let mut iterations = 0;
        while flag_clone.load(Ordering::Relaxed) {
            iterations += 1;
            thread::sleep(Duration::from_millis(10));
        }
        println!("Фоновый поток завершен после {} итераций", iterations);
    });

    thread::sleep(Duration::from_millis(50));
    // Сигнализируем фоновому потоку о необходимости останова
    running_flag.store(false, Ordering::Relaxed);

    handle.join().unwrap();
}
```

**2. Атомарные флаги остановки (Lock-Free Signals)**
- Использование `AtomicBool` для передачи сигналов остановки между потоками не требует тяжелой блокировки мьютекса.


---

## 2. Проблема переупорядочивания инструкций (Instruction Reordering)

Современные процессоры и оптимизирующие компиляторы стремительно переупорядочивают инструкции выполнения для достижения максимального параллелизма на уровне инструкций (ILP).

Если поток 1 сначала пишет `data = 42`, а затем устанавливает `flag = true`, проц или компилятор могут легко поменять эти инструкции местами, если между ними нет зависимости по данным!

В итоге поток 2 может увидеть `flag == true`, но прочитать из `data` мусорное или старое значение.

Для решения этой проблемы в вызовы атомиков передается параметр **`Ordering`** — барьер памяти (**Memory Barrier**).

---

## 3. Модели упорядочивания памяти (Memory Ordering)

В Rust предусмотрено 5 уровней порядка памяти:

1. **`Ordering::Relaxed`**:
   - Гарантирует только атомарность самой операции над данной ячейкой памяти.
   - **Не создает никаких барьеров** вокруг: соседние операции чтения и записи могут свободно переупорядочиваться.
   - Подходит для изолированных счетчиков метрик.
2. **`Ordering::Release` (для записи)**:
   - Создает барьер публикации: компилятор и процессор **не имеют права переносить** операции записи, расположенные *до* `Release`, на место *после* него.
3. **`Ordering::Acquire` (для чтения)**:
   - Создает барьер чтения: операции чтения, расположенные *после* `Acquire`, **не могут быть выполнены до* него.
   - Пара **`Release` + `Acquire`** образует синхронизационную связь (Happens-Before relation) между потоками без единой системной блокировки!
4. **`Ordering::SeqCst` (Sequentially Consistent)**:
   - Гарантирует строгий глобальный единый порядок всех операций во всей программе.
   - Самый безопасный режим (используется по умолчанию во многих языках), но несет наибольшие накладные расходы на шине процессора.

> [!TIP]
> В 90% практических случаев применения lock-free флагов используется пара **`Ordering::Release`** при установке флага и **`Ordering::Acquire`** при его проверке.

Запустите примеры барьеров памяти `Acquire`/`Release` и операции `compare_exchange`:


### 💻 Барьеры памяти Acquire/Release и CAS


#### Шаг 1

```rust
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::Arc;
use std::thread;

// Шаг 1: Синхронизация данных через Acquire/Release барьер памяти
fn main() {
    let data = Arc::new(AtomicU8::new(0));
    let ready_flag = Arc::new(AtomicBool::new(false));

    let data_writer = Arc::clone(&data);
    let flag_writer = Arc::clone(&ready_flag);

    // Поток-Писатель публикует данные
    thread::spawn(move || {
        data_writer.store(42, Ordering::Relaxed);
        // Ordering::Release гарантирует, что ВСЕ предыдущие записи видны после этого момента
        flag_writer.store(true, Ordering::Release);
    });

    let data_reader = Arc::clone(&data);
    let flag_reader = Arc::clone(&ready_flag);

    // Поток-Читатель ожидает флаг
    let handle = thread::spawn(move || {
        // Ordering::Acquire создает барьер чтения
        while !flag_reader.load(Ordering::Acquire) {
            thread::yield_now();
        }
        // Защищено барьером: data_reader гарантированно видит значение 42!
        println!("Прочитанные данные через Acquire/Release: {}", data_reader.load(Ordering::Relaxed));
    });

    handle.join().unwrap();
}
```

**1. Барьеры памяти Acquire и Release**
- `Ordering::Release` создаёт барьер при записи: гарантирует, что все операции с памятью ДО записи не будут переупорядочены ПОСЛЕ неё.
- `Ordering::Acquire` создаёт барьер при чтении: гарантирует, что операции ПОСЛЕ чтения не будут выполнены ДО него.
- Пара `Release` + `Acquire` синхронизирует состояние между потоками без блокировок.


#### Шаг 2

```rust
use std::sync::atomic::{AtomicUsize, Ordering};

// Шаг 2: Атомарная операция Compare-And-Swap (CAS) через compare_exchange
fn main() {
    let current_val = AtomicUsize::new(10);

    let expected = 10; // Ожидаемое значение
    let new_val = 20;  // Новое значение

    // Сравниваем: если текущее значение равно 10, заменяем его на 20:
    let result = current_val.compare_exchange(
        expected,
        new_val,
        Ordering::SeqCst,  // Порядок при успехе
        Ordering::Relaxed  // Порядок при ошибке
    );

    match result {
        Ok(prev) => println!("Успешный CAS: предыдущее значение было {prev}, стало {}", current_val.load(Ordering::Relaxed)),
        Err(actual) => println!("CAS провален: фактическое значение {actual}"),
    }
}
```

**2. Атомарный Compare-And-Swap (CAS)**
- Функция `compare_exchange` является фундаментом всех lock-free структур данных (очередей, стеков, каналов).
- Она проверяет ожидаемое значение и обновляет его ровно за одну атомарную операцию процессора.


---

## Проверь свои знания!

Пройдите короткий тест по атомикам и Memory Ordering:


### 📝 Атомики и Memory Ordering


**1. В чем заключается главное преимущество атомарных типов (`AtomicBool`, `AtomicUsize`)?**

- [ ] Они позволяют модифицировать данные по иммутабельной ссылке (`&self`) без блокировки мьютекса за счет аппаратных инструкций процессора
- [ ] Они автоматически удваивают пропускную способность сетевой карты
- [ ] Они отменяют необходимость использования ключевого слова fn

**2. Какую задачу решают модели упорядочивания памяти (Memory Ordering)?**

- [ ] Задают барьеры памяти для компилятора и процессора, предотвращая нежелательное переупорядочение инструкций вокруг атомарной операции
- [ ] Сортируют элементы векторов по возрастанию
- [ ] Определяют приоритет выполнения потоков в операционной системе

**3. Как работают синхронизационные барьеры `Ordering::Release` и `Ordering::Acquire`?**

- [ ] `Release` при записи публикует все предыдущие изменения памяти, а `Acquire` при чтении гарантирует видимость опубликованных данных
- [ ] `Release` удаляет переменные из стека, а `Acquire` переносит их в кучу
- [ ] `Release` работает только в однопоточных программах

**4. Для чего используется атомарная операция `compare_exchange` (Compare-And-Swap / CAS)?**

- [ ] Для атомарного сравнения значения с ожидаемым и записи нового значения строго в случае совпадения
- [ ] Для автоматического перевода строк из нижнего регистра в верхний
- [ ] Для подсчета общего времени работы программы

