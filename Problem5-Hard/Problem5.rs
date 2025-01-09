fn matrixRotation(matrix: &[Vec<i32>], r: i32) {
    let m = matrix.len();
    let n = matrix[0].len();
    let layers = std::cmp::min(m, n) / 2;

    let mut result = matrix.to_vec();

    for layer in 0..layers {
        // Extract the layer into a vector
        let mut layer_elements = Vec::new();

        // Top row
        for col in layer..(n - layer) {
            layer_elements.push(matrix[layer][col]);
        }

        // Right column
        for row in (layer + 1)..(m - layer) {
            layer_elements.push(matrix[row][n - layer - 1]);
        }

        // Bottom row
        for col in (layer..(n - layer - 1)).rev() {
            layer_elements.push(matrix[m - layer - 1][col]);
        }

        // Left column
        for row in ((layer + 1)..(m - layer - 1)).rev() {
            layer_elements.push(matrix[row][layer]);
        }

        // Rotate the layer elements
        let len = layer_elements.len();
        let rotation = (r as usize) % len;
        layer_elements.rotate_left(rotation);

        // Place the rotated layer back into the result matrix
        let mut idx = 0;

        // Top row
        for col in layer..(n - layer) {
            result[layer][col] = layer_elements[idx];
            idx += 1;
        }

        // Right column
        for row in (layer + 1)..(m - layer) {
            result[row][n - layer - 1] = layer_elements[idx];
            idx += 1;
        }

        // Bottom row
        for col in (layer..(n - layer - 1)).rev() {
            result[m - layer - 1][col] = layer_elements[idx];
            idx += 1;
        }

        // Left column
        for row in ((layer + 1)..(m - layer - 1)).rev() {
            result[row][layer] = layer_elements[idx];
            idx += 1;
        }
    }

    // Print the result matrix
    for row in result {
        println!("{}", row.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let first_multiple_input: Vec<usize> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let m = first_multiple_input[0];
    let n = first_multiple_input[1];
    let r = first_multiple_input[2] as i32;

    let mut matrix = vec![vec![0; n]; m];
    for i in 0..m {
        matrix[i] = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .split(' ')
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
    }

    matrixRotation(&matrix, r);
}
