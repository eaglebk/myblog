# Using and Choosing std::collections in Rust: BTreeMap, HashSet, BinaryHeap, and VecDeque

> A practical guide to collections in Rust: when to use VecDeque, how BTreeMap compares to HashMap, and why you almost never need LinkedList.

- **URL:** https://eagle2000.space/en/rust/rust-collections-overview/
- **Date:** 2026-09-18

- **Tags:** rust, dev, collections, btreemap, vecdeque, binaryheap

---
## Introduction

Most of the time in Rust, `Vec<T>` and `HashMap<K, V>` are all you need. But when you hit message queues, specialized queries, or task schedulers, the default containers start falling short.

The standard `std::collections` module covers most application needs without third-party crates. Still, it is worth looking beyond just `O(1)` or `O(log N)` complexity ratings and paying attention to how data actually sits in memory. Modern CPUs love contiguous memory: good data placement and L1/L2 cache hits often outweigh theoretical algorithmic bounds in practice.

Let's walk through four key structures from the standard library: `VecDeque`, `BTreeMap`, `BTreeSet`, and `BinaryHeap`.

---

## 1. Collections: VecDeque vs Vec vs LinkedList

### Dynamic Array (`Vec<T>`)
`Vec<T>` stores elements as a contiguous array on the heap. Because of this, iterating over a vector runs very fast, and indexing is rated at `O(1)`.

The vector's weak spot is inserting or removing elements at the front or middle: you have to shift the entire tail for `O(N)`.

### Double-Ended Queue (`VecDeque<T>`)
If you need a queue (FIFO) or a ring buffer of recent events, reach for `VecDeque<T>`.

Under the hood, it is a dynamic ring buffer:
* Push and pop at the back (`push_back` / `pop_back`) — `O(1)`.
* Push and pop at the front (`push_front` / `pop_front`) — `O(1)` without shifting the entire array.

> [!TIP]
> In almost every scenario where you might think about a linked list, `VecDeque` wins in both speed and memory efficiency.

### Linked List (`LinkedList<T>`)
Doubly linked `LinkedList<T>` is almost never seen in real-world Rust code. Each node lives in its own heap allocation. Traversing the list forces the CPU to jump across memory and constantly hit cache misses.

---

## 2. Maps: BTreeMap vs HashMap

For working with key-value pairs, the standard library offers two tools.

### Hash Tables (`HashMap<K, V>` and `HashSet<T>`)
* **Under the hood:** SwissTable algorithm.
* **Complexity:** lookup, insertion, and deletion run in average `O(1)`.
* **Characteristics:** element order is arbitrary. Keys must implement `Hash + Eq`.

### B-Trees (`BTreeMap<K, V>` and `BTreeSet<T>`)
* **Under the hood:** a B-Tree where each node holds a small array of elements. This packs keys tightly into CPU cache lines.
* **Complexity:** lookup, insertion, and deletion guarantee `O(log N)` time.
* **Characteristics:** keys are always sorted. Keys must implement `Ord`.

```rust
use std::collections::BTreeMap;

let mut map = BTreeMap::new();
map.insert(3, "Third");
map.insert(1, "First");

// Iterates in ascending key order: 1, then 3
for (key, val) in &map {
    println!("{key}: {val}");
}
```

> [!NOTE]
> Why choose `BTreeMap` if `HashMap` offers `O(1)`?
> 1. You need keys sorted by default.
> 2. You need range queries via `.range(min..max)`.
> 3. You need predictable runtime guarantees without worst-case `O(N)` hash collisions.

---

## 3. Priority Queue: BinaryHeap

`BinaryHeap<T>` is a binary Max-Heap implemented on top of a vector. The collection is built for task schedulers and event queues where you need to process the highest-priority item first.

* Adding an item via `.push(item)` takes `O(log N)`.
* Extracting the maximum item via `.pop()` takes `O(log N)`.
* Peeking at the top item via `.peek()` takes `O(1)`.

> [!NOTE]
> To turn `BinaryHeap` into a Min-Heap (popping the smallest item first), wrap values in `std::cmp::Reverse(x)`.

---

## 4. Complexity Cheatsheet

| Collection | Access | Front Insert | Back Insert | Element Ordering |
| :--- | :---: | :---: | :---: | :---: |
| **`Vec<T>`** | `O(1)` by index | `O(N)` | `O(1)` amortized | Unsorted |
| **`VecDeque<T>`** | `O(1)` by index | `O(1)` amortized | `O(1)` amortized | Unsorted |
| **`HashMap<K, V>`** | `O(1)` avg | — | — | Random |
| **`BTreeMap<K, V>`** | `O(log N)` | — | — | Sorted |
| **`BinaryHeap<T>`** | `O(1)` for Max | — | `O(log N)` | Priority |

Below are interactive slides for each collection:


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


---

## Quick Quiz

A short test to check your understanding of Rust collections:


### 📝 Standard Collections in Rust


**1. What is the primary advantage of `VecDeque<T>` over a standard `Vec<T>`?**

- [ ] O(1) insertion and removal at both the front and back of the queue
- [ ] Automatic element sorting upon insertion
- [ ] Built-in thread-safe synchronization

**2. How does `BTreeMap<K, V>` differ from `HashMap<K, V>`?**

- [ ] BTreeMap keeps keys sorted and natively supports range queries via .range()
- [ ] BTreeMap is always faster than HashMap on arbitrary random keys
- [ ] BTreeMap allocates exclusively on the stack

**3. How does `BinaryHeap<T>` organize and retrieve elements?**

- [ ] It is a Max-Heap where .pop() always returns the item with the highest priority
- [ ] It is a standard doubly linked list
- [ ] It is a LIFO stack

**4. What is the asymptotic time complexity of a key lookup in `BTreeMap`?**

- [ ] O(log N)
- [ ] O(1)
- [ ] O(N^2)

**5. How do you turn `BinaryHeap<T>` into a Min-Heap (popping smallest items first)?**

- [ ] Wrap elements in the standard `std::cmp::Reverse(x)` adapter
- [ ] Use the `BinaryHeap::min_heap()` constructor
- [ ] Invoke `.sort_asc()` before every `.pop()`

**6. Why is `LinkedList<T>` rarely recommended in production Rust code?**

- [ ] Nodes are allocated individually on the heap, triggering frequent CPU cache misses
- [ ] LinkedList is disallowed on Rust Nightly compiler builds
- [ ] LinkedList has a hardcoded cap of 1024 elements

**7. Which traits are required for the key type `K` in `BTreeMap<K, V>`?**

- [ ] Ord (total ordering)
- [ ] Hash + Eq
- [ ] Copy + Clone

**8. What are `HashSet<T>` and `BTreeSet<T>` under the hood?**

- [ ] Thin wrappers around `HashMap<T, ()>` and `BTreeMap<T, ()>` respectively
- [ ] Low-level bitmasks
- [ ] Arrays of unique strings

