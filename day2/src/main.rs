use std::fs;

const FILENAME: &str = "./inputs/input";

const MAX_INCREASE: i32 = 3;
const MAX_VIOLATIONS: i32 = 1;

fn main() {
    let mut safe_count = 0;
    let mut total_count = 0;
    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        let nums = line.split_whitespace().map(|v| v.parse::<i32>().unwrap()).collect::<Vec<_>>();
        let is_safe = _safe(&nums, 0);

        if is_safe {
            safe_count += 1
        }

        // let mut valid = true;
        // let mut last_num = nums[0].parse::<i32>().unwrap();

        // for i in 2..nums.len() {
        //     let num = nums[i].parse::<i32>().unwrap();
        //     let diff = last_num - num;
        //     valid = valid && diff > 0 && diff <= 2;
        //     last_num = num;
        // }

        // if valid {
        //     count += 1
        // }
        // println!("{:?} | {}", nums, is_safe);
        total_count += 1;
    }
    println!("{} safe reports out of {}", safe_count, total_count)
}

fn __safe(nums: &Vec<i32>, violation_count: i32) -> bool {
    let first_num = nums[0];
    let second_num = nums[1];
    let diff = second_num - first_num;
    if diff == 0 || diff.abs() > MAX_INCREASE {
        // if violation_count == MAX_VIOLATIONS {
        //     return false
        // }
        let without_index_0 = &nums[1..].to_vec();
        let without_index_1 = [&nums[0..1], &nums[2..]].concat();
        return _safe(without_index_0, violation_count + 1) || _safe(&without_index_1, violation_count + 1)
    }

    let third_num = nums[2];
    let increasing = diff > 0;
    if is_violation(&second_num, &third_num, increasing) {
        let without_index_0 = &nums[1..].to_vec();
        return _safe(without_index_0, violation_count + 1)
    }
    return true
}

fn _safe(nums: &Vec<i32>, violation_count: i32) -> bool {
    let first_num = nums[0];
    let second_num = nums[1];
    let diff = second_num - first_num;

    if diff == 0 || diff.abs() > MAX_INCREASE {
        if violation_count == MAX_VIOLATIONS {
            return false
        }
        let without_index_0 = &nums[1..].to_vec();
        let without_index_1 = [&nums[0..1], &nums[2..]].concat();
        return _safe(without_index_0, violation_count + 1) || _safe(&without_index_1, violation_count + 1)
    }
    let increasing = diff > 0;

    // if is_violation(&second_num, &third_num, increasing) {
    //     if violation_count == MAX_VIOLATIONS {
    //         return false
    //     }


    // }

    // let mut prev_num = second_num;

    for i in 1..(nums.len() - 1) {
        let prev_num = nums[i];
        let curr_num = nums[i+1];
        if is_violation(&prev_num, &curr_num, increasing) {
            if violation_count == MAX_VIOLATIONS {
                return false
            }

            let without_prev = [&nums[0..(i)], &nums[(i+1)..]].concat();
            let without_curr = [&nums[0..(i+1)], &nums[(i+2)..]].concat();
            let without_first =   &nums[1..].to_vec();
            return
                _safe(&without_prev, violation_count + 1)
                || _safe(&without_curr, violation_count + 1)
                || _safe(without_first, violation_count + 1)
        }
        // prev_num = x
    }

    if violation_count == 1 {
        println!("{:?}", nums);
    }
    return true
}

fn safe(nums: &Vec<i32>, violation_count: i32) -> bool {
    // let mut is_safe = true;

    let first_num = nums[0];
    let second_num = nums[1];
    let diff = second_num - first_num;
    // let mut violation_count = 0;

    if diff == 0 || diff.abs() > MAX_INCREASE {
        if violation_count == MAX_VIOLATIONS {
            return false
        }
        // return safe(&nums[1..].to_vec(), violation_count + 1);
        if (safe(&nums[1..].to_vec(), violation_count + 1)) {
            // println!("first: {:?}", nums);
        }
        
    
        let without_index_1 = [&nums[0..1], &nums[2..]].concat();
        if safe(&without_index_1, violation_count + 1) {
            // println!("second: {:?}", nums);
        }
        // println!("{:?}", nums);
        // println!("{:?}", without_index_1);
        return safe(&nums[1..].to_vec(), violation_count + 1) || safe(&without_index_1, violation_count + 1)
    }

    let mut prev_num = second_num;
    let increasing = diff > 0;
    return safe_with_dir(&nums[1..].to_vec(), first_num, increasing, violation_count)

    // return is_safe_in_direction(&nums[1..].to_vec(), first_num, increasing, violation_count);

    // for i in 2..nums.len() {
    //     let num = nums[i];
    //     let diff = num - prev_num;

    //     if is_violation(&num, &prev_num, increasing) {
    //         if violation_count == 0 {
    //             let mut to_check = vec![num];
    //             to_check.append(&nums[i+1..].to_vec());
    //             return safe(&nums[i+1..].to_vec()) || safe(&to_check)
    //         }
    //         return false
    //     }
    //     prev_num = num;
    // }

    // return true
}

fn safe_with_dir(nums: &Vec<i32>, prev_num: i32, increasing: bool, violation_count: i32) -> bool {
    if nums.len() == 1 {
        return true
    }
    let first_num = nums[0];
    // let second_num = nums[1];
    // let mut violation_count = 0;

    if is_violation(&prev_num, &first_num, increasing) {
        if violation_count == MAX_VIOLATIONS {
            return false
        }
        // if (safe_with_dir(&nums[1..].to_vec(), first_num, increasing, violation_count + 1)) {
        //     println!("first: {:?}", nums);
        // }
        let without_index_1 = [&nums[0..1], &nums[2..]].concat();
        // println!("{:?}", nums);
        // println!("{:?}", without_index_1);
        return safe_with_dir(&nums[1..].to_vec(), prev_num, increasing, violation_count + 1) || safe_with_dir(&without_index_1, prev_num, increasing, violation_count + 1)
    }

    return safe_with_dir(&nums[1..].to_vec(), first_num, increasing, violation_count);
}

fn is_safe_in_direction(nums: &Vec<i32>, start_num: i32, increasing: bool, violation_count: i32) -> bool {
    let mut prev_num = start_num;

    for i in 0..nums.len() {
        let curr_num = nums[i];
        // let diff = curr_num - prev_num;

        if is_violation(&prev_num, &curr_num, increasing) {
            if violation_count == MAX_VIOLATIONS {
                return false
            } else {
            let without_index_1 = [&nums[(i)..(i+1)], &nums[(i+2)..]].concat();
                return is_safe_in_direction(&nums[(i+1)..].to_vec(), prev_num, increasing, violation_count + 1)
                 || is_safe_in_direction(&without_index_1, prev_num, increasing, violation_count + 1)
            }
        }
        prev_num = curr_num;
    }

    return true
}

fn is_violation (num1: &i32, num2: &i32, increasing: bool) -> bool {
    let diff = num2 - num1;
    return diff == 0 || diff.abs() > MAX_INCREASE || (diff < 0 && increasing) || (diff > 0 && !increasing)
}