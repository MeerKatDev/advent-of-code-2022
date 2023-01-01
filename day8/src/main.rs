use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    if let Ok(lines) = read_lines(file_path) {
        let tree_heights: Vec<Vec<usize>> =
            lines.map(|line| line_to_int_vec(line.unwrap())).collect();

        // much nicer but with a higher memory footprint
        println!("Result to Part One: {}", part_one(&tree_heights));
        println!("Result to Part Two: {}", part_two(&tree_heights));
    }
}

fn part_one(tree_heights: &[Vec<usize>]) -> usize {
    tree_heights
        .iter()
        .enumerate()
        .map(|(r, tree_line)| {
            tree_line
                .iter()
                .enumerate()
                .filter(|(c, &tree)| {
                    visible_from_up(tree_heights, tree, r, *c)
                        || visible_from_down(tree_heights, tree, r, *c)
                        || visible_from_left(tree_line, tree, *c)
                        || visible_from_right(tree_line, tree, *c)
                })
                .count()
        })
        .sum()
}

fn part_two(tree_heights: &[Vec<usize>]) -> usize {
    tree_heights
        .iter()
        .enumerate()
        .map(|(r, tree_line)| {
            tree_line
                .iter()
                .enumerate()
                .map(|(c, &tree)| {
                    scenic_score_from_up(tree_heights, tree, r, c)
                        * scenic_score_from_down(tree_heights, tree, r, c)
                        * scenic_score_from_left(tree_line, tree, c)
                        * scenic_score_from_right(tree_line, tree, c)
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

fn visible_from_up(tree_heights: &[Vec<usize>], tree: usize, r: usize, c: usize) -> bool {
    !tree_heights
        .iter()
        .take(r)
        .any(|other_tree| tree <= other_tree[c])
}

fn visible_from_left(tree_line: &[usize], tree: usize, c: usize) -> bool {
    !tree_line
        .iter()
        .take(c)
        .any(|&other_tree| tree <= other_tree)
}

fn visible_from_down(tree_heights: &[Vec<usize>], tree: usize, r: usize, c: usize) -> bool {
    !tree_heights
        .iter()
        .skip(r + 1)
        .any(|other_tree| tree <= other_tree[c])
}

fn visible_from_right(tree_line: &[usize], tree: usize, c: usize) -> bool {
    !tree_line
        .iter()
        .skip(c + 1)
        .any(|&other_tree| tree <= other_tree)
}

// part two

fn scenic_score_from_up(tree_heights: &[Vec<usize>], tree: usize, r: usize, c: usize) -> usize {
    let mut score = 0;

    for i in (0..r).rev() {
        score += 1;

        if tree <= tree_heights[i][c] {
            break;
        }
    }

    score
}

fn scenic_score_from_left(tree_line: &[usize], tree: usize, c: usize) -> usize {
    let mut score = 0;

    for i in (0..c).rev() {
        score += 1;

        if tree <= tree_line[i] {
            break;
        }
    }

    score
}

fn scenic_score_from_down(tree_heights: &[Vec<usize>], tree: usize, r: usize, c: usize) -> usize {
    let mut score = 0;

    for other_tree in tree_heights.iter().skip(r + 1) {
        score += 1;

        if tree <= other_tree[c] {
            break;
        }
    }

    score
}

fn scenic_score_from_right(tree_line: &[usize], tree: usize, c: usize) -> usize {
    let mut score = 0;

    for &other_tree in tree_line.iter().skip(c + 1) {
        score += 1;

        if tree <= other_tree {
            break;
        }
    }

    score
}

fn line_to_int_vec(row: String) -> Vec<usize> {
    return row
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
}

fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
