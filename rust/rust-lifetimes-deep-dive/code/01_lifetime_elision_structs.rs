// Шаг 1: Правила опускания времен жизни (Lifetime Elision Rules)
fn first_word<'a>(s: &'a str) -> &'a str {
    // Явная запись 'a (компилятор делает это автоматически благодаря Правилу 1 и 2)
    s.split_whitespace().next().unwrap_or("")
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // Когда входных параметров с разными ссылками несколько, компилятору нужна явная метка 'a
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let text1 = "Кот";
    let text2 = "Собака";
    
    let result = longest(text1, text2);
    println!("Самое длинное слово: {result}");
    println!("Первое слово: {}", first_word(text2));
}

// ! **1. Автоматические правила опускания (Elision Rules)**
// ! - Правило 1: Каждому параметру-ссылке компилятор присваивает свой lifetime (`fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`).
// ! - Правило 2: Если входной параметр-ссылка ровно один, его lifetime присваивается всем выходным ссылкам.
// ! - Если ссылок несколько (как в `longest`), мы обязаны явно связать выходящую ссылку с входящими.

// ---

// Шаг 2: Хранение ссылок внутри структур данных
struct TextBuffer<'a> {
    content: &'a str,
}

impl<'a> TextBuffer<'a> {
    fn preview(&self) -> &'a str {
        self.content
    }
}

fn main() {
    let data = String::from("Данные сенсора питомца");
    let buffer = TextBuffer { content: &data };

    println!("Предпросмотр буфера: {}", buffer.preview());
}

// ! **2. Ссылки внутри структур**
// ! - Если структура содержит ссылку `&'a str`, она не владеет данными, а лишь заимствует их.
// ! - Аннотация `'a` гарантирует, что структура `TextBuffer` не проживет дольше, чем `data`.
