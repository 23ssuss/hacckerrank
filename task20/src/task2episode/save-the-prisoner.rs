use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'saveThePrisoner' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. INTEGER m
 *  3. INTEGER s
 */

fn saveThePrisoner(n: i32, m: i32, s: i32) -> i32 {
    let result = (s + m - 1) % n;
    if result == 0 {
        n
    } else {
        result
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
            .split(' ')
            .map(|s| s.to_string())
            .collect();

        let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

        let m = first_multiple_input[1].trim().parse::<i32>().unwrap();

        let s = first_multiple_input[2].trim().parse::<i32>().unwrap();

        let result = saveThePrisoner(n, m, s);

        writeln!(&mut fptr, "{}", result).ok();
    }
}
