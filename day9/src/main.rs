use std::cmp::Ordering;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const CENTER: usize = 400;
const KNOTS_NUM: usize = 10;

type Coords = Vec<(usize, usize)>;

struct Grid {
    knots_positions: [(usize, usize);KNOTS_NUM],
    visited: Coords,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            knots_positions: [(CENTER,CENTER);KNOTS_NUM],
            visited: vec![(CENTER, CENTER)],
        }
    }

    pub fn move_right_by(&mut self, n: usize, acc: &mut usize) {
        for _step in 0..n {
            self.knots_positions[0].0 += 1;

            for idx in 1..KNOTS_NUM {
                if self.too_far_from_previous(idx) {
                    self.knots_positions[idx].0 += 1;
                    self.tail_recover_vertical(idx);
                    if idx == (KNOTS_NUM - 1) { self.register_position(acc); }
                }
            }
        }
    }

    pub fn move_up_by(&mut self, n: usize, acc: &mut usize) {
        for _step in 0..n {
            self.knots_positions[0].1 += 1;

            for idx in 1..KNOTS_NUM {
                if self.too_far_from_previous(idx) {
                    self.knots_positions[idx].1 += 1;
                    self.tail_recover_horizontal(idx);
                    if idx == (KNOTS_NUM - 1) { self.register_position(acc); }
                }
            }
        }
    }

    pub fn move_left_by(&mut self, n: usize, acc: &mut usize) {
        for _step in 0..n {
            self.knots_positions[0].0 -= 1;

            for idx in 1..KNOTS_NUM {
                if self.too_far_from_previous(idx) {
                    self.knots_positions[idx].0 -= 1;
                    self.tail_recover_vertical(idx);
                    if idx == (KNOTS_NUM - 1) { self.register_position(acc); }
                }
            }
        }
    }

    pub fn move_down_by(&mut self, n: usize, acc: &mut usize) {
        for _step in 0..n {
            self.knots_positions[0].1 -= 1;

            for idx in 1..KNOTS_NUM {
                if self.too_far_from_previous(idx) {
                    self.knots_positions[idx].1 -= 1;
                    self.tail_recover_horizontal(idx);
                    if idx == (KNOTS_NUM - 1) { self.register_position(acc); }
                }
            }
        }
    }

    fn register_position(&mut self, acc: &mut usize) {
        if !self.visited.contains(&self.knots_positions[KNOTS_NUM - 1]) {
            *acc += 1;
            println!("cx: ({}, {})", self.knots_positions[KNOTS_NUM - 1].0, self.knots_positions[KNOTS_NUM - 1].1 );
            self.visited.push(self.knots_positions[KNOTS_NUM - 1])
        }
    }

    fn tail_recover_horizontal(&mut self, knot_idx: usize) {
        match self.knots_positions[knot_idx].0.cmp(&self.knots_positions[knot_idx - 1].0) {
            Ordering::Greater => self.knots_positions[knot_idx].0 -= 1,
            Ordering::Less => self.knots_positions[knot_idx].0 += 1,
            Ordering::Equal => (),
        }
    }

    fn tail_recover_vertical(&mut self, knot_idx: usize) {
        match self.knots_positions[knot_idx].1.cmp(&self.knots_positions[knot_idx - 1].1) {
            Ordering::Greater => self.knots_positions[knot_idx].1 -= 1,
            Ordering::Less => self.knots_positions[knot_idx].1 += 1,
            Ordering::Equal => (),
        }
    }

    fn too_far_from_previous(&self, knot_idx: usize) -> bool {
        self.knots_positions[knot_idx].0.abs_diff(self.knots_positions[knot_idx - 1].0) > 1
            || self.knots_positions[knot_idx].1.abs_diff(self.knots_positions[knot_idx - 1].1) > 1
    }
}

struct GraphicGrid {
    visited: Coords,
    hor_offset: usize,
    ver_offset: usize,
}

impl GraphicGrid {
    fn new(grid: &Grid) -> Self {
        let mut min_x: usize = CENTER;
        let mut max_x: usize = CENTER;
        let mut min_y: usize = CENTER;
        let mut max_y: usize = CENTER;

        for (x, y) in grid.visited.iter() {
            min_x = (*x).min(min_x);
            min_y = (*y).min(min_y);
            max_x = (*x).max(max_x);
            max_y = (*y).max(max_y);
        }

        let visited_rescaled = grid.visited.iter().map(|(x,y)| (x - min_x, y - min_y)).collect();

        Self { visited: visited_rescaled, hor_offset: max_x - min_x, ver_offset: max_y - min_y }
    }

    pub fn draw(&self) {
        println!("ver_offset: {}, hor_offset: {}", self.ver_offset, self.hor_offset);

        for y in 0..=self.ver_offset {
            for x in 0..=self.hor_offset {
                print!("{}", if self.visited.contains(&(x,self.ver_offset-y)) { "#" } else { "." });
            }
            println!("{}", "");
        }
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

            println!("{}", row);

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
            let gg = GraphicGrid::new(&grid);
            gg.draw();
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
