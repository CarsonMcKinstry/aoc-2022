use std::{fs, time::Instant};

pub(crate) fn index_to_coords(i: usize, width: usize) -> (usize, usize) {
    let x = i % (width);
    let y = i / (width);

    (x, y)
}

pub(crate) fn coords_to_index(coords: (usize, usize), width: usize) -> usize {
    let (x, y) = coords;

    x + ((width) * y)
}

pub(crate) fn find_neighbors(
    coords: (usize, usize),
    width: usize,
    height: usize,
) -> [Vec<usize>; 4] {
    let (x, y) = coords;

    let top: Vec<usize> = (0..y).map(|n| coords_to_index((x, n), width)).collect();
    let right: Vec<usize> = (x + 1..width)
        .map(|n| coords_to_index((n, y), width))
        .collect();
    let bottom: Vec<usize> = (y + 1..height)
        .map(|n| coords_to_index((x, n), width))
        .collect();
    let left: Vec<usize> = (0..x).map(|n| coords_to_index((n, y), width)).collect();

    [top, right, bottom, left]
}

pub(crate) fn count_visible_trees(forest: &Vec<usize>, width: usize, height: usize) -> Vec<usize> {

    forest
        .iter()
        .enumerate()
        .filter_map(|(i, tree)| {
            let (x, y) = index_to_coords(i, width);

            // Edges are always eligible
            // top | left | bottom | right
            if x == 0 || y == 0 || y == (height - 1) || x == (width - 1) {
                return Some(*tree);
            }

            let neighbors = find_neighbors((x, y), width, height);

            let is_visible = neighbors
                .iter()
                .any(|line| line.iter().all(|j| forest[*j] < *tree));

            if is_visible {
                return Some(*tree);
            }

            None
        })
        .collect::<Vec<usize>>()
}

enum Direction {
    POS,
    NEG,
}

pub(crate) fn get_view(tree: usize, line: Vec<usize>, direction: Direction) -> usize {
    let line: Vec<&usize> = match direction {
        Direction::POS => line.iter().collect(),
        Direction::NEG => line.iter().rev().collect(),
    };

    let mut count: usize = 0;

    for neighbor in line.iter() {
        count += 1;
        if *neighbor >= &tree {
            break;
        }
    }

    count
}

pub(crate) fn get_scenic_score(tree: usize, neighbors: [Vec<usize>; 4]) -> usize {
    let [top, right, bottom, left] = neighbors;

    let top_view = get_view(tree, top, Direction::NEG);
    let right_view = get_view(tree, right, Direction::POS);
    let bottom_view = get_view(tree, bottom, Direction::POS);
    let left_view = get_view(tree, left, Direction::NEG);

    top_view * right_view * bottom_view * left_view
}

pub(crate) fn find_best_scenic_score(forest: &Vec<usize>, width: usize, height: usize) -> usize {

    let scenic_scores = forest.iter().enumerate().map(|(i, tree)| {
        let (x, y) = index_to_coords(i, width);

        // Edges are always 0
        // top | left | bottom | right
        if x == 0 || y == 0 || y == (height - 1) || x == (width - 1) {
            return 0;
        }

        let [top, right, bottom, left] = find_neighbors((x, y), width, height);

        let neighbors: [Vec<usize>; 4] = [
            top.iter().map(|j| forest[*j]).collect::<Vec<usize>>(),
            right.iter().map(|j| forest[*j]).collect::<Vec<usize>>(),
            bottom.iter().map(|j| forest[*j]).collect::<Vec<usize>>(),
            left.iter().map(|j| forest[*j]).collect::<Vec<usize>>(),
        ];

        get_scenic_score(*tree, neighbors)
    });

    scenic_scores.max().unwrap()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read input");

    let forest = input
        .lines()
        .map(|l| l.trim())
        .into_iter()
        .collect::<String>()
        .chars()
        .map(|c| c.to_string().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let start = Instant::now();
    let visible_trees = count_visible_trees(&forest, width, height);
    let duration = start.elapsed();
    println!("Part 1 took: {:?}", duration);
    println!("{}", visible_trees.len());

    let start = Instant::now();
    let best_scenic_score = find_best_scenic_score(&forest, width, height);
    let duration = start.elapsed();
    println!("Part 2 took: {:?}", duration);
    println!("{}", best_scenic_score);
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coords_to_index() {
        let width = 5;
        // top left
        assert_eq!(coords_to_index((0, 0), width), 0);
        // top right
        assert_eq!(coords_to_index((4, 0), width), 4);
        // botom left
        assert_eq!(coords_to_index((0, 4), width), 20);
        // bottom right
        assert_eq!(coords_to_index((4, 4), width), 24);
        // somewhere in the middle
        assert_eq!(coords_to_index((1, 1), width), 6)
    }

    #[test]
    fn test_index_to_coords() {
        let width = 5;
        // top left
        assert_eq!(index_to_coords(0, width), (0, 0));
        // top right
        assert_eq!(index_to_coords(4, width), (4, 0));
        // botom left
        assert_eq!(index_to_coords(20, width), (0, 4));
        // bottom right
        assert_eq!(index_to_coords(24, width), (4, 4));
        // somewhere in the middle
        assert_eq!(index_to_coords(6, width), (1, 1))
    }

    #[test]
    fn neighbors_from_coords() {
        let coords: (usize, usize) = (1, 1);
        let width = 5;
        let height = 5;

        let output = find_neighbors(coords, width, height);

        assert_eq!(output, [vec![1], vec![7, 8, 9], vec![11, 16, 21], vec![5]])
    }

    const FOREST: [usize; 25] = [3,0,3,7,3,2,5,5,1,2,6,5,3,3,2,3,3,5,4,9,3,5,3,9,0];

    #[test]
    fn small_forest() {
        let forest: Vec<usize> = FOREST.iter().map(|u| *u).collect();
        let visible_trees = count_visible_trees(&forest, 5, 5);

        assert_eq!(visible_trees.len(), 21)
    }

    #[test]
    fn small_forest_best_scenic_score() {
        let forest: Vec<usize> = FOREST.iter().map(|u| *u).collect();

        assert_eq!(find_best_scenic_score(&forest, 5, 5), 8);
    }

    #[test]
    fn find_scenic_score() {
        let tree = 5;
        let neighbors = [vec![3], vec![1, 2], vec![3, 5, 3], vec![2, 5]];

        assert_eq!(get_scenic_score(tree, neighbors), 4);

        let tree = 5;
        let neighbors = [vec![3, 5, 3], vec![4, 9], vec![3], vec![3, 3]];
        assert_eq!(get_scenic_score(tree, neighbors), 8);
    }

    #[test]
    fn get_view_count() {
        let tree = 5;

        let neighbors = vec![3, 5, 3];

        assert_eq!(get_view(tree, neighbors, Direction::NEG), 2);

        let neighbors = vec![4, 9];

        assert_eq!(get_view(tree, neighbors, Direction::POS), 2);
        let neighbors = vec![3];

        assert_eq!(get_view(tree, neighbors, Direction::POS), 1);
        let neighbors = vec![3, 5, 3];

        assert_eq!(get_view(tree, neighbors, Direction::NEG), 2);
    }
}
