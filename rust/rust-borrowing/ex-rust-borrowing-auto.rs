fn borrow_car(borrower: &String) {
    println!("{}", borrower);
}

fn main() {
    let owner = String::from("automobile");
    borrow_car(&owner);
    println!("{}", owner); // всё ещё доступен
}

// ! **1. Этот код не скомпилируется.**
// ! - Ошибка: `Rc<Vec<i32>> cannot be sent between threads safely`
