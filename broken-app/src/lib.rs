pub mod algo;
pub mod concurrency;

/// Сумма чётных значений.
/// Безопасный проход по срезу без выхода за границы.
pub fn sum_even(values: &[i64]) -> i64 {
    values.iter().copied().filter(|v| v % 2 == 0).sum()
}

/// Подсчёт ненулевых байтов без промежуточной аллокации и утечки.
pub fn leak_buffer(input: &[u8]) -> usize {
    input.iter().filter(|b| **b != 0).count()
}

/// Нормализация строки: убираем все пробельные символы и приводим к нижнему регистру.
pub fn normalize(input: &str) -> String {
    input.split_whitespace().collect::<String>().to_lowercase()
}

/// Среднее только по положительным элементам.
pub fn average_positive(values: &[i64]) -> f64 {
    let (sum, count) = values
        .iter()
        .copied()
        .filter(|v| *v > 0)
        .fold((0_i64, 0_usize), |(sum, count), value| {
            (sum + value, count + 1)
        });

    if count == 0 {
        return 0.0;
    }

    sum as f64 / count as f64
}

/// Исторически здесь был use-after-free; оставляем ту же вычислимую семантику без UB.
pub fn use_after_free() -> i32 {
    let value = Box::new(42_i32);
    *value * 2
}
