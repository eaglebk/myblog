// Step 1: Rules 1 and 2 — Single Input Reference
fn first_word(s: &str) -> &str {
    // No need to write <'a>(s: &'a str) -> &'a str.
    // The compiler assigns s lifetime 'a (Rule 1)
    // and passes the same lifetime 'a to the returned slice (Rule 2).
    s.split_whitespace().next().unwrap_or("")
}

fn main() {
    let greeting = String::from("Rust 2026: Concurrency");
    let word = first_word(&greeting);
    
    println!("First word: {word}");
}

// ! **1. Rules 1 and 2: Single Input Reference**
// ! - Rule 1: The compiler assigns an independent lifetime parameter to each input reference.
// ! - Rule 2: If there is exactly one input lifetime parameter, that lifetime is assigned to all output references.
// ! - The signature stays clean and readable without manual `'a` annotations.

// ---

// Step 2: Rule 3 — Struct Methods with &self

// Rust requires explicit <'a> on the struct definition so the compiler knows:
// a LogEntry instance must not outlive the string slice it borrows.
struct LogEntry<'a> {
    message: &'a str,
}

impl<'a> LogEntry<'a> {
    // In method signatures, manual lifetime annotations are omitted.
    // Even with two reference parameters (&self and _fallback: &str),
    // Rule 3 binds the returned reference directly to &self.
    // rustc treats this signature as: fn extract_tag<'b>(&'b self, _fallback: &str) -> &'b str
    fn extract_tag(&self, _fallback: &str) -> &str {
        if self.message.starts_with('[') {
            if let Some(end) = self.message.find(']') {
                return &self.message[1..end];
            }
        }
        "GENERAL"
    }
}

fn main() {
    let entry = LogEntry {
        message: "[AUTH] User authenticated successfully",
    };
    let default_tag = "DEFAULT";
    
    let tag = entry.extract_tag(default_tag);
    println!("Log entry tag: {tag}");
}

// ! **2. Rule 3: Methods with &self or &mut self**
// ! - Why does the struct have `'a`? The `message` field holds a borrowed `&'a str`, requiring an annotation on the type.
// ! - The `extract_tag` method needs no annotations: Rule 3 automatically binds the output lifetime to `&self`.
// ! - The compiler does not confuse `&self` with `_fallback`: return lifetime priority always belongs to the struct instance.

// ---

// Step 3: When Elision Cannot Help — Ambiguity
fn choose_longest<'a>(first: &'a str, second: &'a str) -> &'a str {
    // Two input references, one output reference. The compiler cannot know
    // which branch executes at runtime, so it requires explicit <'a> annotations.
    if first.len() > second.len() {
        first
    } else {
        second
    }
}

fn main() {
    let word1 = "Ferris";
    let word2 = "Rustacean";

    let winner = choose_longest(word1, word2);
    println!("Longer word: {winner}");
}

// ! **3. Handling Ambiguity: When Elision Cannot Infer Lifetimes**
// ! - When a function takes multiple independent references and returns one, Rule 2 does not apply.
// ! - The signature `fn choose_longest<'a>(x: &'a str, y: &'a str) -> &'a str` states:
// ! - The returned reference lives as long as the shorter of the two input lifetimes.
