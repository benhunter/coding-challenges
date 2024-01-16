use itertools::Itertools;

use util::ParseError;

/*
input.txt
z_max = 326
z_min = 1
x_max = 9
x_min = 0
y_max = 9
y_min = 0

test.txt
z_max = 9
z_min = 1
x_max = 2
x_min = 0
y_max = 2
y_min = 0
*/

pub fn solve_part1(input: &str) -> Result<i32, String> {
    let stack = parse(input)?;
    stack.check_invalid();
    stack.bricks.iter().for_each(|brick| {
        // println!("{:?}", brick);
    });
    stack.render_xz();


    todo!()
}

pub fn solve_part2(input: &str) -> Result<i32, String> {
    todo!()
}

#[derive(Debug, PartialEq, Clone, Default)]
struct Stack {
    bricks: Vec<Brick>,
    attribute: i32,
}

impl Stack {
    fn render_xz(&self) -> String {
        let mut output = String::new();

        let z_max = self.bricks.iter()
            .map(|brick| {
                brick.position_end.2
            })
            .max().unwrap();
        dbg!(z_max);

        let x_max = self.bricks.iter()
            .map(|brick| {
                brick.position_end.0
            })
            .max().unwrap();
        dbg!(x_max);

        let y_max = self.bricks.iter()
            .map(|brick| {
                brick.position_end.1
            })
            .max().unwrap();
        dbg!(y_max);

        println!("\n{:=>x_max$}", "=", x_max = (x_max + 3) as usize);
        // output.push_str(&format!("{:=>x_max$}", "=", x_max = (x_max + 3) as usize));

        let x_label_position = x_max / 2 + 1;
        println!("{}", format!("{: >width_x$}", "x", width_x = x_label_position as usize));
        output.push_str(&format!("{}", format!("{: >width_x$}\n", "x", width_x = x_label_position as usize)));

        (0..=x_max).for_each(|x| {
            print!("{}", x);
            output.push_str(&format!("{}", x));
        });
        println!();
        output.push('\n');

        // render rows from z_max to 0
        (1..=z_max).rev().for_each(|z| {
            (0..=x_max).for_each(|x| {
                let closest_y = self.bricks.iter()
                    .filter(|brick| {
                        brick.position_start.0 <= x && brick.position_end.0 >= x
                            && brick.position_start.2 <= z && brick.position_end.2 >= z
                    })
                    .map(|brick| brick.id)
                    .max().unwrap_or('.');
                print!("{}", closest_y);
                output.push(closest_y);
            });

            print!(" {}", z);
            output.push_str(&format!(" {}", z));
            if z == z_max / 2 + 1 {
                print!(" z");
                output.push_str(" z");
            }
            println!();
            output.push('\n');
        });

        println!("{:->x_max$} 0", "-", x_max = (x_max + 1) as usize);
        output.push_str(&format!("{:->x_max$} 0", "-", x_max = (x_max + 1) as usize));
        println!("{:=>x_max$}", "=", x_max = (x_max + 3) as usize);
        // output.push_str(&format!("{:=>x_max$}", "=", x_max = (x_max + 3) as usize));
        output
    }

    fn check_invalid(&self) {
        // find bricks with any start coord greater than any end coord
        let invalid = self.bricks.iter().filter(|brick| {
            brick.position_start.0 > brick.position_end.0
                || brick.position_start.1 > brick.position_end.1
                || brick.position_start.2 > brick.position_end.2
        }).map(|brick| {
            println!("Found {:?}", brick);
            brick
        }).collect::<Vec<&Brick>>();

        if invalid.len() > 0 {
            panic!("Found invalid bricks: {:?}", invalid);
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
struct Brick {
    position_start: (i32, i32, i32),
    position_end: (i32, i32, i32),
    id: char,
}

fn parse(input: &str) -> Result<Stack, ParseError> {
    let bricks = input.lines()
        .enumerate()
        .map(|(i, line)| {
            let parts: Vec<(i32, i32, i32)> = line
                .split('~')
                .map(|part| {
                    let coords: (i32, i32, i32) = part
                        .split(',')
                        .map(|coord| coord.parse::<i32>().unwrap())
                        // .collect::<Vec<i32>>();
                        .collect_tuple().unwrap();
                    coords
                }).collect();

            // println!("char from i: {:?}", char::from_u32(i as u32 + 'A' as u32));
            let id = char::from_u32(i.rem_euclid(26) as u32 + 'A' as u32).unwrap();

            Brick {
                position_start: parts[0],
                position_end: parts[1],
                id,
            }
        }).collect::<Vec<Brick>>();
    // println!("{:?}", bricks);
    Ok(Stack { bricks, attribute: 0 })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() -> Result<(), String> {
        let input = include_str!("../test.txt");
        let actual = parse(input)?;
        let expected_bricks = 7;
        assert_eq!(actual.bricks.len(), expected_bricks);

        // last brick 1,1,8~1,1,9
        let last_brick = Brick {
            position_start: (1, 1, 8),
            position_end: (1, 1, 9),
            id: 'G',
        };
        assert_eq!(actual.bricks[6], last_brick);
        Ok(())
    }

    #[test]
    fn test_view_snapshot_xz() -> Result<(), String> {
        let expected = " x
012
.G. 9
.G. 8
... 7
FFF 6
..E 5 z
D.. 4
CCC 3
BBB 2
.A. 1
--- 0";
        let input = include_str!("../test.txt");
        let stack = parse(input)?;
        let actual = stack.render_xz();
        assert_eq!(actual.to_string(), expected);
        Ok(())
    }

    // #[test]
    fn test_part1() -> Result<(), String> {
        let input = include_str!("../test.txt");
        let actual = solve_part1(input)?;
        let solution = 0;
        assert_eq!(actual, solution);
        Ok(())
    }

    // #[test]
    fn test_solve_part1() -> Result<(), String> {
        let input = include_str!("../input.txt");
        let actual = solve_part1(input)?;
        let solution = 0;
        assert_eq!(actual, solution);
        Ok(())
    }

    // #[test]
    fn test_part2() -> Result<(), String> {
        let input = include_str!("../test.txt");
        let actual = solve_part2(input)?;
        let solution = 0;
        assert_eq!(actual, solution);
        Ok(())
    }

    // #[test]
    fn test_solve_part2() -> Result<(), String> {
        let input = include_str!("../input.txt");
        let actual = solve_part2(input)?;
        let solution = 0;
        assert_eq!(actual, solution);
        Ok(())
    }
}
