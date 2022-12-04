use std::{collections::HashSet, fs};

type Assignments = [HashSet<i32>; 2];

pub(crate) fn string_to_range_vector(s: &str) -> HashSet<i32> {
    let [start, end]: [i32; 2] = s
        .split("-")
        .into_iter()
        .take(2)
        .filter_map(|n| n.parse::<i32>().ok())
        .collect::<Vec<i32>>()
        .try_into()
        .unwrap();

    (start..=end).collect::<HashSet<i32>>()
}

pub(crate) fn group_into_assignments(s: &str) -> Assignments {
    s.split(',')
        .into_iter()
        .take(2)
        .map(string_to_range_vector)
        .collect::<Vec<HashSet<i32>>>()
        .try_into()
        .unwrap()
}

pub(crate) fn find_complete_overlaps(assignments: &Vec<Assignments>) -> i32 {
    assignments
        .iter()
        .filter(|assignment| {
            let [left, right] = assignment;

            left.is_subset(&right) || right.is_subset(&left)
        })
        .count() as i32
}

pub(crate) fn find_any_overlaps(assignments: &Vec<Assignments>) -> i32 {
    assignments
        .iter()
        .filter(|assignment| {
            let [left, right] = assignment;

            let intersection = left.intersection(right);

            intersection.into_iter().count() > 0
        })
        .count() as i32
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("Unable to read input");

    let assignments = file
        .lines()
        .map(group_into_assignments)
        .collect::<Vec<Assignments>>();

    let overlaps = find_complete_overlaps(&assignments);
    let any_overlaps = find_any_overlaps(&assignments);

    println!("{overlaps}");
    println!("{any_overlaps}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_from_string() {
        let out = string_to_range_vector("1-5");

        assert_eq!(out, HashSet::from([1, 2, 3, 4, 5]));
    }
}
