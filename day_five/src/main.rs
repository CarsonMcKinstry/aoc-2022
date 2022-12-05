mod stack;

use std::fs;

use stack::Stack;

// Initial State
// it'll be faster to just bring this in manually
//             [J] [Z] [G]
//             [Z] [T] [S] [P] [R]
// [R]         [Q] [V] [B] [G] [J]
// [W] [W]     [N] [L] [V] [W] [C]
// [F] [Q]     [T] [G] [C] [T] [T] [W]
// [H] [D] [W] [W] [H] [T] [R] [M] [B]
// [T] [G] [T] [R] [B] [P] [B] [G] [G]
// [S] [S] [B] [D] [F] [L] [Z] [N] [L]
//  1   2   3   4   5   6   7   8   9

struct Instruction {
    pub(crate) m: usize,
    pub(crate) src: usize,
    pub(crate) dest: usize,
}

impl Instruction {
    pub(crate) fn from_str(input: &str) -> Self {
        let [m, src, dest]: [usize; 3] = input
            .split_whitespace()
            .filter_map(|inst| inst.parse::<usize>().ok())
            .take(3)
            .into_iter()
            .collect::<Vec<usize>>()
            .try_into()
            .unwrap();

        Self {
            m,
            src: src - 1,
            dest: dest - 1,
        }
    }
}

fn main() {
    let state: [Stack; 9] = [
        Stack::from_str("STHFWR"),
        Stack::from_str("SGDQW"),
        Stack::from_str("BTW"),
        Stack::from_str("DRWTNQZJ"),
        Stack::from_str("FBHGLVTZ"),
        Stack::from_str("LPTCVBSG"),
        Stack::from_str("ZBRTWGP"),
        Stack::from_str("NGMTCJR"),
        Stack::from_str("LGBW"),
    ];

    let instructions = fs::read_to_string("input.txt").expect("Unable to read input");

    let instructions = instructions.lines().map(Instruction::from_str).collect::<Vec<Instruction>>();

    let mut part_one_state = state.clone();

    for instruction in &instructions {
        let Instruction { m, src, dest } = instruction;

        for _ in 0..*m {
            let value = part_one_state.get_mut(*src).unwrap().pop().unwrap();

            part_one_state.get_mut(*dest).unwrap().push(value)
        }
    }

    let tops = part_one_state
        .into_iter()
        .map(|stack| stack.peek().unwrap().to_owned())
        .collect::<String>();

    println!("{:?}", tops);

    let mut part_two_state = state.clone();

    for instruction in &instructions {
        let Instruction { m, src, dest } = instruction;

        let mut temp =  Stack::new();

        for _ in 0..*m {
            let value = part_two_state.get_mut(*src).unwrap().pop().unwrap();

            temp.push(value);
        }

        while let Some(c) = temp.pop() {
            part_two_state.get_mut(*dest).unwrap().push(c);
        }
    }

    let tops = part_two_state
        .into_iter()
        .map(|stack| stack.peek().unwrap().to_owned())
        .collect::<String>();

    println!("{:?}", tops);
}
