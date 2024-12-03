use regex::Regex;
use std::fs;
/*
// PART 1
pub fn day3() {
    // read data
    let raw = fs::read_to_string("./inputs/day3.txt").expect("cant read line");
    // build regex to capture `mul(NNN,NNN)`
    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").expect("cant build regex");
    let s = re
        .find_iter(&raw)
        .map(|m| {
            let m_pre = m
                .as_str()
                .replace("(", "")
                .replace(")", "")
                .replace("mul", "");
            // convert to i32 and multiply
            let m_split = m_pre.split(",").collect::<Vec<_>>();
            m_split[0].parse::<i32>().expect("cant parse to i32")
                * m_split[1].parse::<i32>().expect("cant parse to i32")
        })
        .sum::<i32>();
    println!("Sum {:?}", s);
}
*/
// find the closest item in `in_vec` in front of probe
fn closest(in_vec: &Vec<i32>, probe: i32) -> i32 {
    let mut closest_idx: usize = 0;
    let mut closest_dist = f32::NEG_INFINITY as i32;
    for (ci, i) in in_vec.iter().enumerate() {
        let dist = i - probe;
        // stop if behind probe
        if dist > 0 {
            break;
        }
        if dist > closest_dist {
            closest_dist = dist;
            closest_idx = ci;
        }
    }
    in_vec[closest_idx]
}
pub fn day3() {
    // read data
    let raw = fs::read_to_string("./inputs/day3.txt").expect("cant read line");
    // build regex to capture `mul(NNN,NNN)`
    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").expect("cant build regex");
    // regex for `do()`
    let start_do = Regex::new(r"do\(\)").expect("cant build regex");
    // regex for `don't();
    let start_dont = Regex::new(r"don't\(\)").expect("cant build regex");
    // where all the `do()` start
    let dos = start_do
        .find_iter(&raw)
        .map(|x| x.start() as i32)
        .collect::<Vec<i32>>();
    // where all the `don't()` start
    let donts = start_dont
        .find_iter(&raw)
        .map(|x| x.start() as i32)
        .collect::<Vec<i32>>();
    let xm = re
        .find_iter(&raw)
        .map(|m| {
            let m_start = m.start() as i32;
            let closest_do = closest(&dos, m_start);
            let closest_dont = closest(&donts, m_start);
            // is enabled if no `don't()` was in front of `m` or if a `do()` was closer and in
            // front of `m`
            if m_start < closest_dont || (closest_do > closest_dont && closest_do < m_start) {
                let m_pre = m
                    .as_str()
                    .replace("(", "")
                    .replace(")", "")
                    .replace("mul", "");
                let m_split = m_pre.split(",").collect::<Vec<_>>();
                m_split[0].parse::<i32>().expect("cant parse to i32")
                    * m_split[1].parse::<i32>().expect("cant parse to i32")
            } else {
                0
            }
        })
        .sum::<i32>();
    println!("Sum {:?}", xm);
}
