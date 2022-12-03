use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    
    if let Ok(lines) = read_lines(file_path) {
        let mut acc = 0;

        for line in lines {
            let rucksack_cnt = line.unwrap();
            let total = rucksack_cnt.len();
            let half = total/2;
            let fst_half = &rucksack_cnt[..half];
            let snd_half = &rucksack_cnt[half..];
            let ascii_code;

            'outer: for c in fst_half.chars() { 
                '_inner: for cc in snd_half.chars() { 
                    if c == cc {
                        ascii_code = c as u32;
                        acc += if (96..123).contains(&ascii_code) {
                            ascii_code - 96
                        } else {
                            ascii_code - 38
                        };
                        
                        break 'outer;
                    }
                }
            }
        }
        println!("score: {:?}", acc);
    }
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}