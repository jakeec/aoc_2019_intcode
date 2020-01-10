fn main() {
    run("1,0,0,0,99", 1, 2);
    intcode(vec![1, 0, 0, 3, 99], vec![]);
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
}
