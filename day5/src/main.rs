use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    const CRANES_NUM: usize = 9;

    if let Ok(lines) = read_lines(file_path) {
        const INIT_VEC: Vec<char> = Vec::new();
        let mut cranes = [INIT_VEC; CRANES_NUM];
        let mut parsing_crane: bool = true;
        let mut move_cx: [usize; CRANES_NUM] = [1; CRANES_NUM]; // moving coordinates
        let mut tmp;

        // regex capturing [A] .. [Z] crates or spaces between
        let re_crane = Regex::new(r"([A-Z]|\s\s\s\s)\w*").unwrap();
        // regex capturing all numbers on a line
        let re_digits = Regex::new(r"\d+").unwrap();

        for line in lines {
            let row = line.unwrap();

            // assumption: the initial state and the movement list
            // are divided by an empty line
            if row.as_str() == "" {
                parsing_crane = false;
            }

            if parsing_crane {
                for (idx, symb) in re_crane.captures_iter(row.as_str()).enumerate() {
                    match symb[0].chars().next() {
                        Some(c) if c != ' ' => cranes[idx].insert(0, c),
                        _ => (),
                    }
                }
            } else {
                for (idx, number) in re_digits.captures_iter(row.as_str()).enumerate() {
                    move_cx[idx] = number[0].parse::<usize>().unwrap();
                }

                for _ in 0..(move_cx[0]) {
                    tmp = cranes[move_cx[1] - 1].pop().unwrap();
                    cranes[move_cx[2] - 1].push(tmp);
                }
            }
        }
        // collecting last crates
        let result: String = cranes.map(|mut v| v.pop().unwrap()).iter().collect();

        println!("result: {:?}", result);
    }
}

fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
