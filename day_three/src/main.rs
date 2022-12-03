use std::{collections::HashSet, fs, ops::Deref};

pub(crate) fn letter_to_number(c: &char) -> u8 {
    let letter_as_byte: u8 = c.to_string().as_bytes().iter().next().unwrap().to_owned();

    let factor: u8 = if letter_as_byte >= 91 {
        b'a'
    } else {
        b'A' - 26
    };

    let num = letter_as_byte - factor;

    num + 1
}

fn chunk_lines(string: &str, n: usize) -> Vec<Vec<&str>> {
    let mut out: Vec<Vec<&str>> = vec![];

    let mut temp: Vec<&str> = vec![];

    for line in string.lines() {
        temp.push(line);
        if temp.len() == 3 {
            out.push(temp);
            temp = vec![];
        }
    }

    out
}

fn find_mistake_in_rucksack(rucksack: &str) -> char {
    let (left, right) = rucksack.split_at(rucksack.len() / 2);

    let left_chars = left.chars().collect::<HashSet<char>>();
    let right_chars = right.chars().collect::<HashSet<char>>();

    let intersection: HashSet<&char> = left_chars.intersection(&right_chars).collect();

    let c = intersection.into_iter().next().unwrap();

    *c
}

fn part_one(rucksacks: &str) {
    let mut total: i32 = 0;

    for line in rucksacks.lines() {
        let mistake = find_mistake_in_rucksack(line);

        let mistake_as_number = letter_to_number(&mistake);

        total += mistake_as_number as i32;
    }

    println!("{total}");
}

fn part_two(rucksacks: &str) {
    let groups = chunk_lines(rucksacks, 3);

    let mut total: i32 = 0;

    let mut temp: HashSet<char>;

    for group in groups {
        let mut iter = group.iter();

        temp = iter
            .next()
            .map(|g| g.chars().collect::<HashSet<char>>())
            .unwrap();

        while let Some(rucksack) = iter.next() {
            let h = rucksack.chars().collect::<HashSet<char>>();

            temp = h
                .intersection(&temp)
                .into_iter()
                .map(|c| *c)
                .collect::<HashSet<char>>();
        }
        let c = temp.into_iter().next().unwrap();

        total += letter_to_number(&c) as i32;
    }

    println!("{total}");
}

fn main() {
    let rucksacks = fs::read_to_string("input.txt").expect("unable to read input");

    part_one(rucksacks.as_str());
    part_two(rucksacks.as_str());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn letters_to_number_should_return_value_in_range_1_to_52() {
        let all_letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

        let chars = all_letters.chars();

        let out = chars.map(|c| letter_to_number(&c)).collect::<Vec<u8>>();

        let expected = (1..53).collect::<Vec<u8>>();

        assert_eq!(out, expected);
    }

    #[test]
    fn chunk_lines_into_threes() {
        let string = "a\nb\nc\na\nb\nc\na\nb\nc";

        let out = chunk_lines(string, 3);

        assert_eq!(
            out,
            vec!(
                vec!("a", "b", "c"),
                vec!("a", "b", "c"),
                vec!("a", "b", "c")
            )
        )
    }
}
