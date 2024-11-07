use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::collections::HashMap;

fn migratoryBirds(arr: &[i32]) -> i32 {
    let mut counts = HashMap::new();
    for &bird in arr {
        *counts.entry(bird).or_insert(0) += 1;
    }

    let mut max_count = 0;
    let mut result = i32::MAX;

    for (&bird, &count) in counts.iter() {
        if count > max_count || (count == max_count && bird < result) {
            max_count = count;
            result = bird;
        }
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _arr_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = migratoryBirds(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}
