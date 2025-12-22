use std::fs;
use regex::Regex;

const FILENAME: &str = "./inputs/input";

const A_COST: i64 = 3;
const B_COST: i64 = 1;
const EPSILON: f64 = 0.001;
const PRIZE_OFFSET: i64 = 10000000000000;

fn main() {
    let machines = parse_input();
    let mut total = 0;
    
    for claw in machines {
        let determinant = det(claw.a, claw.b);
        if determinant == 0 {
            let a_presses = claw.prize.0 / claw.a.0;
            let b_presses = claw.prize.0 / claw.a.0;
            if claw.a.0 * a_presses == claw.prize.0 && claw.a.1 * a_presses == claw.prize.1 {
                total += a_presses * A_COST;
            } else if claw.b.0 * b_presses == claw.prize.0 && claw.b.1 * b_presses == claw.prize.1 {
                total += b_presses * B_COST;
            }
        } else {
            let inv_matrix = inverse(claw.a, claw.b);
            let (a_presses, b_presses) = multiply(inv_matrix, claw.prize);

            if almost_int(a_presses) && almost_int(b_presses) {
                let ap_i64 = a_presses.round() as i64;
                let bp_i64 = b_presses.round() as i64;

                total += ap_i64 * A_COST;
                total += bp_i64 * B_COST;
            }
        }
    }
    println!("Total cost: {} tokens", total);

}

fn almost_int (v: f64) -> bool {
    let v_i64 = v.round() as i64;
    return (v_i64 as f64 - v).abs() < EPSILON;
}


fn multiply (inv_matrix: ((f64, f64), (f64, f64)), prize: (i64, i64)) -> (f64, f64) {
    let prize_f64 = (prize.0 as f64, prize.1 as f64);

    return (
        inv_matrix.0.0 * prize_f64.0 + inv_matrix.1.0 * prize_f64.1,
        inv_matrix.0.1 * prize_f64.0 + inv_matrix.1.1 * prize_f64.1
    )

}

fn det (a: (i64, i64), b: (i64, i64)) -> i64 {
    return (a.0 * b.1) - (a.1 * b.0);
}

fn inverse (a: (i64, i64), b: (i64, i64)) -> ((f64, f64), (f64, f64)) {
    let inv_det = 1_f64 / det(a, b) as f64;
    let inv = (
        (b.1 as f64 * inv_det, -a.1 as f64 * inv_det),
        (-b.0 as f64 * inv_det, a.0 as f64 * inv_det),
    );

    return inv;
}

fn parse_input () -> Vec<Claw> {
    let re_x = Regex::new(r"X\+(?<num>[0-9]+)\,").unwrap();
    let re_y = Regex::new(r"Y\+(?<num>[0-9]+)").unwrap();
    let re_xeq = Regex::new(r"X\=(?<num>[0-9]+)\,").unwrap();
    let re_yeq = Regex::new(r"Y\=(?<num>[0-9]+)").unwrap();

    let mut input = vec![];
    let mut button_a = (0, 0);
    let mut button_b = (0, 0);
    let mut prize;

    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        if line.len() == 0 {
            continue;
        }
        let parsed = line.split_whitespace().collect::<Vec<_>>();
        if parsed[1] == "A:" {
            let ax = re_x.captures(parsed[2]).unwrap().name("num").unwrap().as_str().parse::<i64>().unwrap();
            let ay = re_y.captures(parsed[3]).unwrap().name("num").unwrap().as_str().parse::<i64>().unwrap();
            button_a = (ax, ay);
        } else if parsed[1] == "B:" {
            let bx = re_x.captures(parsed[2]).unwrap().name("num").unwrap().as_str().parse::<i64>().unwrap();
            let by = re_y.captures(parsed[3]).unwrap().name("num").unwrap().as_str().parse::<i64>().unwrap();
            button_b = (bx, by);
        } else if parsed[0] == "Prize:" {
            let px = re_xeq.captures(parsed[1]).unwrap().name("num").unwrap().as_str().parse::<i64>().unwrap();
            let py = re_yeq.captures(parsed[2]).unwrap().name("num").unwrap().as_str().parse::<i64>().unwrap();
            prize = (px + PRIZE_OFFSET, py + PRIZE_OFFSET);

            let claw = Claw {
                a: button_a,
                b: button_b,
                prize: prize
            };
            input.push(claw);
        }
    }
    return input;
}

struct Claw {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64)
}