use std::{collections::HashMap, fs};

const FILENAME: &str = "./inputs/input";

fn main() {
    println!("{}", 3/2 + 1);
    let (rules, updates) = parse_input();

    let mut count = 0;
    let mut total = 0;
    let mut fixed_total = 0;

    for update in updates {
        if validate(&rules, &update) {
            count += 1;
            total += get_middle_val(&update);
        } else {
            println!("fixing: {:?}", update);
            let fixed = fix_ordering(&rules, &update);
            println!("fixed: {:?}", fixed);

            fixed_total += get_middle_val(&fixed);
        }
    }

    println!("valid updates: {}", count);
    println!("sum of middles: {}", total);
    println!("sum of fixed middles: {}", fixed_total);
    println!("sum of both middles: {}", total + fixed_total);
}

fn parse_input() -> (Vec<[i32; 2]>, Vec<Vec<i32>>) {
    let mut rules: Vec<[i32; 2]> = vec![];
    let mut updates = vec![];

    let mut stage = "rules";

    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        if line == "" {
            stage = "updates";
            continue
        }

        if stage == "rules" {
            let nums: Vec<_> = line.split('|').collect();
            // println!("{:?}", nums);
            let num1 = nums[0].parse::<i32>().unwrap();
            let num2 = nums[1].parse::<i32>().unwrap();
            // rules.insert(num1, num2);
            rules.push([num1, num2]);
        } else if stage == "updates" {
            // println!("{:?}", line);
            let pages: Vec<i32> = line.split(',').map(|s|  s.parse::<i32>().unwrap()).collect();
            updates.push(pages);
        }
    }
    return (rules, updates);
}

fn validate (rules: &Vec<[i32; 2]>, update: &Vec<i32>) -> bool {
    let update_map = update_to_map(update);

    for rule in rules {
        // if !update_map.contains_key(&rule[0]) || !update_map.contains_key(&rule[1]) {
        //     continue
        // }

        // if update_map.get(&rule[0]).unwrap() > update_map.get(&rule[1]).unwrap() {
        //     return false
        // }
        if violates_rule(rule, &update_map) {
            return false
        }
    }

    return true;
}

fn violates_rule (rule: &[i32; 2], update_map: &HashMap<&i32, usize>) -> bool {
    if !update_map.contains_key(&rule[0]) || !update_map.contains_key(&rule[1]) {
        return false;
    }
    if update_map.get(&rule[0]).unwrap() > update_map.get(&rule[1]).unwrap() {
        return true;
    }
    return false;
}

fn update_to_map(update: &Vec<i32>) -> HashMap<&i32, usize> {
    let mut update_map: HashMap<&i32, usize> = HashMap::new();

    for (i, page) in update.iter().enumerate() {
        update_map.insert(page, i);
    }
    return update_map
}

fn get_middle_val (update: &Vec<i32>) -> i32 {
    let middle_index = update.len() / 2;
    return update[middle_index];
}

fn fix_ordering(rules: &Vec<[i32; 2]>, update: &Vec<i32>) -> Vec<i32> {
    let mut update_map = update_to_map(update);

    for rule in rules {
        if violates_rule(rule, &update_map) {
            let position_1 = *update_map.get(&rule[0]).unwrap();
            let position_2 = *update_map.get(&rule[1]).unwrap();

            update_map.insert(&rule[0], position_2);
            update_map.insert(&rule[1], position_1);
        }
    }

    let fixed_update = map_to_update(&update_map);

    if !validate(rules, update) {
        return fix_ordering(rules, &fixed_update)
    }

    return fixed_update;


    return vec![];
}

fn map_to_update(update_map: &HashMap<&i32, usize>) -> Vec<i32> {
    let mut update = vec![];
    // let mut inverse_map = HashMap::new();

    let mut ind_vec: Vec<(&&i32, &usize)> = update_map.iter().collect();
    ind_vec.sort_by(|(p1, i1), (p2, i2)| i1.cmp(i2));

    for (page, index) in ind_vec {
        update.push(**page);
    }

    return update;
}