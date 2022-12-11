use std::cmp::Ordering;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const CENTER: usize = 400;

struct Grid {
    head_pos_x: usize,
    head_pos_y: usize,
    tail_pos_x: usize,
    tail_pos_y: usize,
    visited: Vec<(usize, usize)>,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            head_pos_x: CENTER,
            head_pos_y: CENTER,
            tail_pos_x: CENTER,
            tail_pos_y: CENTER,
            visited: vec![(CENTER, CENTER)],
        }
    }

    pub fn move_right_by(&mut self, n: usize, acc: &mut usize) {
        for _step in 0..n {
            self.head_pos_x += 1;

            if self.too_far() {
                self.tail_pos_x += 1;
                self.tail_recover_vertical();
                self.register_position(acc);
            }
        }
    }

    pub fn move_up_by(&mut self, n: usize, acc: &mut usize) {
        for _step in 0..n {
            self.head_pos_y += 1;

            if self.too_far() {
                self.tail_pos_y += 1;
                self.tail_recover_horizontal();
                self.register_position(acc);
            }
        }
    }

    pub fn move_left_by(&mut self, n: usize, acc: &mut usize) {
        for _step in 0..n {
            self.head_pos_x -= 1;

            if self.too_far() {
                self.tail_pos_x -= 1;
                self.tail_recover_vertical();
                self.register_position(acc);
            }
        }
    }

    pub fn move_down_by(&mut self, n: usize, acc: &mut usize) {
        for _step in 0..n {
            self.head_pos_y -= 1;

            if self.too_far() {
                self.tail_pos_y -= 1;
                self.tail_recover_horizontal();
                self.register_position(acc);
            }
        }
    }

    // debug
    // pub fn print_coords(&self) {
    //     println!("H({},{}), T({},{})", self.head_pos_x, self.head_pos_y, self.tail_pos_x, self.tail_pos_y);
    // }

    fn register_position(&mut self, acc: &mut usize) {
        if !self.visited.contains(&(self.tail_pos_x, self.tail_pos_y)) {
            *acc += 1;
            self.visited.push((self.tail_pos_x, self.tail_pos_y))
        }
    }

    fn tail_recover_horizontal(&mut self) {
        match self.tail_pos_x.cmp(&self.head_pos_x) {
            Ordering::Greater => self.tail_pos_x -= 1,
            Ordering::Less => self.tail_pos_x += 1,
            Ordering::Equal => (),
        }
    }

    fn tail_recover_vertical(&mut self) {
        match self.tail_pos_y.cmp(&self.head_pos_y) {
            Ordering::Greater => self.tail_pos_y -= 1,
            Ordering::Less => self.tail_pos_y += 1,
            Ordering::Equal => (),
        }
    }

    fn too_far(&self) -> bool {
        self.head_pos_x.abs_diff(self.tail_pos_x) > 1
            || self.head_pos_y.abs_diff(self.tail_pos_y) > 1
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    if let Ok(lines) = read_lines(file_path) {
        let mut row; // keeps line
        let mut acc: usize = 1; // keeps accumulator for sum
        let mut grid = Grid::new();

        for line in lines {
            row = line.unwrap();

            if let Some((dir, n)) = row
                .split_once(' ')
                .map(|(dir, n)| (dir, n.parse::<usize>().unwrap()))
            {
                match dir {
                    "R" => grid.move_right_by(n, &mut acc),
                    "L" => grid.move_left_by(n, &mut acc),
                    "U" => grid.move_up_by(n, &mut acc),
                    "D" => grid.move_down_by(n, &mut acc),
                    _ => (),
                };
            }
        }
        println!("result: {acc}");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
