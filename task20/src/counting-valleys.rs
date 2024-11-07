use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn countingValleys(steps: i32, path: &str) -> i32 {
    let mut level = 0;
    let mut valleys = 0;

    for step in path.chars() {
        if step == 'U' {
            level += 1;
        } else if step == 'D' {
            level -= 1;
        }

        if level == 0 && step == 'U' {
            valleys += 1;
        }
    }

    valleys
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let steps = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let path = stdin_iterator.next().unwrap().unwrap();

    let result = countingValleys(steps, &path);

    writeln!(&mut fptr, "{}", result).ok();
}
