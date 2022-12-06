use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");
    let text = from_file(path);

    println!("marker: {}", get_marker(text.as_str(), 14));
}

fn from_file(path: &Path) -> String {
    let mut text = String::new();
    let mut input = File::open(path).unwrap();
    input.read_to_string(&mut text).ok();
    text
}

fn get_marker(s: &str, num: usize) -> usize {
    let mut tmp: &str;
    let mut idx_fwd: usize = 0;

    for idx in 0..(s.len() - num) {
        idx_fwd = idx + num;
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

    // packet marker

    #[test]
    fn demo_zero() {
        assert_eq!(get_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
    }

    #[test]
    fn demo_one() {
        assert_eq!(get_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
    }

    #[test]
    fn demo_two() {
        assert_eq!(get_marker("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
    }

    #[test]
    fn demo_three() {
        assert_eq!(get_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
    }

    #[test]
    fn demo_four() {
        assert_eq!(get_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
    }

    // message marker
    
    #[test]
    fn demo_zero_p2() {
        assert_eq!(get_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
    }

    #[test]
    fn demo_one_p2() {
        assert_eq!(get_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
    }

    #[test]
    fn demo_two_p2() {
        assert_eq!(get_marker("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
    }

    #[test]
    fn demo_three_p2() {
        assert_eq!(get_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
    }

    #[test]
    fn demo_four_p2() {
        assert_eq!(get_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
    }
}
