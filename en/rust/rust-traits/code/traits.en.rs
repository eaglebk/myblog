pub trait Say {
    fn say(&self) -> String;
}

struct Dog;

impl Say for Dog {
    fn say(&self) -> String {
        "Woof!".to_string()
    }
}

fn main() {
    let dog = Dog;
    println!("Dog says: {}", dog.say());
}

// ! **Trait Declaration and Implementation**
// ! - Trait `Say` describes contract: any type implementing it must be able to speak.
// ! - Struct `Dog` implements method `say` from trait `Say`.
// ! - After implementation, method becomes available on the struct using dot notation.

// ---

pub trait HasId {
    const ID: usize;
    type Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> where Self: Sized;
}

struct MyStruct;

impl HasId for MyStruct {
    const ID: usize = 10;
    type Err = std::convert::Infallible;
    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(MyStruct)
    }
}

fn main() {
    println!("Struct ID: {}", MyStruct::ID);
    let _s = MyStruct::from_str("test").unwrap();
}

// ! **Associated Items**
// ! - Constants (`const ID`) and types (`type Err`) are declared inside trait and refined in implementations.
// ! - Associated type acts as a type placeholder concrete upon implementation.
// ! - Avoids polluting trait method signatures with extra generic parameters.

// ---

// ?hidden:start
use std::fmt;
// ?hidden:end

// We cannot implement Display for Vec<i32> directly as both are foreign types.
// But we can wrap Vec inside our custom tuple struct (Newtype)!
struct MyVec(Vec<i32>);

impl fmt::Display for MyVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "My vector with {} elements", self.0.len())
    }
}

fn main() {
    let my_vec = MyVec(vec![100, 200, 300]);
    println!("{}", my_vec);
}

// ! **Newtype Pattern (Wrapper)**
// ! - Orphan Rules forbid implementing foreign traits for foreign types.
// ! - Creating wrapper struct `MyVec` makes the type local.
// ! - Now we can implement any external trait for it (such as `Display`).

// ---

// Trait used as a constraint
trait Loud {
    fn loud_say(&self) -> String;
}

// Generic struct
struct Point<T> {
    x: T,
    y: T,
}

// Blanket implementation: implement Loud for any Point<T> if T implements Loud
impl<T: Loud> Loud for Point<T> {
    fn loud_say(&self) -> String {
        format!("X: {}, Y: {}", self.x.loud_say(), self.y.loud_say())
    }
}

struct Dog;
impl Loud for Dog {
    fn loud_say(&self) -> String {
        "WOOF!".to_string()
    }
}

fn main() {
    let p = Point { x: Dog, y: Dog };
    println!("Point shouts: {}", p.loud_say());
}

// ! **Generics and Blanket Implementations**
// ! - Trait bounds `T: Loud` guarantee that type `T` supports required operations.
// ! - Blanket implementation allows implementing trait for a whole family of matching types at once.
// ! - Compiler monomorphizes generic code, generating separate functions for each concrete type.

// ---

// ?hidden:start
use std::mem::size_of;
// ?hidden:end

trait A {
    fn get(&self) -> &str;
}

trait B {
    fn get(&self) -> &str;
}

struct S;

impl A for S {
    fn get(&self) -> &str { "implementation A" }
}

impl B for S {
    fn get(&self) -> &str { "implementation B" }
}

fn main() {
    let s = S;
    // s.get(); // Compile error: method name get is ambiguous!
    
    // Fully Qualified Syntax resolves the conflict:
    let from_a = <S as A>::get(&s);
    let from_b = <S as B>::get(&s); 
    
    // Turbofish for explicit type specification:
    let float_size = size_of::<f64>();
    
    println!("A: {}, B: {}, f64 size: {}", from_a, from_b, float_size);
}

// ! **Turbofish and Fully Qualified Syntax**
// ! - Turbofish `::<>` hints types to compiler where inference cannot resolve automatically.
// ! - Fully Qualified Syntax `<S as A>::get` resolves collisions when a single type implements multiple traits with identical method names.
