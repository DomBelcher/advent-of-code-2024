use std::fs;
const RADIX: u32 = 10;

const FILENAME: &str = "./inputs/input";

fn main() {
    println!("parsing");
    let disk_map = parse_input();
    let (block_ranges, free_spaces) = parse_input_();
    if disk_map.len() != block_ranges.len() + free_spaces.len() {
        println!("{}, {}, {}", disk_map.len(), block_ranges.len(), free_spaces.len());
        panic!("oh no");
    }
    println!("blocking");
    // let blocks = map_to_blocks(&disk_map);
    let blocks = map_to_(&disk_map);
    let cs = checksum(&blocks);
    println!("{}", cs);
    let filesystem = map_to_filesystem(&block_ranges, &free_spaces);
    if filesystem.len() != blocks.len() {
        println!("{}, {}", filesystem.len(), blocks.len());
        panic!("oh no");
    }
    println!("Checksum: {}", checksum(&filesystem));

    println!("{}", blocks.len());
    println!("defragging");
    let defragged = defrag_(&blocks);
    let checksum_1 = checksum(&defragged);
    println!("Checksum: {}", checksum_1);
    println!("second defragging");
    let defragged_2 = defrag_2(&block_ranges, &free_spaces);
    let checksum_2 = checksum(&defragged_2);
    println!("Checksum: {}", checksum_2);
    // println!("{:?}", defragged)
    // let defragged = defrag(&blocks);
}

fn parse_input () -> Vec<u32> {
    let mut input = vec![];
    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        input.append(&mut line.chars().map(|c| char::to_digit(c, RADIX).unwrap()).collect::<Vec<u32>>());   
    }
    return input
}

fn map_to_ (map: &Vec<u32>) -> Vec<i32> {
    let mut blocks = vec![];

    let mut i = 0;
    while i < map.len() {
        let c;
        if i % 2 == 0 {
            c = (i / 2) as i32;
        } else {
            c = -1;
        }
        for j in 0..map[i] {
            blocks.push(c);
        }
        i += 1;
    }

    return blocks;
}

fn defrag_(blocks: &Vec<i32>) -> Vec<i32> {
    let mut defragged = vec![];
    let mut s: usize = 0;
    let mut e = blocks.len() - 1;

    while s < e {
        if blocks[s] != -1 {
            defragged.insert(s, blocks[s]);
            s += 1;
            // defragged.push(value);
            continue
        }
        if blocks[e] == -1 {
            defragged.push(-1);
            e -= 1;
            continue
        }
        defragged.insert(s, blocks[e]);
        defragged.push(-1);
        s += 1;
        e -= 1;
    }

    return defragged;
}

fn parse_input_() -> (Vec<FileBlock>, Vec<EmptyBlock>) {
    let mut block_ranges = vec![];
    let mut free_spaces = vec![];
    
    let mut index = 0;
    let mut pos = 0;
    // let mut 
    for line in fs::read_to_string("./input").unwrap().lines() {
        for c in line.chars() {
            let block_size = char::to_digit(c, RADIX).unwrap() as usize;
            if index % 2 == 0 {
                block_ranges.push(FileBlock {
                    index: index,
                    start: pos,
                    size: block_size,
                    id: index / 2
                });
            } else {
                free_spaces.push(EmptyBlock {
                    index: index,
                    start: pos,
                    size: block_size,
                })
            }

            pos += block_size;
            index += 1;
        }
    }

    return (block_ranges, free_spaces);
}

fn map_to_filesystem (block_ranges: &Vec<FileBlock>, free_spaces: &Vec<EmptyBlock>) -> Vec<i32> {
    let mut filesystem = vec![];

    let mut b_idx = 0;
    let mut f_idx = 0;
    while b_idx < block_ranges.len() || f_idx < free_spaces.len() {
        let br = &block_ranges[b_idx];
        let fr = if f_idx < free_spaces.len() { &free_spaces[f_idx] } else { &EmptyBlock{ index: br.index + 1, size: 0, start: br.start + br.size} };

        if br.index == fr.index && fr.size != 0 {
            panic!("oh no");
        }

        if br.index < fr.index {
            for _ in 0..br.size {
                filesystem.push(br.id as i32);
            }
            b_idx += 1
        } else {
            for _ in 0..fr.size {
                filesystem.push(-1);
            }
            f_idx += 1
        }
    } 

    return filesystem;
}

fn defrag_2 (block_ranges: &Vec<FileBlock>, free_spaces: &Vec<EmptyBlock>) -> Vec<i32> {
    let mut defragged = map_to_filesystem(block_ranges, free_spaces);
    let mut d_block_ranges = block_ranges.clone();
    let mut d_free_spaces = free_spaces.clone();

    let mut b = d_block_ranges.len() - 1;
    loop {
        let br = &d_block_ranges[b];
        let mut fs_idx = 0;
        while fs_idx < d_free_spaces.len() {
            let free_space = &d_free_spaces[fs_idx];

            if free_space.size >= br.size && free_space.start < br.start {
                // println!("{}, {}", free_space.index, br.index);
                for i in 0..br.size {
                    defragged[free_space.start + i] = br.id as i32;
                    defragged[br.start + i] = -1;
                }

                d_free_spaces[fs_idx].size -= br.size;
                d_free_spaces[fs_idx].start += br.size;

                break;

                // let mut insert_index = 0;
                // for (idx, b_) in d_block_ranges.iter().enumerate() {
                //     if f_.index > b_.index {
                //         insert_index = idx;
                //     }
                // }
            }
            fs_idx += 1;
        }
        if b == 0 {
            break;
        }
        b -= 1;
    }


    return defragged;
}

fn checksum (blocks: &Vec<i32>) -> i64 {
    let mut total = 0;
    for (index, &val) in blocks.iter().enumerate() {
        if val != -1 {
            total += index as i64 * val as i64;
        }
    }
    return total
}

struct FileBlock {
    index: usize,
    size: usize,
    id: usize,
    start: usize
}

struct EmptyBlock {
    index: usize,
    size: usize,
    start: usize
}

impl Clone for FileBlock {
    fn clone (&self) -> FileBlock {
        return FileBlock { index: self.index, size: self.size, id: self.id, start: self.start }
    }
}

impl Clone for EmptyBlock {
    fn clone (&self) -> EmptyBlock {
        return EmptyBlock { index: self.index, size: self.size,  start: self.start }
    }
}


// impl DiskBlock {
//     fn clone (&self) -> DiskBlock {
//         return DiskBlock {
//             file_id: self.file_id,
//             file_block_size: self.file_block_size,
//             empty_space: self.empty_space
//         }
//     }
// }