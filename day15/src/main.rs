use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type Point = (isize, isize);

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let re = Regex::new(r"(-\d+)|(\d+)").unwrap();
    let mut sensors: HashMap<Point, usize> = HashMap::new();
    let mut beacons: HashSet<Point> = HashSet::new();
    let mut manhattan: usize;
    let mut max_manhattan = 0;

    // grid properties
    let mut rightmost_point: isize = 0;
    let mut leftmost_point: isize = -10;
    let mut upmost_point: isize = 0; // this is the upper part
    let mut downmost_point: isize = -10;

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            let [xs, ys, xb, yb] = take_coordinates(&re, &line.unwrap());
            manhattan = xs.abs_diff(xb) + ys.abs_diff(yb);
            max_manhattan = manhattan.max(max_manhattan);

            // get extrema
            rightmost_point = rightmost_point.max(xs).max(xb);
            leftmost_point = leftmost_point.min(xs).min(xb);
            upmost_point = upmost_point.max(ys).max(yb);
            downmost_point = downmost_point.min(ys).min(yb);

            sensors.insert((xs, ys), manhattan);
            beacons.insert((xb, yb));
        }

        let mut acc = 0;
        for xx in (leftmost_point - (max_manhattan as isize))
            ..=(rightmost_point + (max_manhattan as isize))
        {
            if in_range(&sensors, xx, 2000000) && !beacons.contains(&(xx, 2000000)) {
                acc += 1;
            }
        }

        // println!("---- printing initial map ---");
        // for yy in downmost_point..=upmost_point {
        //     print!("{:2}", yy); // index

        //     for xx in (leftmost_point - ((max_manhattan / 2) as isize))
        //         ..=(rightmost_point + ((max_manhattan / 2) as isize))
        //     {
        //         if sensors.contains_key(&(xx, yy)) {
        //             print!(" S ");
        //         } else if beacons.contains(&(xx, yy)) {
        //             print!(" B ");
        //         } else if in_range(&sensors, xx, yy) {
        //             print!(" # ");
        //         } else {
        //             print!(" . ");
        //         }
        //     }
        //     println!();
        // }

        println!("Solution to Part One: {acc}");
    }
}

fn in_range(sensors: &HashMap<Point, usize>, x: isize, y: isize) -> bool {
    sensors
        .iter()
        .any(|((xx, yy), &d)| xx.abs_diff(x) + yy.abs_diff(y) <= d)
}

fn take_coordinates<const N: usize>(re: &Regex, row: &str) -> [isize; N] {
    re.captures_iter(row)
        .map(|x| x.get(0).unwrap().as_str().parse::<isize>().unwrap())
        .collect::<Vec<isize>>()
        .try_into()
        .unwrap()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
