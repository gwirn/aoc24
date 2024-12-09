use core::panic;
use std::{fs, usize};

// PART 1
pub fn day9_1() {
    let raw_input = fs::read_to_string("./inputs/day9.txt").expect("cant read file");
    //  split input and parse to i64
    let input_split = raw_input
        .trim()
        .split("")
        .filter(|y| y.len() > 0)
        .map(|x| x.parse::<i64>().expect("cant parse to i64"))
        .collect::<Vec<i64>>();
    if input_split.len() % 2 == 0 {
        panic!("invalid input")
    }
    // create current disc representation
    let mut disc: Vec<i64> = vec![];
    let mut id_counter = 0;
    // use `-1` as dot representation
    for (ci, i) in input_split.iter().enumerate() {
        for _ in 0..*i {
            if ci % 2 == 0 {
                disc.push(id_counter);
            } else {
                disc.push(-1)
            }
        }
        if ci % 2 == 0 {
            id_counter += 1;
        }
    }
    // size of the disk
    let n_elem = disc.len();
    loop {
        // find first gap occurrence
        let gaps = disc.iter().position(|&x| x < 0).expect("cant find gap");
        // find the last entry on disk that is not an empty space
        let mut last_block = 0;
        for ci in (0..n_elem).rev() {
            if disc[ci] >= 0 {
                last_block = ci;
                break;
            }
        }
        // if the first gap if behind the last file block the end is reached
        if gaps > last_block {
            break;
        };
        // exchange file block with empty space
        disc.swap(gaps, last_block);
    }
    // calculate the file checksum
    let mut check_sum = 0;
    for (ci, i) in disc.iter().enumerate() {
        if *i < 0 {
            break;
        }
        check_sum += ci as i64 * *i
    }
    println!("Checksum: {}", check_sum);
}

// PART 2
pub fn day9() {
    let raw_input = fs::read_to_string("./inputs/day9.txt").expect("cant read file");
    //  split input and parse to i64
    let input_split = raw_input
        .trim()
        .split("")
        .filter(|y| y.len() > 0)
        .map(|x| x.parse::<i64>().expect("cant parse to i64"))
        .collect::<Vec<i64>>();
    if input_split.len() % 2 == 0 {
        panic!("invalid input")
    }
    // create current disc representation
    let mut disc: Vec<i64> = vec![];
    let mut id_counter = 0;
    for (ci, i) in input_split.iter().enumerate() {
        for _ in 0..*i {
            if ci % 2 == 0 {
                disc.push(id_counter);
            } else {
                disc.push(-1)
            }
        }
        if ci % 2 == 0 {
            id_counter += 1;
        }
    }
    let n_elem = disc.len() as i64;

    // files that are to large to fit into one contiguous free space
    let mut to_large: Vec<i64> = vec![];
    // whether something changed from the previous iteration
    loop {
        // start and end of a file block
        let mut block_start_end: (i64, i64) = (0, 0);
        // current file
        let mut current_block = -2;
        // find last block of files that is not to big to move
        for ce in (0..n_elem).rev() {
            let test_elem = disc[ce as usize];
            if !to_large.contains(&test_elem) {
                if test_elem >= 0 && current_block == -2 {
                    current_block = test_elem;
                    block_start_end.1 = ce + 1;
                }
                if test_elem != current_block && current_block != -2 {
                    block_start_end.0 = ce + 1;
                    break;
                }
            }
        }
        // needed space for the file block
        let needed_space = block_start_end.1 - block_start_end.0;

        // check all free spaces if one if big enough
        let mut space_start: i64 = n_elem;
        let mut space_matched = false;
        let mut space_started = false;
        for cs in 0..n_elem {
            let test_space = disc[cs as usize];
            if test_space < 0 {
                if !space_started {
                    space_start = cs;
                }
                space_started = true;
            }
            // found big enough free space
            if test_space >= 0 && space_start < n_elem && cs - space_start >= needed_space {
                space_matched = true;
                break;
            }
            // encountered end of free space
            if test_space >= 0 {
                space_started = false;
                space_matched = false;
                space_start = n_elem
            }
        }
        let mut tolarge_bool = false;
        let mut disc_mut = false;
        // not enough space or big enough space is behind current location
        if !space_matched || space_start > block_start_end.0 {
            if current_block > 0 {
                to_large.push(current_block);
                tolarge_bool = true;
            }
        } else {
            // move file to free memory
            let mut space_filling_idx = 0;
            for cb in block_start_end.0..block_start_end.1 {
                disc.swap(cb as usize, (space_start + space_filling_idx) as usize);
                space_filling_idx += 1;
            }
            disc_mut = true;
        }
        // end reached if no updates are possible
        if !tolarge_bool && !disc_mut {
            break;
        }
    }
    // calculate checksum
    let mut check_sum = 0;
    for (ci, i) in disc.iter().enumerate() {
        if *i > 0 {
            check_sum += ci as i64 * *i
        }
    }
    println!("{}", check_sum);
}
