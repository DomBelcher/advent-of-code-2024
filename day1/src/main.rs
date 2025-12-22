use std::{collections::HashMap, fs};

const FILENAME: &str = "./inputs/input";

fn main() {
    let (mut nums1, mut nums2) = read_input();
    nums1.sort();
    nums2.sort();

    if nums1.len() != nums2.len() {
        panic!("oh no");
    }

    let mut total = 0;

    for i in 0..nums1.len() {
        total += (nums1[i] - nums2[i]).abs();
    }

    println!("{total}");

    let mut counts = HashMap::<i32,i32>::new();

    for i in 0..nums2.len() {
        counts
            .entry(nums2[i])
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }

    let mut similarity_score = 0;

    for i in 0..nums2.len() {
        if counts.contains_key(&nums1[i]) {
            similarity_score += nums1[i] * counts.get(&nums1[i]).unwrap()
        }
    }

    println!("{similarity_score}");
}

fn read_input () -> (Vec<i32>, Vec<i32>) {
    let mut list1 = Vec::<i32>::new();
    let mut list2 = Vec::<i32>::new();

    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        let nums = line.split_whitespace().collect::<Vec<_>>();
        list1.push(nums[0].parse::<i32>().unwrap());
        list2.push(nums[1].parse::<i32>().unwrap());
    }
    return (list1, list2)
}
