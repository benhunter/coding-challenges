fn main() {
    let input = include_str!("../input.txt");
    let result = solve_part1(input);
    println!("✅ part1: {}", result);

    let result = solve_part2(input);
    println!("✅ part2: {}", result);
}

fn solve_part1(input: &str) -> i32 {
    let values = parse(input);
    let mut next_values: Vec<i32> = Vec::new();

    values.iter().for_each(|value| {
        let mut sequences: Vec<Vec<i32>> = Vec::new();
        sequences.push(value.history.clone());

        let mut count = 0;
        while !sequences.last().unwrap().iter().all(|x| x == &0) {
            let mut sequence: Vec<i32> = Vec::new();

            let last = sequences.last().unwrap();
            last.windows(2).for_each(|x| {
                let diff = (x[1] - x[0]); // TODO ??? .abs();
                sequence.push(diff);
            });

            sequences.push(sequence);

            count += 1;
        }


        sequences.iter().for_each(|x| println!("{:?} len: {:?}", x, x.len()));
        println!("sequences.last(): {:?}", sequences.last().unwrap());
        assert!(sequences.last().unwrap().iter().all(|x| x == &0));
        assert!(sequences.last().unwrap().len() > 0);

        sequences.reverse();
        sequences.iter().for_each(|x| println!("{:?} len: {:?}", x, x.len()));
        println!("sequences[0]: {:?}", sequences[0].clone());
        println!("sequences[1]: {:?}", sequences[1].clone());
        for i in 0..(sequences.len() - 1) {
            if sequences[i].len() == 0 {
                continue;
            }

            // println!("before {}: {:?}", i + 1, sequences[i + 1]);
            let curr = sequences[i][sequences[i].len() - 1];
            let next = sequences[i + 1][sequences[i + 1].len() - 1];
            sequences[i + 1].push(curr + next);
            println!("after {}: {:?}", i + 1, sequences[i + 1]);
        }
        next_values.push(*sequences.last().unwrap().last().unwrap());
    });

    next_values.iter().sum()
}

fn solve_part2(input: &str) -> i32 {
    let values = parse(input);
    let mut next_values: Vec<i32> = Vec::new();

    values.iter().for_each(|value| {
        let mut sequences: Vec<Vec<i32>> = Vec::new();
        sequences.push(value.history.clone());

        let mut count = 0;
        while !sequences.last().unwrap().iter().all(|x| x == &0) {
            let mut sequence: Vec<i32> = Vec::new();

            let last = sequences.last().unwrap();
            last.windows(2).for_each(|x| {
                let diff = (x[1] - x[0]); // TODO ??? .abs();
                sequence.push(diff);
            });

            sequences.push(sequence);

            count += 1;
        }

        // extrapolation done
        sequences.iter().for_each(|x| println!("{:?} len: {:?}", x, x.len()));
        println!("sequences.last(): {:?}", sequences.last().unwrap());
        assert!(sequences.last().unwrap().iter().all(|x| x == &0));
        assert!(sequences.last().unwrap().len() > 0);

        sequences.reverse();
        sequences.iter().for_each(|x| println!("{:?} len: {:?}", x, x.len()));
        println!("sequences[0]: {:?}", sequences[0].clone());
        println!("sequences[1]: {:?}", sequences[1].clone());
        for i in 0..(sequences.len() - 1) {
            if sequences[i].len() == 0 {
                continue;
            }

            // println!("before {}: {:?}", i + 1, sequences[i + 1]);
            let curr = sequences[i][0];
            let next = sequences[i + 1][0];
            sequences[i + 1].insert(0, next - curr);
            println!("after {}: {:?}", i + 1, sequences[i + 1]);
        }
        next_values.push(*sequences.last().unwrap().first().unwrap());
    });

    next_values.iter().sum()
}

#[derive(Debug, PartialEq, Clone)]
struct Value {
    history: Vec<i32>,
}

fn parse(input: &str) -> Vec<Value> {
    input.lines().map(|line|
        Value {
            history: line.split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .collect(),
        }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = include_str!("../test.txt");
        let actual = parse(input);
        let actual = actual[0].history.len();
        let expected = 6;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_next() {
        let input = "0 3 6 9 12 15";
        let actual = solve_part1(input);
        let solution = 18;
        assert_eq!(actual, solution);
    }

    #[test]
    fn test_correct_sequence() {
        // []
        // sequences[0]: []
        // sequences[1]: [8]
        // after 2: [0, 8, 16]
        // after 3: [8, 8, 0, 16]
        // after 4: [0, 8, 0, 0, 16]
        // after 5: [8, 8, 0, 0, 0, 16]
        // after 6: [0, 8, 0, 0, 0, 0, 16]
        // after 7: [8, 8, 0, 0, 0, 0, 0, 16]
        // after 8: [0, 8, 0, 0, 0, 0, 0, 0, 16]
        // after 9: [8, 8, 0, 0, 0, 0, 0, 0, 0, 16]
        // after 10: [13, 5, 13, 13, 13, 13, 13, 13, 13, 13, 29]
        // after 11: [17, 30, 35, 48, 61, 74, 87, 100, 113, 126, 139, 168]
        // after 12: [26, 9, 39, 74, 122, 183, 257, 344, 444, 557, 683, 822, 990]
        // after 13: [3, 29, 38, 77, 151, 273, 456, 713, 1057, 1501, 2058, 2741, 3563, 4553]
        // after 14: [1, 4, 33, 71, 148, 299, 572, 1028, 1741, 2798, 4299, 6357, 9098, 12661, 17214]
        // after 15: [3, 4, 8, 41, 112, 260, 559, 1131, 2159, 3900, 6698, 10997, 17354, 26452, 39113, 56327]
        // after 16: [10, 7, 3, 11, 52, 164, 424, 983, 2114, 4273, 8173, 14871, 25868, 43222, 69674, 108787, 165114]
        // after 17: [20, 30, 37, 40, 51, 103, 267, 691, 1674, 3788, 8061, 16234, 31105, 56973, 100195, 169869, 278656, 443770]
        // after 18: [22, 42, 72, 109, 149, 200, 303, 570, 1261, 2935, 6723, 14784, 31018, 62123, 119096, 219291, 389160, 667816, 1111586]
        // after 19: [19, 41, 83, 155, 264, 413, 613, 916, 1486, 2747, 5682, 12405, 27189, 58207, 120330, 239426, 458717, 847877, 1515693, 2627279]
        // after 20: [17, 36, 77, 160, 315, 579, 992, 1605, 2521, 4007, 6754, 12436, 24841, 52030, 110237, 230567, 469993, 928710, 1776587, 3292280, 5919559]
        // after 21: [13, 30, 66, 143, 303, 618, 1197, 2189, 3794, 6315, 10322, 17076, 29512, 54353, 106383, 216620, 447187, 917180, 1845890, 3622477, 6914757, 12834316]

        let input = "13 30 66 143 303 618 1197 2189 3794 6315 10322 17076 29512 54353 106383 216620 447187 917180 1845890 3622477 6914757";
        let actual = solve_part1(input);
        let solution = 12834300;
        assert_eq!(actual, solution);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../test.txt");
        let actual = solve_part1(input);
        let solution = 114;
        assert_eq!(actual, solution);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../test.txt");
        let actual = solve_part2(input);
        let solution = 2;
        assert_eq!(actual, solution);
    }

    #[test]
    fn test_solve_part1() {
        let input = include_str!("../input.txt");
        let actual = solve_part1(input);
        let solution = 2038472161;
        assert_eq!(actual, solution);
    }

    // #[test]
    fn test_solve_part2() {
        let input = include_str!("../input.txt");
        let actual = solve_part2(input);
        let solution = 0;
        assert_eq!(actual, solution);
    }
}
