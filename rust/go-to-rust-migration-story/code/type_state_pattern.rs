struct Disconnected;
struct Connected {
    session_token: String,
}

struct ServiceClient<State> {
    target_url: String,
    state: State,
}

impl ServiceClient<Disconnected> {
    fn new(url: &str) -> Self {
        ServiceClient {
            target_url: url.to_string(),
            state: Disconnected,
        }
    }

    // Переход в подключенное состояние поглощает (consume) текущий объект
    fn connect(self, token: &str) -> ServiceClient<Connected> {
        ServiceClient {
            target_url: self.target_url,
            state: Connected {
                session_token: token.to_string(),
            },
        }
    }
}

impl ServiceClient<Connected> {
    fn send_request(&self, payload: &str) -> String {
        format!(
            "Отправка '{}' на {} с токеном '{}'",
            payload, self.target_url, self.state.session_token
        )
    }
}

fn main() {
    let client = ServiceClient::new("https://api.eagleblog.space");
    
    // Ошибка компиляции, если попробовать вызвать send_request у Disconnected!
    // client.send_request("hello"); // -> method not found in `ServiceClient<Disconnected>`

    let connected_client = client.connect("secret-token-123");
    let response = connected_client.send_request("GET /status");
    println!("{}", response);
}
// ! **Шаблон Typestate: Невозможные состояния некомпилируемы**
// ! - В Go проверка подключения обычно делается через runtime-флаг `if !c.isConnected { return err }`.
// ! - В Rust состояния выносятся в типы. Метод `send_request` существует ТОЛЬКО для `ServiceClient<Connected>`.
// ! - Попытка вызова метода не в том состоянии не скомпилируется — ошибки отсекаются на этапе сборки.

// ---

struct RawPayload;
struct ValidatedPayload(String);

struct Transaction<State> {
    data: State,
}

impl Transaction<RawPayload> {
    fn new() -> Self {
        Transaction { data: RawPayload }
    }

    fn validate(self, input: &str) -> Result<Transaction<ValidatedPayload>, &'static str> {
        if input.is_empty() {
            Err("Данные не могут быть пустыми")
        } else {
            Ok(Transaction {
                data: ValidatedPayload(input.to_uppercase()),
            })
        }
    }
}

impl Transaction<ValidatedPayload> {
    fn commit(&self) {
        println!("Транзакция выполнена с данными: {}", self.data.0);
    }
}

fn main() {
    let tx = Transaction::new();
    if let Ok(valid_tx) = tx.validate("pay 100 usd") {
        valid_tx.commit();
    }
}
// ! **Строгое цепочное подтверждение транзакций**
// ! - Валидация возвращает новый тип `Transaction<ValidatedPayload>`.
// ! - Метод `commit()` физически недоступен у `Transaction<RawPayload>`.
// ! - Для ИИ-агентов такой синтаксис служит непробиваемым ограждением: модель просто не сможет случайно отправить невалидированную транзакцию.
