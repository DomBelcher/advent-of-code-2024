use std::fs;
use regex::Regex;

const FILENAME: &str = "./inputs/input";

fn main() {
    let mut i = 0;
    let mut total_1 = 0;
    let mut total_2 = 0;
    let mut active = true;
    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        total_1 += parse(line);
        
        let (res, act) = parse_with_conditionals(line, active);
        total_2 += res;
        active = act;
        i += 1;
    }
    println!("{}", total_1);
    println!("{}", total_2);
}

fn parse(line: &str) -> i32 {
    let re = Regex::new(r"mul\((?<num1>[0-9]{1,3})\,(?<num2>[0-9]{1,3})\)").unwrap();

    let mults: Vec<i32> = re.captures_iter(line).map(
        |m| m.name("num1").unwrap().as_str().parse::<i32>().unwrap()
        * m.name("num2").unwrap().as_str().parse::<i32>().unwrap()
    ).collect();
    return mults.iter().sum();
}

fn parse_with_conditionals(line: &str, start_active_state: bool) -> (i32, bool) {
    let re =  Regex::new(r"(?<conditional>(do\(\))|(don\'t\(\)))|(?<mult>mul\((?<num1>[0-9]{1,3})\,(?<num2>[0-9]{1,3})\))").unwrap();
    let mut active = start_active_state;
    let mut total = 0;

    re.captures_iter(line).for_each(|caps| {
        if caps.name("conditional").is_some() {
            println!("{:?}",caps.name("conditional").unwrap());
            match caps.name("conditional").unwrap().as_str() {
                "do()" => active = true,
                "don't()" => active = false,
                _ => panic!("oh no")
            }
        } else if caps.name("mult").is_some() {
            if active {
                total += caps.name("num1").unwrap().as_str().parse::<i32>().unwrap()
                * caps.name("num2").unwrap().as_str().parse::<i32>().unwrap()
            }
        }
    });
       
    return (total, active)
}

