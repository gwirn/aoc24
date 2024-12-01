use std::collections::HashMap;
use std::fs;
// check how often a number appears in a vec
fn n_appear(probe: i32, num_vec: &Vec<i32>) -> i32 {
    num_vec
        .iter()
        .map(|x| if *x == probe { 1 } else { 0 })
        .sum::<i32>()
}
pub fn day1() {
    // read input
    let raw = fs::read_to_string("./inputs/day1.txt").expect("cant open input file");
    // split input and convert to i32
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];
    for i in raw.split("\n").filter(|x| x.len() > 0) {
        for (cj, j) in i.split(" ").filter(|x| x.len() > 0).enumerate() {
            match cj {
                0 => left.push(j.parse::<i32>().expect("cant parse to i32")),
                1 => right.push(j.parse::<i32>().expect("cant parse to i32")),
                _ => {}
            }
        }
    }
    left.sort();
    right.sort();
    // PART 1
    // calc absolute distance between each entry of left and right
    let dist = left
        .iter()
        .zip(right.clone())
        .map(|(l, r)| (l - r).abs())
        .sum::<i32>();
    println!("Sum distances: {}", dist);

    // PART 2
    // map for already calculated similarity scores
    let mut already_checked: HashMap<i32, i32> = HashMap::new();
    let mut similarity_score = 0;
    // either calculate the similarity and add to map or get it from map
    for i in left {
        match already_checked.get(&i) {
            Some(&v) => similarity_score += i * v,
            None => {
                let n_i = n_appear(i, &right);
                already_checked.insert(i, n_i);
                similarity_score += i * n_i
            }
        };
    }
    println!("Similarity score: {}", similarity_score);
}
