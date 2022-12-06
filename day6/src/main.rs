use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");
    let text = from_file(path);

    println!("marker: {}", get_marker(text.as_str()));
}

fn from_file(path: &Path) -> String {
    let mut text = String::new();
    let mut input = File::open(path).unwrap();
    input.read_to_string(&mut text).ok();
    text
}

fn get_marker(s: &str) -> usize {
    let mut tmp: &str;
    let mut idx_fwd: usize = 0;

    for idx in 0..(s.len() - 4) {
        idx_fwd = idx + 4;
        tmp = &s[idx..idx_fwd];
        if is_uniq(tmp) {
            break;
        }
    }

    idx_fwd
}

fn is_uniq(s: &str) -> bool {
    let mut v: Vec<char> = s.chars().collect();
    v.sort();
    v.dedup();

    v.len() == s.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_zero() {
        assert_eq!(get_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
    }

    #[test]
    fn demo_one() {
        assert_eq!(get_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    }

    #[test]
    fn demo_two() {
        assert_eq!(get_marker("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    }

    #[test]
    fn demo_three() {
        assert_eq!(get_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    }

    #[test]
    fn demo_four() {
        assert_eq!(get_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }
}
