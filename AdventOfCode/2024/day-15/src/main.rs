fn main() -> Result<(), String> {
    // day_15::simulate_large_example()?;

    let input = include_str!("../input.txt");
    // let input = include_str!("../test.txt");

    let result = day_15::solve_part1(input)?;
    println!("✅ part1: {}", result);

    // let result = day_15::solve_part2(input)?;
    // println!("✅ part2: {}", result);

    Ok(())
}
