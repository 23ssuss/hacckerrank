use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'circularArrayRotation' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY a
 *  2. INTEGER k
 *  3. INTEGER_ARRAY queries
 */

fn circularArrayRotation(a: &[i32], k: i32, queries: &[i32]) -> Vec<i32> {
    let n = a.len();
    let rotation = (k as usize) % n;
    let rotated_array = [&a[n - rotation..], &a[..n - rotation]].concat();

    queries.iter().map(|&index| rotated_array[index as usize]).collect()
}


fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let q = first_multiple_input[2].trim().parse::<i32>().unwrap();

    let a: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let mut queries: Vec<i32> = Vec::with_capacity(q as usize);

    for _ in 0..q {
        let queries_item = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
        queries.push(queries_item);
    }

    let result = circularArrayRotation(&a, k, &queries);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            writeln!(&mut fptr).ok();
        }
    }

    writeln!(&mut fptr).ok();
}
