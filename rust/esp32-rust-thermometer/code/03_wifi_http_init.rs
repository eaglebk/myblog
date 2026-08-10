// ?hidden:start
#[derive(Debug)]
pub enum HardwareError {
    WifiFailed,
    ServerFailed,
}

pub struct WifiDriver;
pub struct HttpServer;

impl WifiDriver {
    pub fn connect() -> Result<Self, HardwareError> {
        Ok(Self)
    }
}

impl HttpServer {
    pub fn bind(_wifi: &WifiDriver) -> Result<Self, HardwareError> {
        Ok(Self)
    }
}
// ?hidden:end

fn init_esp32_iot_node() -> Result<(WifiDriver, HttpServer), HardwareError> {
    // Оператор ? передает ошибку вверх, если инициализация периферии не удалась
    let wifi = WifiDriver::connect()?;
    let server = HttpServer::bind(&wifi)?;

    println!("✅ Wi-Fi подключен, HTTP-сервер успешно запущен!");
    Ok((wifi, server))
}

fn main() {
    match init_esp32_iot_node() {
        Ok((_wifi, _server)) => println!("🌐 Термометр доступен в локальной сети!"),
        Err(err) => println!("❌ Ошибка запуска железа: {:?}", err),
    }
}

// ! **Оператор ? и Result для инициализации периферии**
// ! - Оператор `?` лаконично прерывает цепочку инициализации в случае аппаратного сбоя.
// ! - В `no_std` и embedded-разработке явная передача `Result` гарантирует безопасность без паник.
