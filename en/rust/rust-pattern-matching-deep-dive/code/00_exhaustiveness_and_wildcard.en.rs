// ?compile_fail
enum ConnectionState {
    Disconnected,
    Connecting,
    Connected { ip: String },
}

fn handle_connection(state: ConnectionState) {
    match state {
        ConnectionState::Disconnected => println!("Offline"),
        ConnectionState::Connected { ip } => println!("Connected to {ip}"),
        // Compile error: `Connecting` variant is not covered!
    }
}

fn main() {
    let state = ConnectionState::Connecting;
    handle_connection(state);
}

// ! **1. Compile Error: Non-exhaustive match**
// ! - The Rust compiler strictly enforces exhaustiveness checks across all enum variants.
// ! - The `Connecting` variant is missing, so compilation fails with error E0004.

// ---

enum ConnectionState {
    Disconnected,
    Connecting,
    Connected { ip: String },
}

fn handle_connection(state: ConnectionState) {
    match state {
        ConnectionState::Disconnected => println!("Offline"),
        ConnectionState::Connected { ip } => println!("Connected to {ip}"),
        // The `_` wildcard catches all remaining variants:
        _ => println!("Other state"),
    }
}

fn main() {
    handle_connection(ConnectionState::Disconnected);
    handle_connection(ConnectionState::Connecting);
    handle_connection(ConnectionState::Connected {
        ip: String::from("192.168.1.1"),
    });
}

// ! **2. Fixing with the `_` wildcard**
// ! - The `_` pattern catches any unhandled variants, satisfying exhaustiveness checks.
// ! - Although this compiles, overusing `_` is discouraged: when new enum variants are added, the compiler will not warn you to handle them explicitly.
