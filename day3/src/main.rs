use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    if let Ok(lines) = read_lines(file_path) {
        let mut acc = 0;
        let mut elves_group: [String; 3] =
            [String::from("1"), String::from("2"), String::from("3")];

        let mut idx = 0;

        for line in lines {
            elves_group[idx] = line.unwrap();

            idx += 1;

            if idx == 3 {
                acc += elves_group[0]
                    .chars()
                    .into_iter()
                    .find(|&c| elves_group[1].contains(c) && elves_group[2].contains(c))
                    .map_or(0, |c| char_to_priority(c));

                idx = 0;
            }
        }
        println!("score: {:?}", acc);
    }
}

fn char_to_priority(c: char) -> u32 {
    let ascii_code = c as u32;
    if (96..123).contains(&ascii_code) {
        ascii_code - 96
    } else {
        ascii_code - 38
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
