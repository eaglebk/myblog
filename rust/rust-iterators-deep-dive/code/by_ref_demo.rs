fn main() {
    let mut it = [10, 20, 30, 40, 50].into_iter();
    let header: Vec<i32> = it.by_ref().take(2).collect();
    let rest: Vec<i32> = it.collect();

    println!("header = {:?}, rest = {:?}", header, rest);
}

// ! **Отделяем заголовок через .by_ref()**
// ! - `take(2)` заимствует итератор по мутабельной ссылке, поэтому после него `it` остаётся живым и отдаёт оставшиеся элементы `[30, 40, 50]`.

