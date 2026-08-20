// Step 1: Associated Types and Associated Constants

trait Container {
    // Associated type determined by the implementation:
    type Item;

    // Associated constant:
    const CAPACITY_LIMIT: usize = 100;

    fn add(&mut self, item: Self::Item);
    fn count(&self) -> usize;
}

struct IntStack {
    items: Vec<i32>,
}

impl Container for IntStack {
    type Item = i32;

    fn add(&mut self, item: Self::Item) {
        if self.items.len() < Self::CAPACITY_LIMIT {
            self.items.push(item);
        }
    }

    fn count(&self) -> usize {
        self.items.len()
    }
}

fn main() {
    let mut stack = IntStack { items: vec![] };
    stack.add(42);
    println!("Stack size: {}, Max limit: {}", stack.count(), IntStack::CAPACITY_LIMIT);
}

// ! **1. Associated Types and Constants**
// ! - `type Item;` binds an internal type to the trait without cluttering function signatures with generic parameters like `trait Container<T>`.
// ! - `const CAPACITY_LIMIT: usize` defines a constant scoped to the trait namespace.

// ---

// Step 2: Fully Qualified Syntax (<Type as Trait>::method) for Disambiguation
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("Pilot: take off into the sky!");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Wizard: fly on a broomstick!");
    }
}

impl Human {
    fn fly(&self) {
        println!("Human: waving arms on the ground.");
    }
}

fn main() {
    let person = Human;

    // 1. Calling the type's inherent method:
    person.fly();

    // 2. Using Fully Qualified Syntax to call a specific trait method:
    Pilot::fly(&person);
    Wizard::fly(&person);
    <Human as Pilot>::fly(&person);
}

// ! **2. Fully Qualified Syntax (<Type as Trait>::method)**
// ! - Disambiguates method calls when a type implements multiple traits with identical method names.
// ! - The `<Type as Trait>::function(...)` syntax explicitly tells the compiler which implementation to invoke.
