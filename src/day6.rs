use std::collections::{HashMap, HashSet};
use std::{fs, usize};
pub fn day6() {
    //  where the directions are in the grid
    let mut directions: HashMap<&str, (i32, i32)> = HashMap::new();
    directions.insert("^", (-1, 0));
    directions.insert("v", (1, 0));
    directions.insert(">", (0, 1));
    directions.insert("<", (0, -1));
    // all possible directions
    let looks = directions.keys().into_iter().map(|x| x).collect::<Vec<_>>();
    // where each direction looks after a turn
    let direction_change = HashMap::from([("^", ">"), ("v", "<"), (">", "v"), ("<", "^")]);

    let raw_input = fs::read_to_string("./inputs/day6.txt").expect("cant open file");
    // generate grid
    let mut grid = raw_input
        .split("\n")
        .filter(|z| z.len() > 0)
        .into_iter()
        .map(|x| x.split("").filter(|y| y.len() > 0).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    // maximal grid dimensions
    let x_max = grid[0].len() as i32;
    let y_max = grid.len() as i32;

    // is starting position found
    let mut found_start = false;
    // starting position coordinates
    let mut starting_pos = (0, 0);
    for (ci, i) in grid.iter().enumerate() {
        match i.iter().position(|&x| looks.contains(&&x)) {
            Some(p) => {
                starting_pos = (ci as i32, p as i32);
                found_start = true
            }
            None => {}
        }
    }
    if !found_start {
        panic!("Unable to find starting position")
    }
    // current position
    let mut current_position = starting_pos;
    // the movement direction
    let mut current_look = grid[starting_pos.0 as usize][starting_pos.1 as usize];
    // needed for faster PART 2
    let mut visited: HashSet<_> = HashSet::new();
    // number of unique fields visited
    let mut uniq_fields = 0;
    loop {
        // direction of movement
        let move_dir = directions
            .get(current_look)
            .expect("cant get look direction");
        // the next step
        let next_pos = (
            current_position.0 + move_dir.0,
            current_position.1 + move_dir.1,
        );
        // reached end
        if next_pos.0 < 0 || next_pos.1 < 0 || next_pos.0 >= y_max || next_pos.1 >= x_max {
            uniq_fields += 1;
            break;
        }
        // the next steps field value
        let next_field = grid[next_pos.0 as usize][next_pos.1 as usize];
        // count for not visited possible fields
        if next_field == "." {
            uniq_fields += 1;
        }
        // if not obstacle update current position, mark step and add to visited fields
        // else change direction of movement
        if next_field != "#" {
            current_position = next_pos;
            grid[current_position.0 as usize][current_position.1 as usize] = "X";
            visited.insert((current_position.0, current_position.1));
        } else {
            current_look = direction_change
                .get(&current_look)
                .expect("cant get rotation")
        }
    }
    {
        // get fresh grid
        let mut grid = raw_input
            .split("\n")
            .filter(|z| z.len() > 0)
            .into_iter()
            .map(|x| x.split("").filter(|y| y.len() > 0).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        // number of possible obstacle places
        let mut obst_pos = 0;
        // iterate over all places of the guards path that are not the starting position
        for i in &visited
            .iter()
            .filter(|x| **x != starting_pos)
            .collect::<Vec<_>>()
        {
            let mut current_position = starting_pos;
            let mut current_look = grid[starting_pos.0 as usize][starting_pos.1 as usize];
            // track the visited field with their direction of movement
            // if visited with same direction loop is detected
            let mut visited2 = HashSet::new();
            // place obstacle
            grid[i.0 as usize][i.1 as usize] = "#";
            loop {
                // same as PART 1
                let move_dir = directions
                    .get(current_look)
                    .expect("cant get look direction");
                let next_pos = (
                    current_position.0 + move_dir.0,
                    current_position.1 + move_dir.1,
                );
                // reached end
                if next_pos.0 < 0 || next_pos.1 < 0 || next_pos.0 >= y_max || next_pos.1 >= x_max {
                    break;
                }
                let next_field = grid[next_pos.0 as usize][next_pos.1 as usize];
                if next_field != "#" {
                    current_position = next_pos;
                } else {
                    current_look = direction_change
                        .get(&current_look)
                        .expect("cant get rotation")
                }
                // check if we entered a loop and if so update obstacle position count
                match visited2.get(&(current_position, current_look)) {
                    Some(_) => {
                        grid[i.0 as usize][i.1 as usize] = "O";
                        obst_pos += 1;
                        break;
                    }
                    None => {
                        visited2.insert((current_position, current_look));
                    }
                }
            }
            grid[i.0 as usize][i.1 as usize] = ".";
        }
        println!("Obsticle places: {:?}", obst_pos);
    }

    println!("{:?}", uniq_fields);
}
