// Шаг 1: Структуры, хранящие заимствованные данные
struct UserProfile<'a> {
    username: &'a str,
    role: &'a str,
}

impl<'a> UserProfile<'a> {
    fn summary(&self) -> String {
        format!("{}: {}", self.role, self.username)
    }

    fn name_slice(&self) -> &'a str {
        // Возвращаем ссылку, связанную с временем жизни данных профиля
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

    println!("Карточка пользователя: {}", profile.summary());
    println!("Имя: {}", profile.name_slice());
}

// ! **1. Структуры с ссылками**
// ! - Структура `UserProfile<'a>` не владеет своими строками, а заимствует их через `&'a str`.
// ! - Аннотация `'a` гарантирует, что `UserProfile` не сможет пережить переменные `username` и `role`.

// ---

// Шаг 2: Защита компилятора от висячих указателей (Dangling Reference)
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
        println!("Датчик {} зафиксировал {}°C", reading.device_id, reading.temp_celsius);
    }

    // Здесь device_name всё ещё жив, поэтому чтение безопасно
    println!("Проверка статуса: датчик {} в норме", reading.device_id);
}

// ! **2. Защита от висячих ссылок (Dangling Reference)**
// ! - Компилятор строго проверяет границы жизни: нельзя сохранить ссылку на значение, которое умрёт раньше структуры.
// ! - Если переместить создание `device_name` внутрь фигурных скобок, компилятор мгновенно пресечет утечку висячей ссылки.
