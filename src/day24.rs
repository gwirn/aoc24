use std::{collections::HashMap, fs, isize};
pub fn day24() {
    let raw_input = fs::read_to_string("./inputs/day24.txt").expect("cant read file");
    let raw_split = raw_input.split("\n\n").collect::<Vec<&str>>();
    // gates have an output assigned
    let mut assigns: HashMap<&str, bool> = raw_split[0]
        .split("\n")
        .filter(|y| y.len() > 0)
        .map(|x| (&x[..3], x.ends_with("1")))
        .collect();
    // convert input to usable format
    let mut data: Vec<(&str, &str, &str, &str)> = vec![];
    for i in raw_split[1].split("\n").filter(|y| y.len() > 0) {
        let i_split = i.split(" ").collect::<Vec<&str>>();
        let k0 = i_split[0];
        let operator = i_split[1];
        let k1 = i_split[2];
        let k_res = i_split[4];
        data.push((k0, operator, k1, k_res));
    }
    let mut n_assigns = 0;
    loop {
        let tmp_assigns = n_assigns.clone();
        'inner: for i in &data {
            // key already calculated
            if assigns.contains_key(&i.3) {
                continue 'inner;
            }
            // get both gate values
            let k0o = assigns.get(i.0);
            let k1o = assigns.get(i.2);
            // skip if one or both are not assigned
            if k0o.is_none() || k1o.is_none() {
                continue 'inner;
            }
            let Some(k0v) = k0o else {
                continue 'inner;
            };
            let Some(k1v) = k1o else {
                continue 'inner;
            };
            // apply operator
            let calc_res = match i.1 {
                "AND" => *k0v && *k1v,
                "OR" => *k0v || *k1v,
                "XOR" => *k0v ^ *k1v,
                _ => panic!("unknown operator"),
            };
            // insert in hashmap
            assigns.insert(&i.3, calc_res);
            n_assigns += 1;
        }
        // if number of assignments does not change all are done
        if tmp_assigns == n_assigns {
            break;
        }
    }
    //  need ordering and then convert binary to int
    let mut assign_sorted = assigns
        .iter()
        .filter(|(k, _)| k.starts_with("z"))
        .collect::<Vec<_>>();
    assign_sorted.sort_by_key(|x| x.0);
    // convert to string to convert to num
    let b_vec = assign_sorted
        .iter()
        .rev()
        .map(|(_, x)| if **x { "1" } else { "0" })
        .collect::<Vec<_>>()
        .join("");

    println!(
        "Num: {:?}",
        isize::from_str_radix(&b_vec, 2).expect("unable to convert to decimal")
    );
}
