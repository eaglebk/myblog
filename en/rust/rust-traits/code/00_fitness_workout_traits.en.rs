// ?compile_fail
struct Pushups {
    reps: u32,
}

impl Pushups {
    fn calories_burned(&self) -> u32 {
        self.reps * 1
    }
}

// Attempting to write a generic function WITHOUT a trait bound T
fn total_calories<T>(exercises: &[T]) -> u32 {
    exercises.iter().map(|e| e.calories_burned()).sum()
}

fn main() {
    let pushups = vec![Pushups { reps: 20 }, Pushups { reps: 30 }];
    let total = total_calories(&pushups);
    println!("Total calories: {}", total);
}

// ! **Step 1: Compile error without trait bound**
// ! - Attempting to invoke method `calories_burned()` on generic type `T` without contract declaration.
// ! - The compiler does not know if type `T` possesses such a method and blocks compilation.

// ---

pub trait Workout {
    fn calories_burned(&self) -> u32;
}

struct Pushups {
    reps: u32,
}

struct Plank {
    duration_sec: u32,
}

impl Workout for Pushups {
    fn calories_burned(&self) -> u32 {
        self.reps * 1
    }
}

impl Workout for Plank {
    fn calories_burned(&self) -> u32 {
        self.duration_sec * 2
    }
}

// Unified generic function with Trait Bound constraint (T: Workout)
fn total_calories<T: Workout>(exercises: &[T]) -> u32 {
    exercises.iter().map(|e| e.calories_burned()).sum()
}

fn main() {
    let pushups = vec![Pushups { reps: 20 }, Pushups { reps: 30 }];
    let planks = vec![Plank { duration_sec: 60 }, Plank { duration_sec: 45 }];

    println!("Pushup calories: {}", total_calories(&pushups));
    println!("Plank calories: {}", total_calories(&planks));
}

// ! **Step 2: Resolution with explicit Workout trait**
// ! - Trait `Workout` declares mandatory contract: presence of `calories_burned` method.
// ! - Structs `Pushups` and `Plank` implement trait via `impl Workout for ...`.
// ! - Function `total_calories` accepts any type `T: Workout`, while compiler generates optimized code for each type with zero runtime overhead.
