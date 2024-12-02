use std::fs;
// check if a vec contains only increasing numbers and the differences between them are bigger than
// one and smaller or equal to `max_jump`
fn increasing(v: &Vec<i32>, max_jump: i32) -> bool {
    let n_items = v.len();
    let mut meets_cond = false;
    for i in 0..n_items - 1 {
        if v[i + 1] > v[i] && v[i + 1] - v[i] >= 1 && v[i + 1] - v[i] <= max_jump {
            meets_cond = true;
        } else {
            meets_cond = false;
            break;
        }
    }
    meets_cond
}
// check if a vec contains only decreasing numbers and the differences between them are bigger than
// one and smaller or equal to `max_jump`
fn decreasing(v: &Vec<i32>, max_jump: i32) -> bool {
    let n_items = v.len();
    let mut meets_cond = false;
    for i in 0..n_items - 1 {
        if v[i] > v[i + 1] && v[i] - v[i + 1] >= 1 && v[i] - v[i + 1] <= max_jump {
            meets_cond = true;
        } else {
            meets_cond = false;
            break;
        }
    }
    meets_cond
}
pub fn day2() {
    // read input
    let raw = fs::read_to_string("./inputs/day2.txt").expect("cant open input file");
    let split_data = raw.split("\n").filter(|x| x.len() > 0).collect::<Vec<_>>();
    // get report and parse it to i32
    let reports = split_data
        .iter()
        .map(|x| {
            x.split(" ")
                .map(|y| y.parse::<i32>().expect("Cant parse to i32"))
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<_>>();
    // count number of save reports
    let n_safe = reports
        .iter()
        .map(|x| {
            if decreasing(x, 3) || increasing(x, 3) {
                1
            } else {
                // START PART 2
                for ci in 0..x.len() {
                    let i_pop = x
                        .iter()
                        .enumerate()
                        .filter(|(cx, _)| *cx != ci)
                        .map(|(_, x)| *x)
                        .collect::<Vec<_>>();
                    if decreasing(&i_pop, 3) || increasing(&i_pop, 3) {
                        return 1;
                    }
                }
                // END PART 2
                0
            }
        })
        .sum::<i32>();
    println!("Safe reports: {:?}", n_safe);
}
