use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn dayOfProgrammer(year: i32) -> String {
    if year == 1918 {
        return "26.09.1918".to_string();
    }

    if (year < 1918 && year % 4 == 0) || (year > 1918 && (year % 400 == 0 || (year % 4 == 0 && year % 100 != 0))) {
        return format!("12.09.{}", year);
    } else {
        return format!("13.09.{}", year);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let year = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let result = dayOfProgrammer(year);

    writeln!(&mut fptr, "{}", result).ok();
}
