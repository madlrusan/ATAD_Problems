use std::collections::HashSet;
use std::io::{self, BufRead};

fn queensAttack(n: i32, _k: i32, r_q: i32, c_q: i32, obstacles: &[Vec<i32>]) -> i32 {
    // Represent obstacles as a HashSet for O(1) lookup
    let obstacle_set: HashSet<(i32, i32)> = obstacles.iter().map(|obs| (obs[0], obs[1])).collect();

    // Initialize directions (row_delta, col_delta)
    let directions = [
        (-1, 0),  // Up
        (1, 0),   // Down
        (0, -1),  // Left
        (0, 1),   // Right
        (-1, -1), // Up-Left
        (-1, 1),  // Up-Right
        (1, -1),  // Down-Left
        (1, 1),   // Down-Right
    ];

    let mut total_moves = 0;

    // Iterate over each direction
    for (dr, dc) in directions.iter() {
        let mut row = r_q + dr;
        let mut col = c_q + dc;

        // Traverse in the current direction until a boundary or obstacle is reached
        while row >= 1 && row <= n && col >= 1 && col <= n && !obstacle_set.contains(&(row, col)) {
            total_moves += 1;
            row += dr;
            col += dc;
        }
    }

    total_moves
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let first_multiple_input: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let n = first_multiple_input[0];
    let k = first_multiple_input[1];

    let second_multiple_input: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let r_q = second_multiple_input[0];
    let c_q = second_multiple_input[1];

    let mut obstacles = Vec::new();

    for _ in 0..k {
        let obstacle: Vec<i32> = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        obstacles.push(obstacle);
    }

    let result = queensAttack(n, k, r_q, c_q, &obstacles);

    println!("{}", result);
}
