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

pub fn solve_part2(input: &str) -> Result<[i64; 4], String> {
    let secrets = parse(input)?;
    solve_part2_up_to(secrets, 2000, true)
}

pub fn solve_part2_up_to(secrets: Vec<i64>, max_secrets: i64, debug: bool) -> Result<[i64; 4], String> {
    let mut sequences = vec![];
    for secret in secrets {
        if debug {
            println!("{:>9}: {}", secret, secret % 10);
        }
        let mut sequence = vec![];
        let mut next = secret;
        for _ in 0..max_secrets - 1 {
            let prev = next % 10;
            next = solve_next(next);
            let next_mod = next % 10;

            //println!("{}", prev - next_mod);
            let diff = next_mod - prev;
            sequence.push(diff);

            if debug {
                print!("{:>9}: {} ({})", next, next_mod, diff);
                println!();
            }
        }
        //println!("{:?}", sequence);
        sequences.push(sequence);
    }

    if debug {
        //println!("sequences={:?}", sequences);
    }

    let mut best_scores: Vec<i64> = vec![];
    let mut best_score = 0;
    let mut best: &[i64] = &sequences[0][0..4];
    let mut candidate: &[i64] = &sequences[0][0..4];
    let mut candidate_scores: Vec<i64>;

    for s in &sequences {
        for i in 0..s.len() - 4 {
            if debug {
                //println!("{}", i);
            }
            candidate = &s[i..i + 4];
            candidate_scores = compute(&sequences, candidate, false);
            let candidate_score = candidate_scores.iter().sum();
            if candidate_score > best_score {
                best = candidate;
                best_score = candidate_score;
                best_scores = candidate_scores;
            };
        }
    }

    if debug {
        println!();
        println!("Solution:");
        println!("best={:?}, best_score={:?}, best_scores={:?}", best, best_score, best_scores);

        compute(&sequences, best, true);
    }

    let solution_array: [i64; 4] = best.try_into().map_err(|_| "Failed to convert slice to array")?;
    Ok(solution_array)
}

fn compute(sequences: &Vec<Vec<i64>>, best: &[i64], debug: bool) -> Vec<i64> {
    //println!("{}", sequences.len());
    let mut prices = vec![];
    for s in sequences {
        for i in 0..s.len() - 4 {
            if s[i..i + 4] == *best {
                prices.push(s[i+3]);
                if debug {
                    println!("DEBUG: {:?}", &s[i..i+4]);
                }
            }
        }
    }
    prices
}

fn parse(input: &str) -> Result<Vec<i64>, ParseError> {
    let secrets = input.lines().map(|l| l.parse::<i64>().expect("gib")).collect();
    Ok(secrets)
}

fn solve_next(value: i64) -> i64 {
    let result = ((value * 64) ^ value) % 16777216;
    let result = ((result / 32) ^ result) % 16777216;
    let result = ((result * 2048) ^ result) % 16777216;

    result
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
        let solution = [-1,-1,0,2];
        assert_eq!(actual, solution);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), String> {
        let input = include_str!("../test.txt");
        let actual = solve_part2(input)?;
        let solution = [-2,1,-1,3];
        assert_eq!(actual, solution);
        Ok(())
    }

    // #[test]
    fn test_solve_part2() -> Result<(), String> {
        let input = include_str!("../input.txt");
        let actual = solve_part2(input)?;
        let solution = [0; 4];
        assert_eq!(actual, solution);
        Ok(())
    }
}
