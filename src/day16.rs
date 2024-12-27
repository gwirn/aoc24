use core::panic;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    fs,
};
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Node {
    cost: usize,
    coords: (usize, usize),
    facing: (usize, usize),
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.coords.cmp(&other.coords))
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

/*
dijkstra shortest path through maze
*/
fn dijkstra(grid: &Vec<Vec<&str>>, start: (usize, usize), end: (usize, usize)) -> Option<usize> {
    // distances from start to all visited nodes
    let mut dists: HashMap<(usize, usize), usize> = HashMap::new();
    // parents of the nodes
    let mut parents: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    // priority queue
    let mut heap = BinaryHeap::new();
    // insert start node
    dists.insert(start, 0);
    heap.push(Node {
        cost: 0,
        coords: start,
        facing: (0, 1),
    });
    while let Some(Node {
        cost,
        coords,
        facing,
    }) = heap.pop()
    {
        // stop when end is reached
        if coords == end {
            /*
            let mut current = vec![end];
                        loop {
                            let p = parents
                                .get(&current[current.len() - 1])
                                .expect("cant find parent");
                            current.push(*p);
                            if *p == start {
                                break;
                            }
                        }
                        for i in current.iter().rev() {
                            println!("{:?}", i);
                        }
            */
            match dists.get(&coords) {
                Some(v) => return Some(*v),
                None => panic!("cant get end cost"),
            }
        }
        // if cost to coords is bigger a better way was already found
        if cost > *dists.get(&coords).expect("cant get dist for coords") {
            continue;
        }
        // check in all DIRECTIONS if there is a possible next step
        'd: for d in DIRECTIONS {
            // next step coordinates
            let dy = (coords.0 as i32 + d.0) as usize;
            let dx = (coords.1 as i32 + d.1) as usize;
            if grid[dy][dx] != "." {
                continue 'd;
            };
            // default step cost
            let mut step_cost = 1;
            // type conversion
            let d_u = (d.0 as usize, d.1 as usize);
            // add rotation cost
            if d_u != facing {
                step_cost += 1000;
            }
            //  construct possible next step
            let next = Node {
                cost: cost + step_cost,
                coords: (dy, dx),
                facing: d_u,
            };
            // replaces the need to set all distances to all nodes to INF at the start
            match dists.get(&next.coords) {
                Some(v) => {
                    // next step if it is at lower cost
                    if next.cost < *v {
                        heap.push(next);
                        dists.insert(next.coords, next.cost);
                        parents.insert(next.coords, coords);
                    };
                }
                None => {
                    heap.push(next);
                    dists.insert(next.coords, next.cost);
                    parents.insert(next.coords, coords);
                }
            }
        }
    }
    None
}
pub fn day16() {
    let raw_input = fs::read_to_string("./inputs/day16.txt").expect("cant read file");
    // generate grid
    let mut grid = raw_input
        .split("\n")
        .filter(|z| !z.is_empty())
        .map(|x| x.split("").filter(|y| !y.is_empty()).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    // grid dimensions
    let ydim = grid.len();
    let xdim = grid[0].len();
    // find start and end coordinates
    let mut start = (0, 0);
    let mut end = (0, 0);
    for y in 0..ydim {
        for x in 0..xdim {
            if grid[y][x] == "S" {
                start = (y, x);
            } else if grid[y][x] == "E" {
                end = (y, x);
            }
        }
    }
    // replace them so they are valid points
    grid[start.0][start.1] = ".";
    grid[end.0][end.1] = ".";
    // search for shortest path
    match dijkstra(&grid, start, end) {
        Some(v) => println!("Distance: {}", v),
        None => println!("Cant find path to end"),
    }
}
