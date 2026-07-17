fn find_number(option: Option<i32>) -> Option<i32> {
    // Оператор ? возвращает значение внутри Some(x), либо делает досрочный возврат None
    let num = option?;
    Some(num * 2)
}

fn parse_and_double(s: &str) -> Result<i32, std::num::ParseIntError> {
    // Оператор ? возвращает i32, либо немедленно пробрасывает ошибку Err вверх
    let val = s.parse::<i32>()?;
    Ok(val * 2)
}

fn main() {
    let result_opt = find_number(Some(21));
    let none_opt = find_number(None);

    let result_res = parse_and_double("10");
    let err_res = parse_and_double("abc");

    println!("opt success: {:?}, opt none: {:?}", result_opt, none_opt);
    println!("res success: {:?}, res err: {:?}", result_res, err_res);
}
// ! **Оператор ? для пропагации ошибок**
// ! - Оператор `?` используется для быстрого извлечения значений из `Option` и `Result`.
// ! - Для `Option`: если результат `Some(x)`, извлекает `x`; если `None`, делает досрочный возврат `None` из функции.
// ! - Для `Result`: если результат `Ok(x)`, извлекает `x`; если `Err(e)`, делает досрочный возврат `Err(e.into())`.

// ---

fn handle_event(event: Option<String>) {
    // let-else извлекает значение из Some, либо выполняет расходящийся блок else
    let Some(name) = event else {
        println!("Событие не содержит имени, завершаем функцию.");
        return; // Обязательный расходящийся выход
    };

    println!("Обработка события: {}", name);
}

fn main() {
    handle_event(Some("Click".to_string()));
    handle_event(None);

    // Пример let-else в цикле с continue
    for i in 0..5 {
        let opt = if i % 2 == 0 { Some(i) } else { None };
        
        let Some(val) = opt else {
            continue; // Пропускаем нечетные числа
        };
        println!("Четное число: {}", val);
    }
}
// ! **Конструкция let-else (Diverging let)**
// ! - `let-else` позволяет сопоставить опровержимый шаблон (например, `Some(x)`) и связать переменную в текущей области видимости.
// ! - Если шаблон не совпал, выполняется блок `else`, который обязан быть **расходящимся** (diverging).
// ! - Расходящийся блок `else` не имеет права просто вернуть управление; он обязан завершиться с помощью `return`, `break`, `continue` или `panic!()`.
