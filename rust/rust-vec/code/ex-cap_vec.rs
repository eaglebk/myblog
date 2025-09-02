fn main() {
    let mut crew = Vec::new();

    for i in 1..=5 {
        crew.push(format!("Член экипажа #{i}"));
        println!(
            "Добавлен {i}-й: len={}, capacity={}",
            crew.len(),
            crew.capacity()
        );
    }
}
// ! **Запустите код и вы увидите, что capacity растёт скачками (примерно в 2 раза).**
// ! - Это нужно, чтобы `push` в среднем был очень быстрым (`O(1)`).
