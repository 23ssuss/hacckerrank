use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'utopianTree' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER n as parameter.
 */

fn utopianTree(n: i32) -> i32 {
    let mut height = 1;
    for i in 1..=n {
        if i % 2 != 0 {
            height *= 2; // Spring: tree doubles in height
        } else {
            height += 1; // Summer: tree grows by 1 meter
        }
    }
    height
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

        let result = utopianTree(n);

        writeln!(&mut fptr, "{}", result).ok();
    }
}
