use std::collections::BTreeSet;

fn ashtonString(s: &str, k: i32) -> char {
    let mut substrings = BTreeSet::new();

    // Collect all distinct substrings
    for i in 0..s.len() {
        for j in i + 1..=s.len() {
            substrings.insert(&s[i..j]);
        }
    }

    // Convert substrings to a sorted vector
    let sorted_substrings: Vec<&str> = substrings.into_iter().collect();

    // Locate the k-th character in the concatenated substrings
    let mut current_length = 0;
    for substring in sorted_substrings {
        let substring_length = substring.len();
        if current_length + substring_length >= k as usize {
            return substring.chars().nth(k as usize - current_length - 1).unwrap();
        }
        current_length += substring_length;
    }

    unreachable!() // `k` is guaranteed to be valid as per problem constraints
}

fn main() {
    use std::env;
    use std::fs::File;
    use std::io::{self, BufRead, Write};

    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let s = stdin_iterator.next().unwrap().unwrap();
        let k = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
        let res = ashtonString(&s, k);
        writeln!(&mut fptr, "{}", res).ok();
    }
}
