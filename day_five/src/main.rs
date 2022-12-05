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
        let mut inst = input
            .split_whitespace()
            .filter_map(|inst| inst.parse::<usize>().ok());

        let m = inst.next().unwrap();
        let src = inst.next().map(|inst| inst - 1).unwrap();
        let dest = inst.next().map(|inst| inst - 1).unwrap();

        Self { m, src, dest }
    }
}

fn filter_alphabetics(input: &[char]) -> Vec<&char> {
    input
        .into_iter()
        .filter(|c| c.is_alphabetic())
        .collect()
}

fn parse_input(input: &str) -> (&str, Vec<Stack>) {
    let input = input.split("\n\n").collect::<Vec<&str>>();

    let raw_stacks = input.iter().next().unwrap();
    let rest = input.iter().last().unwrap();

    let mut lines = raw_stacks.lines().rev();

    let num_stacks = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .map(|n| n.parse::<usize>().unwrap())
        .unwrap();

    let mut stacks = vec![Stack::new(); num_stacks];

    for line in lines {
        let chars = line.chars().collect::<Vec<char>>();
        let chunks = chars
            .chunks(4)
            .map(filter_alphabetics).collect::<Vec<Vec<&char>>>();

        for (stack, chunk) in chunks.iter().enumerate() {
            
            match chunk.iter().next() {
                Some(c) => {
                    stacks.get_mut(stack).unwrap().push(**c)
                },
                None => {}
            }
        }
    }

    (rest, stacks)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read input");

    let (instructions, state) = parse_input(input.as_str());

    let instructions = instructions
        .lines()
        .map(Instruction::from_str)
        .collect::<Vec<Instruction>>();

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

        let mut temp = Stack::new();

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

#[cfg(test)]
mod tests {

    use super::*;

    const SAMPLE_INPUT: &str = "[D]        
[N] [C]    
[Z] [M] [P]
 1   2   3

move 1 from 3 to 2";



    #[test]
    fn do_a_thing() {
        let (_, out) = parse_input(SAMPLE_INPUT);

        let expected = vec!(
            Stack::from_str("ZND"),
            Stack::from_str("MC"),
            Stack::from_str("P")
        );

        assert_eq!(out, expected);

        let first = out.first().unwrap();

        assert_eq!(first.peek(), Some(&'D'));
    }
}
