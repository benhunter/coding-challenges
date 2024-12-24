use std::{collections::HashMap, time::Instant};

use util::ParseError;

pub fn solve_part1(input: &str) -> Result<i64, String> {
    let secrets = parse(input)?;
    let mut solutions = vec![];
    for secret in secrets {
        println!("{}", secret);
        let mut next = secret;
        for _ in 0..2000 {
            next = solve_next(next);
        }
        println!("{}", next);
        solutions.push(next);
    }
    Ok(solutions.iter().sum())
}

pub fn solve_part2(input: &str) -> Result<i64, String> {
    let secrets = parse(input)?;
    Ok(solve_part2_up_to(secrets, 2000, false)?.1)
}

pub fn solve_part2_up_to(start_secrets: Vec<i64>, max_secrets: i64, debug: bool) -> Result<([i64; 4], i64), String> {
    let mut secrets_sequences: Vec<Vec<i64>> = vec![];
    let mut diffs_sequences: Vec<Vec<i64>> = vec![];
    for secret in &start_secrets {
        if debug {
            println!("{:>9}: {}", secret, secret % 10);
        }
        let mut secret_sequence = vec![];
        let mut diff_sequence = vec![];
        let mut next = *secret;
        for i in 0..max_secrets {
            let prev_mod = next % 10;
            next = solve_next(next);
            let next_mod = next % 10;

            //println!("{}", prev - next_mod);
            let diff = next_mod - prev_mod;
            secret_sequence.push(next);
            diff_sequence.push(diff);

            if debug {
                print!("{}: {:>9}: {} ({})", i, next, next_mod, diff);
                println!();
            }
        }
        //println!("{:?}", sequence);
        secrets_sequences.push(secret_sequence);
        diffs_sequences.push(diff_sequence);
    }
    assert_eq!(&start_secrets.len(), &secrets_sequences.len());
    assert_eq!(&start_secrets.len(), &diffs_sequences.len());

    let start_time = Instant::now();

    let best_sequence = compute_best(&diffs_sequences, &secrets_sequences, debug);

    if debug {
        println!();
        println!("Solution:");
        println!("{:?}", best_sequence);
    }

    println!("Elapsed {:?}", start_time.elapsed());

    let solution_array: [i64; 4] = best_sequence.0.try_into().map_err(|_| "Failed to convert slice to array")?;
    println!("{:?}", solution_array);
    Ok((solution_array, best_sequence.1))
}

fn compute_best(diffs_sequences: &[Vec<i64>], secrets_sequences: &[Vec<i64>], _debug: bool) -> (Vec<i64>, i64) {
    let mut sequence_values: HashMap<&[i64], Vec<i64>> = Default::default();

    for (index, s) in diffs_sequences.iter().enumerate() {
        for i in 0..s.len() - 4 {
            let curr: &[i64] = &s[i..i + 4];
            let entry = sequence_values.entry(curr).or_insert(vec![0; s.len()]);
            if entry[index] == 0 {
                entry[index] = secrets_sequences[index][i + 3] % 10;
            }
        }
    }
    //println!("sequence_values={:?}", sequence_values);
    let max = sequence_values.into_iter().max_by_key(|(_k, v)| v.iter().sum::<i64>());
    println!("max={:?}", max);

    let max = max.expect("better be a max");
    (max.0.to_vec(), max.1.iter().sum())
}

fn parse(input: &str) -> Result<Vec<i64>, ParseError> {
    let secrets = input.lines().map(|l| l.parse::<i64>().expect("gib")).collect();
    Ok(secrets)
}

fn solve_next(value: i64) -> i64 {
    let result = ((value * 64) ^ value) % 16777216;
    let result = ((result / 32) ^ result) % 16777216;
    ((result * 2048) ^ result) % 16777216
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() -> Result<(), String> {
        let input = include_str!("../test.txt");
        let actual = parse(input)?.len();
        let expected = 4;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_solve() -> Result<(), String> {
        assert!(42 ^ 15 == 37);

        let actual = solve_next(123);
        let expected = 15887950;
        assert_eq!(actual, expected);

        let actual = solve_next(expected);
        let expected = 16495136;
        assert_eq!(actual, expected);

        let actual = solve_next(expected);
        let expected = 527345;
        assert_eq!(actual, expected);

        let actual = solve_next(expected);
        let expected = 704524;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let input = include_str!("../test.txt");
        let actual = solve_part1(input)?;
        let solution = 37327623;
        assert_eq!(actual, solution);
        Ok(())
    }

    #[test]
    fn test_solve_part1() -> Result<(), String> {
        let input = include_str!("../input.txt");
        let actual = solve_part1(input)?;
        let solution = 13234715490;
        assert_eq!(actual, solution);
        Ok(())
    }

    #[test]
    fn test_part2_simple() -> Result<(), String> {
        // Expected output:
        //
        //     123: 3
        //15887950: 0 (-3)
        //16495136: 6 (6)
        //  527345: 5 (-1)
        //  704524: 4 (-1)
        // 1553684: 4 (0)
        //12683156: 6 (2)
        //11100544: 4 (-2)
        //12249484: 4 (0)
        // 7753432: 2 (-2)

        let input = [123].to_vec();
        let actual = solve_part2_up_to(input, 10, true).unwrap();
        let solution = ([-1i64, -1i64, 0i64, 2i64],6);
        assert_eq!(actual, solution);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let input = include_str!("../test2.txt");
        let secrets = parse(input)?;
        let actual = solve_part2_up_to(secrets, 2000, false)?.0;
        let solution = [-2,1,-1,3];
        assert_eq!(actual, solution);
        Ok(())
    }

    #[test]
    fn test_solve_part2() -> Result<(), String> {
        let input = include_str!("../input.txt");
        let actual = solve_part2(input)?;
        let solution = 1490;
        assert_eq!(actual, solution);
        Ok(())
    }
}
