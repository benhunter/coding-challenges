# Solutions

## Advent of Code

[https://adventofcode.com/](https://adventofcode.com/)

- [2024](https://github.com/benhunter/coding-challenges/tree/main/AdventOfCode/2024) in Rust.
    - [Day 13](https://github.com/benhunter/coding-challenges/blob/main/AdventOfCode/2024/day-13/src/lib.rs)
- [2023](https://github.com/benhunter/coding-challenges/tree/main/AdventOfCode/2023) in Rust.
- [2022](https://github.com/benhunter/coding-challenges/tree/main/AdventOfCode/2022) in Kotlin.
- [2021](https://github.com/benhunter/coding-challenges/tree/main/AdventOfCode/2021) in Python.
- [2020](https://github.com/benhunter/coding-challenges/tree/main/AdventOfCode/2020) in Python.

## LeetCode


```
leetup pick X -l rust
cargo +nightly -Zscript best-time-to-buy-and-sell-stock.rs 
```

## Exercism

Java - https://exercism.io/my/tracks/java

- to fix the build error with Exercism v2 - change the following in `build.gradle`:

```java
dependencies {
	testImplementation "org.junit.vintage:junit-vintage-engine:5.7.2"
	testRuntimeOnly "org.junit.jupiter:junit-jupiter-engine:5.7.2"
}
```
