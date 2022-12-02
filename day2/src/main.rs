use std::env;
// use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    
    if let Ok(lines) = read_lines(file_path) {
        let mut acc = 0; // keeps accumulator for sum

        for line in lines {
            let score = second_strategy(line.unwrap());
            acc = acc + score;
        }
        println!("score: {:?}", acc);
    }
}

fn first_strategy(owned_string: String) -> u32 {
    match owned_string.as_str() {
        "A X" => 1 + 3,
        "A Y" => 2 + 6,
        "A Z" => 3 + 0,
        
        "B X" => 1 + 0,
        "B Y" => 2 + 3,
        "B Z" => 3 + 6,

        "C X" => 1 + 6,
        "C Y" => 2 + 0,
        "C Z" => 3 + 3,
        _ => panic!("wtf"),
    }
}

fn second_strategy(owned_string: String) -> u32 {
    match owned_string.as_str() {
        "A X" => 0 + 3,
        "A Y" => 3 + 1,
        "A Z" => 6 + 2,
        
        "B X" => 0 + 1,
        "B Y" => 3 + 2,
        "B Z" => 6 + 3,

        "C X" => 0 + 2,
        "C Y" => 3 + 3,
        "C Z" => 6 + 1,
        _ => panic!("wtf"),
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}