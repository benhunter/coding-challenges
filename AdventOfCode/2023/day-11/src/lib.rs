use itertools::Itertools;

use util::{Coord, parse_grid, ParseError};

pub fn solve_part1(input: &str) -> Result<i32, String> {
    let mut research = parse(input)?;

    research.expand();

    let galaxies = research.galaxies();
    let galaxies = research.expanded_galaxies();
    let pairs: Vec<Vec<&Galaxy>> = galaxies.iter().combinations(2).collect();
    let distances = pairs.iter().map(|pair| {
        let galaxy1 = pair[0];
        let galaxy2 = pair[1];
        // dbg!(galaxy1);
        // dbg!(galaxy2);
        let distance = ((galaxy1.coord.x as i32 - galaxy2.coord.x as i32) as i32).abs() + ((galaxy1.coord.y as i32 - galaxy2.coord.y as i32) as i32).abs();
        println!("galaxy1: {:?}, galaxy2: {:?}, distance: {:?}", galaxy1, galaxy2, distance);
        Ok(distance)
    }).collect::<Result<Vec<i32>, String>>()?;

    let sum = distances.iter().sum();
    dbg!(sum);

    Ok(sum)
}

pub fn solve_part2(input: &str) -> i32 {
    0
}

#[derive(Debug, PartialEq, Clone, Default)]
struct Research {
    image: Vec<Vec<Pixel>>,
    expanded: Vec<Vec<Pixel>>,
}

impl Research {
    pub(crate) fn galaxies_count(&self) -> i32 {
        self.image.iter().flatten().filter(|x| x == &&Pixel::Galaxy).count() as i32
    }

    pub(crate) fn galaxies(&self) -> Vec<Galaxy> {
        let mut galaxies: Vec<Galaxy> = Vec::new();
        let mut galaxy_id = 0;
        for y in 0..self.image.len() {
            for x in 0..self.image[y].len() {
                if self.image[y][x] == Pixel::Galaxy {
                    let galaxy = Galaxy {
                        id: galaxy_id,
                        coord: Coord { x, y },
                    };
                    galaxies.push(galaxy);
                    galaxy_id += 1;
                }
            }
        }

        galaxies
    }

    pub(crate) fn expanded_galaxies(&self) -> Vec<Galaxy> {
        let mut galaxies: Vec<Galaxy> = Vec::new();
        let mut galaxy_id = 0;
        for y in 0..self.expanded.len() {
            for x in 0..self.expanded[y].len() {
                if self.expanded[y][x] == Pixel::Galaxy {
                    let galaxy = Galaxy {
                        id: galaxy_id,
                        coord: Coord { x, y },
                    };
                    galaxies.push(galaxy);
                    galaxy_id += 1;
                }
            }
        }

        galaxies
    }

    pub(crate) fn expand(&mut self) {
        let expanded_rows = self.image.iter()
            .fold(Vec::new(), |mut acc, row| {
                if row.contains(&Pixel::Galaxy) {
                    acc.push(row.clone());
                } else {
                    acc.push(row.clone());
                    acc.push(row.clone());
                }
                acc
            });

        let mut expanded: Vec<Vec<Pixel>> = vec![vec![]; expanded_rows.len()];
        for column in 0..expanded_rows[0].len() {
            // if column does not contain a galaxy, expand it for each row
            if !expanded_rows.iter().any(|row| row[column] == Pixel::Galaxy) {
                for row in 0..expanded_rows.len() {
                    expanded[row].push(expanded_rows[row][column].clone());
                    expanded[row].push(expanded_rows[row][column].clone());
                }
            } else {
                for row in 0..expanded_rows.len() {
                    expanded[row].push(expanded_rows[row][column].clone());
                }
            }
        }

        dbg!(expanded_rows.clone());
        self.expanded = expanded
    }
}

#[derive(Debug, PartialEq)]
struct Galaxy {
    id: i32,
    coord: Coord,
}

#[derive(Debug, PartialEq, Clone, Default)]
enum Pixel {
    Galaxy,
    #[default]
    Empty,
}

impl Pixel {
    fn parse(c: char) -> Result<Pixel, ParseError> {
        match c {
            '#' => Ok(Pixel::Galaxy),
            '.' => Ok(Pixel::Empty),
            _ => Err(ParseError::InvalidCharacter(c)),
        }
    }
}

fn parse(input: &str) -> Result<Research, ParseError> {
    let grid = parse_grid(input, Pixel::parse)?;
    Ok(
        Research {
            image: grid,
            ..Default::default()
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() -> Result<(), String> {
        let input = include_str!("../test.txt");
        let actual_research = parse(input).unwrap();
        let expected_rows = 10;
        assert_eq!(actual_research.image.len(), expected_rows);

        let expected_galaxies = 9;
        assert_eq!(actual_research.galaxies_count(), expected_galaxies);
        Ok(())
    }

    #[test]
    fn test_galaxies() -> Result<(), String> {
        let input = include_str!("../test.txt");
        let research = parse(input).unwrap();

        let first_expected_galaxy = Galaxy {
            id: 0,
            coord: Coord { x: 3, y: 0 },
        };

        let galaxies = research.galaxies();
        assert_eq!(galaxies[0], first_expected_galaxy);

        let expected_galaxies = 9;
        assert_eq!(galaxies.len(), expected_galaxies);

        Ok(())
    }

    #[test]
    fn test_expand_simple() -> Result<(), String> {
        let input = "..\n..";
        let mut research = parse(input).unwrap();
        research.expand();

        let expected_expanded_rows = 4;
        assert_eq!(research.expanded.len(), expected_expanded_rows);

        let expected_expanded_cols = 4;
        dbg!(research.expanded.clone());
        assert_eq!(research.expanded[0].len(), expected_expanded_cols);

        Ok(())
    }

    #[test]
    fn test_expand() -> Result<(), String> {
        let input = include_str!("../test.txt");
        let mut research = parse(input).unwrap();
        research.expand();

        let expected_expanded_rows = 12;
        assert_eq!(research.expanded.len(), expected_expanded_rows);

        let expected_expanded_cols = 13;
        assert_eq!(research.expanded[0].len(), expected_expanded_cols);

        Ok(())
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let input = include_str!("../test.txt");
        let actual = solve_part1(input)?;
        let solution = 374;
        assert_eq!(actual, solution);
        Ok(())
    }

    #[test]
    fn test_solve_part1() -> Result<(), String> {
        let input = include_str!("../input.txt");
        let actual = solve_part1(input)?;
        let solution = 9591768;
        assert_eq!(actual, solution);
        Ok(())
    }

    // #[test]
    fn test_part2() {
        let input = include_str!("../test.txt");
        let actual = solve_part2(input);
        let solution = 0;
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
