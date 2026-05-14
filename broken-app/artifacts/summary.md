# Сводка по замерам

## Бенчмарк baseline

| бенчмарк | до (среднее) | после (среднее) | ускорение |
| --- | ---: | ---: | ---: |
| sum_even | 6.847 мкс | 6.292 мкс | 1.1× |
| slow_fib | 3533.917 мкс | 0.083 мкс | 42 407× |
| slow_dedup | 6535.806 мкс | 61.930 мкс | 105.5× |

## Criterion (после)

- `sum_even_broken`: `6.9527 мкс / 7.0390 мкс / 7.1496 мкс`
- `slow_fib_broken`: `13.304 нс / 13.417 нс / 13.556 нс`
- `slow_dedup_broken`: `66.355 мкс / 68.399 мкс / 70.894 мкс`

## Инструменты безопасности

- `cargo test`: зелёные прогоны на macOS (`cargo_test_after.log`) и под
  Linux-контейнером (`cargo_test_linux.log`); исходный падающий прогон
  сохранён в `cargo_test_before.log`.
- `cargo +nightly miri test`: успешно (`miri.log`).
- `valgrind` по `target/debug/demo`: `0` definite/indirect/possible-утечек,
  `0` ошибок; остаётся один блок `still reachable` от std-рантайма
  (`valgrind_binary.log`).
- `ASan` по `demo`: завершился без находок (`asan.log`).
- `TSan` по `thread_demo`: завершился без репортов о гонках (`tsan.log`).
- `gdb` с брейкпоинтом на `average_positive`: лог в
  `gdb_average_positive.log`.
- `perf` недоступен под Docker Desktop (ограничения ядра); как замена
  снят профиль Callgrind: `callgrind.out` и `callgrind_annotate.txt`.

## Заметка по профилированию

- В `callgrind_annotate.txt` видно, что `broken_app::algo::slow_dedup`
  больше не доминирует по количеству инструкций в оптимизированном
  сценарии `demo`.
