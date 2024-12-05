use std::fs;
pub fn day5_1() {
    // PART 1
    // read input
    let raw = fs::read_to_string("./inputs/day5.txt").expect("cant open input file");
    // split input by rules and page orderings
    let rules_order_split = raw
        .split("\n\n")
        .filter(|x| x.len() > 0)
        .collect::<Vec<_>>();
    // get rules
    let rules = rules_order_split[0].split("\n").collect::<Vec<_>>();
    let cor_pages_middle_sum = rules_order_split[1]
        .split("\n")
        .filter(|x| x.len() > 0)
        .into_iter()
        .map(|x| {
            // pages as own element
            let x_split = x.split(",").collect::<Vec<_>>();
            // number of pages in current ordering
            let n_pages = x_split.len();
            // test first against all, then second against all but first, third against all but
            // first and second ... and count correct ordered orderings
            let mut idx_count = 0;
            let mut correct_pages = 0;
            for _ in 0..n_pages {
                let mut correct_order = 0;
                for i in idx_count + 1..n_pages {
                    // check if page x_split[idx_count] and x_split[i] obey the rules
                    let test_rule = format!("{}|{}", x_split[idx_count], x_split[i]);
                    if rules.contains(&test_rule.as_str()) {
                        correct_order += 1;
                    }
                }
                // idx_count against i all obey the rules
                if correct_order == n_pages - idx_count - 1 {
                    correct_pages += 1
                }
                idx_count += 1;
            }
            // if all pages obey the rules the number of correct ones is the number of pages
            if correct_pages == n_pages {
                // get middle item
                x_split[(n_pages as f32 / 2.0).floor() as usize]
                    .parse::<i32>()
                    .expect("cant parse to i32")
            } else {
                0
            }
        })
        .sum::<i32>();
    println!("{}", cor_pages_middle_sum);
}
pub fn day5_2() {
    // same as PART 1
    let raw = fs::read_to_string("./inputs/day5.txt").expect("cant open input file");
    let rules_order_split = raw
        .split("\n\n")
        .filter(|x| x.len() > 0)
        .collect::<Vec<_>>();
    let rules = rules_order_split[0].split("\n").collect::<Vec<_>>();
    let incor_pages_middle_sum = rules_order_split[1]
        .split("\n")
        .filter(|x| x.len() > 0)
        .into_iter()
        .map(|x| {
            let mut x_split = x.split(",").collect::<Vec<_>>();
            let n_pages = x_split.len();
            let mut idx_count = 0;
            // flag if incorrect order was encountered
            let mut incorrect_encountered = false;
            for _ in 0..n_pages {
                for i in idx_count + 1..n_pages {
                    let test_rule = format!("{}|{}", x_split[idx_count], x_split[i]);
                    // if not obeying the rule - reorder and enforce the rule
                    if !rules.contains(&test_rule.as_str()) {
                        incorrect_encountered = true;
                        x_split.swap(idx_count, i);
                    }
                }
                idx_count += 1;
            }
            // count if a wrong order was encountered
            if incorrect_encountered {
                x_split[(n_pages as f32 / 2.0).floor() as usize]
                    .parse::<i32>()
                    .expect("cant parse to i32")
            } else {
                0
            }
        })
        .sum::<i32>();
    println!("{}", incor_pages_middle_sum);
}
