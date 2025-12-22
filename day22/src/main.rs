use std::{collections::{HashMap}, fs};
use std::time::Instant;

const FILENAME: &str = "./inputs/input.txt";
const MODULO: i64 = 16777216;
const N_STEPS: usize = 2000;

fn main() {
    let start_time = Instant::now();
    let secrets = parse_input();
    // let secrets = vec![123];

    let mut part1_total = 0;

    let mut sequences: HashMap<Sequence, i16> = HashMap::new();


    for secret in secrets {
        // println!("Start secret: {}", secret);
        let mut result = secret;

        let mut last_price = (result % 10) as i16;
        let mut partial_sequence = vec![];
        let mut last_sequence = Sequence { values: (0,0,0,0) };

        let mut sequence_prices: HashMap<Sequence, i16> = HashMap::new();

        for i in 0..N_STEPS {
            let price = (result % 10) as i16;
            let change = (price - last_price) as i8;

            if i < 3 {
                partial_sequence.push(change);
            } else if i == 3 {
                last_sequence = Sequence { values: (partial_sequence[0], partial_sequence[1], partial_sequence[2], change) }
            } else if i > 3 {
                last_sequence = last_sequence.for_next_value(change);
            }

            if i >= 3 {
                if !sequence_prices.contains_key(&last_sequence) {
                    sequence_prices.insert(last_sequence.clone(), price);
                }
            }
            result = next_secret(result);
            last_price = price;
        }

        for (seq, price) in sequence_prices {
            let sequence_value_ref = sequences.entry(seq).or_insert(0);
            *sequence_value_ref += price;
        }
        part1_total += result;
    }

    println!("Part 1: {}", part1_total);

    let best_sequence =sequences.iter().max_by(|(_, v1), (_, v2)| v1.cmp(v2)).unwrap();
    println!("Best price: {}", best_sequence.1);
    println!("sequence: {:?}", best_sequence.0);

    println!("Ran in {}ms", start_time.elapsed().as_millis());
}

fn next_secret (secret: i64) -> i64 {
    let step1 = prune(mix(secret, secret * 64));
    // println!("Step 1: {}", step1);
    let step2 = prune(mix(step1, step1 / 32));
    // println!("Step 2: {}", step2);
    let step3 = prune(mix(step2 , step2 * 2048));
    // println!("Step 3: {}", step3);
    return step3;
}

fn mix (n1: i64, n2: i64) -> i64 {
    // println!("xor: {}", n1 ^ n2);
    return n1 ^ n2;
}

fn prune (n: i64) -> i64 {
    // println!("prune: {}", n % MODULO)
    return n % MODULO;
}

fn parse_input () -> Vec<i64> {
    let mut secrets = vec![];

    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        secrets.push(line.parse::<i64>().unwrap())
    }

    return secrets;
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Sequence {
    values: (i8, i8, i8, i8)
}

impl Sequence {
    fn for_next_value (&self, next_value: i8) -> Sequence {
        return Sequence {
            values: (self.values.1, self.values.2, self.values.3, next_value )
        };
    }
}