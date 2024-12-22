use std::collections::HashMap;
use std::str::FromStr;

use util::{parse_grid_chars, Direction, ParseError};
use util::Coord;

pub fn solve_part1(input: &str) -> Result<usize, String> {
    Ok(input.parse::<Racetrack>()?.find_cheats(100))
}

pub fn solve_part2(_input: &str) -> Result<i64, String> {
    todo!()
}

#[derive(Debug, PartialEq, Clone, Default)]
struct Racetrack {
    grid: Vec<Vec<char>>,
    start: Option<Coord>,
    end: Option<Coord>,
}

impl FromStr for Racetrack {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles = parse_grid_chars(s)?;
        let mut track = Racetrack { grid: tiles, start: None, end: None };

        track.start = track.clone().find('S');
        track.end = track.clone().find('E');

        Ok(track)
        //Err(ParseError::BadInput)
    }

}

impl Racetrack {
    fn find(self, ch: char) -> Option<Coord> {
        for (r, row) in self.grid.iter().enumerate() {
            for (c, col) in row.iter().enumerate() {
                if *col == ch {
                    return Some(Coord::new(c as i64, r as i64));
                }
            }
        }
        None
    }

    fn find_cheats(&self, picos_saved: i64) -> usize {
        // Solve track and assign numbers to each tile.
        let mut track: Vec<Vec<Option<i64>>> = vec![vec![None; self.grid[0].len()]; self.grid.len()];
        let mut curr = self.start.unwrap();
        let mut count = 0;
        track[curr.y as usize][curr.x as usize] = Some(count);

        let mut loops = 0;
        while curr != self.end.unwrap() {
            for neighbor in Direction::iter() {
                let neighbor_coord = curr.go(neighbor);
                //println!("curr={:?}", curr);
                if ['.', 'E'].contains(&self.grid[neighbor_coord.y as usize][neighbor_coord.x as usize]) && track[neighbor_coord.y as usize][neighbor_coord.x as usize] == None {
                    count += 1;
                    track[neighbor_coord.y as usize][neighbor_coord.x as usize] = Some(count);
                    curr = neighbor_coord;
                    break
                }
            }
            loops += 1;
        }

        //println!("{:?}", track);
        println!();
        for (yi, y) in track.iter().enumerate() {
            for (xi, x) in y.iter().enumerate() {
                let c = match x {
                    Some(n) => n.to_string(),
                    //None => "#".to_string(),
                    None => self.grid[yi][xi].to_string(),
                };
                print!("{:<4}", c);
            }
            println!();
        }
        println!();

        // Find all cheats. Cheat allows bypass through wall to a higher numbered tile.
        let cheats: HashMap<Coord, i64> = find_cheats_part1(&track);

        //for c in &cheats {
        //    println!("{:?}", c);
        //}
        // Render cheats

        println!();
        for y in 0..track.len() {
            for x in 0..track[0].len() {
                let c = if cheats.contains_key(&Coord::new(x.try_into().unwrap(), y.try_into().unwrap())) {
                    '@'.to_string()
                } else {
                    //println!("track[{}][{}]={:?}", y, x, track[y][x]);
                    match self.grid[y][x] {
                        '.' => {
                            track[y][x].unwrap().to_string()
                            //if track[y][x].is_some() {
                            //    track[y][x].unwrap().to_string()
                            //} else {
                            //    '.'.to_string()
                            //}
                        }
                        c => c.to_string(),
                    }
                };
                print!("{:<4}", c);
            }
            println!();
        }
        println!();

        // Calc savings.
        let count = cheats.into_iter().filter(|c| c.1 >= picos_saved).map(|c| c.1).collect::<Vec<i64>>().len();
        count
    }
}

fn find_cheats_part1(track: &Vec<Vec<Option<i64>>>) -> HashMap<Coord, i64> {
    let mut cheats: HashMap<Coord, i64> = Default::default();
    for y in 1..track.len() - 1 {
        for x in 1..track[0].len() - 1 {
            if track[y][x].is_some() {
                continue;
            }
            if track[y][x-1].is_some() && track[y][x+1].is_some() {
                let score = (track[y][x-1].unwrap() - track[y][x+1].unwrap()).abs() - 2;
                cheats.insert(Coord::new(x.try_into().unwrap(), y.try_into().unwrap()), score);
            } else if track[y-1][x].is_some() && track[y+1][x].is_some() {
                let score = (track[y-1][x].unwrap() - track[y+1][x].unwrap()).abs() - 2;
                cheats.insert(Coord::new(x.try_into().unwrap(), y.try_into().unwrap()), score);
            }
        }
    }
    cheats
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse() -> Result<(), String> {
        let input = include_str!("../test.txt");
        let actual = input.parse::<Racetrack>()?;
        let expected_tiles_len = 15;
        assert_eq!(actual.grid.len(), expected_tiles_len);
        let expected_tiles_0_len = 15;
        assert_eq!(actual.grid[0].len(), expected_tiles_0_len);
        let expected_start = Coord::new(1, 3);
        assert_eq!(actual.start, Some(expected_start));
        let expected_end = Coord::new(5, 7);
        assert_eq!(actual.end, Some(expected_end));
        Ok(())
    }

    #[test]
    fn test_find_cheat() -> Result<(), String> {
        let input = include_str!("../test.txt");
        let actual = input.parse::<Racetrack>()?.find_cheats(64); // Find all cheats that save at least 64 picoseconds.
        let expected = 1;
        assert_eq!(actual, expected);

        let actual = input.parse::<Racetrack>()?.find_cheats(40); // Find all cheats that save at least 64 picoseconds.
        let expected = 2;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let input = include_str!("../test.txt");
        let actual = solve_part1(input)?;
        let solution = 0;
        assert_eq!(actual, solution);
        Ok(())
    }

     #[test]
    fn test_solve_part1() -> Result<(), String> {
        let input = include_str!("../input.txt");
        let actual = solve_part1(input)?;
        let solution = 1375;
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
