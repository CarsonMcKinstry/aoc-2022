use std::{fs};
use std::iter::Take;
use std::slice::Iter;

fn get_sorted_total_calories_by_elf (input: String) -> Vec<i32> {

    let mut temp: i32 = 0;

    let mut calories_by_elf: Vec<i32> = vec!();

    for line in input.lines() {
        let calories = line.parse::<i32>();

        match calories {
            Ok(calories) => {
                temp += calories;
            },
            Err(_) => {
                calories_by_elf.push(temp);
                temp = 0;
            }
        }
    }

    calories_by_elf.push(temp);

    calories_by_elf.sort_by(|a, b| b.cmp(a));

    calories_by_elf
}

fn get_first_n_elves (calories_by_elf: &Vec<i32>, n: usize) -> Take<Iter<'_, i32>> {
    calories_by_elf
        .iter()
        .take(n)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to open input");

    let mut calories_by_elf = get_sorted_total_calories_by_elf(input);

    let top_elf = get_first_n_elves(&calories_by_elf, 1);
    let top_three_elves = get_first_n_elves(&calories_by_elf, 3);

    // Part One - Total calories from the elf with the most calories
    let max = top_elf.sum::<i32>();

    // Part Two - Total calories from the top 3 elves with the most calories
    let max_of_three = top_three_elves.sum::<i32>();

    println!("Part One - Total calories from the elf with the most calories: {}", max);
    println!("Part Two - Total calories from the top 3 elves: {}", max_of_three);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn sorted_calories_by_elf () {
        let output = get_sorted_total_calories_by_elf(INPUT.to_string());

        assert_eq!(output, vec!(24000, 11000, 10000, 6000, 4000))
    }

    #[test]
    fn top_elf () {
        let output = get_sorted_total_calories_by_elf(INPUT.to_string());

        assert_eq!(
            get_first_n_elves(&output, 1).sum::<i32>(),
            24000
        )
    }

    #[test]
    fn top_three_elves() {
        let output = get_sorted_total_calories_by_elf(INPUT.to_string());

        assert_eq!(
            get_first_n_elves(&output, 3).sum::<i32>(),
            45000
        )
    }

}