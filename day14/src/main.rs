use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    const CENTER: usize = 500;
    // part two
    const OFFSET: usize = 200;

    if let Ok(lines) = read_lines(file_path) {
        // let mut row: Vec<&str>;
        let mut tuples: Vec<(usize, usize)>;
        let mut prev_x: usize;
        let mut prev_y: usize;
        let mut grid = HashSet::new();
        // let mut rescaled_grid: HashSet<(usize, usize)>;
        let mut tups_iter;
        let mut rightmost_point: usize = 0;
        let mut leftmost_point: usize = CENTER;
        let mut upmost_point: usize = 0; // this is the upper part
        let mut downmost_point: usize = 0;

        for line in lines {
            // println!("line: {:?}", line);
            tuples = line
                .unwrap()
                .split(" -> ")
                .map(|s| {
                    s.split_once(',')
                        .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
                        .unwrap()
                })
                .collect();

            tups_iter = tuples.iter();

            if let Some(&(x, y)) = tups_iter.next() {
                // get extrema
                rightmost_point = rightmost_point.max(x);
                leftmost_point = leftmost_point.min(x);
                upmost_point = upmost_point.max(y);
                downmost_point = downmost_point.min(y);

                prev_x = x;
                prev_y = y;
            } else {
                panic!("something invalid here");
            }

            // build path point to point
            for &(x, y) in tups_iter {
                // get extrema
                rightmost_point = rightmost_point.max(x);
                leftmost_point = leftmost_point.min(x);
                upmost_point = upmost_point.max(y);
                downmost_point = downmost_point.min(y);

                if x != prev_x {
                    for tx in x.min(prev_x)..=x.max(prev_x) {
                        grid.insert((tx, y));
                    }
                } else {
                    for ty in y.min(prev_y)..=y.max(prev_y) {
                        grid.insert((x, ty));
                    }
                };
                prev_x = x;
                prev_y = y;

                // println!("{x} {y} {:?}", grid);
            }
        }

        println!("rightmost = {rightmost_point}, upmost = {upmost_point}");
        println!("leftmost = {leftmost_point}, downmost = {downmost_point}");

        println!("---- printing initial map ---");
        for yy in downmost_point..=upmost_point {
            print!("{yy} "); // index

            for xx in leftmost_point..=rightmost_point {
                if grid.contains(&(xx, yy)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }

        // part two
        let bottom = upmost_point + 2;
        for x in (leftmost_point-OFFSET)..=(rightmost_point+OFFSET) {
            grid.insert((x, bottom));
        }

        // falling sands
        let mut sand_particle;
        // let mut tmp_sand_particle;
        let mut sand_limit = 0;

        'outer: loop {
            // throwing sands
            sand_particle = (CENTER, 0);

            loop { // travelling sand

                if grid.contains(&(CENTER, 0)) {
                    break 'outer;
                }

                if !grid.contains(&(sand_particle.0, sand_particle.1 + 1)) { 
                    sand_particle = (sand_particle.0, sand_particle.1 + 1);
                } else if !grid.contains(&(sand_particle.0 - 1, sand_particle.1 + 1)) {
                    sand_particle = (sand_particle.0 - 1, sand_particle.1 + 1);
                } else if !grid.contains(&(sand_particle.0 + 1, sand_particle.1 + 1)) {
                    sand_particle = (sand_particle.0 + 1, sand_particle.1 + 1);
                } else {
                    sand_limit += 1;
                    // println!("sand_limit={sand_limit}");
                    grid.insert(sand_particle);
                    break;
                }
            }
        }
        println!("---- printing finally map ---");
        for yy in downmost_point..=bottom {
            for xx in (leftmost_point-OFFSET)..=(rightmost_point+OFFSET) {
                if grid.contains(&(xx, yy)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }

        println!("Sand limit: {}", sand_limit);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
