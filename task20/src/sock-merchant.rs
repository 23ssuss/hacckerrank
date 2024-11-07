use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::collections::HashMap;

fn sockMerchant(n: i32, ar: &[i32]) -> i32 {
    let mut count_map = HashMap::new();
    let mut pairs = 0;

    for &sock in ar {
        let entry = count_map.entry(sock).or_insert(0);
        *entry += 1;
        if *entry % 2 == 0 {
            pairs += 1;
        }
    }

    pairs
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let ar: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = sockMerchant(n, &ar);

    writeln!(&mut fptr, "{}", result).ok();
}
