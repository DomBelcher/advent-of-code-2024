use std::{collections::{HashMap, HashSet}, fs};

const FILENAME: &str = "./inputs/input";

fn main() {
    let (antenna_locs, width, height) = parse_input();
    let mut antinode_locs: HashSet<(i32, i32)> = HashSet::new();
    
    for (freq, locs) in antenna_locs {
        let n_locs = locs.len();
        let loc_vec = locs.into_iter().collect::<Vec<(i32, i32)>>();

        for i in 0..n_locs {
            for j in 0..n_locs {
                if i == j {
                    continue
                }
                let mut step = 0;
                loop {
                    let antinode = get_antinode(loc_vec.get(i).unwrap(), loc_vec.get(j).unwrap(), step);
                    if antinode.0 < 0 || antinode.0 >= width || antinode.1 < 0 || antinode.1 >= height {
                        // continue
                        break
                    }
                    antinode_locs.insert(antinode);
                    step += 1;
                }
            }
        }
    }

    println!("There are {} antinodes", antinode_locs.len())
}

fn get_antinode(a1: &(i32, i32), a2: &(i32, i32), step: i32) -> (i32, i32) {
    let mut d = (a1.0 - a2.0, a1.1 - a2.1);

    if d.0 % 2 == 0 && d.1 % 2 == 0 {
        d = (d.0 / 2, d.1 / 2);
    }
    let loc = (a1.0 + (d.0 * step), a1.1 + (d.1 * step));

    return loc
}



fn parse_input() -> (HashMap<char, HashSet<(i32, i32)>>, i32, i32) {
    let mut antenna_locs = HashMap::new();
    // let mut antenna_freqs = HashMap::new();

    let mut x = 0;
    let mut width = 0;
    let mut height = 0;

    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        let mut y = 0;

        for c in line.chars() {
            if c.is_alphanumeric() {
                if !antenna_locs.contains_key(&c) {
                    antenna_locs.insert(c, HashSet::new());
                }
                antenna_locs.get_mut(&c).unwrap().insert((x, y));
            }
            y += 1;
        }
        height = y;
        x += 1;
    }
    width = x;
    return (antenna_locs, width, height)
}