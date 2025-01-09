fn climbingLeaderboard(ranked: &[i32], player: &[i32]) -> Vec<i32> {
    let mut unique_ranked: Vec<i32> = ranked.iter().cloned().collect();
    unique_ranked.dedup();

    let mut result = Vec::new();

    let mut index = unique_ranked.len();

    for &score in player {
        while index > 0 && score >= unique_ranked[index - 1] {
            index -= 1; // Move up the leaderboard
        }
        result.push((index + 1) as i32);
    }

    result
}

fn main() {
    use std::env;
    use std::fs::File;
    use std::io::{self, BufRead, Write};

    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _ranked_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let ranked: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let _player_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let player: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = climbingLeaderboard(&ranked, &player);

    for i in 0..result.len() {
        writeln!(&mut fptr, "{}", result[i]).ok();
    }
}
