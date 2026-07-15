#[derive(Debug)]
struct Sensor {
    id: String,
    temperature: f32,
}

fn main() {
    let sensors = vec![
        "sensor1 22.5",
        "sensor2 35.7",
        "sensor3 29.8",
        "sensor4 41.3",
        "sensor5 30.0"
    ];

    let critical_threshold = 30.0;
    let mut critical_sensors = vec![];

    // Проходимся циклом по датчикам
    for s in sensors {
        let mut s = s.split(' ');
        let id = s.next();
        let temperature = s.next();

        if id.is_some() && temperature.is_some() {
            let id = id.unwrap().to_owned();
            let temperature = temperature.unwrap().parse::<f32>();

            if temperature.is_ok() {
                let temperature = temperature.unwrap();
                if temperature > critical_threshold {
                    critical_sensors.push(Sensor { id, temperature });
                }
            }
        }
    }

    for s in critical_sensors {
        println!("{:?}", s);
    }
}
// ! **Императивный стиль**
// ! - Мы вручную разбираем строки, выполняем проверки и наполняем изменяемый вектор `critical_sensors`.
// ! - Код требует явного управления состоянием и частых вызовов `unwrap()`.

// ---

#[derive(Debug)]
struct Sensor {
    id: String,
    temperature: f32,
}

fn main() {
    let sensors = vec![
        "sensor1 22.5",
        "sensor2 35.7",
        "sensor3 29.8",
        "sensor4 41.3",
        "sensor5 30.0"
    ];

    let critical_threshold = 30.0;

    // Функциональный стиль с цепочкой итераторов
    let critical_sensors: Vec<Sensor> = sensors.iter()
        .map(|s| {
            let mut s = s.split(' ');
            let id = s.next()?.to_owned();
            let temperature = s.next()?.parse::<f32>().ok()?;
            Some(Sensor { id, temperature })
        })
        .flatten()
        .filter(|s| s.temperature > critical_threshold)
        .collect();

    for s in critical_sensors {
        println!("{:?}", s);
    }
}
// ! **Функциональный стиль**
// ! - Использование цепочки методов (`map`, `flatten`, `filter`) избавляет от необходимости мутировать вектор вручную.
// ! - Метод `collect()` собирает элементы итератора обратно в результирующий вектор `Vec<Sensor>`.
