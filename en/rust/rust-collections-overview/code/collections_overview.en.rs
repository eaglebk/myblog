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

// ! **Double-Ended Queue: VecDeque**
// ! - `VecDeque<T>` is implemented as a contiguous ring buffer.
// ! - Allows `O(1)` insertions and deletions at both ends (`push_front`/`pop_front`, `push_back`/`pop_back`).
// ! - Ideal for FIFO task queues, message buffers, and sliding windows.

// ---

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

// ! **Sorted Map: BTreeMap**
// ! - `BTreeMap<K, V>` is structured as a B-Tree. Keys are always stored in sorted order.
// ! - Guarantees logarithmic complexity `O(log N)` for lookup, insertion, and removal.
// ! - Enables efficient range queries via `.range()`.

// ---

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

// ! **Priority Queue: BinaryHeap**
// ! - `BinaryHeap<T>` is a Max-Heap. Calling `.pop()` always yields the maximum item first.
// ! - Insertion `push()` and removal `pop()` operate in `O(log N)`.
// ! - Requires elements to implement `Ord` and `Eq`. Perfect for schedulers and event processors.
