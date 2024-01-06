fn main() {
    // let input = include_str!("../input.txt");
    let input = include_str!("../test.txt");
    let input_steps = (input, 64);
    let input_steps = (input, 6);

    let result = day_21::solve_part1(input_steps);
    println!("✅ part1: {}", result);

    let input_steps = (input, 26501365);
    let result = day_21::solve_part2(input_steps);
    // println!("✅ part2: {}", result);
}
