use std::fs;

// recursively modify the input
fn mod_input_p1(input_parsed: Vec<i64>, mut count: usize, max_iter: usize) -> Vec<i64> {
    // the round started
    println!("Count: {:?}", count);
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
        return moded;
    }
    mod_input_p1(moded, count, max_iter)
}

pub fn day11() {
    let raw_input = fs::read_to_string("./inputs/day11.txt").expect("cant read file");
    let input_parsed = raw_input
        .trim()
        .split(" ")
        .filter(|y| y.len() > 0)
        .map(|x| x.parse::<i64>().expect("cant parse to i64"))
        .collect::<Vec<i64>>();
    let moded = mod_input_p1(input_parsed, 0, 24);
    println!("{:?}", moded.len());
}
