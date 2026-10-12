# Using and Choosing std::collections in Rust: BTreeMap, HashSet, BinaryHeap, and VecDeque

> A practical guide to collections in Rust: when to use VecDeque, how BTreeMap compares to HashMap, and why you almost never need LinkedList.

- **URL:** https://eagle2000.space/en/rust/rust-collections-overview/
- **Date:** 2026-09-18

- **Tags:** rust, dev, collections, btreemap, vecdeque, binaryheap

---


<h2 class="relative group">Introduction
    <div id="introduction" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#introduction" aria-label="Anchor">#</a>
    </span>
    
</h2>
<p>Most of the time in Rust, <code>Vec&lt;T&gt;</code> and <code>HashMap&lt;K, V&gt;</code> are all you need. But when you hit message queues, specialized queries, or task schedulers, the default containers start falling short.</p>
<p>The standard <code>std::collections</code> module covers most application needs without third-party crates. Still, it is worth looking beyond just <code>O(1)</code> or <code>O(log N)</code> complexity ratings and paying attention to how data actually sits in memory. Modern CPUs love contiguous memory: good data placement and L1/L2 cache hits often outweigh theoretical algorithmic bounds in practice.</p>
<p>Let&rsquo;s walk through four key structures from the standard library: <code>VecDeque</code>, <code>BTreeMap</code>, <code>BTreeSet</code>, and <code>BinaryHeap</code>.</p>
<hr>

<h2 class="relative group">1. Collections: VecDeque vs Vec vs LinkedList
    <div id="1-collections-vecdeque-vs-vec-vs-linkedlist" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#1-collections-vecdeque-vs-vec-vs-linkedlist" aria-label="Anchor">#</a>
    </span>
    
</h2>

<h3 class="relative group">Dynamic Array (<code>Vec&lt;T&gt;</code>)
    <div id="dynamic-array-vect" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#dynamic-array-vect" aria-label="Anchor">#</a>
    </span>
    
</h3>
<p><code>Vec&lt;T&gt;</code> stores elements as a contiguous array on the heap. Because of this, iterating over a vector runs very fast, and indexing is rated at <code>O(1)</code>.</p>
<p>The vector&rsquo;s weak spot is inserting or removing elements at the front or middle: you have to shift the entire tail for <code>O(N)</code>.</p>

<h3 class="relative group">Double-Ended Queue (<code>VecDeque&lt;T&gt;</code>)
    <div id="double-ended-queue-vecdequet" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#double-ended-queue-vecdequet" aria-label="Anchor">#</a>
    </span>
    
</h3>
<p>If you need a queue (FIFO) or a ring buffer of recent events, reach for <code>VecDeque&lt;T&gt;</code>.</p>
<p>Under the hood, it is a dynamic ring buffer:</p>
<ul>
<li>Push and pop at the back (<code>push_back</code> / <code>pop_back</code>) — <code>O(1)</code>.</li>
<li>Push and pop at the front (<code>push_front</code> / <code>pop_front</code>) — <code>O(1)</code> without shifting the entire array.</li>
</ul>
<div class="admonition relative overflow-hidden rounded-lg border-l-4 my-3 px-4 py-3 shadow-sm" data-type="tip">
      <div class="flex items-center gap-2 font-semibold text-inherit">
        <div class="flex shrink-0 h-5 w-5 items-center justify-center text-lg"><span class="relative block icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512"><path fill="currentColor" d="M112.1 454.3c0 6.297 1.816 12.44 5.284 17.69l17.14 25.69c5.25 7.875 17.17 14.28 26.64 14.28h61.67c9.438 0 21.36-6.401 26.61-14.28l17.08-25.68c2.938-4.438 5.348-12.37 5.348-17.7L272 415.1h-160L112.1 454.3zM191.4 .0132C89.44 .3257 16 82.97 16 175.1c0 44.38 16.44 84.84 43.56 115.8c16.53 18.84 42.34 58.23 52.22 91.45c.0313 .25 .0938 .5166 .125 .7823h160.2c.0313-.2656 .0938-.5166 .125-.7823c9.875-33.22 35.69-72.61 52.22-91.45C351.6 260.8 368 220.4 368 175.1C368 78.61 288.9-.2837 191.4 .0132zM192 96.01c-44.13 0-80 35.89-80 79.1C112 184.8 104.8 192 96 192S80 184.8 80 176c0-61.76 50.25-111.1 112-111.1c8.844 0 16 7.159 16 16S200.8 96.01 192 96.01z"/></svg>
</span></div>
        <div class="grow">
          Tip
        </div>
      </div><div class="admonition-content mt-3 text-base leading-relaxed text-inherit"><p>In almost every scenario where you might think about a linked list, <code>VecDeque</code> wins in both speed and memory efficiency.</p></div></div>
<h3 class="relative group">Linked List (<code>LinkedList&lt;T&gt;</code>)
    <div id="linked-list-linkedlistt" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#linked-list-linkedlistt" aria-label="Anchor">#</a>
    </span>
    
</h3>
<p>Doubly linked <code>LinkedList&lt;T&gt;</code> is almost never seen in real-world Rust code. Each node lives in its own heap allocation. Traversing the list forces the CPU to jump across memory and constantly hit cache misses.</p>
<hr>

<h2 class="relative group">2. Maps: BTreeMap vs HashMap
    <div id="2-maps-btreemap-vs-hashmap" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#2-maps-btreemap-vs-hashmap" aria-label="Anchor">#</a>
    </span>
    
</h2>
<p>For working with key-value pairs, the standard library offers two tools.</p>

<h3 class="relative group">Hash Tables (<code>HashMap&lt;K, V&gt;</code> and <code>HashSet&lt;T&gt;</code>)
    <div id="hash-tables-hashmapk-v-and-hashsett" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#hash-tables-hashmapk-v-and-hashsett" aria-label="Anchor">#</a>
    </span>
    
</h3>
<ul>
<li><strong>Under the hood:</strong> SwissTable algorithm.</li>
<li><strong>Complexity:</strong> lookup, insertion, and deletion run in average <code>O(1)</code>.</li>
<li><strong>Characteristics:</strong> element order is arbitrary. Keys must implement <code>Hash + Eq</code>.</li>
</ul>

<h3 class="relative group">B-Trees (<code>BTreeMap&lt;K, V&gt;</code> and <code>BTreeSet&lt;T&gt;</code>)
    <div id="b-trees-btreemapk-v-and-btreesett" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#b-trees-btreemapk-v-and-btreesett" aria-label="Anchor">#</a>
    </span>
    
</h3>
<ul>
<li><strong>Under the hood:</strong> a B-Tree where each node holds a small array of elements. This packs keys tightly into CPU cache lines.</li>
<li><strong>Complexity:</strong> lookup, insertion, and deletion guarantee <code>O(log N)</code> time.</li>
<li><strong>Characteristics:</strong> keys are always sorted. Keys must implement <code>Ord</code>.</li>
</ul>
<div class="highlight-wrapper"><div class="highlight"><pre tabindex="0" class="chroma"><code class="language-rust" data-lang="rust"><span class="line"><span class="cl"><span class="k">use</span><span class="w"> </span><span class="n">std</span>::<span class="n">collections</span>::<span class="n">BTreeMap</span><span class="p">;</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="kd">let</span><span class="w"> </span><span class="k">mut</span><span class="w"> </span><span class="n">map</span><span class="w"> </span><span class="o">=</span><span class="w"> </span><span class="n">BTreeMap</span>::<span class="n">new</span><span class="p">();</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="n">map</span><span class="p">.</span><span class="n">insert</span><span class="p">(</span><span class="mi">3</span><span class="p">,</span><span class="w"> </span><span class="s">&#34;Third&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="n">map</span><span class="p">.</span><span class="n">insert</span><span class="p">(</span><span class="mi">1</span><span class="p">,</span><span class="w"> </span><span class="s">&#34;First&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="c1">// Iterates in ascending key order: 1, then 3
</span></span></span><span class="line"><span class="cl"><span class="k">for</span><span class="w"> </span><span class="p">(</span><span class="n">key</span><span class="p">,</span><span class="w"> </span><span class="n">val</span><span class="p">)</span><span class="w"> </span><span class="k">in</span><span class="w"> </span><span class="o">&amp;</span><span class="n">map</span><span class="w"> </span><span class="p">{</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="w">    </span><span class="fm">println!</span><span class="p">(</span><span class="s">&#34;</span><span class="si">{key}</span><span class="s">: </span><span class="si">{val}</span><span class="s">&#34;</span><span class="p">);</span><span class="w">
</span></span></span><span class="line"><span class="cl"><span class="p">}</span></span></span></code></pre></div></div>
<div class="admonition relative overflow-hidden rounded-lg border-l-4 my-3 px-4 py-3 shadow-sm" data-type="note">
      <div class="flex items-center gap-2 font-semibold text-inherit">
        <div class="flex shrink-0 h-5 w-5 items-center justify-center text-lg"><span class="relative block icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path fill="currentColor" d="M256 0C114.6 0 0 114.6 0 256s114.6 256 256 256s256-114.6 256-256S397.4 0 256 0zM256 128c17.67 0 32 14.33 32 32c0 17.67-14.33 32-32 32S224 177.7 224 160C224 142.3 238.3 128 256 128zM296 384h-80C202.8 384 192 373.3 192 360s10.75-24 24-24h16v-64H224c-13.25 0-24-10.75-24-24S210.8 224 224 224h32c13.25 0 24 10.75 24 24v88h16c13.25 0 24 10.75 24 24S309.3 384 296 384z"/></svg>
</span></div>
        <div class="grow">
          Note
        </div>
      </div><div class="admonition-content mt-3 text-base leading-relaxed text-inherit"><p>Why choose <code>BTreeMap</code> if <code>HashMap</code> offers <code>O(1)</code>?</p>
<ol>
<li>You need keys sorted by default.</li>
<li>You need range queries via <code>.range(min..max)</code>.</li>
<li>You need predictable runtime guarantees without worst-case <code>O(N)</code> hash collisions.</li>
</ol></div></div><hr>

<h2 class="relative group">3. Priority Queue: BinaryHeap
    <div id="3-priority-queue-binaryheap" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#3-priority-queue-binaryheap" aria-label="Anchor">#</a>
    </span>
    
</h2>
<p><code>BinaryHeap&lt;T&gt;</code> is a binary Max-Heap implemented on top of a vector. The collection is built for task schedulers and event queues where you need to process the highest-priority item first.</p>
<ul>
<li>Adding an item via <code>.push(item)</code> takes <code>O(log N)</code>.</li>
<li>Extracting the maximum item via <code>.pop()</code> takes <code>O(log N)</code>.</li>
<li>Peeking at the top item via <code>.peek()</code> takes <code>O(1)</code>.</li>
</ul>
<div class="admonition relative overflow-hidden rounded-lg border-l-4 my-3 px-4 py-3 shadow-sm" data-type="note">
      <div class="flex items-center gap-2 font-semibold text-inherit">
        <div class="flex shrink-0 h-5 w-5 items-center justify-center text-lg"><span class="relative block icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path fill="currentColor" d="M256 0C114.6 0 0 114.6 0 256s114.6 256 256 256s256-114.6 256-256S397.4 0 256 0zM256 128c17.67 0 32 14.33 32 32c0 17.67-14.33 32-32 32S224 177.7 224 160C224 142.3 238.3 128 256 128zM296 384h-80C202.8 384 192 373.3 192 360s10.75-24 24-24h16v-64H224c-13.25 0-24-10.75-24-24S210.8 224 224 224h32c13.25 0 24 10.75 24 24v88h16c13.25 0 24 10.75 24 24S309.3 384 296 384z"/></svg>
</span></div>
        <div class="grow">
          Note
        </div>
      </div><div class="admonition-content mt-3 text-base leading-relaxed text-inherit"><p>To turn <code>BinaryHeap</code> into a Min-Heap (popping the smallest item first), wrap values in <code>std::cmp::Reverse(x)</code>.</p></div></div><hr>

<h2 class="relative group">4. Complexity Cheatsheet
    <div id="4-complexity-cheatsheet" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#4-complexity-cheatsheet" aria-label="Anchor">#</a>
    </span>
    
</h2>
<table>
	<thead>
			<tr>
					<th style="text-align: left">Collection</th>
					<th style="text-align: center">Access</th>
					<th style="text-align: center">Front Insert</th>
					<th style="text-align: center">Back Insert</th>
					<th style="text-align: center">Element Ordering</th>
			</tr>
	</thead>
	<tbody>
			<tr>
					<td style="text-align: left"><strong><code>Vec&lt;T&gt;</code></strong></td>
					<td style="text-align: center"><code>O(1)</code> by index</td>
					<td style="text-align: center"><code>O(N)</code></td>
					<td style="text-align: center"><code>O(1)</code> amortized</td>
					<td style="text-align: center">Unsorted</td>
			</tr>
			<tr>
					<td style="text-align: left"><strong><code>VecDeque&lt;T&gt;</code></strong></td>
					<td style="text-align: center"><code>O(1)</code> by index</td>
					<td style="text-align: center"><code>O(1)</code> amortized</td>
					<td style="text-align: center"><code>O(1)</code> amortized</td>
					<td style="text-align: center">Unsorted</td>
			</tr>
			<tr>
					<td style="text-align: left"><strong><code>HashMap&lt;K, V&gt;</code></strong></td>
					<td style="text-align: center"><code>O(1)</code> avg</td>
					<td style="text-align: center">—</td>
					<td style="text-align: center">—</td>
					<td style="text-align: center">Random</td>
			</tr>
			<tr>
					<td style="text-align: left"><strong><code>BTreeMap&lt;K, V&gt;</code></strong></td>
					<td style="text-align: center"><code>O(log N)</code></td>
					<td style="text-align: center">—</td>
					<td style="text-align: center">—</td>
					<td style="text-align: center">Sorted</td>
			</tr>
			<tr>
					<td style="text-align: left"><strong><code>BinaryHeap&lt;T&gt;</code></strong></td>
					<td style="text-align: center"><code>O(1)</code> for Max</td>
					<td style="text-align: center">—</td>
					<td style="text-align: center"><code>O(log N)</code></td>
					<td style="text-align: center">Priority</td>
			</tr>
	</tbody>
</table>
<p>Below are interactive slides for each collection:</p>
### 💻 Working with VecDeque, BTreeMap, and BinaryHeap


#### Step 1


```rust
use std::collections::VecDeque;

fn main() {
    // Double-ended queue (Ring Buffer)
    let mut queue = VecDeque::with_capacity(5);

    // Appending to the back:
    queue.push_back("Task 1");
    queue.push_back("Task 2");

    // Urgent push to the front:
    queue.push_front("Priority task!");

    println!("Queue elements: {:?}", queue);

    // Pop from opposite ends in O(1):
    if let Some(first) = queue.pop_front() {
        println!("Popped from front: {first}");
    }

    if let Some(last) = queue.pop_back() {
        println!("Popped from back: {last}");
    }
}
```


**Double-Ended Queue: VecDeque**
- `VecDeque<T>` is implemented as a contiguous ring buffer.
- Allows `O(1)` insertions and deletions at both ends (`push_front`/`pop_front`, `push_back`/`pop_back`).
- Ideal for FIFO task queues, message buffers, and sliding windows.


#### Step 2


```rust
use std::collections::BTreeMap;

fn main() {
    let mut sensor_data = BTreeMap::new();

    // Inserting keys in arbitrary order
    sensor_data.insert(104, "Server Room");
    sensor_data.insert(101, "Hallway");
    sensor_data.insert(103, "Kitchen");
    sensor_data.insert(102, "Living Room");

    println!("--- Iteration is always strictly sorted by key ---");
    for (id, room) in &sensor_data {
        println!("Sensor ID {id}: {room}");
    }

    println!("\n--- Key range query (Range query 102..=103) ---");
    for (id, room) in sensor_data.range(102..=103) {
        println!("Range: ID {id} -> {room}");
    }
}
```


**Sorted Map: BTreeMap**
- `BTreeMap<K, V>` is structured as a B-Tree. Keys are always stored in sorted order.
- Guarantees logarithmic complexity `O(log N)` for lookup, insertion, and removal.
- Enables efficient range queries via `.range()`.


#### Step 3


```rust
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
struct Job {
    priority: u32,
    name: String,
}

// Implement Ord for sorting the heap by priority
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

    heap.push(Job { priority: 10, name: "Background log flush".to_string() });
    heap.push(Job { priority: 90, name: "Emergency power cutoff".to_string() });
    heap.push(Job { priority: 50, name: "Config update".to_string() });

    println!("--- Fetching jobs by priority (Max-Heap) ---");
    while let Some(job) = heap.pop() {
        println!("Executing [Priority {}]: {}", job.priority, job.name);
    }
}
```


**Priority Queue: BinaryHeap**
- `BinaryHeap<T>` is a Max-Heap. Calling `.pop()` always yields the maximum item first.
- Insertion `push()` and removal `pop()` operate in `O(log N)`.
- Requires elements to implement `Ord` and `Eq`. Perfect for schedulers and event processors.


<hr>

<h2 class="relative group">Quick Quiz
    <div id="quick-quiz" class="anchor"></div>
    
    <span
        class="absolute top-0 w-6 transition-opacity opacity-0 -start-6 not-prose group-hover:opacity-100 select-none">
        <a class="text-primary-300 dark:text-neutral-700 !no-underline" href="#quick-quiz" aria-label="Anchor">#</a>
    </span>
    
</h2>
<p>A short test to check your understanding of Rust collections:</p>
### 📝 Standard Collections in Rust

**1. What is the primary advantage of `VecDeque<T>` over a standard `Vec<T>`?**

- [x] O(1) insertion and removal at both the front and back of the queue
- [ ] Automatic element sorting upon insertion
- [ ] Built-in thread-safe synchronization

> 💡 *`VecDeque<T>` is implemented as a contiguous ring buffer, enabling `push_front` and `pop_front` in O(1) time without shifting all trailing elements in memory.*


**2. How does `BTreeMap<K, V>` differ from `HashMap<K, V>`?**

- [x] BTreeMap keeps keys sorted and natively supports range queries via .range()
- [ ] BTreeMap is always faster than HashMap on arbitrary random keys
- [ ] BTreeMap allocates exclusively on the stack

> 💡 *`BTreeMap` is structured as a B-Tree. Keys remain sorted at all times, making range scans efficient and deterministic in O(log N) time.*


**3. How does `BinaryHeap<T>` organize and retrieve elements?**

- [x] It is a Max-Heap where .pop() always returns the item with the highest priority
- [ ] It is a standard doubly linked list
- [ ] It is a LIFO stack

> 💡 *`BinaryHeap` implements a priority queue as a binary Max-Heap. The greatest element according to the `Ord` trait is popped first.*


**4. What is the asymptotic time complexity of a key lookup in `BTreeMap`?**

- [x] O(log N)
- [ ] O(1)
- [ ] O(N^2)

> 💡 *Search, insertion, and deletion operations in a B-Tree are guaranteed to execute in logarithmic time O(log N).*


**5. How do you turn `BinaryHeap<T>` into a Min-Heap (popping smallest items first)?**

- [x] Wrap elements in the standard `std::cmp::Reverse(x)` adapter
- [ ] Use the `BinaryHeap::min_heap()` constructor
- [ ] Invoke `.sort_asc()` before every `.pop()`

> 💡 *In Rust, `BinaryHeap` is always a Max-Heap. Wrapping values in `std::cmp::Reverse(T)` reverses the `Ord` comparison order to produce a Min-Heap.*


**6. Why is `LinkedList<T>` rarely recommended in production Rust code?**

- [x] Nodes are allocated individually on the heap, triggering frequent CPU cache misses
- [ ] LinkedList is disallowed on Rust Nightly compiler builds
- [ ] LinkedList has a hardcoded cap of 1024 elements

> 💡 *Scattered node allocations degrade data locality and CPU L1/L2 cache utilization. Consequently, `Vec` and `VecDeque` outperform `LinkedList` in almost all practical benchmarks.*


**7. Which traits are required for the key type `K` in `BTreeMap<K, V>`?**

- [x] Ord (total ordering)
- [ ] Hash + Eq
- [ ] Copy + Clone

> 💡 *Because `BTreeMap` organizes data in a sorted tree structure, keys must implement `Ord`. In contrast, `HashMap` requires `Hash + Eq`.*


**8. What are `HashSet<T>` and `BTreeSet<T>` under the hood?**

- [x] Thin wrappers around `HashMap<T, ()>` and `BTreeMap<T, ()>` respectively
- [ ] Low-level bitmasks
- [ ] Arrays of unique strings

> 💡 *Sets in `std::collections` are implemented directly on top of their map counterparts, storing the empty unit type `()` as the map value.*




