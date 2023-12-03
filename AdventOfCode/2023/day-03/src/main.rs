fn main() {
    let input = include_str!("../input.txt");
    let result = solve_part1(input);
    println!("✅ part1: {}", result);

    let result = solve_part2(input);
    println!("✅ part2: {}", result);

    count_numbers(include_str!("../input.txt"));
}

fn solve_part1(input: &str) -> i32 {
    let engine = parse(input);
    let mut sum = 0;
    let mut current = Coordinate { x: 0, y: 0 };

    loop {
        let span = engine.next_part_location(current);
        if span.is_none() {
            break;
        }
        let span = span.unwrap();
        if engine.is_adjacent_to_symbol(span.clone(), is_symbol) {
            // part_count += 1;
            let x = if span.end.x == 0 {
                engine.schematic[span.end.y as usize].len() as i32
            } else {
                span.end.x
            };

            let part_number = engine.schematic[span.start.y as usize]
                .chars()
                .skip(span.start.x as usize)
                .take(x as usize - span.start.x as usize)
                .collect::<String>();
            let part_value = part_number.parse::<i32>().unwrap();
            sum += part_value;
        }
        current = span.end;
    }
    sum
}

fn solve_part2(input: &str) -> i32 {
    let mut engine = parse(input);
    let mut part = engine.next_gear_part(Coordinate { x: 0, y: 0 }, is_asterisk);

    while part.is_some() {
        engine.add_gear_part(part.clone().unwrap());
        part = engine.next_gear_part(part.unwrap().span.end, is_asterisk);
    }

    let mut solution = 0;
    let mut count_matches = 0;
    let mut count_misses = 0;

    while let Some(current) = engine.gear_parts.pop() {
        engine.gear_parts.iter()
            .for_each(|x| {
                if x.gear == current.gear {
                    solution += current.value * x.value;
                    count_matches += 1;
                } else {
                    count_misses += 1;
                }
            });
    }
    solution
}

fn parse(input: &str) -> Engine {
    Engine {
        schematic:
        input
            .split('\n')
            .map(String::from)
            .collect(),
        ..Default::default()
    }
}

fn count_numbers(input: &str) -> usize {
    input.split('\n')
        .map(|line| line.split(|c: char| !c.is_ascii_digit())
            .filter(|x| !x.is_empty())
            .count()
        )
        .sum()
}

#[derive(Debug, PartialEq, Clone, Default)]
struct Engine {
    schematic: Vec<String>,
    gear_parts: Vec<GearPart>,
}

impl Engine {
    // returns the start and end coordinates of the part number. end is exclusive
    pub(crate) fn next_part_location(&self, start: Coordinate) -> Option<Span> {
        let mut current = start;
        let mut started = false;
        let mut next_start = start;

        loop {
            if current.x >= self.schematic[current.y as usize].len() as i32 {
                break;
            }
            let current_char = self.schematic[current.y as usize].chars().nth(current.x as usize).unwrap();

            if current_char == '.' {
                if started {
                    break;
                }
            } else if current_char.is_ascii_digit() {
                if !started {
                    started = true;
                    next_start = current;
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

        if started {
            Some(Span { start: next_start, end: current })
        } else {
            None
        }
    }


    pub(crate) fn is_adjacent_to_symbol(&self, part_location: Span, is_symbol: fn(char) -> bool) -> bool {
        let y = part_location.start.y;
        let x = part_location.start.x;
        let end = self.calculate_end(&part_location, y);

        if self.get_symbol_above(is_symbol, y, x, end).is_some() {
            return true;
        }

        if self.get_symbol_below(is_symbol, y, x, end).is_some() {
            return true;
        }

        let x = part_location.start.x;
        if self.get_symbol_left(&part_location, is_symbol, y, x).is_some() {
            return true;
        }

        let x = part_location.end.x;
        if self.get_symbol_right(part_location, is_symbol, y, x).is_some() {
            return true;
        }

        false
    }

    pub(crate) fn part_at_span(&self, span: Span) -> i32 {
        let x = if span.end.x == 0 {
            self.schematic[span.end.y as usize].len() as i32
        } else {
            span.end.x
        };
        let part_number = self.schematic[span.start.y as usize]
            .chars()
            .skip(span.start.x as usize)
            .take(x as usize - span.start.x as usize)
            .collect::<String>();
        part_number.parse::<i32>().unwrap()
    }

    pub(crate) fn add_gear_part(&mut self, partial_gear: GearPart) {
        self.gear_parts.push(partial_gear);
    }

    pub(crate) fn next_gear_part(&self, coord: Coordinate, is_gear: fn(char) -> bool) -> Option<GearPart> {
        let mut current = coord;
        loop {
            let span = self.next_part_location(current);
            current = if span.is_none() {
                return None;
            } else {
                span.clone().unwrap().end
            };

            if self.is_adjacent_to_symbol(span.clone().unwrap(), is_gear) {
                return Some(GearPart {
                    span: span.clone().unwrap(),
                    gear: self.get_gear_location(span.clone().unwrap(), is_gear).unwrap(),
                    value: self.part_at_span(span.unwrap()),
                });
            }
        };
    }

    fn get_gear_location(&self, part_location: Span, is_gear: fn(char) -> bool) -> Option<Coordinate> {
        let y = part_location.start.y;
        let x = part_location.start.x;

        let end = self.calculate_end(&part_location, y);

        // above
        if let Some(value) = self.get_symbol_above(is_gear, y, x, end) {
            return value;
        }

        // below
        if let Some(value) = self.get_symbol_below(is_gear, y, x, end) {
            return value;
        }

        let x = part_location.start.x;
        // left
        if let Some(value) = self.get_symbol_left(&part_location, is_gear, y, x) {
            return value;
        }

        let x = part_location.end.x;
        // right
        if let Some(value) = self.get_symbol_right(part_location, is_gear, y, x) {
            return value;
        }

        None
    }

    fn get_symbol_above(&self, is_symbol: fn(char) -> bool, y: i32, x: i32, end: i32) -> Option<Option<Coordinate>> {
        if y > 0 {
            for x in (x - 1)..end {
                if x < 0 || x >= self.schematic[y as usize - 1].len() as i32 {
                    continue;
                }
                let above = self.schematic[y as usize - 1].chars().nth(x as usize).unwrap();
                if is_symbol(above) {
                    return Some(Some(Coordinate { x, y: y - 1 }));
                }
            }
        }
        None
    }

    fn get_symbol_below(&self, is_symbol: fn(char) -> bool, y: i32, x: i32, end: i32) -> Option<Option<Coordinate>> {
        if y + 1 < self.schematic.len() as i32 {
            for x in (x - 1)..end {
                if x < 0 || x >= self.schematic[y as usize + 1].len() as i32 {
                    continue;
                }
                let below = self.schematic[y as usize + 1].chars().nth(x as usize).unwrap();
                if is_symbol(below) {
                    return Some(Some(Coordinate { x, y: y + 1 }));
                }
            }
        }
        None
    }

    fn get_symbol_right(&self, part_location: Span, is_gear: fn(char) -> bool, y: i32, x: i32) -> Option<Option<Coordinate>> {
        if x < self.schematic[y as usize].len() as i32 {
            for y in y - 1..(part_location.end.y + 1) {
                if y < 0 || y >= self.schematic.len() as i32 {
                    continue;
                }
                let right = self.schematic[y as usize].chars().nth(x as usize).unwrap();
                if is_gear(right) {
                    return Some(Some(Coordinate { x, y }));
                }
            }
        }
        None
    }

    fn get_symbol_left(&self, part_location: &Span, is_gear: fn(char) -> bool, y: i32, x: i32) -> Option<Option<Coordinate>> {
        if x > 0 {
            for y in y..(part_location.end.y + 1) {
                if y < 0 || y >= self.schematic.len() as i32 {
                    continue;
                }
                let left = self.schematic[y as usize].chars().nth(x as usize - 1).unwrap();
                if is_gear(left) {
                    return Some(Some(Coordinate { x: x - 1, y }));
                }
            }
        }
        None
    }

    fn calculate_end(&self, part_location: &Span, y: i32) -> i32 {
        if part_location.end.x == 0 {
            self.schematic[y as usize - 1].len() as i32
        } else {
            part_location.end.x + 1
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
struct GearPart {
    span: Span,
    gear: Coordinate,
    value: i32,
}

#[derive(Debug, PartialEq, Clone, Default)]
struct Span {
    start: Coordinate,
    end: Coordinate,
}

#[derive(Debug, PartialEq, Clone, Copy, Default)]
struct Coordinate {
    x: i32,
    y: i32,
}

fn is_symbol(c: char) -> bool {
    "*#+$&-/@%=".contains(c)
}

fn is_asterisk(c: char) -> bool {
    c == '*'
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
            ],
            ..Default::default()
        };
        assert_eq!(parse(input), output);
    }

    #[test]
    fn test_next_part_number_location() {
        let input = "467..114..";
        let engine = parse(input);
        let actual = engine.next_part_location(Coordinate { x: 0, y: 0 });
        let expected = Span {
            start: Coordinate { x: 0, y: 0 },
            end: Coordinate { x: 3, y: 0 },
        };
        assert_eq!(actual.unwrap(), expected);

        let actual = engine.next_part_location(expected.end);
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
        let actual = engine.next_part_location(Coordinate { x: 0, y: 0 }).unwrap();
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
        let part_span = engine.next_part_location(Coordinate { x: 0, y: 0 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span, is_symbol);
        let expected = true;
        assert_eq!(actual, expected);

        /*
            .-.........
            .467..114..
         */
        let input = ".-.........\n.467..114..";
        let engine = parse(input);
        let part_span = engine.next_part_location(Coordinate { x: 0, y: 0 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span, is_symbol);
        let expected = true;
        assert_eq!(actual, expected);

        /*
            .&........
            467..114..
         */
        let input = ".&........\n467..114..";
        let engine = parse(input);
        let part_span = engine.next_part_location(Coordinate { x: 0, y: 0 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span, is_symbol);
        let expected = true;
        assert_eq!(actual, expected);

        /*
            ..*.......
            467..114..
         */
        let input = "..*.......\n467..114..";
        let engine = parse(input);
        let part_span = engine.next_part_location(Coordinate { x: 0, y: 0 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span, is_symbol);
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
        let part_span = engine.next_part_location(Coordinate { x: 0, y: 0 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span, is_symbol);
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
        let part_span = engine.next_part_location(Coordinate { x: 0, y: 0 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span, is_symbol);
        let expected = true;
        assert_eq!(actual, expected);

        /*
            .*.........
            ..467..114.
         */
        let input = ".*.........\n..467..114.";
        let engine = parse(input);
        let part_span = engine.next_part_location(Coordinate { x: 0, y: 0 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span, is_symbol);
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
        let part_span = engine.next_part_location(Coordinate { x: 0, y: 0 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span, is_symbol);
        let expected = true;
        assert_eq!(actual, expected);

        /*
            ..........
            467#.114..
         */
        let input = "..........\n467#.114..";
        let engine = parse(input);
        let part_span = engine.next_part_location(Coordinate { x: 0, y: 0 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span, is_symbol);
        let expected = true;
        assert_eq!(actual, expected);

        /*
            ..........
            ......145%
         */
        let input = "..........\n......145%";
        let engine = parse(input);
        let part_span = engine.next_part_location(Coordinate { x: 0, y: 0 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span, is_symbol);
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
        let part_span = engine.next_part_location(Coordinate { x: 0, y: 0 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span, is_symbol);
        let expected = true;
        assert_eq!(actual, expected);

        /*
            467.
            ...*
         */
        let input = "467.\n...*";
        let engine = parse(input);
        let part_span = engine.next_part_location(Coordinate { x: 0, y: 0 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span, is_symbol);
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
        let part_span = engine.next_part_location(Coordinate { x: 0, y: 0 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span, is_symbol);
        let expected = true;
        assert_eq!(actual, expected);

        /*
            .467..114.
            ..*.......
         */
        let input = ".467..114.\n..*.......";
        let engine = parse(input);
        let part_span = engine.next_part_location(Coordinate { x: 0, y: 0 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span, is_symbol);
        let expected = true;
        assert_eq!(actual, expected);

        /*
            .467..114.
            .../......
         */
        let input = ".467..114.\n.../......";
        let engine = parse(input);
        let part_span = engine.next_part_location(Coordinate { x: 0, y: 0 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span, is_symbol);
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
        let part_span = engine.next_part_location(Coordinate { x: 0, y: 0 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span, is_symbol);
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
        let part_span = engine.next_part_location(Coordinate { x: 0, y: 0 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span, is_symbol);
        let expected = true;
        assert_eq!(actual, expected);

        /*
            ..........
            *467..114.
            ..........
         */
        let input = "..........\n*467..114.\n..........";
        let engine = parse(input);
        let part_span = engine.next_part_location(Coordinate { x: 0, y: 0 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span, is_symbol);
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
        let part_span = engine.next_part_location(Coordinate { x: 0, y: 0 }).unwrap().end;
        let actual = engine.next_part_location(part_span).unwrap();
        let expected = Span { start: Coordinate { x: 7, y: 0 }, end: Coordinate { x: 0, y: 1 } };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_bug_part1() {
        let input =
            "............%.........................*770.............253..*....515..926..................................=........45.946..............*...\n\
             ....155..573..103.24..............................@......*...179..*........275......................*...................*................134\n\
             ....*............*......963...........444......801...656.796.....524.84#......*433.......997.....122.500....711.......447...................";
        let engine = parse(input);
        let part_span = engine.next_part_location(Coordinate { x: 0, y: 0 }).unwrap();
        let part_span = engine.next_part_location(part_span.end).unwrap();
        let part_span = engine.next_part_location(part_span.end).unwrap();
        let part_span = engine.next_part_location(part_span.end).unwrap();
        let part_span = engine.next_part_location(part_span.end).unwrap();
        let part_span = engine.next_part_location(part_span.end).unwrap();
        let part_span = engine.next_part_location(part_span.end).unwrap();
        let part_span = engine.next_part_location(part_span.end).unwrap();
        let part_span = engine.next_part_location(part_span.end).unwrap();
        let part_span = engine.next_part_location(part_span.end).unwrap();
        let part_span = engine.next_part_location(part_span.end).unwrap();
        let part_span = engine.next_part_location(part_span.end).unwrap();
        let part_span = engine.next_part_location(part_span.end).unwrap();
        // dbg!(engine.part_at_span(part_span.clone()));
        // dbg!(part_span.clone());
        let actual = engine.is_adjacent_to_symbol(part_span, is_symbol);
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gear_left() {
        let input =
            ".....\n\
             *123.\n\
             .....";
        let engine = parse(input);
        let part_span = engine.next_part_location(Coordinate { x: 0, y: 1 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span, is_asterisk);
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gear_right() {
        let input =
            ".....\n\
             .123*\n\
             .....";
        let engine = parse(input);
        let part_span = engine.next_part_location(Coordinate { x: 0, y: 1 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span, is_asterisk);
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gear_above_left() {
        let input =
            "*....\n\
             .123.\n\
             .....";
        let engine = parse(input);
        let part_span = engine.next_part_location(Coordinate { x: 0, y: 1 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span, is_asterisk);
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gear_above_right() {
        let input =
            "....*\n\
             .123.\n\
             .....";
        let engine = parse(input);
        let part_span = engine.next_part_location(Coordinate { x: 0, y: 1 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span, is_asterisk);
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gear_above() {
        let input =
            ".*...\n\
             .123.\n\
             .....";
        let engine = parse(input);
        let part_span = engine.next_part_location(Coordinate { x: 0, y: 1 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span, is_asterisk);
        let expected = true;
        assert_eq!(actual, expected);

        let input =
            "*...\n\
             123.\n\
             ....";
        let engine = parse(input);
        let part_span = engine.next_part_location(Coordinate { x: 0, y: 1 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span, is_asterisk);
        let expected = true;
        assert_eq!(actual, expected);

        let input =
            "..*..\n\
             .123.\n\
             .....";
        let engine = parse(input);
        let part_span = engine.next_part_location(Coordinate { x: 0, y: 1 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span, is_asterisk);
        let expected = true;
        assert_eq!(actual, expected);

        let input =
            "...*.\n\
             .123.\n\
             .....";
        let engine = parse(input);
        let part_span = engine.next_part_location(Coordinate { x: 0, y: 1 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span, is_asterisk);
        let expected = true;
        assert_eq!(actual, expected);

        let input =
            "...*\n\
             .123\n\
             ....";
        let engine = parse(input);
        let part_span = engine.next_part_location(Coordinate { x: 0, y: 1 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span, is_asterisk);
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gear_below_left() {
        let input =
            ".....\n\
             .123.\n\
             *....";
        let engine = parse(input);
        let part_span = engine.next_part_location(Coordinate { x: 0, y: 1 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span, is_asterisk);
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gear_below_right() {
        let input =
            ".....\n\
             .123.\n\
             ....*";
        let engine = parse(input);
        let part_span = engine.next_part_location(Coordinate { x: 0, y: 1 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span, is_asterisk);
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gear_below() {
        let input =
            ".....\n\
             .123.\n\
             .*...";
        let engine = parse(input);
        let part_span = engine.next_part_location(Coordinate { x: 0, y: 1 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span, is_asterisk);
        let expected = true;
        assert_eq!(actual, expected);

        let input =
            "....\n\
             123.\n\
             *...";
        let engine = parse(input);
        let part_span = engine.next_part_location(Coordinate { x: 0, y: 1 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span, is_asterisk);
        let expected = true;
        assert_eq!(actual, expected);

        let input =
            ".....\n\
             .123.\n\
             ..*..";
        let engine = parse(input);
        let part_span = engine.next_part_location(Coordinate { x: 0, y: 1 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span, is_asterisk);
        let expected = true;
        assert_eq!(actual, expected);

        let input =
            ".....\n\
             .123.\n\
             ...*.";
        let engine = parse(input);
        let part_span = engine.next_part_location(Coordinate { x: 0, y: 1 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span, is_asterisk);
        let expected = true;
        assert_eq!(actual, expected);

        let input =
            "....\n\
             .123\n\
             ...*";
        let engine = parse(input);
        let part_span = engine.next_part_location(Coordinate { x: 0, y: 1 }).unwrap();
        let actual = engine.is_adjacent_to_symbol(part_span, is_asterisk);
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_bug_part2() {
        let input =
            "..............-........#...911......494...*...355...159.......................848..676.................../.....*......35....................\n\
             ......81$.....544..67...............*.....159....*............209.747*29........./..........812.........430.232...................199*587...\n\
             760.................*...#........331.................%...158...................#.....................29.................596...477...........";
        let mut engine = parse(input);
        let mut part = engine.next_gear_part(Coordinate { x: 0, y: 0 }, is_asterisk);
        engine.add_gear_part(part.clone().unwrap());
        let mut part = engine.next_gear_part(part.unwrap().span.end, is_asterisk);
        engine.add_gear_part(part.clone().unwrap());
        let mut part = engine.next_gear_part(part.unwrap().span.end, is_asterisk);
        engine.add_gear_part(part.clone().unwrap());
        let mut part = engine.next_gear_part(part.unwrap().span.end, is_asterisk);
        engine.add_gear_part(part.clone().unwrap());
        let mut part = engine.next_gear_part(part.unwrap().span.end, is_asterisk);
        engine.add_gear_part(part.clone().unwrap());
        let mut part = engine.next_gear_part(part.unwrap().span.end, is_asterisk);
        engine.add_gear_part(part.clone().unwrap());
        let mut part = engine.next_gear_part(part.unwrap().span.end, is_asterisk);
        engine.add_gear_part(part.clone().unwrap());
        let mut part = engine.next_gear_part(part.unwrap().span.end, is_asterisk);
        engine.add_gear_part(part.clone().unwrap());
        let mut part = engine.next_gear_part(part.unwrap().span.end, is_asterisk);
        engine.add_gear_part(part.clone().unwrap());
        let mut part = engine.next_gear_part(part.unwrap().span.end, is_asterisk);
        engine.add_gear_part(part.clone().unwrap());
        let mut part = engine.next_gear_part(part.unwrap().span.end, is_asterisk);
        assert!(part.is_none());

        for g in engine.gear_parts.clone() {
            eprintln!("gear: {:?}", g);
        }
        assert_eq!(engine.gear_parts.len(), 10);
        // assert_eq!(solve_part2(input), 0);
    }

    #[test]
    fn test_part1() {
        // two numbers are not part numbers because they are not adjacent to a symbol: 114 (top right) and 58 (middle right). Every other number is adjacent to a symbol and so is a part number; their sum is 4361.
        let input = include_str!("../test1.txt");
        let solution = 4361;
        assert_eq!(solve_part1(input), solution);
    }

    #[test]
    fn test_solve2() {
        let input = include_str!("../test1.txt");
        let solution = 467835;
        assert_eq!(solve_part2(input), solution);
    }

    #[test]
    fn test_solve_part1() {
        let input = include_str!("../input.txt");
        let solution = 546312;
        assert_eq!(solve_part1(input), solution);
    }

    #[test]
    fn test_solve_part2() {
        let input = include_str!("../input.txt");
        let solution = 87449461;
        assert_eq!(solve_part2(input), solution);
    }
}