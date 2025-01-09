use std::io::{self, BufRead};

fn extraLongFactorials(n: i32) {
    let mut result = vec![1]; // Start with 1 as the factorial value

    for i in 2..=n {
        multiply(&mut result, i);
    }

    // Convert result to a string and print
    let result_str: String = result.iter().rev().map(|d| d.to_string()).collect();
    println!("{}", result_str);
}

// Multiply the current large number (in `result`) by `multiplier`
fn multiply(result: &mut Vec<u32>, multiplier: i32) {
    let mut carry: u64 = 0; // Use u64 for intermediate calculations

    for digit in result.iter_mut() {
        let product = *digit as u64 * multiplier as u64 + carry;
        *digit = (product % 10) as u32; // Keep the last digit
        carry = product / 10; // Carry over the rest remains as u64
    }

    // Handle remaining carry
    while carry > 0 {
        result.push((carry % 10) as u32);
        carry /= 10;
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    extraLongFactorials(n);
}
