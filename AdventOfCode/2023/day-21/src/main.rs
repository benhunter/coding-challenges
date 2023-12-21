fn main() {
    let input = (include_str!("../input.txt"), 64);
    // let input = (include_str!("../test.txt"), 6);

    let result = day_21::solve_part1(input);
    println!("✅ part1: {}", result);

    // let result = day_21::solve_part2(input);
    // println!("✅ part2: {}", result);
}
