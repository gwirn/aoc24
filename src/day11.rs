use std::{collections::HashMap, fs};

/*
more efficient way than for part 1
*/
fn mod_input_p2(input_parsed: Vec<i64>, blinks: usize) -> usize {
    // stones and their appearance count
    let mut stones: HashMap<i64, usize> = HashMap::new();
    // insert initial stones
    for i in &input_parsed {
        stones.insert(*i, 1);
    }
    for _ in 0..blinks {
        // stones and their appearance count
        let mut n_stones: HashMap<i64, usize> = HashMap::new();
        // change all (unique) existing stones according to the rules and update their count
        for s in stones.keys() {
            let s_string = format!("{}", s);
            if *s == 0 {
                *n_stones.entry(1 as i64).or_insert(0) += stones
                    .get(&(0 as i64))
                    .expect(&format!("Never seen stone [{}]", s));
            } else if s_string.len() % 2 == 0 {
                let half_idx = (s_string.len() / 2) as usize;
                let s_split = s_string.split_at(half_idx);
                let left = s_split.0.parse::<i64>().expect("cant parse to i64");
                let right = s_split.1.parse::<i64>().expect("cant parse to i64");
                *n_stones.entry(left).or_insert(0) +=
                    stones.get(s).expect(&format!("Never seen stone [{}]", s));
                *n_stones.entry(right).or_insert(0) +=
                    stones.get(s).expect(&format!("Never seen stone [{}]", s));
            } else {
                *n_stones.entry(s * 2024).or_insert(0) +=
                    stones.get(s).expect(&format!("Never seen stone [{}]", s));
            }
        }
        // update the stones appearance map
        stones = n_stones;
    }
    stones.values().sum::<usize>()
}

// recursively modify the input
fn mod_input_p1(input_parsed: &Vec<i64>, mut count: usize, max_iter: usize) -> usize {
    // new vec
    let mut moded: Vec<i64> = vec![];
    for i in input_parsed.iter() {
        // process based on rules
        let i_string = format!("{}", i);
        if *i == 0 {
            moded.push(1)
        } else if i_string.len() % 2 == 0 {
            let half_idx = (i_string.len() / 2) as usize;
            let i_split = i_string.split_at(half_idx);
            moded.push(i_split.0.parse::<i64>().expect("cant parse to i64"));
            moded.push(i_split.1.parse::<i64>().expect("cant parse to i64"));
        } else {
            moded.push(i * 2024)
        }
    }
    count += 1;
    // stop if number of iterations is reached
    if count > max_iter {
        return moded.len();
    }
    mod_input_p1(&moded, count, max_iter)
}

pub fn day11() {
    let raw_input = fs::read_to_string("./inputs/day11.txt").expect("cant read file");
    let input_parsed = raw_input
        .trim()
        .split(" ")
        .filter(|y| y.len() > 0)
        .map(|x| x.parse::<i64>().expect("cant parse to i64"))
        .collect::<Vec<i64>>();
    let moded = mod_input_p1(&input_parsed, 0, 24);
    let moded2 = mod_input_p2(input_parsed, 75);
    println!("Part 1: {:?}", moded);
    println!("Part 2: {:?}", moded2);
}
