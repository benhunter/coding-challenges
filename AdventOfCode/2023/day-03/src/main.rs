use std::os::unix::raw::off_t;

fn main() {
    // let test = include_str!("../test1.txt");
    // let engine = parse(test);
    // let start = Coordinate { x: 0, y: 0 };
    // let span = engine.next_part_number_location(start).unwrap();
    // dbg!(span.clone());
    // let span = engine.next_part_number_location(span.end).unwrap();
    // dbg!(span);

    let input = include_str!("../input.txt");
    let result = solve_part1(input);
    println!("✅ part1: {}", result);

    let count_of_nums_in_input = count_numbers(include_str!("../input.txt"));
    dbg!(count_of_nums_in_input);

    // let result = solve_part2(input);
    // println!("✅ part2: {}", result);
}

fn count_numbers(input: &str) -> i32 {
    let mut count = 0;
    for line in input.split('\n') {
        // count integers, not digits
        line.split(|c: char| !c.is_ascii_digit())
            .filter(|x| !x.is_empty())
            .for_each(|_| count += 1)
    }
    count
}

#[derive(Debug, PartialEq, Clone)]
struct Span {
    start: Coordinate,
    end: Coordinate,
}

#[derive(Debug, PartialEq, Clone)]
struct Engine {
    schematic: Vec<String>,
}

impl Engine {
    /* returns the start and end coordinates of the part number
        end is exclusive
    */
    pub(crate) fn next_part_number_location(&self, start: Coordinate) -> Option<Span> {
        // dbg!(self);
        let mut start = start;
        let mut current = start.clone();
        let mut started = false;

        loop {
            // dbg!("at top of loop", current);
            // dbg!("current line length", self.schematic[current.y as usize].len());
            if current.x >= self.schematic[current.y as usize].len() as i32 {
                // dbg!("end of line");
                break;
            }
            let current_char = self.schematic[current.y as usize].chars().nth(current.x as usize).unwrap();
            // dbg!("current_char: {:?}", current_char);

            if current_char == '.' {
                if started {
                    break;
                }
            } else if current_char.is_ascii_digit() {
                if !started {
                    started = true;
                    start = current.clone();
                }
            } else if is_symbol(current_char) {
                if started {
                    break;
                }
            } else {
                panic!("?? wut: {:?}", current_char);
            }

            current.x += 1;
            if current.x >= self.schematic[current.y as usize].len() as i32 {
                current.x = 0;
                current.y += 1;
                if started {
                    break;
                }
                if current.y >= self.schematic.len() as i32 {
                    return None;
                }
            }
        }

        // dbg!(start);
        // dbg!("at end", current);
        return if started {
            Some(Span { start, end: current })
        } else {
            None
        };
    }


    pub(crate) fn is_adjacent_to_symbol(&self, part_number_location: Span) -> bool {
        // dbg!(part_number_location.clone());
        let y = part_number_location.start.y;
        let x = part_number_location.start.x;

        // above
        // dbg!("checking above");
        if y > 0 {
            // dbg!("found row above");
            for x in x - 1..(part_number_location.end.x + 1) {
                if x < 0 {
                    continue;
                } else if x >= self.schematic[y as usize - 1].len() as i32 {
                    continue;
                }
                let above = self.schematic[y as usize - 1].chars().nth(x as usize).unwrap();
                if above != '.' && !above.is_ascii_digit() {
                    return true;
                }
                // if is_symbol(above) {
                //     return true;
                // }
            }
        }

        // below
        // dbg!("checking below");
        // dbg!(self.schematic.len());
        // dbg!(y);
        if y + 1 < self.schematic.len() as i32 {
            // dbg!("checking below");
            for x in (x - 1..part_number_location.end.x + 1) {
                // dbg!(x);
                if x < 0 {
                    continue;
                } else if x >= self.schematic[y as usize + 1].len() as i32 {
                    continue;
                }
                // dbg!("x: {:?}", x);
                let below = self.schematic[y as usize + 1].chars().nth(x as usize).unwrap();
                if below != '.' && !below.is_ascii_digit() {
                    return true;
                }
                // if is_symbol(below) {
                //     return true;
                // }
            }
        }

        let x = part_number_location.start.x;
        // left
        // dbg!("checking left");
        if x > 0 {
            for y in y..(part_number_location.end.y + 1) {
                if y < 0 {
                    continue;
                } else if y >= self.schematic.len() as i32 {
                    continue;
                }

                let left = self.schematic[y as usize].chars().nth(x as usize - 1).unwrap();
                if left != '.' && !left.is_ascii_digit() {
                    return true;
                }
                // if is_symbol(left) {
                //     return true;
                // }
            }
        }

        let x = part_number_location.end.x;
        // right
        // dbg!("checking right");
        // dbg!(x);
        // dbg!(self.schematic[y as usize].len());
        if x < self.schematic[y as usize].len() as i32 {
            // dbg!(part_number_location.end.y);
            for y in y - 1..(part_number_location.end.y + 1) {
                // dbg!(x, y);
                if y < 0 {
                    continue;
                } else if y >= self.schematic.len() as i32 {
                    continue;
                }

                let right = self.schematic[y as usize].chars().nth(x as usize).unwrap();
                // dbg!(right);
                if right != '.' && !right.is_ascii_digit() {
                    return true;
                }
                // if is_symbol(right) {
                //     return true;
                // }
            }
        }

        false
    }
}

// '*' | '#' | '+' | '$' | '&' | '-' | '/' | '@' | '%' | '='
fn is_symbol(c: char) -> bool {
    "*#+$&-/@%=".contains(c)
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32,
}

fn parse(input: &str) -> Engine {
    Engine {
        schematic:
        input
            .split('\n')
            .map(|x| String::from(x))
            .collect()
    }
}

fn solve_part1(input: &str) -> i32 {
    let engine = parse(input);
    let mut sum = 0;
    let mut current = Coordinate { x: 0, y: 0 };
    let mut loop_count = 0;
    let mut part_count = 0;

    loop {
        let span = engine.next_part_number_location(current);
        if span.is_none() {
            // panic!("no more part numbers");
            break;
        }
        let span = span.unwrap();
        // dbg!(span.clone());
        if engine.is_adjacent_to_symbol(span.clone()) {
            part_count += 1;
            // dbg!(span.clone());
            let x = if span.end.x == 0 {
                engine.schematic[span.end.y as usize].len() as i32
            } else {
                span.end.x
            };
            // dbg!(x);

            let part_number = engine.schematic[span.start.y as usize]
                .chars()
                .skip(span.start.x as usize)
                .take(x as usize - span.start.x as usize)
                .collect::<String>();
            // dbg!(part_number.clone());
            let part_value = part_number.parse::<i32>().unwrap();
            dbg!(part_value);
            sum += part_value;
        }
        current = span.end;
        loop_count += 1;
    }
    dbg!(loop_count);
    dbg!(part_count);
    sum
}

fn solve_part2(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "467..114..";
        let output = Engine {
            schematic: vec![
                String::from("467..114.."),
            ]
        };
        assert_eq!(parse(input), output);
    }

    #[test]
    fn test_next_part_number_location() {
        let input = "467..114..";
        let engine = parse(input);
        let actual = engine.next_part_number_location(Coordinate { x: 0, y: 0 });
        let expected = Span {
            start: Coordinate { x: 0, y: 0 },
            end: Coordinate { x: 3, y: 0 },
        };
        assert_eq!(actual.unwrap(), expected);

        let actual = engine.next_part_number_location(expected.end);
        let expected = Span { start: Coordinate { x: 5, y: 0 }, end: Coordinate { x: 8, y: 0 } };
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn test_next_part_number_on_next_row() {
        /*
            ..*.......
            467..114..
         */
        let input = "..*.......\n467..114..";
        let engine = parse(input);
        let actual = engine.next_part_number_location(Coordinate { x: 0, y: 0 }).unwrap();
        let expected = Span {
            start: Coordinate { x: 0, y: 1 },
            end: Coordinate { x: 3, y: 1 },
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_is_adjacent_to_symbol_above() {
        /*
            -.........
            467..114..
         */
        let input = "-.........\n467..114..";
        let engine = parse(input);
        let part_span = engine.next_part_number_location(Coordinate { x: 0, y: 0 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span);
        let expected = true;
        assert_eq!(actual, expected);

        /*
            .-.........
            .467..114..
         */
        let input = ".-.........\n.467..114..";
        let engine = parse(input);
        let part_span = engine.next_part_number_location(Coordinate { x: 0, y: 0 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span);
        let expected = true;
        assert_eq!(actual, expected);

        /*
            .&........
            467..114..
         */
        let input = ".&........\n467..114..";
        let engine = parse(input);
        let part_span = engine.next_part_number_location(Coordinate { x: 0, y: 0 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span);
        let expected = true;
        assert_eq!(actual, expected);

        /*
            ..*.......
            467..114..
         */
        let input = "..*.......\n467..114..";
        let engine = parse(input);
        let part_span = engine.next_part_number_location(Coordinate { x: 0, y: 0 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span);
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_is_adjacent_to_symbol_above_right() {
        /*
            ...*......
            467..114..
         */
        let input = "...*......\n467..114..";
        let engine = parse(input);
        let part_span = engine.next_part_number_location(Coordinate { x: 0, y: 0 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span);
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_is_adjacent_to_symbol_above_left() {
        /*
            *.........
            .467..114.
         */
        let input = "*.........\n.467..114.";
        let engine = parse(input);
        let part_span = engine.next_part_number_location(Coordinate { x: 0, y: 0 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span);
        let expected = true;
        assert_eq!(actual, expected);

        /*
            .*.........
            ..467..114.
         */
        let input = ".*.........\n..467..114.";
        let engine = parse(input);
        let part_span = engine.next_part_number_location(Coordinate { x: 0, y: 0 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span);
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_is_adjacent_to_symbol_right() {
        /*
            467#.114..
         */
        let input = "467#.114..";
        let engine = parse(input);
        let part_span = engine.next_part_number_location(Coordinate { x: 0, y: 0 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span);
        let expected = true;
        assert_eq!(actual, expected);

        /*
            ..........
            467#.114..
         */
        let input = "..........\n467#.114..";
        let engine = parse(input);
        let part_span = engine.next_part_number_location(Coordinate { x: 0, y: 0 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span);
        let expected = true;
        assert_eq!(actual, expected);

        /*
            ..........
            ......145%
         */
        let input = "..........\n......145%";
        let engine = parse(input);
        let part_span = engine.next_part_number_location(Coordinate { x: 0, y: 0 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span);
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_is_adjacent_to_symbol_below_right() {
        /*
            467..114..
            ...*......
         */
        let input = "467..114..\n...*......";
        let engine = parse(input);
        let part_span = engine.next_part_number_location(Coordinate { x: 0, y: 0 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span);
        let expected = true;
        assert_eq!(actual, expected);

        /*
            467.
            ...*
         */
        let input = "467.\n...*";
        let engine = parse(input);
        let part_span = engine.next_part_number_location(Coordinate { x: 0, y: 0 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span);
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_is_adjacent_to_symbol_below() {
        /*
            .467..114.
            .&........
         */
        let input = ".467..114.\n.&........";
        let engine = parse(input);
        let part_span = engine.next_part_number_location(Coordinate { x: 0, y: 0 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span);
        let expected = true;
        assert_eq!(actual, expected);

        /*
            .467..114.
            ..*.......
         */
        let input = ".467..114.\n..*.......";
        let engine = parse(input);
        let part_span = engine.next_part_number_location(Coordinate { x: 0, y: 0 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span);
        let expected = true;
        assert_eq!(actual, expected);

        /*
            .467..114.
            .../......
         */
        let input = ".467..114.\n.../......";
        let engine = parse(input);
        let part_span = engine.next_part_number_location(Coordinate { x: 0, y: 0 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span);
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_is_adjacent_to_symbol_below_left() {
        /*
            .467..114.
            *.........
         */
        let input = ".467..114.\n*.........";
        let engine = parse(input);
        let part_span = engine.next_part_number_location(Coordinate { x: 0, y: 0 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span);
        let expected = true;
        assert_eq!(actual, expected);
    }


    #[test]
    fn test_is_adjacent_to_symbol_left() {
        /*
            *467..114.
            ..........
         */
        let input = "*467..114.\n..........";
        let engine = parse(input);
        let part_span = engine.next_part_number_location(Coordinate { x: 0, y: 0 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span);
        let expected = true;
        assert_eq!(actual, expected);

        /*
            ..........
            *467..114.
            ..........
         */
        let input = "..........\n*467..114.\n..........";
        let engine = parse(input);
        let part_span = engine.next_part_number_location(Coordinate { x: 0, y: 0 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span);
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_span_ends_at_end_of_line() {
        /*
            .467...114
            123.......
         */
        let input = ".467...114\n112.......";
        let engine = parse(input);
        let part_span = engine.next_part_number_location(Coordinate { x: 0, y: 0 }).unwrap().end;
        let actual = engine.next_part_number_location(part_span).unwrap();
        let expected = Span { start: Coordinate { x: 7, y: 0 }, end: Coordinate { x: 0, y: 1 } };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1() {
        // two numbers are not part numbers because they are not adjacent to a symbol: 114 (top right) and 58 (middle right). Every other number is adjacent to a symbol and so is a part number; their sum is 4361.
        let input = include_str!("../test1.txt");
        let solution = 4361;
        assert_eq!(solve_part1(input), solution);
    }

    // #[test]
    fn test_solve2() {
        let input = include_str!("../test1.txt");
        let solution = 0;
        assert_eq!(solve_part2(input), solution);
    }

    #[test]
    fn test_solve_part1() {
        let input = include_str!("../input.txt");
        let lower_bound = 545185; // too low
        // assert_eq!(solve_part1(input), solution);
        let upper_bound = 612377; // too high
        let solution = solve_part1(input);
        println!("solution: {}", solution);

        assert!(solution > lower_bound);
        assert!(solution < upper_bound);
    }

    // #[test]
    fn test_solve_part2() {
        let input = include_str!("../input.txt");
        let solution = 0;
        assert_eq!(solve_part2(input), solution);
    }
}