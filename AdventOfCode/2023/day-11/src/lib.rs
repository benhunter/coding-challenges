use itertools::Itertools;
use util::{Coord, parse_grid, ParseError};

pub fn solve_part1(input: &str) -> Result<i32, String> {
    let mut research = parse(input)?;
    research.expand();

    let galaxies = research.expanded_galaxies();
    let pairs: Vec<Vec<&Galaxy>> = galaxies.iter().combinations(2).collect();
    let distances = pairs.iter().map(distance).collect::<Result<Vec<i32>, String>>()?;

    let sum = distances.iter().sum();
    Ok(sum)
}

pub fn solve_part2(input: &str) -> Result<i64, String> {
    Ok(solve_part2_expand_to(input, 1000000)?)
}

pub fn solve_part2_expand_to(input: &str, expand_to: i32) -> Result<i64, String> {
    let mut research = parse(input)?;
    research.expand_to(expand_to);
    // print_expanded(&mut research);

    let galaxies = research.expanded_galaxies();
    let pairs: Vec<Vec<&Galaxy>> = galaxies.iter().combinations(2).collect();
    let distances = pairs.iter()
        .map(|pair| research.distance_expanded(pair))
        .collect::<Result<Vec<i64>, String>>()?;

    let sum = distances.iter().sum();
    Ok(sum)
}

fn print_expanded(research: &mut Research) {
    research.expanded.iter().for_each(|row| {
        row.iter().for_each(|pixel| {
            match pixel {
                Pixel::Galaxy => print!(" # "),
                Pixel::Empty => print!(" . "),
                Pixel::Expanded(d) => print!("{:3}", d),
            }
        });
        println!();
    });
    println!();
}

fn distance(pair: &Vec<&Galaxy>) -> Result<i32, String> {
    let galaxy1 = pair[0];
    let galaxy2 = pair[1];
    let distance = (galaxy1.coord.x as i32 - galaxy2.coord.x as i32).abs()
        + (galaxy1.coord.y as i32 - galaxy2.coord.y as i32).abs();
    Ok(distance)
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

        self.expanded = expanded
    }

    pub(crate) fn expand_to(&mut self, distance: i32) {
        let expanded_rows = self.image.iter()
            .fold(Vec::new(), |mut acc, row| {
                if row.contains(&Pixel::Galaxy) {
                    acc.push(row.clone());
                } else {
                    acc.push(vec![Pixel::Expanded(distance); row.len()]);
                }
                acc
            });

        let mut expanded: Vec<Vec<Pixel>> = vec![vec![]; expanded_rows.len()];
        for column in 0..expanded_rows[0].len() {
            if expanded_rows.iter().any(|row| row[column] == Pixel::Galaxy) {
                for row in 0..expanded_rows.len() {
                    expanded[row].push(expanded_rows[row][column].clone());
                }
            } else { // Empty column
                for row in 0..expanded_rows.len() {
                    expanded[row].push(Pixel::Expanded(distance));
                }
            }
        }

        self.expanded = expanded
    }

    fn distance_expanded(&self, pair: &Vec<&Galaxy>) -> Result<i64, String> {
        let galaxy1 = pair[0];
        let galaxy2 = pair[1];

        let iter_row = galaxy1.coord.y..galaxy2.coord.y;

        let min_x = galaxy1.coord.x.min(galaxy2.coord.x);
        let max_x = galaxy1.coord.x.max(galaxy2.coord.x);
        let iter_col = min_x..max_x;

        let row_distances: Vec<i32> = iter_row.map(|y| {
            if y == galaxy1.coord.y {
                return 1;
            }
            match self.expanded[y][galaxy1.coord.x] {
                Pixel::Expanded(d) => d,
                Pixel::Empty => 1,
                Pixel::Galaxy => 1,
            }
        }).collect();

        let col_distances: Vec<i32> = iter_col.map(|x| {
            if x == galaxy1.coord.x {
                return 1;
            }
            match self.expanded[galaxy2.coord.y][x] {
                Pixel::Expanded(d) => d,
                Pixel::Empty => 1,
                Pixel::Galaxy => 1,
            }
        }).collect();

        let distance = row_distances.iter().sum::<i32>() + col_distances.iter().sum::<i32>();
        Ok(distance as i64)
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
    Expanded(i32),
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

    #[test]
    fn test_expand_to() -> Result<(), String> {
        let expand_to = 10;
        let input = include_str!("../test.txt");
        let mut research = parse(input).unwrap();
        research.expand_to(expand_to);

        let expected_expanded_rows = 10;
        assert_eq!(research.expanded.len(), expected_expanded_rows);

        let expected_expanded_cols = 10;
        assert_eq!(research.expanded[0].len(), expected_expanded_cols);

        let expected_expanded_row = vec![
            Pixel::Empty,
            Pixel::Empty,
            Pixel::Expanded(expand_to),
            Pixel::Galaxy,
            Pixel::Empty,
            Pixel::Expanded(expand_to),
            Pixel::Empty,
            Pixel::Empty,
            Pixel::Expanded(expand_to),
            Pixel::Empty,
        ];
        assert_eq!(research.expanded[0], expected_expanded_row);

        let expected_row_index_3 = vec![Pixel::Expanded(expand_to); expected_expanded_cols];
        assert_eq!(research.expanded[3], expected_row_index_3);
        Ok(())
    }

    #[test]
    fn test_simple_expand_to_10() -> Result<(), String> {
        let input = "#..\n...\n..#";
        let expand_to = 10;
        let actual = solve_part2_expand_to(input, expand_to)?;

        let expected_distance = 22;
        assert_eq!(actual, expected_distance);
        Ok(())
    }

    #[test]
    fn test_column_dist() -> Result<(), String> {
        let input = include_str!("../test.txt");
        let mut research = parse(input)?;
        let expand_to = 10;
        research.expand_to(expand_to);

        let one = Galaxy { id: 6, coord: Coord { x: 7, y: 8 } };
        let two = Galaxy { id: 8, coord: Coord { x: 4, y: 9 } };
        let pair = vec![&one, &two];

        let actual = research.distance_expanded(&pair)?;
        let expected_distance = 13;
        assert_eq!(actual, expected_distance);
        Ok(())
    }

    #[test]
    fn test_simple_expand_to_100() -> Result<(), String> {
        let input = "#..\n...\n..#";
        let expand_to = 100;
        let actual = solve_part2_expand_to(input, expand_to)?;

        let expected_distance = 202;
        assert_eq!(actual, expected_distance);
        Ok(())
    }

    #[test]
    fn test_part2_expand_to_10() -> Result<(), String> {
        let expand_to = 10;
        let input = include_str!("../test.txt");
        let actual = solve_part2_expand_to(input, expand_to)?;

        let expected_distance = 1030;
        assert_eq!(actual, expected_distance);
        Ok(())
    }

    #[test]
    fn test_part2_expand_to_100() -> Result<(), String> {
        let expand_to = 100;
        let expected_distance = 8410;

        let input = include_str!("../test.txt");
        let actual = solve_part2_expand_to(input, expand_to)?;
        assert_eq!(actual, expected_distance);
        Ok(())
    }

    #[test]
    fn test_solve_part2() -> Result<(), String> {
        let input = include_str!("../input.txt");
        let actual = solve_part2(input)?;

        let solution = 746962097860;
        assert_eq!(actual, solution);
        Ok(())
    }
}
