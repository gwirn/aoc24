use std::fs;

pub fn day4_1() {
    let raw_in = fs::read_to_string("./inputs/day4.txt").expect("cant read file");
    let raw_split = raw_in
        .split("\n")
        .filter(|x| x.len() > 0)
        .collect::<Vec<_>>();
    // XMAS pattern forward and reverse
    let forward = "XMAS";
    let reverse = "SAMX";
    // idx of newline symbols
    let lines_nl = raw_in
        .match_indices("\n")
        .map(|(cx, _)| cx)
        .collect::<Vec<_>>();
    // number of input lines
    let n_lines = lines_nl.len();
    // length of a line
    let line_len = lines_nl.first().expect("empty line count");
    // check for pattern vertically
    let vertical = raw_split
        .iter()
        .map(|y| y.matches(reverse).count() + y.matches(forward).count())
        .sum::<usize>();
    // check for pattern vertically
    let horizontal = (0..*line_len)
        .map(|i| {
            let i_col = raw_split
                .iter()
                .map(|x| {
                    x.chars()
                        .nth(i)
                        .expect("cant get char at position")
                        .to_string()
                })
                .collect::<Vec<_>>();
            let i_str = i_col.join("");
            i_str.matches(reverse).count() + i_str.matches(forward).count()
        })
        .sum::<usize>();
    // check for pattern diagonally
    let diag0 = {
        let mut diag_count = 0;
        for i in 0..*line_len {
            //  upper and lower triangles of the diagonals
            let mut upper: Vec<String> = Vec::new();
            let mut lower: Vec<String> = Vec::new();
            for j in 0..n_lines {
                // where to start the diagonal
                let upper_idx = (i as i32 - j as i32) as usize;
                match raw_split[j].chars().nth(upper_idx) {
                    Some(c) => upper.push(c.to_string()),
                    None => {}
                }
                // same as above put but from other directions
                let lower_idx = (upper_idx as i32 - *line_len as i32).abs() as usize;
                match raw_split[n_lines - 1 - j].chars().nth(lower_idx) {
                    Some(c) => lower.push(c.to_string()),
                    None => {}
                }
            }
            let upper_join = upper.join("");
            let lower_join = lower.join("");
            // check for matches in the diagonals
            diag_count += lower_join.matches(forward).count();
            diag_count += lower_join.matches(reverse).count();
            diag_count += upper_join.matches(forward).count();
            diag_count += upper_join.matches(reverse).count();
        }
        diag_count
    };
    // same as diag0 but other diagonals
    let diag1 = {
        let mut diag_count = 0;
        for i in (0..*line_len).rev() {
            let mut upper: Vec<String> = Vec::new();
            let mut lower: Vec<String> = Vec::new();
            for j in (0..n_lines).rev() {
                let upper_idx = (j as i32 - i as i32) as usize;
                match raw_split[j].chars().nth(upper_idx) {
                    Some(c) => upper.push(c.to_string()),
                    None => {}
                }
                let lower_idx = (upper_idx as i32 - *line_len as i32).abs() as usize;
                match raw_split[n_lines - 1 - j].chars().nth(lower_idx) {
                    Some(c) => lower.push(c.to_string()),
                    None => {}
                }
            }
            let upper_join = upper.join("");
            let lower_join = lower.join("");
            diag_count += lower_join.matches(forward).count();
            diag_count += lower_join.matches(reverse).count();
            diag_count += upper_join.matches(forward).count();
            diag_count += upper_join.matches(reverse).count();
        }
        diag_count
    };
    println!("{:?}", vertical + horizontal + diag0 + diag1);
}

pub fn day4() {
    // read file
    let raw_in = fs::read_to_string("./inputs/day4.txt").expect("cant read file");
    // remove newlines
    let raw_nnl = raw_in.replace("\n", "");
    // get input as chars
    let raw_chars = raw_nnl.chars().collect::<Vec<char>>();
    // idx of newlines
    let lines_nl = raw_in
        .match_indices("\n")
        .map(|(cx, _)| cx)
        .collect::<Vec<_>>();
    // number of lines
    let n_lines = lines_nl.len();
    //  line length
    let line_len = lines_nl.first().expect("empty line count");
    let kernel_size = 3;
    // pattern to check as rectangle - ignoring other indices in rectangle
    /*
    0x2
    x4x
    6x8
    */
    // all possible patterns
    let k1 = "MSAMS";
    let k2 = "MMASS";
    let k3 = "SSAMM";
    let k4 = "SMASM";
    let kernels = vec![k1, k2, k3, k4];
    let mut counts = 0;
    for i in 0..(n_lines - kernel_size + 1) {
        for j in 0..(*line_len - kernel_size + 1) {
            // index the input (which is now one line) to get rectangle at current index
            let idx_oi = vec![
                // are on the first line
                0 + j + line_len * i,
                2 + j + line_len * i,
                // is on the second line
                4 + j + line_len * i + line_len - kernel_size,
                // are on the third line
                6 + j + line_len * i + (line_len - kernel_size) * 2,
                8 + j + line_len * i + (line_len - kernel_size) * 2,
            ];
            // extract the rectangle from the input
            let to_test_pattern = idx_oi
                .iter()
                .map(|x| raw_chars[*x].to_string())
                .collect::<Vec<_>>();
            let joined_pattern = to_test_pattern.join("");
            // test all kernels if they are the pattern
            counts += kernels
                .iter()
                .map(|x| if *x == joined_pattern { 1 } else { 0 })
                .sum::<i32>();
        }
    }
    println!("{:?}", counts);
}
