const PUZZLE_INPUT: &str = include_str!("../input.txt");

const NUM_STACKS: i32 = 9;

fn main() {
    let (stack_input, instrs) = PUZZLE_INPUT
        .split_once("\n\n")
        .expect("Invalid input format");

    let mut stacks: [Vec<char>; NUM_STACKS as usize] = Default::default();

    stack_input.split('\n').rev().skip(1).for_each(|l| {
        l.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, c)| c != &' ')
            .for_each(|(idx, c)| stacks[idx].push(c));
    });

    instrs.split('\n').for_each(|l| {
        let mut n: i32 = 0;
        let mut f: i32 = 0;
        let mut t: i32 = 0;

        l.split_whitespace()
            .skip(1)
            .step_by(2)
            .enumerate()
            .for_each(|(idx, num)| match idx {
                0 => n = num.parse::<i32>().unwrap(),
                1 => f = num.parse::<i32>().unwrap(),
                _ => t = num.parse::<i32>().unwrap(),
            });

        for _ in 0..n {
            let tmp = stacks[f as usize - 1].pop().unwrap();
            stacks[t as usize - 1].push(tmp);
        }
    });

    stacks.iter().for_each(|s| print!("{}", s.last().unwrap()))
}
