// Step 1: Structs holding borrowed data
struct UserProfile<'a> {
    username: &'a str,
    role: &'a str,
}

impl<'a> UserProfile<'a> {
    fn summary(&self) -> String {
        format!("{}: {}", self.role, self.username)
    }

    fn name_slice(&self) -> &'a str {
        // Returns a reference tied to the lifetime of the profile's underlying data
        self.username
    }
}

fn main() {
    let username = String::from("ferris_developer");
    let role = String::from("admin");

    let profile = UserProfile {
        username: &username,
        role: &role,
    };

    println!("User profile: {}", profile.summary());
    println!("Username: {}", profile.name_slice());
}

// ! **1. Structs with references**
// ! - `UserProfile<'a>` does not own its strings; it borrows them through `&'a str`.
// ! - The `'a` annotation ensures `UserProfile` cannot outlive the `username` and `role` variables.

// ---

// Step 2: Preventing dangling references
struct SensorReading<'a> {
    device_id: &'a str,
    temp_celsius: f32,
}

fn main() {
    let reading: SensorReading;
    let device_name = String::from("ESP32_LIVING_ROOM");

    {
        reading = SensorReading {
            device_id: &device_name,
            temp_celsius: 23.5,
        };
        println!("Sensor {} registered {}°C", reading.device_id, reading.temp_celsius);
    }

    // device_name is still alive here, so accessing the reading is safe
    println!("Status check: sensor {} is operational", reading.device_id);
}

// ! **2. Dangling reference prevention**
// ! - The compiler verifies lifetime boundaries: you cannot keep a reference to a value that drops before the struct.
// ! - If `device_name` were declared inside the inner block, rustc would block the dangling reference at compile time.
