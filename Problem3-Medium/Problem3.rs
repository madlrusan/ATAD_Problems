fn formingMagicSquare(s: &[Vec<i32>]) -> i32 {
    // Predefined all possible 3x3 magic squares
    let magic_squares = [
        [8, 1, 6, 3, 5, 7, 4, 9, 2],
        [6, 1, 8, 7, 5, 3, 2, 9, 4],
        [4, 9, 2, 3, 5, 7, 8, 1, 6],
        [2, 9, 4, 7, 5, 3, 6, 1, 8],
        [8, 3, 4, 1, 5, 9, 6, 7, 2],
        [4, 3, 8, 9, 5, 1, 2, 7, 6],
        [6, 7, 2, 1, 5, 9, 8, 3, 4],
        [2, 7, 6, 9, 5, 1, 4, 3, 8],
    ];

    // Flatten the input square
    let flat_s: Vec<i32> = s.iter().flat_map(|row| row.iter().copied()).collect();

    // Compute the minimal cost to transform `s` into any magic square
    magic_squares
        .iter()
        .map(|magic| {
            magic.iter().zip(&flat_s).map(|(m, &s)| (m - s).abs()).sum()
        })
        .min()
        .unwrap()
}

fn main() {
    use std::env;
    use std::fs::File;
    use std::io::{self, BufRead, Write};

    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let mut s: Vec<Vec<i32>> = Vec::with_capacity(3);

    for _ in 0..3 {
        s.push(
            stdin_iterator
                .next()
                .unwrap()
                .unwrap()
                .trim_end()
                .split(' ')
                .map(|s| s.parse::<i32>().unwrap())
                .collect(),
        );
    }

    let result = formingMagicSquare(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
