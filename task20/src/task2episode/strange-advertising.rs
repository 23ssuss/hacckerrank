use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'viralAdvertising' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER n as parameter.
 */

fn viralAdvertising(n: i32) -> i32 {
    let mut shared = 5;
    let mut cumulative_likes = 0;

    for _ in 0..n {
        let liked = shared / 2;
        cumulative_likes += liked;
        shared = liked * 3;
    }

    cumulative_likes
}


fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let result = viralAdvertising(n);

    writeln!(&mut fptr, "{}", result).ok();
}
