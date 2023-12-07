mod test;

fn main() {
    let input = include_str!("../input.txt");
    let result = solve_part1(input);
    println!("✅ part1: {}", result);

    let result = solve_part2(input);
    println!("✅ part2: {}", result);
}

fn solve_part1(input: &str) -> u64 {
    let almanac = parse(input);
    almanac.seeds.iter()
        .map(|seed| almanac.seed_to_location(*seed))
        .min().unwrap()
}

fn solve_part2(input: &str) -> u64 {
    let almanac = parse(input);
    let mut pairs = almanac.seeds.chunks(2);
    pairs.map(|pair| {
        (pair[0]..(pair[0] + pair[1]))
            // .filter(|seed| seed < &(pair[0] + 1))
            // .inspect(|seed| println!("{}", seed))
            .map(|seed| {
                if seed % 100000000 == 0 {
                    println!("pair: {:?}, seed: {}", pair, seed);
                }
                almanac.seed_to_location(seed)
            }).min().unwrap()
    }).min().unwrap()
}

#[derive(Clone, Debug, PartialEq)]
struct Almanac {
    seeds: Vec<u64>,
    // Map: destination range start, the source range start, and the range length.
    seed_to_soil_map: Vec<Map>,
    soil_to_fertilizer_map: Vec<Map>,
    fertilizer_to_water_map: Vec<Map>,
    water_to_light_map: Vec<Map>,
    light_to_temperature_map: Vec<Map>,
    temperature_to_humidity_map: Vec<Map>,
    humidity_to_location_map: Vec<Map>,
}

impl Almanac {
    pub(crate) fn map(&self, x_to_y_map: &Vec<Map>, value: u64) -> u64 {
        let range = x_to_y_map.iter()
            .find(|map| value >= map.source_range_start && value < (map.source_range_start + map.range_length));
        match range {
            None => { value }
            Some(range) => {
                let offset = value - range.source_range_start;
                range.destination_range_start + offset
            }
        }
    }

    pub(crate) fn seed_to_location(&self, seed: u64) -> u64 {
        let maps = vec![
            &self.seed_to_soil_map,
            &self.soil_to_fertilizer_map,
            &self.fertilizer_to_water_map,
            &self.water_to_light_map,
            &self.light_to_temperature_map,
            &self.temperature_to_humidity_map,
            &self.humidity_to_location_map,
        ];
        maps.iter()
            .fold(seed, |value, map| {
                self.map(map, value)
            })

        // let actual = almanac.map(&almanac.seed_to_soil_map, seed);
        // let actual = almanac.map(&almanac.soil_to_fertilizer_map, actual);
        // let actual = almanac.map(&almanac.fertilizer_to_water_map, actual);
        // let actual = almanac.map(&almanac.water_to_light_map, actual);
        // let actual = almanac.map(&almanac.light_to_temperature_map, actual);
        // let actual = almanac.map(&almanac.temperature_to_humidity_map, actual);
        // let actual = almanac.map(&almanac.humidity_to_location_map, actual);
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Map {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

fn parse(input: &str) -> Almanac {
    let sections = input.split("\n\n").collect::<Vec<&str>>();

    Almanac {
        seeds: parse_seeds(sections.iter().nth(0).unwrap()),
        seed_to_soil_map: parse_map(sections.iter().nth(1).unwrap()),
        soil_to_fertilizer_map: parse_map(sections.iter().nth(2).unwrap()),
        fertilizer_to_water_map: parse_map(sections.iter().nth(3).unwrap()),
        water_to_light_map: parse_map(sections.iter().nth(4).unwrap()),
        light_to_temperature_map: parse_map(sections.iter().nth(5).unwrap()),
        temperature_to_humidity_map: parse_map(sections.iter().nth(6).unwrap()),
        humidity_to_location_map: parse_map(sections.iter().nth(7).unwrap()),
    }
}

fn parse_seeds(input: &str) -> Vec<u64> {
    input.split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .map(|n| n.parse::<u64>().unwrap())
        .collect()
}

fn parse_map(input: &str) -> Vec<Map> {
    input.lines()
        .skip(1)
        .map(|list| {
            let split = list.split(' ')
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            Map {
                destination_range_start: split[0],
                source_range_start: split[1],
                range_length: split[2],
            }
        })
        .collect::<Vec<Map>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split() {
        let input = "50 98 2";
        let expected = vec!["50", "98", "2"];
        assert_eq!(input.split(' ').collect::<Vec<&str>>(), expected);

        let expected = vec![50, 98, 2];
        assert_eq!(input.split(' ').map(|n| n.parse::<u64>().unwrap()).collect::<Vec<u64>>(), expected);
    }

    #[test]
    fn test_parse_seeds() {
        let input = "seeds: 79 14 55 13";
        let output = vec![79, 14, 55, 13];
        assert_eq!(parse_seeds(input), output);
    }

    #[test]
    fn test_parse_map() {
        let input = "seed-to-soil map:
50 98 2
52 50 48";
        let output = vec![
            Map { destination_range_start: 50, source_range_start: 98, range_length: 2 },
            Map { destination_range_start: 52, source_range_start: 50, range_length: 48 },
        ];
        assert_eq!(parse_map(input), output);
    }

    #[test]
    fn test_parse() {
        let input = include_str!("../test1.txt");
        let output = parse(input);
        // let expected = 0;
        // assert_eq!(parse(input), output);
    }

    // Seed 79, soil 81, fertilizer 81, water 81, light 74, temperature 78, humidity 78, location 82.
    #[test]
    fn test_seed_to_location() {
        let input = include_str!("../test1.txt");
        let almanac = parse(input);
        let seed = 79;
        let actual = almanac.map(&almanac.seed_to_soil_map, seed);
        let expected = 81;
        assert_eq!(actual, expected);

        let actual = almanac.map(&almanac.soil_to_fertilizer_map, actual);
        let expected = 81;
        assert_eq!(actual, expected);

        let actual = almanac.map(&almanac.fertilizer_to_water_map, actual);
        let expected = 81;
        assert_eq!(actual, expected);

        let actual = almanac.map(&almanac.water_to_light_map, actual);
        let expected = 74;
        assert_eq!(actual, expected);

        let actual = almanac.map(&almanac.light_to_temperature_map, actual);
        let expected = 78;
        assert_eq!(actual, expected);

        let actual = almanac.map(&almanac.temperature_to_humidity_map, actual);
        let expected = 78;
        assert_eq!(actual, expected);

        let actual = almanac.map(&almanac.humidity_to_location_map, actual);
        let expected = 82;
        assert_eq!(actual, expected);

        let actual = almanac.seed_to_location(seed);
        let expected = 82;
        assert_eq!(actual, expected);
    }

    // Seed 14, soil 14, fertilizer 53, water 49, light 42, temperature 42, humidity 43, location 43.
    // Seed 55, soil 57, fertilizer 57, water 53, light 46, temperature 82, humidity 82, location 86.
    // Seed 13, soil 13, fertilizer 52, water 41, light 34, temperature 34, humidity 35, location 35.
    // So, the lowest location number in this example is 35.
    #[test]
    fn test_part1() {
        let input = include_str!("../test1.txt");
        let solution = 35;
        assert_eq!(solve_part1(input), solution);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../test1.txt");
        let solution = 46;
        assert_eq!(solve_part2(input), solution);
    }

    #[test]
    fn test_solve_part1() {
        let input = include_str!("../input.txt");
        let solution = 107430936;
        assert_eq!(solve_part1(input), solution);
    }

    // #[test]
    fn test_solve_part2() {
        let input = include_str!("../input.txt");
        let solution = 23738616;
        assert_eq!(solve_part2(input), solution);
    }
}
