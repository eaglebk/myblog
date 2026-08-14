trait Task {
    fn execute(&self) -> String;
}

struct NetworkTask;
impl Task for NetworkTask {
    fn execute(&self) -> String { "Network".to_string() }
}

struct DbTask;
impl Task for DbTask {
    fn execute(&self) -> String { "Database".to_string() }
}

fn main() {
    // Vector holds different types under a common trait object
    let tasks: Vec<Box<dyn Task>> = vec![
        Box::new(NetworkTask),
        Box::new(DbTask),
    ];
    for t in tasks {
        println!("Executing task: {}", t.execute());
    }
}

// ! **Dynamic Polymorphism and dyn Trait**
// ! - Trait object `dyn Task` enables working with heterogeneous collections where types are known only at runtime.
// ! - For this reason, objects must live behind a pointer (e.g., `Box<dyn Task>` or `&dyn Task`).
// ! - Invoked method resolution occurs at runtime via the virtual method table (vtable).

// ---

// By default all generics have an implicit T: Sized bound.
// We relax it with ?Sized so the function can accept slices or dyn Trait.
fn print_debug<T: ?Sized + std::fmt::Debug>(val: &T) {
    println!("Value: {:?}", val);
}

fn main() {
    let s: &str = "Hello, world!"; // str is a DST (Dynamically Sized Type)
    print_debug(s);
}

// ! **Implicit Sized bound and ?Sized**
// ! - Most types have a size known at compile time (Sized) and can live on the stack.
// ! - Dynamically Sized Types (DSTs), such as `str`, `[T]`, or `dyn Trait`, do not have a fixed size.
// ! - Syntax `?Sized` relaxes the implicit fixed-size requirement, allowing DSTs to be passed by reference.

// ---

trait Fly {
    fn fly(&self) -> String;
}

trait Swim {
    fn swim(&self) -> String;
}

// Rust forbids &(dyn Fly + Swim) directly.
// We create a combined supertrait:
trait Duck: Fly + Swim {}

// And write a Blanket Implementation for all matching types:
impl<T: Fly + Swim> Duck for T {}

struct Mallard;
impl Fly for Mallard {
    fn fly(&self) -> String { "Flying".to_string() }
}
impl Swim for Mallard {
    fn swim(&self) -> String { "Swimming".to_string() }
}

fn handle_duck(duck: &dyn Duck) {
    println!("Duck: {}, {}", duck.fly(), duck.swim());
}

fn main() {
    let d = Mallard;
    handle_duck(&d);
}

// ! **Multi-Trait Objects**
// ! - In Rust, you cannot directly combine multiple independent traits into a single trait object: `dyn Trait1 + Trait2` will not compile.
// ! - As a workaround, we construct a supertrait `Duck` inheriting from both required traits.
// ! - Blanket implementation automatically makes any matching type implement `Duck`.

// ---

// Trait contains a `copy_by_val` method returning Self by value.
// This breaks Object Safety (dynamic compatibility).
// But we isolate it using `where Self: Sized`!
trait SafeTrait {
    fn name(&self) -> String;
    
    // This method will not be included in the vtable and won't break dyn SafeTrait
    fn copy_by_val(self) -> Self where Self: Sized {
        self
    }
}

struct Item;
impl SafeTrait for Item {
    fn name(&self) -> String { "Item".to_string() }
}

fn print_name(obj: &dyn SafeTrait) {
    println!("Name: {}", obj.name());
    // obj.copy_by_val(); // Error! Method unavailable for trait object.
}

fn main() {
    let item = Item;
    print_name(&item);
}

// ! **Object Safety**
// ! - Trait objects can only be created for Object-Safe traits (no Self by value, no generic methods, etc.).
// ! - The `where Self: Sized` constraint allows excluding problematic methods from the vtable.
// ! - The trait remains object-safe, while "unsafe" methods can only be invoked on static types.
