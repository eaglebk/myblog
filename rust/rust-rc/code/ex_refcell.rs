use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let users = Rc::new(RefCell::new(vec!["Алиса".to_string()]));

    let session1 = Rc::clone(&users);
    let session2 = Rc::clone(&users);

    session1.borrow_mut().push("Макс".to_string());
    session2.borrow_mut().push("Петр".to_string());

    println!("Онлайн: {:?}", users.borrow());
}
// ! **1. Общий список пользователей в чате**
// ! - Мы моделируем общее состояние `users`, доступное из разных частей приложения. RefCell позволяет менять данные при множественных владельцах.

// ---
#[allow(unused_variables)]
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let shared = Rc::new(RefCell::new(vec![1, 2, 3]));

    let _first = shared.borrow_mut();
    let _second = shared.borrow_mut(); // ❌ паника: уже есть активный borrow_mut
}

// ! **2. Ошибка: двойной borrow_mut при вложенных вызовах**
// ! - `RefCell` проверяет заимствование во время выполнения. Нельзя взять borrow_mut дважды одновременно.

// ---
#[allow(unused_variables)]
use std::rc::Rc;
use std::cell::RefCell;
fn main() {
    let shared = Rc::new(RefCell::new(vec![1, 2, 3]));

    {
        let mut data = shared.borrow_mut();
        data.push(4);
    }

    let again = shared.borrow_mut();
    println!("{:?}", again);
}
// ! **3. Исправление: ограничиваем область действия borrow**
// ! - Вложите `borrow_mut` в отдельный блок — это освободит заимствование до следующего вызова. ✅