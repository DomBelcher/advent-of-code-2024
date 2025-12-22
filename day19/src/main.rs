use std::{collections::HashMap, fs};

const FILENAME: &str = "./inputs/input";

fn main() {
    let (towels, patterns) = parse_input();

    println!("Towels: {} | Patterns: {}", towels.len(), patterns.len());

    let mut count = 0;
    let mut arranements = 0;
    let mut match_counts = HashMap::new();

    for (i, pattern) in patterns.iter().enumerate() {
        let match_result = pattern_match(&pattern.as_slice(), &vec![], &towels);
        println!("Pattern {} | Matches: {}", i, match_result);
        if match_result {
            count += 1;
            let pattern_str = pattern.iter().collect::<String>();
            let possible_towels = possible_towels(&pattern_str, &towels);
            println!("{} possible towels of {} total", possible_towels.len(), towels.len());
            arranements += count_arrangements(&pattern, &possible_towels, 0, &mut match_counts);
        }
    }

    println!("Matched {} patterns", count);
    println!("Total arrangements: {}", arranements);
    println!("Total match counts: {}", match_counts.len());
}

fn possible_towels (pattern: &String, all_towels: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut possible_towels = vec![];

    for towel in all_towels.iter() {
        let towel_str = towel.iter().collect::<String>();
        if pattern.contains(&towel_str) {
            possible_towels.push(towel.clone())
        }
     }
    return possible_towels
}

fn count_arrangements (pattern: &[char], all_towels: &Vec<Vec<char>>, depth: usize, match_counts: &mut HashMap<String, i64>) -> i64 {
    if pattern.len() == 0 {
        return 0;
    }
    let mut count = 0;
    for towel in all_towels {
        if pattern.len() < towel.len() {
            continue
        }
        let remaining_pattern = &pattern[towel.len()..];
        let rp_str = remaining_pattern.iter().collect::<String>();
        if pattern[..towel.len()] == *towel.as_slice() {
            if pattern.len() == towel.len() {
                count += 1
            } else if match_counts.contains_key(&rp_str) {
                count += match_counts.get(&rp_str).unwrap();
            } else {
                let possible_towels = possible_towels(&rp_str, &all_towels);
                if possible_towels.len() == 0 {
                    continue
                }
                println!("Depth {}", depth);
                let mc = count_arrangements(remaining_pattern, &possible_towels, depth + 1, match_counts);
                match_counts.insert(rp_str, mc);
                count += mc;
            }
        }
    }
    return count;
}


fn pattern_match (pattern: &[char], matching_towel: &Vec<char>, all_towels: &Vec<Vec<char>>) -> bool {
    if pattern.len() == 0 && matching_towel.len() == 0 {
        return true;
    } else if pattern.len() < matching_towel.len() {
        return false;
    }
    
    if matching_towel.len() != 0 {
        if matching_towel[0] == pattern[0] {
            let sliced_towel = matching_towel.as_slice()[1..].to_vec();
            return pattern_match(&pattern[1..], &sliced_towel, all_towels);
        }
        return false;
    } else {
        let mut matches = false;
        for towel in all_towels {
            if towel[0] == pattern[0] {
                let sliced_towel = towel.as_slice()[1..].to_vec();
                matches = matches || pattern_match(&pattern[1..], &sliced_towel, all_towels);
            }
        }
        return matches;
    }


}

fn parse_input () -> (Vec<Vec<char>>, Vec<Vec<char>>) {
    let mut towels = vec![];
    let mut patterns = vec![];

    let mut input_mode = "towels";

    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        if line.len() == 0 {
            input_mode = "patterns";
            continue;
        }

        if input_mode == "towels" {
            towels = line.split(", ").map(|s| s.chars().collect()).collect();
        } else if input_mode == "patterns" {
            patterns.push(line.chars().collect());
        }
    }

    return (towels, patterns)
}