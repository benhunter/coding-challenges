use itertools::Itertools;

use util::{Coord, ParseError};

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
    let mut stack = parse(input)?;
    stack.check_invalid();
    stack.bricks.iter().for_each(|brick| {
        // println!("{:?}", brick);
    });
    stack.render_xz();
    stack.render_yz();

    stack.fall();

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
    pub(crate) fn fall(&mut self) {
        let z_max = self.bricks.iter()
            .map(|brick| {
                brick.position_end.2
            })
            .max().unwrap();

        let mut loop_count = 0;
        loop {
            println!("Loop count: {}", loop_count);
            let mut did_any_bricks_move = false;
            let mut moved_bricks = vec![];

            (2..=z_max).for_each(|z| {
                // if z > 5 {
                //     panic!("debug")
                // }
                let updated_bricks = self.bricks.iter()
                    // find bricks at z
                    .filter(|brick| {
                        brick.position_start.2 == z
                    })
                    // .inspect(|&brick| { dbg!(brick); })
                    .filter(|brick| {
                        // check if there is a brick under it
                        let bricks_under = self.bricks.iter()
                            // skip self
                            .filter(|&other| brick.id != other.id)
                            .filter(|&lower_brick| {
                                if !(lower_brick.position_start.2 <= (z - 1)
                                    && lower_brick.position_end.2 >= (z - 1)) {
                                    return false;
                                }

                                // brick corners
                                // nw - ne
                                // sw - se
                                // let nw = (brick.position_start.0, brick.position_end.1);
                                let brick_ne = (brick.position_end.0, brick.position_end.1);
                                let brick_sw = (brick.position_start.0, brick.position_start.1);
                                // let se = (brick.position_end.0, brick.position_start.1);

                                if (brick.position_end.0 < lower_brick.position_start.0 || brick.position_start.0 > lower_brick.position_end.0) {
                                    return false;
                                }

                                if (brick.position_end.1 < lower_brick.position_start.1 || brick.position_start.1 > lower_brick.position_end.1) {
                                    return false;
                                }

                                true
                            }).collect::<Vec<&Brick>>();
                        bricks_under.is_empty()
                    })
                    .map(|brick| {

                        // move brick down
                        Brick {
                            position_start: (brick.position_start.0, brick.position_start.1, brick.position_start.2 - 1),
                            position_end: (brick.position_end.0, brick.position_end.1, brick.position_end.2 - 1),
                            id: brick.id,
                        }
                    })
                    .collect::<Vec<Brick>>();

                // if not, move it down
                // update self.bricks
                self.bricks.iter_mut()
                    .for_each(|brick| {
                        let b = brick.clone();
                        updated_bricks.iter()
                            .filter(|update| {
                                update.id == b.id //&&
                            })
                            .for_each(|update| {
                                *brick = update.clone();
                                moved_bricks.push(brick.id);
                            });
                    });

                self.render_xz();

                if !updated_bricks.is_empty() {
                    println!("Bricks moved");
                    did_any_bricks_move = true;
                }
            });

            if !did_any_bricks_move {
                println!("No bricks moved");
                break;
            }

            loop_count += 1;
        }
    }
}

impl Stack {
    fn render_xz(&self) -> String {
        let mut output = String::new();

        let z_max = self.bricks.iter()
            .map(|brick| {
                brick.position_end.2
            })
            .max().unwrap();

        let x_max = self.bricks.iter()
            .map(|brick| {
                brick.position_end.0
            })
            .max().unwrap();

        let y_max = self.bricks.iter()
            .map(|brick| {
                brick.position_end.1
            })
            .max().unwrap();

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
                let bricks_y = self.bricks.iter()
                    .filter(|brick| {
                        brick.position_start.0 <= x && brick.position_end.0 >= x
                            && brick.position_start.2 <= z && brick.position_end.2 >= z
                    })
                    .collect::<Vec<&Brick>>();
                let closest_y = bricks_y.iter()
                    .map(|brick| brick.id)
                    .max().unwrap_or('.');
                print!("{}", closest_y);

                if bricks_y.len() > 1 {
                    output.push('?');
                } else {
                    output.push(closest_y);
                }
            });

            print!(" {}", z);
            output.push_str(&format!(" {}", z));
            if z == (z_max + 1) / 2 {
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

    fn render_yz(&self) -> String {
        let mut output = String::new();

        let z_max = self.bricks.iter()
            .map(|brick| {
                brick.position_end.2
            })
            .max().unwrap();

        // let x_max = self.bricks.iter()
        //     .map(|brick| {
        //         brick.position_end.0
        //     })
        //     .max().unwrap();

        let y_max = self.bricks.iter()
            .map(|brick| {
                brick.position_end.1
            })
            .max().unwrap();

        println!("\n{:=>y_max$}", "=", y_max = (y_max + 3) as usize);

        let y_label_position = y_max / 2 + 1;
        println!("{}", format!("{: >width_y$}", "y", width_y = y_label_position as usize));
        output.push_str(&format!("{}", format!("{: >width_y$}\n", "y", width_y = y_label_position as usize)));

        (0..=y_max).for_each(|y| {
            print!("{}", y);
            output.push_str(&format!("{}", y));
        });
        println!();
        output.push('\n');

        // render rows from z_max to 0
        (1..=z_max).rev().for_each(|z| {
            (0..=y_max).for_each(|y| {
                let closest_x = self.bricks.iter()
                    .filter(|brick| {
                        brick.position_start.1 <= y && brick.position_end.1 >= y
                            && brick.position_start.2 <= z && brick.position_end.2 >= z
                    })
                    .map(|brick| brick.id)
                    .max().unwrap_or('.');
                print!("{}", closest_x);
                output.push(closest_x);
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

        println!("{:->y_max$} 0", "-", y_max = (y_max + 1) as usize);
        output.push_str(&format!("{:->y_max$} 0", "-", y_max = (y_max + 1) as usize));
        println!("{:=>y_max$}", "=", y_max = (y_max + 3) as usize);
        output
    }

    fn check_invalid(&self) {
        // find bricks with any start coord greater than any end coord
        let invalid = self.bricks.iter().filter(|brick| {
            brick.position_start.0 > brick.position_end.0
                || brick.position_start.1 > brick.position_end.1
                || brick.position_start.2 > brick.position_end.2
                || brick.position_start.0 < 0
                || brick.position_start.1 < 0
                || brick.position_start.2 < 0
        }).map(|brick| {
            println!("Found {:?}", brick);
            brick
        }).collect::<Vec<&Brick>>();

        if !invalid.is_empty() {
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

    #[test]
    fn test_view_snapshot_yz() -> Result<(), String> {
        let expected = " y
012
.G. 9
.G. 8
... 7
.F. 6
EEE 5 z
DDD 4
..C 3
B.. 2
AAA 1
--- 0";
        let input = include_str!("../test.txt");
        let stack = parse(input)?;
        let actual = stack.render_yz();
        assert_eq!(actual.to_string(), expected);
        Ok(())
    }

    #[test]
    fn test_fall() -> Result<(), String> {
        let input = include_str!("../test.txt");
        let mut stack = parse(input)?;
        stack.fall();
        let actual = stack.render_xz();
        let expected = " x
012
.G. 6
.G. 5
FFF 4
D.E 3 z
??? 2
.A. 1
--- 0";
        assert_eq!(actual, expected);
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
