use std::{collections::HashSet, fs};

const FILENAME: &str = "./inputs/input";

const DIRECTIONS: [(i32, i32); 4] = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1)
];
const CORNER_DIRECTIONS: [[(i32, i32); 4]; 4] = [
    [(0, 0), (1, 0), (0, 1), (1, 1)],
    [(0, 0), (1, 0), (0, -1), (1, -1)],
    [(0, 0), (-1, 0), (0, 1), (-1, 1)],
    [(0, 0), (-1, 0), (0, -1), (-1, -1)]
];
const CORNER_POINTS: [(i32, i32); 4] = [
    (1, 1),
    (1, 0),
    (0, 1),
    (0, 0)
];

fn main() {
    let input = parse_input();
    let mut visited_coords: HashSet<(usize, usize)> = HashSet::new();
    let mut regions = vec![];
    let size = (input.len(), input[0].len());

    for x in 0..size.0 {
        for y in 0..size.1 {
            if !visited_coords.contains(&(x, y)) {
                visited_coords.insert((x, y));
                let region_key = input[x][y];

                let region = explore_region(&input, (x as i32, y as i32), size, region_key);
                visited_coords.extend(&region);
                regions.push(region);
            }
        }
    }
    println!("There are {} regions", regions.len());
    let mut total = 0;
    let mut discount_total = 0;

    for region in regions {
        let area = region.len() as i32;
        let perimeter = region_perimiter(&region);
        let sides = region_sides(&region);
        println!("Area: {} | Perimeter: {} | Sides: {}", area, perimeter, sides);
        total += area * perimeter;
        discount_total += area * sides;
    }
    println!("Total cost: {}", total);
    println!("Discount cost: {}", discount_total);
}

fn explore_region(map: &Vec<Vec<char>>, start_coords: (i32, i32), size: (usize, usize), region_key: char) -> HashSet<(usize, usize)> {
    let mut region = HashSet::new();
    region.insert((start_coords.0 as usize, start_coords.1 as usize));
    expand_region(map, start_coords, size, region_key, &mut region);

    return  region;
}

fn expand_region(map: &Vec<Vec<char>>, coords: (i32, i32), size: (usize, usize), region_key: char, region: &mut HashSet<(usize, usize)>) {
    for direction in DIRECTIONS {
        let next_x = coords.0 + direction.0;
        let next_y = coords.1 + direction.1;
        
        if next_x >= 0 && next_x < size.0 as i32 && next_y >= 0 && next_y < size.1 as i32 {
            let coord_key = (next_x as usize, next_y as usize);
            if !region.contains(&coord_key) && map[next_x as usize][next_y as usize] == region_key {
                region.insert(coord_key);
                expand_region(map, (next_x, next_y), size, region_key, region);
            }
        }
    }
}

fn region_perimiter (region: &HashSet<(usize, usize)>) -> i32 {
    let mut perimeter = 0;
    for coords in region {
        for direction in DIRECTIONS {
            let next_x = coords.0 as i32 + direction.0;
            let next_y = coords.1 as i32 + direction.1;

            if next_x < 0 || next_y < 0 {
                perimeter += 1
            } else {
                let coord_key = (next_x as usize, next_y as usize);
                if !region.contains(&coord_key) {
                    perimeter += 1;
                }
            }
        }
    }

    return perimeter;
}

fn is_in_region(region: &HashSet<(usize, usize)>, coords: (i32, i32)) -> bool {
    if coords.0 < 0 || coords.1 < 0 {
        return false
    }
    let coord_key = (coords.0 as usize, coords.1 as usize);
    return region.contains(&coord_key);
}

fn region_sides (region: &HashSet<(usize, usize)>) -> i32 {
    let mut vertices = HashSet::new();
    let mut double_corners = HashSet::new();
    for coords in region {
        for (i, corner_dir) in CORNER_DIRECTIONS.iter().enumerate() {
            let mut in_region = 0;
            for direction in corner_dir {
                let next_x = coords.0 as i32 + direction.0;
                let next_y = coords.1 as i32 + direction.1;

                if is_in_region(region, (next_x, next_y)) {
                    in_region += 1;
                }
            }
            let corner_coords = (
                coords.0 as i32 + CORNER_POINTS[i].0,
                coords.1 as i32 + CORNER_POINTS[i].1
            );
            match in_region {
                1 => vertices.insert(corner_coords),
                2 => {
                    if is_in_region(region, (coords.0 as i32 + corner_dir[3].0, coords.1 as i32 + corner_dir[3].1)) {
                        vertices.insert(corner_coords);
                        double_corners.insert(corner_coords);
                    }
                    false
                }
                3 => vertices.insert(corner_coords),
                _ => false
            };
        }
    }
    return (vertices.len() + double_corners.len()) as i32;
}

fn parse_input () -> Vec<Vec<char>> {
    let mut input = vec![];
    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        input.push(line.chars().collect())
    }

    return input
}