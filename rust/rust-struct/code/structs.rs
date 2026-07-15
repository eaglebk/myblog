#[allow(dead_code)]
#[derive(Debug)]
struct Hub {
    name: String,
    id: String,
    location: String,
    sensors: Vec<Sensor>,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Sensor {
    name: String,
    id: String,
    sensor_type: String,
    value: f32,
    unit: String,
    is_active: bool,
}

fn main() {
    let hub = Hub {
        name: "Main Hub".to_string(),
        id: String::from("HUB001"),
        location: "Room 101".to_string(),
        sensors: Vec::new(), // пустой список сенсоров
    };

    println!("{:#?}", hub);
}
// ! **Объявление и базовая инициализация структур**
// ! - Структуры определяются с помощью ключевого слова `struct`.
// ! - Атрибут `#[derive(Debug)]` позволяет выводить структуру в отладочном формате с помощью `{:#?}`.

// ---

#[allow(dead_code)]
#[derive(Debug)]
struct Hub {
    name: String,
    id: String,
    location: String,
    sensors: Vec<Sensor>,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Sensor {
    name: String,
    id: String,
    sensor_type: String,
    value: f32,
    unit: String,
    is_active: bool,
}

fn main() {
    let sensor1 = Sensor {
        name: "Temperature Sensor".to_string(),
        id: "TEMP001".to_string(),
        sensor_type: "Temperature".to_string(),
        value: 25.3,
        unit: "°C".to_string(),
        is_active: true,
    };

    let sensor2 = Sensor {
        name: "Humidity Sensor".to_string(),
        id: "HUM001".to_string(),
        sensor_type: "Humidity".to_string(),
        value: 60.5,
        unit: "%".to_string(),
        is_active: true,
    };

    let hub = Hub {
        name: "Main Hub".to_string(),
        id: String::from("HUB001"),
        location: "Room 101".to_string(),
        sensors: vec![sensor1, sensor2],
    };

    println!("{:#?}", hub);
}
// ! **Структуры со вложенными коллекциями**
// ! - Мы создали экземпляры `Sensor` и передали их в поле `sensors` структуры `Hub`.
// ! - Макрос `vec!` используется для удобной инициализации вектора `Vec<Sensor>`.
