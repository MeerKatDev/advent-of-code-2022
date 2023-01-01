use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut acc = 0;
    let mut digits;
    let mut tree_heights: Vec<Vec<usize>> = Vec::new();

    if let Ok(lines) = read_lines(file_path) {
        let mut row;

        // parse heights map
        for line in lines {
            row = line.unwrap();
            digits = line_to_int_vec(row);
            tree_heights.push(digits);
        }

        for (r, tree_line) in tree_heights.iter().enumerate() {
            for (c, &tree) in tree_line.iter().enumerate() {
                if visible_from_up(&tree_heights, tree, r, c) || 
                    visible_from_left(&tree_line, tree, c) || 
                    visible_from_down(&tree_heights, tree, r, c) || 
                    visible_from_right(&tree_line, tree, c) {
                    acc += 1;
                }
            }
        }

        println!("Result={}", acc);
    }
}

fn visible_from_up(tree_heights: &Vec<Vec<usize>>, tree: usize, r: usize, c: usize) -> bool {
    let mut visible = true;

    if r == 0 {
        return true;
    }

    for i in 0..r {
        if tree <= tree_heights[i][c] {
            visible = false;
            break;
        }
    }

    visible
}


fn visible_from_left(tree_line: &[usize], tree: usize, c: usize) -> bool {
    let mut visible = true;

    if c == 0 {
        return true;
    }

    for i in 0..c {
        if tree <= tree_line[i] {
            visible = false;
            break;
        }
    }

    visible
}

fn visible_from_down(tree_heights: &Vec<Vec<usize>>, tree: usize, r: usize, c: usize) -> bool {
    let mut visible = true;
    let h = tree_heights.len();

    if r == (h - 1) {
        return true;
    }

    for i in (r+1)..h {
        if tree <= tree_heights[i][c] {
            visible = false;
            break;
        }
    }

    visible
}

fn visible_from_right(tree_line: &[usize], tree: usize, c: usize) -> bool {
    let mut visible = true;
    let w = tree_line.len();

    if c == (w - 1) {
        return true;
    }

    for i in (c+1)..w {
        if tree <= tree_line[i] {
            visible = false;
            break;
        }
    }

    visible
}

fn line_to_int_vec(row: String) -> Vec<usize> {
  return row.chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
}

fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}