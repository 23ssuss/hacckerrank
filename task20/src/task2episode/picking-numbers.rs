use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn pickingNumbers(a: &[i32]) -> i32 {
    let mut frequency = vec![0; 101];

    for &num in a {
        frequency[num as usize] += 1;
    }

    let mut max_count = 0;

    for i in 1..101 {
        max_count = max_count.max(frequency[i] + frequency[i - 1]);
    }

    max_count
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n: i32 = stdin_iterator.next().unwrap().unwrap().trim().parse().unwrap();

    let a: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();

    let result = pickingNumbers(&a);

    writeln!(&mut fptr, "{}", result).ok();
}
