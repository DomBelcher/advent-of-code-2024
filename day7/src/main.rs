use std::fs;

const FILENAME: &str = "./inputs/input";

const N_OPS:i64 = 3;
const TEN:i64 = 10;

fn main() {
    let mut count = 0;
    let mut total:i64 = 0;

    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        let params = line.split(": ").collect::<Vec<_>>();
        let result = params[0].parse::<i64>().unwrap();
        let operands = params[1].split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>();

        let n_operators = operands.len() - 1;

        for i in 0..((N_OPS).pow(n_operators as u32)) {
            let operators: Vec<usize> = get_operators(i, n_operators);
            if evaluate_eq(&operands, operators, result) {
                count += 1;
                total += result;
                break
            }
        }
    }
    println!("Possible sums: {}", count);
    println!("Total: {}", total);
}

fn get_operators(i: i64, n_digits: usize) -> Vec<usize> {
    let mut n = i;
    let mut operators:Vec<usize> = (0..n_digits).map(|_| 0).collect();
    
    let mut j = 0;
    while n > 0 {
        // println!("{}", n);
        // operators.push((n % 2) as usize);
        operators[j] = (n % N_OPS) as usize;
        n = n / N_OPS;
        j += 1;
    }
    // println!("{:?}", operators);

    return operators;
}

fn evaluate_eq (operands: &Vec<i64>, operators: Vec<usize>, expected_result: i64) -> bool {
    let mut total:i64 = operands[0];
    for i in 0..operators.len() {
        if operators[i] == 0 {
            total = total * operands[i+1];
        } else if operators[i] == 1 {
            total = total + operands[i+1];
        } else if operators[i] == 2 {
            let v = operands[i+1];
            let n_digits = count_digits(v);
            total = total * TEN.pow(n_digits) + v;
        }
        if total > expected_result {
            return false;
        }
    }
    if total == expected_result {
        return true;
    }
    return false
}

fn count_digits(n: i64) -> u32 {
    if n < 10 {
        return 1;
    }
    let mut i = 1;
    while (TEN.pow(i)) <= n {
        i += 1;
    }
    return i;
}