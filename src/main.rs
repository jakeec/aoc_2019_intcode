fn main() {
    intcode(vec![1, 0, 0, 3, 99]);
}

fn get_code(codes: &Vec<usize>, index: usize) -> usize {
    match codes.get(index) {
        None => 0,
        Some(x) => *x,
    }
}

fn run(input: &str, arg1: usize, arg2: usize) -> Vec<usize> {
    let mut codes: Vec<usize> = input
        .split(',')
        .map(|o| o.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    codes[1] = arg1;
    codes[2] = arg2;

    intcode(codes)
}

fn intcode(codes: Vec<usize>) -> Vec<usize> {
    let mut mem = codes.clone();

    let mut pc: usize = 0;

    loop {
        let out_addr = get_code(&mem, pc + 3);
        let rand_1_addr = get_code(&mem, pc + 1);
        let rand_2_addr = get_code(&mem, pc + 2);
        match mem[pc] {
            1 => {
                mem[out_addr] = mem[rand_1_addr] + mem[rand_2_addr];
                pc += 4;
            }
            2 => {
                mem[out_addr] = mem[rand_1_addr] * mem[rand_2_addr];
                pc += 4;
            }
            99 => {
                return mem;
            }
            _ => panic!("{} is not a valid opcode", mem[pc]),
        }
    }
}

#[cfg(test)]
mod day_1 {
    use super::*;
    use std::fs;

    #[test]
    fn part_1_example_1() {
        let result = intcode(vec![1, 0, 0, 3, 99]);
        assert_eq!(result, vec![1, 0, 0, 2, 99]);
    }

    #[test]
    fn part_1_example_2() {
        let result = intcode(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
        assert_eq!(result, vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
    }

    #[test]
    fn part_1_example_3() {
        let result = intcode(vec![1, 0, 0, 0, 99]);
        assert_eq!(result, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn part_1_example_4() {
        let result = intcode(vec![2, 3, 0, 3, 99]);
        assert_eq!(result, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn part_1_example_5() {
        let result = intcode(vec![2, 4, 4, 5, 99, 0]);
        assert_eq!(result, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn part_1_example_6() {
        let result = intcode(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        assert_eq!(result, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn part_1_puzzle() {
        let input = fs::read_to_string("./inputs/day1.ic").unwrap();
        let result = run(&input, 12, 2);
        assert_eq!(result[0], 4090689);
    }

    #[test]
    fn part_2_puzzle() {
        let input = fs::read_to_string("./inputs/day1.ic").unwrap();
        let mut noun = 0;
        let mut verb = 0;
        let mut found = false;
        while !found {
            for v in 0..99 {
                verb = v;
                let result = run(&input, noun, verb);
                if result[0] == 19690720 {
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
