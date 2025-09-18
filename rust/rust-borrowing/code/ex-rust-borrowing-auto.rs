fn borrow_car(borrower: &String) {
    println!("{} from borrow_car", borrower);
}

fn main() {
    let owner = String::from("automobile");
    borrow_car(&owner);
    println!("{} from main", owner); // всё ещё доступен
}

// ! **Вместо того чтобы передавать владение переменной (move), можно передать ссылку на неё: `&` или `&mut`.**
// ! - `borrow_car` использует ссылку `&String`
// ! - В `main` переменная `owner` всё ещё жива — потому что её никто не «забрал»
