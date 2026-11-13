fn main() {
    // Split ID range [0..100) into batches of 25
    let mut cursor = Some(0u64);
    let batches = std::iter::from_fn(move || {
        let start = cursor?;
        if start >= 100 {
            cursor = None;
            return None;
        }
        let end = (start + 25).min(100);
        cursor = Some(end);
        Some(start..end)
    });

    let ranges: Vec<_> = batches.collect();
    println!("Batches: {:?}", ranges);
}

// ! **Batch generator with std::iter::from_fn**
// ! - The closure captures `cursor` and yields the next `Range<u64>` on each `.next()` call.

