use core::panic;
use std::{collections::HashMap, str::FromStr, usize};

use util::{parse_grid_chars, Coord, Direction, Distance, ParseError};

pub fn solve_part1(input: &str) -> Result<i64, String> {
    let m = input.parse::<Maze>()?;
    m.solve()
}

pub fn solve_part2(_input: &str) -> Result<i64, String> {
    todo!()
}

#[derive(Debug, PartialEq, Clone, Default)]
struct Maze {
    grid: Vec<Vec<char>>,
    start: Option<Coord>,
    end: Option<Coord>,
}

impl FromStr for Maze {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles = parse_grid_chars(s)?;
        let mut maze = Maze { grid: tiles, start: None, end: None };

        maze.start = maze.clone().find('S');
        maze.end = maze.clone().find('E');

        Ok(maze)
        //Err(ParseError::BadInput)
    }

}

impl Maze {
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

    fn solve(self) -> Result<i64, String> {
        // Attempting A*. See also 2023/day-17
        // From the start Coord, measure the distance to the end.
        // Search down the shortest path.
        let width = self.grid[0].len();
        let height = self.grid.len();
        let start = self.start.expect("need start");
        let end = self.end.expect("need end");
        let mut distances: Vec<Vec<Distance>> = vec![vec![Distance::Infinity; width]; height];
        distances[start.y as usize][start.x as usize] = Distance::Value(0);

        //let mut open_set = vec![self.start.unwrap().clone(); 1]; // Use VecDequeue if we need to .remove(0) every loop
        let mut open_set: Vec<Position> = vec![];
        let start_position = Position { coord: start.clone(), direction: Direction::Up };
        open_set.push(start_position.clone());
        let mut frontier_scores: Vec<Vec<Distance>> = vec![vec![Distance::Infinity; width]; height];
        frontier_scores[start.y as usize][start.x as usize] = Distance::Value(self.heuristic(&start_position));
        let mut came_from: HashMap<Coord, Position> = Default::default();

        let mut count = 0;
        while open_set.len() > 0 {
            //println!("Iteration={} Distances:", count);
            //println!();
            //self.visualize_distances(&distances);

            &open_set.sort_by(|a, b| {
                //frontier_scores[a.coord.y as usize][a.coord.x as usize].cmp(&frontier_scores[b.coord.y as usize][b.coord.x as usize])
                frontier_scores[b.coord.y as usize][b.coord.x as usize].cmp(&frontier_scores[a.coord.y as usize][a.coord.x as usize])
            });
            //println!("frontier_scores={:?}", frontier_scores);
            let current = open_set.pop().unwrap(); // TODO min frontier_scores

            if current.clone().coord == end {
                let d = &frontier_scores[current.coord.y as usize][current.coord.x as usize];

                //println!("Iteration={} Distances:", count);
                //println!();
                //self.visualize_distances(&distances);
                //println!("Score={:?}", d);
                println!();
                self.visualize_came_from(&came_from);

                return if let Distance::Value(v) = d { Ok(*v) } else { panic!("fake news") }
            }

            for neighbor_dir in Direction::iter() {
                //println!("current.direction={:?} neighbor_dir=Direction::{:?} curr==neigh_dir={}", current.direction, neighbor_dir, current.direction == neighbor_dir);

                // Is direction valid, not a #?
                let neighbor = current.coord.go(neighbor_dir);
                if self.grid[neighbor.y as usize][neighbor.x as usize] == '#' {
                    continue
                }

                let current_dir_val: i8 = current.direction.into();
                //let turns_cost: i8 = (current_dir_val - <Direction as Into<i8>>::into(neighbor_dir)).abs();
                let turns_cost: i16 = if current.direction == neighbor_dir {
                    //println!("inner current.direction={:?} Direction::{:?} curr==neigh_dir={}", current.direction, neighbor_dir, current.direction == neighbor_dir);
                    0
                } else {
                    let cost = (current_dir_val - <Direction as Into<i8>>::into(neighbor_dir)).abs() % 2;
                    //println!("inner cost={}", cost);
                    match cost {
                        1 => 1000,
                        0 => 2000,
                        _ => panic!("impossibru")
                    }
                };
                //println!("turns_cost={:?}", turns_cost);

                let dist_curr_to_neighbor = 1 + turns_cost as i64;
                //let tentative_global_score = if let Distance::Value(v) = &distances[current.coord.y as usize][current.coord.x as usize] { v } else { panic!("fake news") } + dist_curr_to_neighbor;
                let tentative_global_score = distances.clone()[current.coord.y as usize][current.coord.x as usize].add_i64(dist_curr_to_neighbor);
                //let neighbor_global_score = if let Distance::Value(v) = distances[neighbor.y as usize][neighbor.x as usize] { v } else { panic!("wut")};
                let neighbor_global_score = distances[neighbor.y as usize][neighbor.x as usize];

                //println!("Checking tentative_global_score={:?} < neighbor_global_score={:?} ={}", tentative_global_score, neighbor_global_score, tentative_global_score < neighbor_global_score);
                if tentative_global_score < neighbor_global_score {
                    //println!("...true");
                    came_from.insert(neighbor, current);

                    //if count > 97 {
                        //println!("came_from: {:?}", came_from);
                        //println!();
                        //self.visualize_came_from(&came_from);
                        //panic!("DEBUG");
                    //}

                    //distances[neighbor.y as usize][neighbor.x as usize] = Distance::Value(tentative_global_score);
                    distances[neighbor.y as usize][neighbor.x as usize] = tentative_global_score;
                    let neighbor_posn = Position { coord: neighbor, direction: neighbor_dir };
                    frontier_scores[neighbor.y as usize][neighbor.x as usize] = tentative_global_score.add_i64(self.heuristic(&neighbor_posn));
                    if !open_set.contains(&neighbor_posn) {
                        //println!("Adding neighbor={:?}", neighbor);
                        open_set.push(neighbor_posn);
                    }

                }
            }

            // Find the next neighbor to the lowest cost tile.
            //let lowest_cost = self.grid
            //    .iter()
            //    .enumerate()
            //    .map(|(yi, y)| {
            //        y
            //        .iter()
            //        .enumerate()
            //        .map(move |(xi, x)| (xi, yi, x))
            //    })
            //    .flatten()
            //    .min_by_key(|(xi, yi, x)| {
            //        match distances[*yi][*xi] {
            //            Distance::Infinity => MAX,
            //            Distance::Value(v) => v,
            //        }
            //    });
            //println!("{:?}", lowest_cost);

            count += 1;
        }
        Ok(0)
    }

    /**
     * Heuristic function for A*. Estimates the cost from n to end.
     * Cost: 1000 * turn + 1 * movement in x and y
     */
    fn heuristic(&self, n: &Position) -> i64 {
        let end = self.end.expect("need end");
        (n.coord.x - end.x).abs() + 1000 + (n.coord.y - end.y).abs()
    }

    fn visualize_distances(&self, distances: &Vec<Vec<Distance>>) {
        for y in 0..distances.len() {
            for x in 0..distances[0].len() {
                match self.grid[y][x] {
                    '.' => match distances[y][x] {
                        Distance::Infinity => print!("{:^8}", "?"),
                        Distance::Value(v) => {
                            print!("{:^8} ", v);
                        }
                    },
                    _ => print!("{:^6}", self.grid[y][x]),
                }
            }
            println!();
        }
        println!();
    }

    fn visualize_came_from(&self, came_from: &HashMap<Coord, Position>) -> () {
        let curr = self.end.expect("end");
        let mut render: Vec<Vec<Option<char>>> = vec![vec![None; self.grid[0].len()]; self.grid.len()];
        let mut prev = came_from.get(&curr);
        println!("End: Came from: {:?}. To get to: {:?}", prev, curr);

        if prev.is_some() {
            let prev_posn = prev.expect("prev");
            render[prev_posn.coord.y as usize][prev_posn.coord.x as usize] = Some(prev_posn.direction.into());
        }

        while prev.is_some() {
            let curr = prev;
            prev = came_from.get(&curr.expect("Some(curr)").coord);
            println!("Came from: {:?}. To get to: {:?}", prev, curr);

            if prev.is_some() {
                let prev_posn = prev.expect("prev");
                //render[prev_posn.coord.y as usize][prev_posn.coord.x as usize] = Some(prev_posn.direction.into());
                render[prev_posn.coord.y as usize][prev_posn.coord.x as usize] = Some(curr.expect("curr").direction.into());
            }

            if curr == prev {
                println!("curr == prev");
                panic!("can't come from self");
            }
            //panic!("DEBUG");
        }

        println!();
        for y in 0..self.grid.len() {
            for x in 0..self.grid[0].len() {
                let c = match self.grid[y][x] {
                    '.' => {
                        match render[y][x] {
                            Some(c) => c,
                            None => '.',
                        }
                    },
                    _ => self.grid[y][x],
                };
                print!("{}", c);
            }
            println!();
        }
        println!();
    }
}

//#[derive(Debug, Clone, PartialEq)]
//pub enum Distance {
//    Infinity,
//    Value(i64),
//}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Position {
    coord: Coord,
    direction: Direction,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() -> Result<(), String> {
        let input = include_str!("../test.txt");
        let actual = input.parse::<Maze>()?;
        let expected_tiles_len = 15;
        assert_eq!(actual.grid.len(), expected_tiles_len);
        let expected_tiles_0_len = 15;
        assert_eq!(actual.grid[0].len(), expected_tiles_0_len);
        let expected_start = Coord::new(1, 13);
        assert_eq!(actual.start, Some(expected_start));
        let expected_end = Coord::new(13, 1);
        assert_eq!(actual.end, Some(expected_end));
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let input = include_str!("../test.txt");
        let actual = solve_part1(input)?;
        let solution = 7036;
        assert_eq!(actual, solution);
        Ok(())
    }

    #[test]
    fn test2_part1() -> Result<(), String> {
        let input = include_str!("../test2.txt");
        let actual = solve_part1(input)?;
        let solution = 11048;
        assert_eq!(actual, solution);
        Ok(())
    }

     #[test]
    fn test_solve_part1() -> Result<(), String> {
        let input = include_str!("../input.txt");
        let actual = solve_part1(input)?;
        let wrong = 137512; // 137512 too high
        assert!(actual < wrong);
        let solution = 1;
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
