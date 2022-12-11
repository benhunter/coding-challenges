# Coding Challenges

## Advent of Code

- 2022 https://adventofcode.com/2022
- 2021 https://adventofcode.com/2021
- 2020 https://adventofcode.com/2020

## Exercism

Java - https://exercism.io/my/tracks/java

- to fix the build error with Exercism v2 - change the following in `build.gradle`:

```java
dependencies {
	testImplementation "org.junit.vintage:junit-vintage-engine:5.7.2"
	testRuntimeOnly "org.junit.jupiter:junit-jupiter-engine:5.7.2"
}
```
