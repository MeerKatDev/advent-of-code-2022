use std::env;
// use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    
    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        let mut ip; // keeps line
        let mut acc = 0; // keeps accumulator for sum
        let mut vec = Vec::new();

        for line in lines {
            ip = line.unwrap();
            if ip.is_empty() {
                vec.push(acc);
                acc = 0; 
            } else {
                let num = ip.as_str().parse::<u32>().unwrap();
                acc = acc + num;
            }
            vec.sort_by(|a, b| b.cmp(a));
            vec.truncate(3);
   
        }
        let sum: u32 = vec.iter().sum();
        println!("max calories: {:?}", sum);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}