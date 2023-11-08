# sha3

## Результати бенчарків
У бенчмарках було виконано [criterion.rs](https://github.com/bheisler/criterion.rs). Для порівняння продуктивності розробленого алгоритму, були взяті дві найпопулярніші реалізації алгоритму SHA-3, такі як:
* [sha3](https://github.com/RustCrypto/hashes)
* [tiny-keccak](https://github.com/debris/tiny-keccak)

Вхідними даними в усіх бенчмарках виступав байтовий рядок довжиною 2048 байт.

### My sha_3 implementation

![alt text](https://github.com/deabrua-dev/sha3/common/images/sha_3_mean.svg)

На рисунку представлено середній час на ітерацію.

Додаткова статистика:
| Tables | Lower bound | Estimate | Upper bound |
|:-------|:-----------:|:--------:|:-----------:|
| Slope | 7.0745 µs | 7.1151 µs | 7.1804 µs |
| R² | 0.8499224 | 0.8533537 | 0.8445614 |
| Mean | 7.1190 µs | 7.1706 µs | 7.2320 µs |
| Std. Dev. | 170.63 ns | 290.96 ns | 404.39 ns |
| Median | 7.0478 µs | 7.0562 µs | 7.0768 µs |
| MAD | 23.787 ns | 41.839 ns | 73.080 ns |

Де µs - мікросекунди, а ns - наносекунди.


### Crate sha3

![alt text](https://github.com/deabrua-dev/sha3/common/images/sha3_mean.svg)

На рисунку представлено середній час на ітерацію.

Додаткова статистика:
| Tables | Lower bound | Estimate | Upper bound |
|:-------|:-----------:|:--------:|:-----------:|
| Slope | 7.1259 µs | 7.1763 µs | 7.2394 µs |
| R² | 0.8360989 | 0.8409200 | 0.8333675 |
| Mean | 7.1419 µs | 7.1940 µs | 7.2554 µs |
| Std. Dev. | 177.30 ns | 293.62 ns | 399.62 ns |
| Median | 7.0647 µs | 7.0798 µs | 7.1082 µs |
| MAD | 40.336 ns | 64.893 ns | 104.50 ns |

Де µs - мікросекунди, а ns - наносекунди.

### Crate tiny-keccak

![alt text](https://github.com/deabrua-dev/sha3/common/images/tiny_keccak_mean.svg)

На рисунку представлено середній час на ітерацію.

Додаткова статистика:
| Tables | Lower bound | Estimate | Upper bound |
|:-------|:-----------:|:--------:|:-----------:|
| Slope | 6.4739 µs | 6.5021 µs | 6.5386 µs |
| R² | 0.9189573 | 0.9211404 | 0.9174769 |
| Mean | 6.5616 µs | 6.6292 µs | 6.7049 µs |
| Std. Dev. | 264.51 ns | 366.86 ns | 458.13 ns |
| Median | 6.4512 µs | 6.4591 µs | 6.4785 µs |
| MAD | 24.405 ns | 36.267 ns | 67.003 ns |

Де µs - мікросекунди, а ns - наносекунди.

