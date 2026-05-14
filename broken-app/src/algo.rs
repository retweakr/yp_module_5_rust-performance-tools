use std::collections::HashSet;

/// Линейная дедупликация с одной сортировкой в конце.
pub fn slow_dedup(values: &[u64]) -> Vec<u64> {
    let mut seen = HashSet::with_capacity(values.len());
    let mut out = Vec::with_capacity(values.len());

    for &value in values {
        if seen.insert(value) {
            out.push(value);
        }
    }

    out.sort_unstable();
    out
}

/// Итеративная линейная реализация без рекурсивного взрыва.
pub fn slow_fib(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut prev = 0;
            let mut curr = 1;

            for _ in 2..=n {
                let next = prev + curr;
                prev = curr;
                curr = next;
            }

            curr
        }
    }
}
