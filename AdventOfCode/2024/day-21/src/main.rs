use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

fn main() -> Result<(), String> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    let input = include_str!("../input.txt");
    // let input = include_str!("../test.txt");

    let result = day_21::solve_part1(input)?;
    info!("✅ part1: {}", result);

    let result = day_21::solve_part2(input)?;
    info!("✅ part2: {}", result);

    Ok(())
}
