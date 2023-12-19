fn main() {
    let input = include_str!("../input.txt");
    // let input = include_str!("../test.txt");

    let result = {{project-name | snake_case}}::solve_part1(input);
    println!("✅ part1: {}", result);

    // let result = {{project-name | snake_case}}::solve_part2(input);
    // println!("✅ part2: {}", result);
}
