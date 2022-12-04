use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    if let Ok(lines) = read_lines(file_path) {
        let mut acc = 0;

        for line in lines {
            let row = line.unwrap();
            let [fst, snd]: [&str; 2] = row.split(',').collect::<Vec<&str>>().try_into().unwrap();

            let (a1, a2) = parse_range(fst);
            let (b1, b2) = parse_range(snd);

            if (a1 >= b1 && a2 <= b2) || (a1 <= b1 && a2 >= b2) {
                acc += 1;
            }
        }
        println!("score: {:?}", acc);
    }
}

fn parse_range(range: &str) -> (u32, u32) {
    let [m1, m2]: [&str; 2] = range.split('-').collect::<Vec<&str>>().try_into().unwrap();
    (m1.parse::<u32>().unwrap(), m2.parse::<u32>().unwrap())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
