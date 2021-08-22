# coding-challenges
 Coding Challenges

## Advent of Code
Advent of Code 2020 https://adventofcode.com/2020

## Exercism
Java - https://exercism.io/my/tracks/java
- to fix the build error with Exercism v2 - change the following in `build.gradle`:
```java
dependencies {
	testImplementation "org.junit.vintage:junit-vintage-engine:5.7.2"
	testRuntimeOnly "org.junit.jupiter:junit-jupiter-engine:5.7.2"
}
```