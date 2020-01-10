fn main() {
    let (memory, output) = intcode(
        vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
        vec![9],
    );
}

const POSITION_MODE: isize = 0;
const IMMEDIATE_MODE: isize = 1;

fn runWithInput(program: &str, input: Vec<isize>) -> (Vec<isize>, Vec<isize>) {
    let mut codes: Vec<isize> = program
        .split(',')
        .map(|o| o.parse::<isize>().unwrap())
        .collect::<Vec<isize>>();

    intcode(codes, input)
}

fn run(program: &str, arg1: isize, arg2: isize) -> (Vec<isize>, Vec<isize>) {
    let mut codes: Vec<isize> = program
        .split(',')
        .map(|o| o.parse::<isize>().unwrap())
        .collect::<Vec<isize>>();

    codes[1] = arg1;
    codes[2] = arg2;

    intcode(codes, vec![])
}

fn parse_opcode(code: isize) -> (isize, isize, isize, isize, isize) {
    let opcode = format!("{:0>5}", code);
    let oc: Vec<&str> = opcode.split("").collect();
    (
        oc[1].parse::<isize>().unwrap(),
        oc[2].parse::<isize>().unwrap(),
        oc[3].parse::<isize>().unwrap(),
        oc[4].parse::<isize>().unwrap(),
        oc[5].parse::<isize>().unwrap(),
    )
}

fn get_code(codes: &Vec<isize>, index: isize, mode: isize) -> isize {
    match mode {
        POSITION_MODE => match codes.get(index as usize) {
            None => 0,
            Some(x) => *x,
        },
        IMMEDIATE_MODE => index,
        _ => panic!("Not a valid parameter mode! Mode given: {}", mode),
    }
}

fn get_addresses(
    mem: &Vec<isize>,
    pc: isize,
    p1: isize,
    p2: isize,
    p3: isize,
) -> (usize, usize, usize) {
    (
        get_code(&mem, pc + 3, p3) as usize,
        get_code(&mem, pc + 1, p1) as usize,
        get_code(&mem, pc + 2, p2) as usize,
    )
}

fn intcode(codes: Vec<isize>, input: Vec<isize>) -> (Vec<isize>, Vec<isize>) {
    let mut mem = codes.clone();

    let mut pc: isize = 0;
    let mut ic: isize = 0;
    let mut output: Vec<isize> = vec![];

    loop {
        let opcode = mem[pc as usize];
        let opcode = parse_opcode(opcode);
        println!("{:?}", opcode);
        match opcode {
            (a, b, c, d, 1) => {
                let (out_addr, rand_1_addr, rand_2_addr) =
                    get_addresses(&mem, pc, c, b, POSITION_MODE);
                mem[out_addr] = mem[rand_1_addr] + mem[rand_2_addr];
                pc += 4;
            }
            (a, b, c, d, 2) => {
                let (out_addr, rand_1_addr, rand_2_addr) =
                    get_addresses(&mem, pc, c, b, POSITION_MODE);
                mem[out_addr] = mem[rand_1_addr] * mem[rand_2_addr];
                pc += 4;
            }
            (a, b, c, d, 3) => {
                let (_, rand_1_addr, _) = get_addresses(&mem, pc, POSITION_MODE, b, a);
                mem[rand_1_addr] = input[ic as usize];
                ic += 1;
                pc += 2;
            }
            (a, b, c, d, 4) => {
                let (_, rand_1_addr, _) = get_addresses(&mem, pc, c, b, a);
                output.push(mem[rand_1_addr]);
                pc += 2;
            }
            (a, b, c, d, 5) => {
                let (out_addr, rand_1_addr, rand_2_addr) = get_addresses(&mem, pc, c, b, a);
                if mem[rand_1_addr] != 0 {
                    pc = mem[rand_2_addr];
                } else {
                    pc += 3;
                }
            }
            (a, b, c, d, 6) => {
                let (out_addr, rand_1_addr, rand_2_addr) = get_addresses(&mem, pc, c, b, a);
                if mem[rand_1_addr] == 0 {
                    pc = mem[rand_2_addr];
                } else {
                    pc += 3;
                }
            }
            (a, b, c, d, 7) => {
                let (out_addr, rand_1_addr, rand_2_addr) = get_addresses(&mem, pc, c, b, a);
                if mem[rand_1_addr] < mem[rand_2_addr] {
                    mem[out_addr] = 1;
                } else {
                    mem[out_addr] = 0;
                }
                pc += 4;
            }
            (a, b, c, d, 8) => {
                let (out_addr, rand_1_addr, rand_2_addr) = get_addresses(&mem, pc, c, b, a);
                if mem[rand_1_addr] == mem[rand_2_addr] {
                    mem[out_addr] = 1;
                } else {
                    mem[out_addr] = 0;
                }
                pc += 4;
            }
            (_, _, _, 9, 9) => {
                return (mem, output);
            }
            _ => panic!("{} is not a valid opcode", mem[pc as usize]),
        }
    }
}

#[cfg(test)]
mod day_2 {
    use super::*;
    use std::fs;

    #[test]
    fn part_1_example_1() {
        let (memory, _) = intcode(vec![1, 0, 0, 3, 99], vec![]);
        assert_eq!(memory, vec![1, 0, 0, 2, 99]);
    }

    #[test]
    fn part_1_example_2() {
        let (memory, _) = intcode(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50], vec![]);
        assert_eq!(memory, vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
    }

    #[test]
    fn part_1_example_3() {
        let (memory, _) = intcode(vec![1, 0, 0, 0, 99], vec![]);
        assert_eq!(memory, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn part_1_example_4() {
        let (memory, _) = intcode(vec![2, 3, 0, 3, 99], vec![]);
        assert_eq!(memory, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn part_1_example_5() {
        let (memory, _) = intcode(vec![2, 4, 4, 5, 99, 0], vec![]);
        assert_eq!(memory, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn part_1_example_6() {
        let (memory, _) = intcode(vec![1, 1, 1, 4, 99, 5, 6, 0, 99], vec![]);
        assert_eq!(memory, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn part_1_puzzle() {
        let input = fs::read_to_string("./inputs/day2.ic").unwrap();
        let (memory, _) = run(&input, 12, 2);
        assert_eq!(memory[0], 4090689);
    }

    #[test]
    fn part_2_puzzle() {
        let input = fs::read_to_string("./inputs/day2.ic").unwrap();
        let mut noun = 0;
        let mut verb = 0;
        let mut found = false;
        while !found {
            for v in 0..99 {
                verb = v;
                let (memory, _) = run(&input, noun, verb);
                if memory[0] == 19690720 {
                    found = true;
                    break;
                }
            }

            if !found {
                noun += 1;
            }
        }
        assert_eq!((100 * noun) + verb, 7733);
    }
}

#[cfg(test)]
mod day_5 {
    use super::*;
    use std::fs;

    #[test]
    fn part_1_example_1() {
        let (memory, output) = intcode(vec![3, 0, 4, 0, 99], vec![1]);
        println!("{:?}", memory);
        println!("{:?}", output);
        assert_eq!(output[0], 1);
    }

    #[test]
    fn part_1_example_2() {
        let (memory, output) = intcode(vec![1002, 4, 3, 4, 33], vec![1]);
        println!("{:?}", memory);
        println!("{:?}", output);
        assert_eq!(memory, vec![1002, 4, 3, 4, 99]);
    }

    #[test]
    fn part_1_example_3() {
        let (memory, output) = intcode(vec![1101, 100, -1, 4, 0], vec![1]);
        println!("{:?}", memory);
        println!("{:?}", output);
        assert_eq!(memory, vec![1101, 100, -1, 4, 99]);
    }

    #[test]
    fn part_1_puzzle() {
        let input = fs::read_to_string("./inputs/day5.ic").unwrap();
        let (memory, output) = runWithInput(&input, vec![1]);
        println!("{:?}", memory);
        println!("{:?}", output);
        assert_eq!(output.last().unwrap(), &14155342);
    }

    #[test]
    fn part_2_example_1_match() {
        let (memory, output) = intcode(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], vec![8]);
        assert_eq!(output[0], 1);
    }

    #[test]
    fn part_2_example_1_no_match() {
        let (memory, output) = intcode(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], vec![7]);
        assert_eq!(output[0], 0);
    }

    #[test]
    fn part_2_example_2_lt() {
        let (memory, output) = intcode(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], vec![7]);
        assert_eq!(output[0], 1);
    }

    #[test]
    fn part_2_example_2_not_lt() {
        let (memory, output) = intcode(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], vec![8]);
        assert_eq!(output[0], 0);
    }

    #[test]
    fn part_2_example_3_match() {
        let (memory, output) = intcode(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], vec![8]);
        assert_eq!(output[0], 1);
    }

    #[test]
    fn part_2_example_3_no_match() {
        let (memory, output) = intcode(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], vec![7]);
        assert_eq!(output[0], 0);
    }

    #[test]
    fn part_2_example_4_lt() {
        let (memory, output) = intcode(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], vec![4]);
        assert_eq!(output[0], 1);
    }

    #[test]
    fn part_2_example_4_not_lt() {
        let (memory, output) = intcode(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], vec![9]);
        assert_eq!(output[0], 0);
    }

    #[test]
    fn part_2_example_5_0() {
        let (memory, output) = intcode(
            vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            vec![0],
        );
        assert_eq!(output[0], 0);
    }

    #[test]
    fn part_2_example_5_not_0() {
        let (memory, output) = intcode(
            vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            vec![9],
        );
        assert_eq!(output[0], 1);
    }

    #[test]
    fn part_2_example_6_0() {
        let (memory, output) = intcode(
            vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
            vec![0],
        );
        assert_eq!(output[0], 0);
    }

    #[test]
    fn part_2_example_6_not_0() {
        let (memory, output) = intcode(
            vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
            vec![9],
        );
        assert_eq!(output[0], 1);
    }

    #[test]
    fn part_2_example_7_lt_8() {
        let (memory, output) = intcode(
            vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            vec![5],
        );
        assert_eq!(output[0], 999);
    }

    #[test]
    fn part_2_example_7_eq_8() {
        let (memory, output) = intcode(
            vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            vec![8],
        );
        assert_eq!(output[0], 1000);
    }

    #[test]
    fn part_2_example_7_gt_8() {
        let (memory, output) = intcode(
            vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            vec![80],
        );
        assert_eq!(output[0], 1001);
    }

    #[test]
    fn part_2_puzzle() {
        let input = fs::read_to_string("./inputs/day5.ic").unwrap();
        let (memory, output) = runWithInput(&input, vec![5]);
        println!("{:?}", memory);
        println!("{:?}", output);
        assert_eq!(output.last().unwrap(), &8684145);
    }
}
