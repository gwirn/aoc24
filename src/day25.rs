use std::fs;

const PIN: char = '#';
const LOCK: &str = "#####";

/*
check whether a key lock combo is not overlapping
*/
fn check_combo(k: &Vec<usize>, l: &Vec<usize>) -> bool {
    for (i, j) in k.iter().zip(l) {
        if i + j > 5 {
            return false;
        }
    }
    true
}
pub fn day25() {
    let raw_input = fs::read_to_string("./inputs/day25.txt").expect("cant read file");
    let input_split = raw_input.split("\n\n").collect::<Vec<&str>>();
    // keys and locks as converted vec
    let mut mkeys: Vec<Vec<usize>> = vec![];
    let mut mlock: Vec<Vec<usize>> = vec![];
    for i in input_split.iter().filter(|x| x.len() > 0) {
        let mut mech_vec: Vec<usize> = vec![0; 5];
        for (cj, j) in i.split("\n").filter(|y| y.len() > 0).enumerate() {
            // only grab key/ lock data
            if cj == 0 || cj == 6 {
                continue;
            }
            for (ck, k) in j.chars().enumerate() {
                if k == PIN {
                    mech_vec[ck] += 1;
                }
            }
        }
        // update key or lock data
        if i.starts_with(LOCK) {
            mlock.push(mech_vec);
        } else {
            mkeys.push(mech_vec);
        }
    }
    // test every key against all locks
    let valid: usize = mkeys
        .iter()
        .map(|k| {
            mlock
                .iter()
                .map(|l| if check_combo(k, l) { 1 } else { 0 })
                .sum::<usize>()
        })
        .sum();
    println!("{:?}", valid);
}
