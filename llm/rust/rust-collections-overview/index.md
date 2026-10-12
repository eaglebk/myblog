# Использование и применение std::collections в Rust: BTreeMap, HashSet, BinaryHeap и VecDeque

> Разбираем коллекции в Rust: когда применять VecDeque, чем BTreeMap отличается от HashMap и почему LinkedList почти никогда не нужен.

- **URL:** https://eagle2000.space/rust/rust-collections-overview/
- **Date:** 2026-09-18

- **Tags:** rust, dev, collections, btreemap, vecdeque, binaryheap

---


<h2 class="relative group">Введение
    <div id="введение" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b2%d0%b2%d0%b5%d0%b4%d0%b5%d0%bd%d0%b8%d0%b5" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>В языке Rust обычно используеются стандартные <code>Vec&lt;T&gt;</code> и <code>HashMap&lt;K, V&gt;</code>. Но когда дело доходит до очередей сообщений, каких-то выборок или планировщиков задач - дефолтных контейнеров уже становится маловато.</p>
<p>Стандартный модуль <code>std::collections</code> закрывает большинство прикладных задач без внешних крейтов. Но при этом важно замечать не только на оценку скорости выполнения <code>O(1)</code> или <code>O(log N)</code>, но и на то, как данные размещаются в памяти. Современные процессоры любят работать с непрерывными кусками памяти: удачное размещение данных и попадание в L1/L2 кэш процессора на практике это часто перекрывают теоретическую сложность алгоритмов.</p>
<p>Разберем четыре ключевые структуры из стандартной библиотеки: <code>VecDeque</code>, <code>BTreeMap</code>, <code>BTreeSet</code> и <code>BinaryHeap</code>.</p>
<hr>

<h2 class="relative group">1. Коллекции: VecDeque vs Vec vs LinkedList
    <div id="1-коллекции-vecdeque-vs-vec-vs-linkedlist" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#1-%d0%ba%d0%be%d0%bb%d0%bb%d0%b5%d0%ba%d1%86%d0%b8%d0%b8-vecdeque-vs-vec-vs-linkedlist" aria-label="Якорь">#</a>
    </span>
    
</h2>

<h3 class="relative group">Динамический массив (<code>Vec&lt;T&gt;</code>)
    <div id="динамический-массив-vect" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b4%d0%b8%d0%bd%d0%b0%d0%bc%d0%b8%d1%87%d0%b5%d1%81%d0%ba%d0%b8%d0%b9-%d0%bc%d0%b0%d1%81%d1%81%d0%b8%d0%b2-vect" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p><code>Vec&lt;T&gt;</code> хранит элементы в виде непрерывного массива в куче. За счет этого итерация по вектору работает очень быстро, а чтение по индексу оценивается как <code>O(1)</code>.</p>
<p>Слабое место вектора — вставка и удаление в начале или середине: для этого приходится сдвигать весь хвост массива за <code>O(N)</code>.</p>

<h3 class="relative group">Двусторонняя очередь (<code>VecDeque&lt;T&gt;</code>)
    <div id="двусторонняя-очередь-vecdequet" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d0%b4%d0%b2%d1%83%d1%81%d1%82%d0%be%d1%80%d0%be%d0%bd%d0%bd%d1%8f%d1%8f-%d0%be%d1%87%d0%b5%d1%80%d0%b5%d0%b4%d1%8c-vecdequet" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Если вам нужна очередь (FIFO) или кольцевой буфер последних событий, берите <code>VecDeque&lt;T&gt;</code>.</p>
<p>Под капотом - динамический кольцевой буфер:</p>
<ul>
<li>Вставка и удаление с конца (<code>push_back</code> / <code>pop_back</code>) — <code>O(1)</code>.</li>
<li>Вставка и удаление с начала (<code>push_front</code> / <code>pop_front</code>) — <code>O(1)</code> без сдвига всего массива.</li>
</ul>
<div class="admonition relative overflow-hidden rounded-lg border-l-4 my-3 px-4 py-3 shadow-sm" data-type="tip">
      <div class="flex items-center gap-2 font-semibold text-inherit">
        <div class="flex shrink-0 h-5 w-5 items-center justify-center text-lg"><span class="relative block icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512"><path fill="currentColor" d="M112.1 454.3c0 6.297 1.816 12.44 5.284 17.69l17.14 25.69c5.25 7.875 17.17 14.28 26.64 14.28h61.67c9.438 0 21.36-6.401 26.61-14.28l17.08-25.68c2.938-4.438 5.348-12.37 5.348-17.7L272 415.1h-160L112.1 454.3zM191.4 .0132C89.44 .3257 16 82.97 16 175.1c0 44.38 16.44 84.84 43.56 115.8c16.53 18.84 42.34 58.23 52.22 91.45c.0313 .25 .0938 .5166 .125 .7823h160.2c.0313-.2656 .0938-.5166 .125-.7823c9.875-33.22 35.69-72.61 52.22-91.45C351.6 260.8 368 220.4 368 175.1C368 78.61 288.9-.2837 191.4 .0132zM192 96.01c-44.13 0-80 35.89-80 79.1C112 184.8 104.8 192 96 192S80 184.8 80 176c0-61.76 50.25-111.1 112-111.1c8.844 0 16 7.159 16 16S200.8 96.01 192 96.01z"/></svg>
</span></div>
        <div class="grow">
          Совет
        </div>
      </div><div class="admonition-content mt-3 text-base leading-relaxed text-inherit"><p>Почти в любом сценарии, где кажется нужным использовать связный список, <code>VecDeque</code> выигрывает и по скорости и по расходу памяти.</p></div></div>
<h3 class="relative group">Связный список (<code>LinkedList&lt;T&gt;</code>)
    <div id="связный-список-linkedlistt" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%81%d0%b2%d1%8f%d0%b7%d0%bd%d1%8b%d0%b9-%d1%81%d0%bf%d0%b8%d1%81%d0%be%d0%ba-linkedlistt" aria-label="Якорь">#</a>
    </span>
    
</h3>
<p>Двунаправленный <code>LinkedList&lt;T&gt;</code> в реальном коде на Rust почти не встречается. Каждый узел живет в куче в собственной аллокации. При обходе списка процессор вынужден постоянно прыгать по памяти и ловить промахи по кешу.</p>
<hr>

<h2 class="relative group">2. Словари: BTreeMap vs HashMap
    <div id="2-словари-btreemap-vs-hashmap" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#2-%d1%81%d0%bb%d0%be%d0%b2%d0%b0%d1%80%d0%b8-btreemap-vs-hashmap" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>стандартная библиотека предлагает два инструмента для работы с парами ключ-значение.</p>

<h3 class="relative group">Хеш-таблицы (<code>HashMap&lt;K, V&gt;</code> и <code>HashSet&lt;T&gt;</code>)
    <div id="хеш-таблицы-hashmapk-v-и-hashsett" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%85%d0%b5%d1%88-%d1%82%d0%b0%d0%b1%d0%bb%d0%b8%d1%86%d1%8b-hashmapk-v-%d0%b8-hashsett" aria-label="Якорь">#</a>
    </span>
    
</h3>
<ul>
<li><strong>Под капотом:</strong> алгоритм SwissTable.</li>
<li><strong>Сложность:</strong> поиск, вставка и удаление в среднем занимают <code>O(1)</code>.</li>
<li><strong>Особенности:</strong> порядок элементов случаен. Для ключей обязательны типажи <code>Hash + Eq</code>.</li>
</ul>

<h3 class="relative group">B-деревья (<code>BTreeMap&lt;K, V&gt;</code> и <code>BTreeSet&lt;T&gt;</code>)
    <div id="b-деревья-btreemapk-v-и-btreesett" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#b-%d0%b4%d0%b5%d1%80%d0%b5%d0%b2%d1%8c%d1%8f-btreemapk-v-%d0%b8-btreesett" aria-label="Якорь">#</a>
    </span>
    
</h3>
<ul>
<li><strong>Под капотом:</strong> B-дерево, где в каждом узле хранится компактный массив элементов. Это отлично укладывается в кэш-линии процессора.</li>
<li><strong>Сложность:</strong> поиск, вставка и удаление гарантированно занимают <code>O(log N)</code>.</li>
<li><strong>Особенности:</strong> ключи всегда отсортированы. Для ключей требуется типаж <code>Ord</code>.</li>
</ul>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">use</span><span class="w"> </span><span class="n">std</span>::<span class="n">collections</span>::<span class="n">BTreeMap</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="k">mut</span><span class="w"> </span><span class="n">map</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">BTreeMap</span>::<span class="n">new</span><span class="p">();</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="n">map</span><span class="p">.</span><span class="n">insert</span><span class="p">(</span><span class="mi">3</span><span class="p">,</span><span class="w"> </span><span class="s">&#34;Третий&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="n">map</span><span class="p">.</span><span class="n">insert</span><span class="p">(</span><span class="mi">1</span><span class="p">,</span><span class="w"> </span><span class="s">&#34;Первый&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="c1">// Итерация гарантированно вернет элементы по возрастанию: 1, затем 3
</span></span></span><span class="line"><span class="cl"><span class="k">for</span><span class="w"> </span><span class="p">(</span><span class="n">key</span><span class="p">,</span><span class="w"> </span><span class="n">val</span><span class="p">)</span><span class="w"> </span><span class="k">in</span><span class="w"> </span><span class="o">&amp;</span><span class="n">map</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;</span><span class="si">{key}</span><span class="s">: </span><span class="si">{val}</span><span class="s">&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<div class="admonition relative overflow-hidden rounded-lg border-l-4 my-3 px-4 py-3 shadow-sm" data-type="note">
      <div class="flex items-center gap-2 font-semibold text-inherit">
        <div class="flex shrink-0 h-5 w-5 items-center justify-center text-lg"><span class="relative block icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path fill="currentColor" d="M256 0C114.6 0 0 114.6 0 256s114.6 256 256 256s256-114.6 256-256S397.4 0 256 0zM256 128c17.67 0 32 14.33 32 32c0 17.67-14.33 32-32 32S224 177.7 224 160C224 142.3 238.3 128 256 128zM296 384h-80C202.8 384 192 373.3 192 360s10.75-24 24-24h16v-64H224c-13.25 0-24-10.75-24-24S210.8 224 224 224h32c13.25 0 24 10.75 24 24v88h16c13.25 0 24 10.75 24 24S309.3 384 296 384z"/></svg>
</span></div>
        <div class="grow">
          Примечание
        </div>
      </div><div class="admonition-content mt-3 text-base leading-relaxed text-inherit"><p>Зачем брать <code>BTreeMap</code>, если у <code>HashMap</code> заявлен <code>O(1)</code>?</p>
<ol>
<li>Нужна сортировка ключей по умолчанию.</li>
<li>Нужны выборки по диапазону через <code>.range(min..max)</code>.</li>
<li>Нужны предсказуемые гарантии времени без риска худшего сценария <code>O(N)</code> при коллизиях хешей.</li>
</ol></div></div><hr>

<h2 class="relative group">3. Очередь с приоритетом: BinaryHeap
    <div id="3-очередь-с-приоритетом-binaryheap" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#3-%d0%be%d1%87%d0%b5%d1%80%d0%b5%d0%b4%d1%8c-%d1%81-%d0%bf%d1%80%d0%b8%d0%be%d1%80%d0%b8%d1%82%d0%b5%d1%82%d0%be%d0%bc-binaryheap" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p><code>BinaryHeap&lt;T&gt;</code> — это двоичная Max-куча реализованная поверх вектора. Коллекция создана для планировщиков задач и очередей событий, где первым нужно обрабатывать самый приоритетный элемент.</p>
<ul>
<li>Добавление элемента <code>.push(item)</code> занимает <code>O(log N)</code>.</li>
<li>Извлечение максимального элемента <code>.pop()</code> занимает <code>O(log N)</code>.</li>
<li>Подсмотреть максимальный элемент через <code>.peek()</code> можно за <code>O(1)</code>.</li>
</ul>
<div class="admonition relative overflow-hidden rounded-lg border-l-4 my-3 px-4 py-3 shadow-sm" data-type="note">
      <div class="flex items-center gap-2 font-semibold text-inherit">
        <div class="flex shrink-0 h-5 w-5 items-center justify-center text-lg"><span class="relative block icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path fill="currentColor" d="M256 0C114.6 0 0 114.6 0 256s114.6 256 256 256s256-114.6 256-256S397.4 0 256 0zM256 128c17.67 0 32 14.33 32 32c0 17.67-14.33 32-32 32S224 177.7 224 160C224 142.3 238.3 128 256 128zM296 384h-80C202.8 384 192 373.3 192 360s10.75-24 24-24h16v-64H224c-13.25 0-24-10.75-24-24S210.8 224 224 224h32c13.25 0 24 10.75 24 24v88h16c13.25 0 24 10.75 24 24S309.3 384 296 384z"/></svg>
</span></div>
        <div class="grow">
          Примечание
        </div>
      </div><div class="admonition-content mt-3 text-base leading-relaxed text-inherit"><p>Чтобы превратить <code>BinaryHeap</code> в Min-кучу (где первым выходит наименьший элемент), оберните значения в <code>std::cmp::Reverse(x)</code>.</p></div></div><hr>

<h2 class="relative group">4. Шпаргалка по сложности
    <div id="4-шпаргалка-по-сложности" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#4-%d1%88%d0%bf%d0%b0%d1%80%d0%b3%d0%b0%d0%bb%d0%ba%d0%b0-%d0%bf%d0%be-%d1%81%d0%bb%d0%be%d0%b6%d0%bd%d0%be%d1%81%d1%82%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h2>
<table>
	<thead>
			<tr>
					<th style="text-align: left">Коллекция</th>
					<th style="text-align: center">Доступ</th>
					<th style="text-align: center">Вставка в начало</th>
					<th style="text-align: center">Вставка в конец</th>
					<th style="text-align: center">Порядок элементов</th>
			</tr>
	</thead>
	<tbody>
			<tr>
					<td style="text-align: left"><strong><code>Vec&lt;T&gt;</code></strong></td>
					<td style="text-align: center"><code>O(1)</code> по индексу</td>
					<td style="text-align: center"><code>O(N)</code></td>
					<td style="text-align: center"><code>O(1)</code> аморт.</td>
					<td style="text-align: center">Не сортирован</td>
			</tr>
			<tr>
					<td style="text-align: left"><strong><code>VecDeque&lt;T&gt;</code></strong></td>
					<td style="text-align: center"><code>O(1)</code> по индексу</td>
					<td style="text-align: center"><code>O(1)</code> аморт.</td>
					<td style="text-align: center"><code>O(1)</code> аморт.</td>
					<td style="text-align: center">Не сортирован</td>
			</tr>
			<tr>
					<td style="text-align: left"><strong><code>HashMap&lt;K, V&gt;</code></strong></td>
					<td style="text-align: center"><code>O(1)</code> среднее</td>
					<td style="text-align: center">—</td>
					<td style="text-align: center">—</td>
					<td style="text-align: center">Случайный</td>
			</tr>
			<tr>
					<td style="text-align: left"><strong><code>BTreeMap&lt;K, V&gt;</code></strong></td>
					<td style="text-align: center"><code>O(log N)</code></td>
					<td style="text-align: center">—</td>
					<td style="text-align: center">—</td>
					<td style="text-align: center">Отсортирован</td>
			</tr>
			<tr>
					<td style="text-align: left"><strong><code>BinaryHeap&lt;T&gt;</code></strong></td>
					<td style="text-align: center"><code>O(1)</code> для Max</td>
					<td style="text-align: center">—</td>
					<td style="text-align: center"><code>O(log N)</code></td>
					<td style="text-align: center">По приоритету</td>
			</tr>
	</tbody>
</table>
<p>Ниже — интерактивные слайды для каждой коллекции:</p>
### 💻 Работа с VecDeque, BTreeMap и BinaryHeap


#### Шаг 1


```rust
use std::collections::VecDeque;

fn main() {
    // Двусторонняя очередь (Ring Buffer)
    let mut queue = VecDeque::with_capacity(5);

    // Добавление в конец очереди:
    queue.push_back("Задача 1");
    queue.push_back("Задача 2");

    // Срочное добавление в начало очереди:
    queue.push_front("Приоритетная задача!");

    println!("Элементы очереди: {:?}", queue);

    // Извлечение с противоположных концов за O(1):
    if let Some(first) = queue.pop_front() {
        println!("Обработана из начала: {first}");
    }

    if let Some(last) = queue.pop_back() {
        println!("Обработана с конца: {last}");
    }
}
```


**Двусторонняя очередь: VecDeque**
- `VecDeque<T>` реализован как кольцевой буфер (Ring Buffer).
- Позволяет выполнять вставку и удаление за `O(1)` как с начала (`push_front`/`pop_front`), так и с конца (`push_back`/`pop_back`).
- Идеальный выбор для реализаций FIFO-очередей задач или буферов последних сообщений.


#### Шаг 2


```rust
use std::collections::BTreeMap;

fn main() {
    let mut sensor_data = BTreeMap::new();

    // Вставка ключей в случайном порядке
    sensor_data.insert(104, "Серверная");
    sensor_data.insert(101, "Прихожая");
    sensor_data.insert(103, "Кухня");
    sensor_data.insert(102, "Гостиная");

    println!("--- Итерация всегда строго отсортирована по ключу ---");
    for (id, room) in &sensor_data {
        println!("Датчик ID {id}: {room}");
    }

    println!("\n--- Срез по диапазону ключей (Range query 102..=103) ---");
    for (id, room) in sensor_data.range(102..=103) {
        println!("Диапазон: ID {id} -> {room}");
    }
}
```


**Упорядоченный словарь: BTreeMap**
- `BTreeMap<K, V>` устроен как B-дерево. Ключи всегда хранятся в отсортированном виде.
- Гарантирует логарифмическую сложность `O(log N)` для поиска, вставки и удаления.
- Позволяет выполнять быструю выборку по диапазонам ключей через метод `.range()`.


#### Шаг 3


```rust
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
struct Job {
    priority: u32,
    name: String,
}

// Реализуем Ord для сортировки очереди по приоритету
impl Ord for Job {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut heap = BinaryHeap::new();

    heap.push(Job { priority: 10, name: "Фоновая отгрузка логов".to_string() });
    heap.push(Job { priority: 90, name: "Аварийное отключение питания".to_string() });
    heap.push(Job { priority: 50, name: "Обновление конфигурации".to_string() });

    println!("--- Выборка задач в порядке приоритета (Max-Heap) ---");
    while let Some(job) = heap.pop() {
        println!("Выполняется [Приоритет {}]: {}", job.priority, job.name);
    }
}
```


**Очередь с приоритетом: BinaryHeap**
- `BinaryHeap<T>` — это Max-Куча. Метод `.pop()` всегда первым возвращает максимальный элемент.
- Вставка `push()` и удаление `pop()` выполняются за `O(log N)`.
- Требует от типа реализации типажа `Ord` и `Eq`. Идеально подходит для планировщиков задач.


<hr>

<h2 class="relative group">Тест для самопроверки
    <div id="тест-для-самопроверки" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#%d1%82%d0%b5%d1%81%d1%82-%d0%b4%d0%bb%d1%8f-%d1%81%d0%b0%d0%bc%d0%be%d0%bf%d1%80%d0%be%d0%b2%d0%b5%d1%80%d0%ba%d0%b8" aria-label="Якорь">#</a>
    </span>
    
</h2>
<p>Несколько вопросов, чтобы закрепить различия между коллекциями:</p>
### 📝 Стандартные коллекции в Rust

**1. Какое преимущество дает использование `VecDeque<T>` перед обычным `Vec<T>`?**

- [x] Возможность быстро вставлять и извлекать элементы с обоих концов за O(1)
- [ ] Автоматическая сортировка элементов при вставке
- [ ] Встроенная многопоточная синхронизация

> 💡 *`VecDeque<T>` устроен как кольцевой буфер (Ring Buffer). Он позволяет делать `push_front` и `pop_front` за время O(1) без сдвига всех элементов в памяти.*


**2. Чем `BTreeMap<K, V>` отличается от `HashMap<K, V>`?**

- [x] BTreeMap сохраняет ключи в отсортированном виде и поддерживает выборку по диапазонам .range()
- [ ] BTreeMap всегда работает быстрее HashMap на случайных ключах
- [ ] BTreeMap аллоцируется исключительно на стеке

> 💡 *`BTreeMap` устроен как B-дерево. Ключи в нем всегда отсортированы, что позволяет удобно итерироваться по диапазонам за время O(log N).*


**3. По какому принципу работает коллекция `BinaryHeap<T>`?**

- [x] Это Max-куча, которая вызовом .pop() всегда возвращает элемент с максимальным приоритетом
- [ ] Это стандартный двунаправленный связный список
- [ ] Это стек с доступом LIFO

> 💡 *`BinaryHeap` реализует приоритетную очередь в виде Max-кучи. Наибольший элемент согласно типажу `Ord` выталкивается первым.*


**4. Какова асимптотическая сложность поиска ключа в `BTreeMap`?**

- [x] O(log N)
- [ ] O(1)
- [ ] O(N^2)

> 💡 *Операции поиска, вставки и удаления в B-дереве гарантированно выполняются за логарифмическое время O(log N).*


**5. Как превратить `BinaryHeap<T>` в Min-Кучу (очередь, извлекающую сначала наименьшие элементы)?**

- [x] Обернуть элементы в контейнер `std::cmp::Reverse(x)`
- [ ] Использовать конструктор `BinaryHeap::min_heap()`
- [ ] Вызывать метод `.sort_asc()` перед каждым `.pop()`

> 💡 *В Rust `BinaryHeap` всегда является Max-кучей. Для получения Min-кучи типы элементов оборачивают в `std::cmp::Reverse(T)`, меняющий порядок сравнения `Ord` на противоположный.*


**6. Почему двунаправленный связный список `LinkedList<T>` в Rust используется крайне редко?**

- [x] Узлы аллоцируются в куче отдельно, вызывая частые промахи мимо кэша (Cache Misses)
- [ ] LinkedList запрещен при работе в ночной ветке Rust (Nightly)
- [ ] LinkedList может хранить не более 1024 элементов

> 💡 *Разрозненное расположение узлов связного списка в памяти ухудшает локальность данных для L1/L2 кэшей процессора. В результате `Vec` и `VecDeque` выигрывают по производительности практически во всех сценариях.*


**7. Какие типажи (traits) обязательны для типа ключа `K` в `BTreeMap<K, V>`?**

- [x] Ord (полный порядок)
- [ ] Hash + Eq
- [ ] Copy + Clone

> 💡 *Поскольку `BTreeMap` поддерживает отсортированное B-дерево, для ключей требуется реализации типажа полного порядка `Ord`. Для `HashMap`, напротив, нужен `Hash + Eq`.*


**8. Чем под капотом являются множества `HashSet<T>` и `BTreeSet<T>`?**

- [x] Тонкими обертками над `HashMap<T, ()>` и `BTreeMap<T, ()>` соответственно
- [ ] Низкоуровневыми битовыми масками
- [ ] Массивами уникальных строк

> 💡 *Множества `HashSet` и `BTreeSet` в std::collections реализовываются как соответствующие словари `HashMap` и `BTreeMap`, где в качестве значения хранится пустой unit-тип `()`.*




