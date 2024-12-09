use std::collections::{HashMap, HashSet};
use std::fs;
pub fn day8() {
    let raw_input = fs::read_to_string("./inputs/day8.txt").expect("cant open file");
    // generate grid
    let grid = raw_input
        .split("\n")
        .filter(|z| z.len() > 0)
        .into_iter()
        .map(|x| x.split("").filter(|y| y.len() > 0).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    // maximal grid dimensions
    let row_max = grid.len() as i32;
    let col_max = grid[0].len() as i32;
    // get all unique frequency antennas and their positions
    let mut positions: HashMap<&str, Vec<(i32, i32)>> = HashMap::new();
    for (cr, row) in grid.iter().enumerate() {
        for (cc, col) in row.iter().enumerate() {
            if *col != "." {
                positions
                    .entry(col)
                    .and_modify(|x| x.push((cr as i32, cc as i32)))
                    .or_insert_with(|| vec![(cr as i32, cc as i32)]);
            }
        }
    }
    // PART 1 START
    let mut antinode_positions: HashSet<(i32, i32)> = HashSet::new();
    // check n*(n-1)/ 2 positions (triangle of matrix)
    for p in positions.values() {
        for r in 0..p.len() {
            for c in r + 1..p.len() {
                // difference between first and next element
                let r_diff = p[r].0 - p[c].0;
                let c_diff = p[r].1 - p[c].1;
                // positions with the same spacing and direction but above or below original
                // antennas (or left and right)
                let ap0 = (p[r].0 + r_diff, p[r].1 + c_diff);
                let ap1 = (p[c].0 - r_diff, p[c].1 - c_diff);
                // add to set if they are inside the field boundaries
                if ap0.0 >= 0 && ap0.1 >= 0 && ap0.0 < row_max && ap0.1 < col_max {
                    antinode_positions.insert(ap0);
                }
                if ap1.0 >= 0 && ap1.1 >= 0 && ap1.0 < row_max && ap1.1 < col_max {
                    antinode_positions.insert(ap1);
                }
            }
        }
    }
    println!("Antinodes Part 1: {:?}", antinode_positions.len());
    // PART 1 END
    // PART 2 START
    let mut antinode_positions: HashSet<(i32, i32)> = HashSet::new();
    for p in positions.values() {
        for r in 0..p.len() {
            for c in r + 1..p.len() {
                // as PART 1
                let r_diff = p[r].0 - p[c].0;
                let c_diff = p[r].1 - p[c].1;
                let mut counter = 1;

                // flag if a diagonal direction of the search is already out of bounds
                let mut dir0_flag = false;
                let mut dir0_1_flag = false;

                loop {
                    // next locations of the search beam in both directions
                    let ap0 = (p[r].0 + r_diff * counter, p[r].1 + c_diff * counter);
                    let ap1 = (p[r].0 - r_diff * counter, p[r].1 - c_diff * counter);

                    // check if next positions are in bounds
                    if ap0.0 >= 0 && ap0.1 >= 0 && ap0.0 < row_max && ap0.1 < col_max {
                        antinode_positions.insert(ap0);
                    } else {
                        dir0_flag = true;
                    }

                    if ap1.0 >= 0 && ap1.1 >= 0 && ap1.0 < row_max && ap1.1 < col_max {
                        antinode_positions.insert(ap1);
                    } else {
                        dir0_1_flag = true;
                    }

                    // if all are out of field bounds the end is reached
                    if dir0_flag && dir0_1_flag {
                        break;
                    }

                    antinode_positions.insert(p[r]);
                    antinode_positions.insert(p[c]);
                    counter += 1;
                }
            }
        }
    }

    println!("Antinodes Part 2: {:?}", antinode_positions.len());
    // PART 2 END
}
