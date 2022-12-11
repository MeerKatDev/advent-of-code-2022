use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;
// use std::ops::{Add, Mul};
use regex::Regex;
use std::fmt;

// type MonkeyOp = fn(usize) -> usize;

#[derive(Debug, Clone)]
struct Monkey
// struct Monkey<F>
// where
//     F: Fn (u32, u32) -> u32
{
    mid: usize,
    items: Vec<usize>,
    operation: (char, usize),
    test_divisor: usize,
    num_if_true: usize,
    num_if_false: usize,
    inspects_num: usize
}

impl Default for Monkey {
    fn default() -> Monkey {
        Monkey {
            mid: 0,
            items: vec![],
            operation: (' ', 0),
            test_divisor: 1,
            num_if_true: 0,
            num_if_false: 0,
            inspects_num: 0
        }
    }
}

impl fmt::Display for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Monkey#{}(
            Items: {:?},
            operation: {:?},
            test_divisor: {},
            num_if_true: {},
            num_if_false: {},
        )", self.mid, self.items, self.operation, self.test_divisor, self.num_if_true, self.num_if_false)
    }
}


struct MonkeysList<'a>(&'a Vec<Monkey>);
impl fmt::Display for MonkeysList<'_>  {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }

        write!(f, "]")
    }
}

// fn mul_old(old: usize) -> usize { old * old }
// fn add_old(old: usize) -> usize { old + old }
// fn mul_num(old: usize, num: usize) -> usize { old * num }
// fn add_num(old: usize, num: usize) -> usize { old + num }
// fn mul_num_and_old(num: usize) -> impl Fn(usize) -> usize { |num| move |y| num * y }
// fn add_num_and_old(num: usize) -> impl Fn(usize) -> usize { |num| move |y| num + y }

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let comma_sep_numbers = Regex::new(r"(?m)(\d+)[^,]?+").unwrap();
    let op_regex = Regex::new(r"([*+] \d+)").unwrap();
    let mut tmp: Monkey = Monkey { ..Default::default() };
    let mut row;
    let mut monkeys: Vec<Monkey> = Vec::new();

    if let Ok(lines) = read_lines(file_path) {

        for line in lines {
            row = line.unwrap();
            
            match row.trim_start().split_once(' ') {
                Some(("Monkey", mid_str)) => {
                    println!("{}", mid_str);
                    let mid = mid_str.trim_matches(':').parse::<usize>().unwrap();
                    tmp = Monkey { mid, ..Default::default() }
                } ,
                Some(("Starting", starting_items)) => {
                    let mut items = Vec::new();

                    for cap in comma_sep_numbers.captures_iter(starting_items) {
                        items.push(cap[0].parse::<usize>().unwrap());
                    }

                    tmp = Monkey { items, ..tmp }
                },
                Some(("Operation:", operation_str)) => {
                    // match parse_operation(&op_regex, operation_str) {
                    //     ('+', 0) =>
                    //         tmp = Monkey { operation: add_old, ..tmp },
                    //     ('*', 0) =>
                    //         tmp = Monkey { operation: mul_old, ..tmp },
                    //     ('+', num) =>
                    //         tmp = Monkey { operation: add_num_and_old(num), ..tmp },
                    //     ('*', num) =>
                    //         tmp = Monkey { operation: mul_num_and_old(num) ..tmp },
                    //     _ => (),
                    // }
                    println!("operation {}", operation_str);
                    let operation = parse_operation(&op_regex, operation_str);
                    tmp = Monkey { operation, ..tmp }
                },
                Some(("Test:", div_by_num)) => {
                    let test_divisor: usize = div_by_num.rsplit_once(' ').map(|(_, num_str)| num_str.parse::<usize>().unwrap()).unwrap();
                    tmp = Monkey { test_divisor, ..tmp }
                },
                Some(("If", phrase)) => {
                    if let Some((boo, num_str)) = phrase.split_once(": throw to monkey ") {
                        let num = num_str.parse::<usize>().unwrap();

                        if boo == "true" {
                            tmp = Monkey { num_if_true: num, ..tmp }
                        } else {
                            tmp = Monkey { num_if_false: num, ..tmp };
                            monkeys.push(tmp.clone());
                        } 
                    }
                },
                sth => println!("others {:?}", sth),
            }

        }
        println!("monkeys: {}", MonkeysList(&monkeys));

        let cap = monkeys.len(); 
        let mut idx = 0;
        let mut loops_idx = 0;
        let mut mon;
        // let mut deg = 0;
        let mut snd_mon;
        let mut new_it;
        let mut it;
        // let mut imm_monkeys = monkeys.clone();
        let loops = cap * 10000;


        let magic_mod: usize = monkeys.iter().map(|m| m.test_divisor).product();

        loop {
            // break;
            if idx % cap == 0 { idx = 0; }

            // println!("idx: {idx}");

            mon = monkeys.get_mut(idx).unwrap().clone();

            mon.inspects_num += mon.items.len() as usize;

            for _ in 0..mon.items.len() {
                it = mon.items.remove(0);
                // println!("popped: {}, op: {:?}", it, mon.operation);
                new_it = match mon.operation {
                    ('+', 0) => it + it,
                    ('*', 0) => it * it,
                    ('+', num) => it + num,
                    ('*', num) => it * num,
                    _ => panic!("WTF"),
                };
                // println!("new_it (1): {new_it}");

                new_it %= magic_mod;
                // println!("new_it (2): {new_it}");

                if new_it % mon.test_divisor == 0 {
                    snd_mon = monkeys.get_mut(mon.num_if_true).unwrap();
                    snd_mon.items.push(new_it);
                } else {
                    snd_mon = monkeys.get_mut(mon.num_if_false).unwrap();
                    snd_mon.items.push(new_it);
                }
                // state(&monkeys);
            }
            // monkeys.insert(idx, mon);
            monkeys[idx] = mon;


            idx += 1;
            loops_idx += 1;
            // imm_monkeys = monkeys.clone();
            if loops_idx == loops { break; }
            // if deg == 12 { break; }
            // deg += 1;
        }
        
        state(&monkeys);

    }
}

fn state(ms: &Vec<Monkey>) {
    println!("------");
    for m in ms.iter() {
        println!("monkey {}: {:?}, inspects_num: {}", m.mid, m.items, m.inspects_num);
    }
}

fn parse_operation(op_regex: &Regex, operation: &str) -> (char, usize) {
    if op_regex.is_match(operation) {
        let op_and_num = op_regex.captures(operation).unwrap();

        op_and_num[0]
        .split_once(' ')
        .map(|(op, num_str)| {
            let num = num_str.parse::<usize>().unwrap();
            (op.chars().next().unwrap(), num)
        })
        .unwrap()     
    } else if operation.contains('+') {
        ('+', 0)
    } else if operation.contains('*') {
        ('*', 0)
    } else {
        panic!("WTF")
    }
}

fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}