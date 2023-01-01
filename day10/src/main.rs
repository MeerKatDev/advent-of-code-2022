use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

const COLS: usize = 40;
const SPRITE_WIDTH: usize = 3;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut reg = 1;
    let mut strength = 0;
    let mut cycle = 0;
    let mut row;

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            row = line.unwrap();
            cycle += 1;
            strength += add_strength(cycle, reg);

            if let Some(("addx", num_str)) = row.split_once(' ') {
                cycle += 1;
                strength += add_strength(cycle, reg);
                reg += num_str.parse::<isize>().unwrap();
            }
        }

        println!("Part One solution: {strength}");
    }
}

fn add_strength(cycle: usize, reg: isize) -> isize {
    // part two
    print_pixel(cycle, reg);

    if cycle % 40 == 20 {
        (cycle as isize) * reg
    } else {
        0
    }
}

fn print_pixel(cycle: usize, reg: isize) {
    let column = (cycle - 1) % COLS;

    if column.abs_diff(reg as usize) <= SPRITE_WIDTH / 2 {
        print!("#");
    } else {
        print!(".");
    }

    if cycle % 40 == 0 {
        println!();
    }
}

fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
