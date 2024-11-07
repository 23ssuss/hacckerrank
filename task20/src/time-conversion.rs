use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'timeConversion' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

fn timeConversion(s: &str) -> String {
    let (time, period) = s.split_at(8); 
    let period = period.trim(); 

    let mut hour: i32 = time[0..2].parse().unwrap(); 
    let minute_second = &time[2..];

    if period == "AM" {
        if hour == 12 {
            hour = 0; 
        }
    } else if period == "PM" {
        if hour != 12 {
            hour += 12; 
        }
    }

    format!("{:02}{}", hour, minute_second)
}


fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = timeConversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
