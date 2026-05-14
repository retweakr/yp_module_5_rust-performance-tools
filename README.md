# Проект «Rust Performance Tools»

Репозиторий содержит два крейта:

- `broken-app` — крейт, который чинился, проверялся и оптимизировался;
- `reference-app` — эталонная реализация для сверки поведения.

Корень репозитория описывает оба крейта как Cargo workspace, поэтому
сборка одной командой: `cargo build --workspace`.

## Что исправлено

В `broken-app` были дефекты корректности, безопасности памяти и
многопоточности. В итоговой версии исправлено:

- off-by-one / UB в `sum_even`;
- утечка в `leak_buffer`;
- обработка пробельных символов в `normalize`;
- усреднение только положительных в `average_positive`;
- бывший путь `use_after_free`;
- гонка данных через `static mut` в `concurrency`.

Добавлены регрессионные тесты на пустой срез, нормализацию пробелов,
случай без положительных значений, корректность многопоточного
инкремента, сброс счётчика и бывший `use_after_free`.

## Что оптимизировано

Переписаны две горячие функции в `broken-app/src/algo.rs`:

- `slow_fib`: экспоненциальная рекурсия → итеративная реализация за O(n);
- `slow_dedup`: повторные линейные сканы и сортировка на каждой вставке →
  `HashSet` + заранее выделенный `Vec` + одна финальная сортировка.

Сводка по замерам — в `broken-app/artifacts/summary.md`.

Ключевые цифры из `broken-app/artifacts/baseline_before.txt` против
`broken-app/artifacts/baseline_after.txt`:

- `slow_fib`: ~`3.53 мс` → `0.083 мкс` (~`42 600×`);
- `slow_dedup`: ~`6.54 мс` → `61.9 мкс` (~`106×`);
- `sum_even`: стабильно около `6.8 мкс`.

## Проверки

Локально (macOS):

```bash
cargo build --workspace
cargo test -p broken-app
cargo test -p reference-app
```

Динамический анализ под Linux-контейнером:

```bash
cargo +nightly miri test
valgrind --leak-check=full target/debug/demo
RUSTFLAGS="-Zsanitizer=address" cargo +nightly run -Zbuild-std --bin demo
CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUSTFLAGS="-Zsanitizer=thread" \
  cargo +nightly run -Zbuild-std --target aarch64-unknown-linux-gnu --bin thread_demo
```

Сессия отладчика:

```bash
gdb target/debug/logic_probe
```

## Артефакты

Все собранные выводы лежат в `broken-app/artifacts/`.

Самые важные:

- `summary.md` — сводка по результатам;
- `cargo_test_before.log` — падающий прогон тестов «до»;
- `cargo_test_after.log` — итоговый зелёный прогон тестов на macOS;
- `cargo_test_linux.log` — зелёный прогон тестов под Linux;
- `cargo_build_workspace.log` — лог `cargo build --workspace` из корня;
- `miri.log` — успешный прогон Miri;
- `valgrind_binary.log` — чистый прогон Valgrind по бинарю demo;
- `asan.log` — прогон ASan по `demo`;
- `tsan.log` — прогон TSan по `thread_demo`;
- `gdb_average_positive.log` — сессия отладчика;
- `baseline_before.txt` / `baseline_after.txt` — бенчмарки до/после;
- `criterion_after.txt` — вывод Criterion;
- `callgrind.out` / `callgrind_annotate.txt` — профиль через Callgrind
  (использовался как замена `perf` под Docker Desktop).

## Замечания

- `reference-app` сохранён как семантический эталон, никаких правок.
- Kernel-счётчики `perf` недоступны внутри Docker Desktop, поэтому
  профилирование снято через Callgrind.
- `valgrind.log` — прогон через тестовый раннер Rust;
  `valgrind_binary.log` — чистая проверка памяти по приложению,
  по которой и сделан финальный вывод.
- `cargo build --workspace` собирается, но Cargo предупреждает про
  одинаковое имя бинаря `demo` в обоих крейтах; на результат сборки
  это не влияет.
