fn main() -> Result<(), String> {
    let input = include_str!("../input.txt");
    // let input = include_str!("../test.txt");

    let result = day_11::solve_part1(input)?;
    println!("✅ part1: {}", result);

    // let input = include_str!("../test.txt");
    // let result = day_11::solve_part2_expand_to(input, 10)?;

    let result = day_11::solve_part2(input)?;
    println!("✅ part2: {}", result);

    Ok(())
}
