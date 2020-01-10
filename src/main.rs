fn main() {
    intcode("1,0,0,3,99");
}

fn get_code(codes: &Vec<usize>, index: usize) -> usize {
    match codes.get(index) {
        None => 0,
        Some(x) => *x,
    }
}

fn intcode(input: &str) -> Vec<usize> {
    let mut codes: Vec<usize> = input
        .split(',')
        .map(|o| o.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut pc: usize = 0;

    loop {
        let out_addr = get_code(&codes, pc + 3);
        let rand_1_addr = get_code(&codes, pc + 1);
        let rand_2_addr = get_code(&codes, pc + 2);
        match codes[pc] {
            1 => {
                codes[out_addr] = codes[rand_1_addr] + codes[rand_2_addr];
                pc += 4;
            }
            2 => {
                codes[out_addr] = codes[rand_1_addr] * codes[rand_2_addr];
                pc += 4;
            }
            99 => {
                return codes;
            }
            _ => panic!("{} is not a valid opcode", codes[pc]),
        }
    }
}

#[cfg(test)]
mod day_1 {
    use super::*;
    use std::fs;

    #[test]
    fn part_1_example_1() {
        let result = intcode("1,0,0,3,99");
        assert_eq!(result, vec![1, 0, 0, 2, 99]);
    }

    #[test]
    fn part_1_example_2() {
        let result = intcode("1,9,10,3,2,3,11,0,99,30,40,50");
        assert_eq!(result, vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
    }

    #[test]
    fn part_1_example_3() {
        let result = intcode("1,0,0,0,99");
        assert_eq!(result, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn part_1_example_4() {
        let result = intcode("2,3,0,3,99");
        assert_eq!(result, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn part_1_example_5() {
        let result = intcode("2,4,4,5,99,0");
        assert_eq!(result, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn part_1_example_6() {
        let result = intcode("1,1,1,4,99,5,6,0,99");
        assert_eq!(result, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn part_1_puzzle() {
        let mut input = fs::read_to_string("./inputs/day1.ic").unwrap();
        println!("{}", &input[2..5]);
        input.replace_range(2..5, "12,02");
        let result = intcode(&input);
        assert_eq!(result[0], 4090689);
    }
}
