// Шаг 1: Информативные утверждения в тесте с контекстными сообщениями

pub fn calculate_health_index(load: f64, errors: usize) -> f64 {
    if load > 100.0 {
        return 0.0;
    }
    let error_penalty = (errors as f64) * 2.5;
    (100.0 - load - error_penalty).max(0.0)
}

fn main() {
    let index = calculate_health_index(20.0, 2);
    println!("Вычисленный индекс здоровья системы: {index}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn health_index_calculates_correctly_under_normal_load() {
        let load = 20.0;
        let errors = 2;
        let want = 75.0; // 100 - 20 - 5.0
        let got = calculate_health_index(load, errors);

        // Информативное сообщение с подробными данными о несовпадении:
        assert_eq!(
            got, want,
            "Индекс здоровья рассчитан неверно для load={load}, errors={errors}: want '{want}', got '{got}'"
        );
    }
}

// ! **1. Информативные юнит-тесты**
// ! - Тесты размещаются в подмодуле `#[cfg(test)] mod tests`, который не компилируется в релизную сборку.
// ! - Использование расширенных контекстных сообщений в `assert_eq!` позволяет мгновенно понять причину сбоя без дебаггера.

// ---

// Шаг 2: Техника Bebugging и самодокументируемые тесты (cargo-testdox)

// В рамках техники Bebugging разработчик намеренно вносит ошибку в формулу
// для проверки того, что тесты действительно падают с понятным отчетом:
pub fn calculate_discount_price(original_price: f64, discount_pct: f64) -> f64 {
    // Намеренный баг для Bebugging: умножение вместо вычитания скидки
    // original_price * (1.0 - discount_pct / 100.0)
    original_price // ОШИБКА: вернет исходную цену без скидки!
}

fn main() {
    let price = calculate_discount_price(200.0, 10.0);
    println!("Итоговая цена: {price}");
}

#[cfg(test)]
mod bebugging_tests {
    use super::*;

    // Именование функций в стиле самодокументируемых предложений для cargo-testdox:
    #[test]
    fn discount_price_applies_percentage_deduction_properly() {
        let price = 200.0;
        let discount = 10.0;
        let want = 180.0;
        let got = calculate_discount_price(price, discount);

        assert_eq!(
            got, want,
            "Bebugging check: test successfully caught the error! want '{want}', got '{got}'"
        );
    }
}

// ! **2. Bebugging и cargo-testdox**
// ! - **Bebugging**: намеренная поломка кода для подтверждения того, что тесты чувствительны к багам.
// ! - **cargo-testdox**: именование тестов полными предложениями генерирует спецификацию требований к системе.
