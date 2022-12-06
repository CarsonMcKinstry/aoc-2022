use std::{collections::HashSet, fs};

pub(crate) fn end_index_of_first_marker(s: &str, n: usize) -> Option<usize> {
    s.chars()
        .collect::<Vec<char>>()
        .windows(n)
        .enumerate()
        .find(|(_, chars)| HashSet::<_>::from_iter(chars.into_iter()).len() == n)
        .map(|(i, _)| i + n)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read input");
    let input = input.as_str();

    let index_of_first_packet_marker = end_index_of_first_marker(input, 4).unwrap();
    let index_of_first_message_marker = end_index_of_first_marker(input, 14).unwrap();

    println!("first packet marker: {}", index_of_first_packet_marker);
    println!("first message marker: {}", index_of_first_message_marker);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_find_the_first_marker_index() {
        assert_eq!(
            end_index_of_first_marker("bvwbjplbgvbhsrlpgdmjqwftvncz, n: usize", 4).unwrap(),
            5
        );
    }
}
