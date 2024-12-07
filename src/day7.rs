use std::fs;

pub fn day7() {
    let raw_binding = fs::read_to_string("./inputs/day7.txt").expect("cant read file");
    let raw_input = raw_binding.split("\n").collect::<Vec<_>>();
    // exclude "|" for PART 1
    let operators = vec!["+", "*", "|"];
    // iterate over all lines and sum the result
    let total_sum = raw_input
        .iter()
        .filter(|y| y.len() > 0)
        .map(|x| {
            // split input line
            let test_num_split = x.split(" ").collect::<Vec<_>>();
            // the result to test
            let result = test_num_split[0]
                .replace(":", "")
                .parse::<i64>()
                .expect("cant parse to i64");
            // the numbers that generate the result
            let nums = test_num_split[1..test_num_split.len()]
                .iter()
                .map(|z| z.parse::<i64>().expect("cant parse to i64"))
                .collect::<Vec<_>>();
            // if only two numbers directly test all possible operators whether they
            // generate the right result else return 0
            if nums.len() == 2 {
                let mut test_result = 0;
                for i in &operators {
                    match i {
                        &"+" => test_result = nums[0] + nums[1],
                        &"*" => test_result = nums[0] * nums[1],
                        //  START Part 2
                        &"|" => {
                            test_result = format!("{}{}", nums[0], nums[1])
                                .parse::<i64>()
                                .expect("cant parse to i64")
                        }
                        //  END Part 2
                        _ => {
                            panic!("invalid operator")
                        }
                    }
                    if test_result == result {
                        return result;
                    } else {
                        test_result = 0
                    }
                }
                test_result
            // for multiple operators
            } else {
                // all possible operator combinations
                let operator_combi: Vec<_> = (2..nums.len() - 1).fold(
                    operators
                        .iter()
                        .map(|c| operators.iter().map(move |&d| d.to_owned() + *c))
                        .flatten()
                        .collect(),
                    |acc, _| {
                        acc.into_iter()
                            .map(|c| operators.iter().map(move |&d| d.to_owned() + &*c))
                            .flatten()
                            .collect()
                    },
                );
                // apply operator on result of the previous operator usage and the next
                // number in nums
                let mut test_result = 0;
                for i in &operator_combi {
                    test_result = 0;
                    for j in 0..nums.len() {
                        if j == 0 {
                            test_result = nums[0];
                        } else {
                            match i.chars().collect::<Vec<_>>()[j - 1] {
                                '+' => test_result = test_result + nums[j],
                                '*' => test_result = test_result * nums[j],
                                //  START Part 2
                                '|' => {
                                    test_result = format!("{}{}", test_result, nums[j])
                                        .parse::<i64>()
                                        .expect("cant parse to i64")
                                }
                                //  END Part 2
                                _ => {
                                    panic!("invalid operator")
                                }
                            }
                        }
                    }
                    if test_result == result {
                        return test_result;
                    } else {
                        test_result = 0
                    }
                }
                test_result
            }
        })
        .sum::<i64>();
    println!("Sum: {:?}", total_sum);
}
