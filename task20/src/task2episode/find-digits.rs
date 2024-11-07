use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'findDigits' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER n as parameter.
 */

fn findDigits(n: i32) -> i32 {
    n.to_string()
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as i32))
        .filter(|&d| d != 0 && n % d == 0)
        .count() as i32
}


fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

        let result = findDigits(n);

        writeln!(&mut fptr, "{}", result).ok();
    }
}
