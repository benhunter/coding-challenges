fn main() -> Result<(), String> {
    let input = include_str!("../input.txt");
    // let input = include_str!("../test.txt");

    let result = day_21::solve_part1(input)?;
    println!("✅ part1: {}", result);

    let result = day_21::solve_part2(input)?;
    println!("✅ part2: {}", result);

    Ok(())
}
