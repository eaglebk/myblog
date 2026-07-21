// Эмуляция разбора AST через syn и генерации кода через quote
struct FieldInfo {
    name: &'static str,
    ty: &'static str,
}

struct ParsedStruct {
    struct_name: &'static str,
    fields: Vec<FieldInfo>,
}

fn simulate_syn_parse() -> ParsedStruct {
    // Библиотека syn превращает сырой TokenStream в типизированную AST-структуру DeriveInput
    ParsedStruct {
        struct_name: "Config",
        fields: vec![
            FieldInfo { name: "db_url", ty: "String" },
            FieldInfo { name: "max_connections", ty: "u32" },
        ],
    }
}

fn simulate_quote_generate(ast: &ParsedStruct) -> String {
    // Библиотека quote! { ... } генерирует конечный код на основе разобранного AST
    let mut code = format!("impl Default for {} {{\n  fn default() -> Self {{\n    Self {{\n", ast.struct_name);
    for field in &ast.fields {
        code.push_str(&format!("      {}: Default::default(),\n", field.name));
    }
    code.push_str("    }\n  }\n}");
    code
}

fn main() {
    let ast = simulate_syn_parse();
    let generated_code = simulate_quote_generate(&ast);

    println!("--- Сгенерированный код через syn & quote ---");
    println!("{generated_code}");
}

// ! **1. Связка syn и quote**
// ! - `syn::parse::<DeriveInput>(tokens)` парсит исходные лексемы кода в выверенное AST-дерево.
// ! - `quote! { ... }` генерирует готовые токены Rust для вставки в итоговую бинарную сборку.

// ---

// Эмуляция работы Function-like макроса sql_query!("SELECT * FROM users WHERE id = $1")
struct SqlQuery {
    raw_query: &'static str,
    params_count: usize,
}

impl SqlQuery {
    fn new(query: &'static str) -> Self {
        let count = query.matches('$').count();
        SqlQuery { raw_query: query, params_count: count }
    }
}

fn main() {
    // В SQL-библиотеках (например, sqlx!) процедурный макрос sql!("SELECT...")
    // проверяет валидность синтаксиса базы данных на этапе КОМПИЛЯЦИИ:
    let q = SqlQuery::new("SELECT name, email FROM users WHERE id = $1 AND active = $2");

    println!("Проверенный запрос: '{}'", q.raw_query);
    println!("Ожидается параметров на вход: {}", q.params_count);
}

// ! **2. Идея Function-like макросов**
// ! - Вызываются как `my_macro!(...)` и принимают произвольный поток токенов в скобках.
// ! - Способны выполнять статическую валидацию запросов к БД, регулярных выражений или конфигураций прямо при сборке.
