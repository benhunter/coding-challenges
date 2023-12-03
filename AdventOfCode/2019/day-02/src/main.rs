fn main() {
    let input = include_str!("../input.txt");
    let result = solve_part1(input, Some(Inputs { noun: 12, verb: 2 }));
    println!("✅ part1: {}", result);

    let result = solve_part2(input);
    println!("✅ part2: {}", result);
}

#[derive(Debug, PartialEq, Clone, Default)]
struct Computer {
    memory: Vec<usize>,
    ip: usize,
    halted: bool,
}

impl Computer {}

impl Computer {
    fn modify_input(&mut self, inputs: Inputs) -> &mut Computer {
        self.memory[1] = inputs.noun;
        self.memory[2] = inputs.verb;
        self
    }
    fn step(&mut self) -> &mut Computer {
        let next = self.next_instruction();
        match next {
            Instruction::Add => self.add(),
            Instruction::Multiply => self.multiply(),
            Instruction::Halt => self.halt(),
        }
        self
    }

    pub(crate) fn next_instruction(&self) -> Instruction {
        match self.memory[self.ip] {
            1 => Instruction::Add,
            2 => Instruction::Multiply,
            99 => Instruction::Halt,
            _ => panic!("🤬 ip is not an opcode")
        }
    }

    fn add(&mut self) {
        let value1 = self.memory[self.memory[self.ip + 1]];
        let value2 = self.memory[self.memory[self.ip + 2]];
        let target = self.memory[self.ip + 3];
        self.memory[target] = value1 + value2;
        self.ip += Instruction::Add.length();
    }

    fn multiply(&mut self) {
        let value1 = self.memory[self.memory[self.ip + 1]];
        let value2 = self.memory[self.memory[self.ip + 2]];
        let target = self.memory[self.ip + 3];
        self.memory[target] = value1 * value2;
        self.ip += Instruction::Multiply.length();
    }

    pub(crate) fn is_halted(&self) -> bool {
        self.halted
    }

    fn halt(&mut self) {
        self.halted = true;
        self.ip += Instruction::Halt.length();
    }

    fn value_zero(&self) -> usize {
        self.memory[0]
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Add = 1,
    Multiply = 2,
    Halt = 99,
}

impl Instruction {
    pub(crate) fn length(&self) -> usize {
        match self {
            Instruction::Add => 4,
            Instruction::Multiply => 4,
            Instruction::Halt => 1,
        }
    }
}

struct Inputs {
    noun: usize,
    verb: usize,
}

fn parse(input: &str) -> Computer {
    let memory = input
        .split(',')
        .map(|x|
            x.parse::<usize>()
                .unwrap_or_else(|_| panic!("out of range"))
        )
        .inspect(|x| {
            if x < &0 { panic!("out of range") }
        })
        .collect();
    Computer { memory, ..Default::default() }
}

fn solve_part1(input: &str, modify: Option<Inputs>) -> usize {
    let mut computer = parse(input);
    if modify.is_some() {
        let inputs = modify.unwrap();
        computer.modify_input(inputs);
    }
    while !computer.is_halted() {
        computer.step();
    }
    computer.value_zero()
}

fn solve_part2(input: &str) -> usize {
    for noun in 0..99 {
        for verb in 0..99 {
            let part1 = solve_part1(input, Some(Inputs { noun, verb }));
            if part1 == 19690720 {
                return 100 * noun + verb
            }
        }
    }
    panic!("no solution")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "1,9,10,3,2,3,11,0,99,30,40,50";
        let output = Computer { memory: vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50], ..Default::default() };
        assert_eq!(parse(input), output);
    }

    #[test]
    #[should_panic(expected = "out of range")]
    fn test_parse_panic_below_0() {
        let input = "-1,9,10,3,2,3,11,0,99,30,40,50";
        parse(input);
    }

    #[test]
    fn test_mut_memory() {
        let mut before = Computer { memory: vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50], ..Default::default() };
        before.memory[0] = 2;
        let after = Computer { memory: vec![2, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50], ..Default::default() };
        assert_eq!(before, after);
    }

    #[test]
    fn test_read_next_instruction() {
        let input = "1,9,10,3,2,3,11,0,99,30,40,50";
        let mut computer = parse(input);
        let expected = Instruction::Add;
        let actual = computer.next_instruction();
        assert_eq!(actual, expected);

        let expected = Instruction::Multiply;
        computer.step();
        let actual = computer.next_instruction();
        assert_eq!(actual, expected);

        let expected = Instruction::Halt;
        computer.step();
        let actual = computer.next_instruction();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_step_add() {
        let input = "1,9,10,3,2,3,11,0,99,30,40,50";
        let mut computer = parse(input);
        let expected = Computer {
            memory: vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
            ip: 4,
            halted: false,
        };
        let actual = computer.step();
        assert_eq!(*actual, expected);
    }

    #[test]
    fn test_step_multiply() {
        let mut computer = Computer {
            memory: vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
            ip: 4,
            halted: false,
        };
        let expected = Computer {
            memory: vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
            ip: 8,
            halted: false,
        };
        let actual = computer.step();
        assert_eq!(*actual, expected);
    }

    #[test]
    fn test_step_halt() {
        let mut computer = Computer {
            memory: vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
            ip: 8,
            halted: false,
        };
        let expected = Computer {
            memory: vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
            ip: 9,
            halted: true,
        };
        let actual = computer.step();
        assert_eq!(*actual, expected);
        assert!(computer.is_halted())
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../test1.txt");
        let expected_final_state = "3500,9,10,70,2,3,11,0,99,30,40,50";
        let expected = 3500;
        let actual = solve_part1(input, None);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_2() {
        // 1,0,0,0,99 becomes 2,0,0,0,99 (1 + 1 = 2).
        let input = "1,0,0,0,99";
        let final_state = "2,0,0,0,99";
        let expected = 2;
        assert_eq!(solve_part1(input, None), expected);
    }

    #[test]
    fn test_part1_3() {
        // 2,3,0,3,99 becomes 2,3,0,6,99 (3 * 2 = 6).
        let input = "2,3,0,3,99";
        let final_state = "2,3,0,6,99";
        let expected = 2;
        assert_eq!(solve_part1(input, None), expected);
    }

    #[test]
    fn test_part1_4() {
        // 2,4,4,5,99,0 becomes 2,4,4,5,99,9801 (99 * 99 = 9801).
        let input = "2,4,4,5,99,0";
        let final_state = "2,4,4,5,99,9801";
        let expected = 2;
        assert_eq!(solve_part1(input, None), expected);
    }

    #[test]
    fn test_part1_5() {
        // 1,1,1,4,99,5,6,0,99 becomes 30,1,1,4,2,5,6,0,99.
        let input = "1,1,1,4,99,5,6,0,99";
        let final_state = "30,1,1,4,2,5,6,0,99";
        let expected = 30;
        assert_eq!(solve_part1(input, None), expected);
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
        let solution = 4023471;
        assert_eq!(solve_part1(input, Some(Inputs { noun: 12, verb: 2 })), solution);
    }

    #[test]
    fn test_solve_part2() {
        let input = include_str!("../input.txt");
        let solution = 8051;
        assert_eq!(solve_part2(input), solution);
    }
}
