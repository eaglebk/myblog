# Hash Map в Rust: полное руководство по HashMap и ключам

> Что такое Hash Map в языке Rust? Узнайте, как эффективно работать с коллекцией HashMap, хэшированием, ключами и значениями на практических примерах.

- **URL:** https://eagle2000.space/rust/rust-hashmap/
- **Date:** 2025-03-05

- **Tags:** rust, dev

---


<h2 class="relative group">Введение
    <div id="введение" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d0%b2%d0%b5%d0%b4%d0%b5%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Hash Map — это коллекция пар &ldquo;ключ-значение&rdquo;, которая позволяет быстро находить значения по ключу. Представьте ее как справочник, где вы моментально получаете нужную информацию.</p>

<h2 class="relative group">Создание HashMap и добавление элементов
    <div id="создание-hashmap-и-добавление-элементов" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%81%d0%be%d0%b7%d0%b4%d0%b0%d0%bd%d0%b8%d0%b5-hashmap-%d0%b8-%d0%b4%d0%be%d0%b1%d0%b0%d0%b2%d0%bb%d0%b5%d0%bd%d0%b8%d0%b5-%d1%8d%d0%bb%d0%b5%d0%bc%d0%b5%d0%bd%d1%82%d0%be%d0%b2" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Для создания пустого HashMap используется функция <code>HashMap::new()</code>. Чтобы наполнить коллекцию данными, её необходимо сделать изменяемой с помощью ключевого слова <code>mut</code>, после чего использовать метод <code>insert(key, value)</code>.</p>
<p>Давайте посмотрим на интерактивном слайде, как создаётся HashMap, вставляются новые значения и как обновить существующее значение:</p>
### 💻 Базовые операции с HashMap: создание, вставка и обновление


#### Шаг 1


```rust
use std::collections::HashMap;

fn main() {
    // Создаем изменяемый HashMap
    let mut smart_home = HashMap::new();

    // Добавляем устройства и их состояние
    smart_home.insert("гостиная_свет".to_string(), "включен".to_string());
    smart_home.insert("спальня_термостат".to_string(), "22°C".to_string());

    println!("Устройства: {:?}", smart_home);
}
```


**Создание HashMap и добавление элементов**
- Для использования `HashMap` импортируем его из `std::collections`.
- Чтобы наполнить коллекцию данными, она должна быть объявлена изменяемой (`mut`).
- Метод `insert(key, value)` добавляет новую пару ключ-значение в мапу.


#### Шаг 2


```rust
use std::collections::HashMap;

fn main() {
    let mut smart_home = HashMap::new();
    smart_home.insert("гостиная_свет".to_string(), "включен".to_string());
    smart_home.insert("спальня_термостат".to_string(), "22°C".to_string());

    // Обновляем значение по существующему ключу
    smart_home.insert("гостиная_свет".to_string(), "выключен".to_string());

    println!("Обновленные устройства: {:?}", smart_home);
}
```


**Обновление значения по ключу**
- Если мы вызываем `insert` с ключом, который уже существует в `HashMap`, старое значение перезаписывается новым.



<h2 class="relative group">Получение значений и Entry API
    <div id="получение-значений-и-entry-api" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d0%be%d0%bb%d1%83%d1%87%d0%b5%d0%bd%d0%b8%d0%b5-%d0%b7%d0%bd%d0%b0%d1%87%d0%b5%d0%bd%d0%b8%d0%b9-%d0%b8-entry-api" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>При поиске значений в HashMap по ключу основным инструментом является метод <code>.get()</code>. Он возвращает перечисление <code>Option&lt;&amp;V&gt;</code>, что защищает программу от ошибок обращения к отсутствующим элементам. Также Rust предоставляет мощный Entry API для условных операций (добавление или изменение значений только при выполнении определенных условий).</p>
<p>Давайте подробно разберем, как искать элементы, обрабатывать их отсутствие и использовать Entry API на интерактивном слайде:</p>
### 💻 Поиск значений, unwrap_or и Entry API


#### Шаг 1


```rust
use std::collections::HashMap;

fn main() {
    let mut smart_home = HashMap::new();
    smart_home.insert("гостиная_свет", "включен");

    // Метод get() возвращает Option<&V>
    let light_status = smart_home.get("гостиная_свет");
    let lock_status = smart_home.get("дверь_замок");

    println!("Свет в гостиной: {:?}", light_status);
    println!("Замок двери: {:?}", lock_status);
}
```


**Поиск значений по ключу через .get()**
- Метод `.get()` принимает ссылку на ключ и возвращает `Option<&V>`.
- Если ключ найден, возвращается `Some(&value)`. Если ключа нет, возвращается `None`.
- Это позволяет безопасно проверять наличие данных без риска паники во время работы.


#### Шаг 2


```rust
use std::collections::HashMap;

fn main() {
    let mut smart_home = HashMap::new();
    smart_home.insert("гостиная_свет", "включен");

    // Получаем значение с дефолтом, если ключ отсутствует
    let default_status = "выключен";
    let light = smart_home.get("гостиная_свет").unwrap_or(&default_status);
    let lock = smart_home.get("дверь_замок").unwrap_or(&default_status);

    println!("Свет: {}", light);
    println!("Замок: {}", lock);
}
```


**Безопасное извлечение с unwrap_or**
- Метод `.unwrap_or()` позволяет задать запасное значение, которое будет использовано, если `.get()` вернул `None`.


#### Шаг 3


```rust
use std::collections::HashMap;

fn main() {
    let mut smart_home = HashMap::new();
    smart_home.insert("гостиная_свет".to_string(), "включен".to_string());

    // or_insert добавляет значение, только если ключа нет
    smart_home.entry("кухня_чайник".to_string()).or_insert("выключен".to_string());
    smart_home.entry("гостиная_свет".to_string()).or_insert("включен".to_string());

    // and_modify изменяет значение, только если ключ есть
    smart_home.entry("гостиная_свет".to_string())
        .and_modify(|status| *status = "выключен".to_string());

    println!("Итоговое состояние: {:?}", smart_home);
}
```


**Entry API: or_insert и and_modify**
- Метод `.entry()` возвращает перечисление Entry, представляющее запись (присутствующую или отсутствующую).
- Метод `.or_insert()` вставляет значение, только если ключа нет, и возвращает мутабельную ссылку на значение.
- Метод `.and_modify()` позволяет выполнить замыкание для изменения существующего значения по ключу.



<h3 class="relative group">Способы обработки результатов поиска
    <div id="способы-обработки-результатов-поиска" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%81%d0%bf%d0%be%d1%81%d0%be%d0%b1%d1%8b-%d0%be%d0%b1%d1%80%d0%b0%d0%b1%d0%be%d1%82%d0%ba%d0%b8-%d1%80%d0%b5%d0%b7%d1%83%d0%bb%d1%8c%d1%82%d0%b0%d1%82%d0%be%d0%b2-%d0%bf%d0%be%d0%b8%d1%81%d0%ba%d0%b0" aria-label="Якорь">#</a>
    </span>
    
</h3>
<ol>
<li>
<p><strong>Обработка отсутствующих ключей с помощью сопоставления с образцом (<code>match</code>)</strong>:
Поскольку <code>.get()</code> возвращает <code>Option&lt;&amp;V&gt;</code>, вы можете использовать сопоставление шаблонов для детальной обработки обоих сценариев (<code>Some</code> и <code>None</code>):</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">match</span><span class="w"> </span><span class="n">smart_home_devices</span><span class="p">.</span><span class="n">get</span><span class="p">(</span><span class="s">&#34;гостиная_свет&#34;</span><span class="p">)</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="nb">Some</span><span class="p">(</span><span class="n">status</span><span class="p">)</span><span class="w"> </span><span class="o">=&gt;</span><span class="w"> </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Статус света: </span><span class="si">{}</span><span class="s">&#34;</span><span class="p">,</span><span class="w"> </span><span class="n">status</span><span class="p">),</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="nb">None</span><span class="w"> </span><span class="o">=&gt;</span><span class="w"> </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;Статус света не найден&#34;</span><span class="p">),</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
</li>
<li>
<p><strong>Значения по умолчанию с <code>unwrap_or</code></strong>:
Если вместо детальной обработки <code>match</code> вам достаточно подставить стандартное значение в случае отсутствия ключа, используйте метод <code>.unwrap_or()</code>:</p>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">default_status</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="s">&#34;выключен&#34;</span><span class="p">.</span><span class="n">to_string</span><span class="p">();</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="n">status</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">smart_home_devices</span><span class="p">.</span><span class="n">get</span><span class="p">(</span><span class="s">&#34;гостиная_свет&#34;</span><span class="p">).</span><span class="n">unwrap_or</span><span class="p">(</span><span class="o">&amp;</span><span class="n">default_status</span><span class="p">);</span></span></span></code></pre></div></div>
</li>
</ol>
<hr>

<h2 class="relative group">Практический пример
    <div id="практический-пример" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%b0%d0%ba%d1%82%d0%b8%d1%87%d0%b5%d1%81%d0%ba%d0%b8%d0%b9-%d0%bf%d1%80%d0%b8%d0%bc%d0%b5%d1%80" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Давайте рассмотрим практический пример, связанный с управлением умным домом. Предположим, у нас есть несколько устройств в доме и нам нужно отслеживать их текущий статус. Например, мы хотим знать, включен ли свет в гостиной, какая температура установлена на термостате в спальне и работает ли кондиционер на кухне. Если статус какого-то устройства неизвестен, мы хотим выводить соответствующее сообщение.</p>
<ul>
<li>У нас есть список всех устройств, которые нас интересуют.</li>
<li>Для каждого устройства нужно вывести его название и текущий статус из HashMap, подставив значение по умолчанию для неизвестных приборов.</li>
</ul>
<p>Давайте посмотрим, как работает этот алгоритм на практике с помощью интерактивного слайда:</p>
### 💻 Практический пример: Мониторинг умного дома



```rust
use std::collections::HashMap;

fn main() {
    // Создаем HashMap для хранения статусов устройств
    let mut home_device_status: HashMap<String, String> = HashMap::new();

    // Добавляем статусы для некоторых устройств
    home_device_status.insert("гостиная_свет".to_string(), "включен".to_string());
    home_device_status.insert("спальня_термостат".to_string(), "22°C".to_string());

    // Список интересующих нас приборов
    let devices = vec!["гостиная_свет", "спальня_термостат", "кухня_кондиционер"];
    let default_device_status = "статус неизвестен".to_string();

    // Перебираем все устройства и выводим их статус
    for device in devices {
        let status = home_device_status.get(device).unwrap_or(&default_device_status);
        println!("{}: {}", device, status);
    }
}
```


**Практический пример: Мониторинг умного дома**
- Мы обходим вектор интересующих нас приборов и ищем состояние каждого в `HashMap`.
- Для прибора `кухня_кондиционер`, которого нет в мапе, метод `.unwrap_or()` подставляет запасное значение `статус неизвестен`.



<h2 class="relative group">Почему HashMap так полезен?
    <div id="почему-hashmap-так-полезен" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d0%be%d1%87%d0%b5%d0%bc%d1%83-hashmap-%d1%82%d0%b0%d0%ba-%d0%bf%d0%be%d0%bb%d0%b5%d0%b7%d0%b5%d0%bd" aria-label="Якорь">#</a>
    </span>
    
</h2>
<ul>
<li>🚀 <strong>Быстрый доступ</strong>: поиск, добавление и удаление элементов происходят за константное время <code>O(1)</code> в среднем.</li>
<li>🔄 <strong>Entry API</strong>: очень лаконичный и безопасный способ условной работы с коллекцией без лишних проверок.</li>
<li>🛡️ <strong>Безопасность типизации</strong>: компилятор Rust гарантирует, что ключи и значения будут строго соответствовать объявленным типам.</li>
</ul>

<h2 class="relative group">Заключение
    <div id="заключение" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b7%d0%b0%d0%ba%d0%bb%d1%8e%d1%87%d0%b5%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>HashMap предоставляет мощный способ работы с парами &ldquo;ключ-значение&rdquo;. Они идеально подходят для задач, требующих быстрого поиска и эффективного управления данными.</p>
<div class="lead text-neutral-500 dark:text-neutral-400 !mb-9 text-xl">
  <p>Используйте HashMap, когда:</p>
<ul>
<li>Вы хотите связать произвольный ключ с произвольным значением.</li>
<li>Вам нужен кэш.</li>
<li>Вам нужна эффективная ассоциативная коллекция без сохранения порядка вставки.</li>
</ul>
</div>

<p><a href="https://doc.rust-lang.org/std/collections/struct.HashMap.html"  target="_blank" rel="noreferrer">Официальная документация по HashMap</a></p>
<hr>

<h2 class="relative group">Проверь свои знания!
    <div id="проверь-свои-знания" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%bf%d1%80%d0%be%d0%b2%d0%b5%d1%80%d1%8c-%d1%81%d0%b2%d0%be%d0%b8-%d0%b7%d0%bd%d0%b0%d0%bd%d0%b8%d1%8f" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Пройдите короткий интерактивный тест, чтобы закрепить понимание работы с HashMap в Rust.</p>
### 📝 HashMap в Rust

**1. Из какого модуля стандартной библиотеки Rust необходимо импортировать HashMap?**

- [x] std::collections::HashMap
- [ ] std::hash::HashMap
- [ ] std::collections::hash_map::HashMap
- [ ] std::map::HashMap

> 💡 *Структура HashMap расположена в модуле `std::collections::HashMap` стандартной библиотеки Rust.*


**2. Какой тип возвращает метод `HashMap::get()` при поиске по ключу?**

- [ ] Непосредственно тип значения V, либо вызывает панику, если ключ отсутствует
- [x] Option<&V> (ссылку на значение, обёрнутую в Option)
- [ ] Result<&V, Error>
- [ ] Option<V> (копию значения, обёрнутую в Option)

> 💡 *Поскольку ключа в HashMap может не оказаться, метод `.get()` возвращает `Option<&V>`: `Some(&V)` при успешном поиске и `None` в противном случае.*


**3. Что произойдет при вызове метода `.insert()` с ключом, который уже существует в HashMap?**

- [ ] Новое значение будет проигнорировано
- [ ] Программа завершится с ошибкой компиляции
- [x] Предыдущее значение будет перезаписано новым
- [ ] Программа упадет с паникой во время выполнения

> 💡 *При повторной вставке по существующему ключу старое значение перезаписывается новым. Метод `.insert()` при этом возвращает `Some(old_value)`.*


**4. Какой метод Entry API позволяет вставить значение по умолчанию только в случае отсутствия ключа?**

- [ ] and_modify()
- [x] or_insert()
- [ ] insert_default()
- [ ] unwrap_or()

> 💡 *После вызова метода `.entry(key)` метод `.or_insert(value)` проверяет, существует ли ключ. Если нет, он вставляет переданное значение.*


**5. Почему порядок обхода пар в HashMap при итерации является произвольным и меняется при перезапусках?**

- [ ] Из-за оптимизации компилятора под разные архитектуры процессоров
- [x] Rust намеренно использует случайную соль для хэширования, чтобы защитить приложение от атак типа HashDoS (переполнение коллизиями)
- [ ] Это вызвано ошибкой в стандартной библиотеке
- [ ] Для уменьшения потребления оперативной памяти

> 💡 *Rust по умолчанию использует криптографически стойкий алгоритм хэширования (SipHash) со случайным зерном (salt) для защиты от атак HashDoS, из-за чего порядок хранения и обхода элементов не гарантирован.*




