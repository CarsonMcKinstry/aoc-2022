use std::fs;


#[derive(Debug, Clone)]
#[repr(i32)]
enum Shape {
    // A, X (naively)
    Rock = 1,
    // B, Y (naively)
    Paper = 2,
    // C, Z (naively)
    Scissor = 3,
}

impl Shape {
    pub fn from_str(m: &str) -> Self {
        match m {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissor,
            _ => panic!("Invalid move supplied: {}", m)
        }
    }
}

#[derive(Debug)]
#[repr(i32)]
enum Outcome {
    Lost = 0,
    Draw = 3,
    Win = 6,
}

impl Outcome {
    pub fn from_str(l: &str) -> Self {
        match l {
            "X" => Outcome::Lost,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Invalid outcome given: {}", l)
        }
    }
}

fn calculate_outcome(round: (&Shape, &Shape)) -> Outcome {
    match round {
        (Shape::Rock, Shape::Scissor)
        | (Shape::Scissor, Shape::Paper)
        | (Shape::Paper, Shape::Rock) => Outcome::Lost,
        (Shape::Scissor, Shape::Rock)
        | (Shape::Rock, Shape::Paper)
        | (Shape::Paper, Shape::Scissor) => Outcome::Win,
        _ => Outcome::Draw
    }
}

fn calculate_expected_move(strat: (&Shape, &Outcome)) -> Shape {
    match strat {
        (Shape::Rock, Outcome::Lost) => Shape::Scissor,
        (Shape::Paper, Outcome::Lost) => Shape::Rock,
        (Shape::Scissor, Outcome::Lost) => Shape::Paper,
        (Shape::Rock, Outcome::Win) => Shape::Paper,
        (Shape::Paper, Outcome::Win) => Shape::Scissor,
        (Shape::Scissor, Outcome::Win) => Shape::Rock,
        (opponent, _) => opponent.clone()
    }
}

fn calculate_naive_score(round: &str) -> i32 {
    let [opponent, player]: [Shape; 2] = round
        .split_whitespace()
        .take(2)
        .map(Shape::from_str)
        .collect::<Vec<Shape>>()
        .try_into().unwrap();

    let outcome = calculate_outcome((&opponent, &player));

    player as i32 + outcome as i32
}

fn calculate_proper_score(round: &str) -> i32 {
    let [opponent, outcome]: [&str; 2] = round
        .split_whitespace()
        .take(2)
        .collect::<Vec<&str>>()
        .try_into().unwrap();

    let opponent = Shape::from_str(opponent);
    let outcome = Outcome::from_str(outcome);

    let right = calculate_expected_move((&opponent, &outcome));

    right as i32 + outcome as i32
}

fn main() {
    let stratagem = fs::read_to_string("input.txt").expect("Unable to load input data");

    let naive_score: i32 = stratagem
        .lines()
        .map(calculate_naive_score)
        .sum();

    let proper_score: i32 = stratagem
        .lines()
        .map(calculate_proper_score)
        .sum();

    println!("naive score: {}", naive_score);
    println!("proper score: {}", proper_score);
}

