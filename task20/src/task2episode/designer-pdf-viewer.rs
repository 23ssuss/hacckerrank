use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'designerPdfViewer' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY h
 *  2. STRING word
 */

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'designerPdfViewer' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY h
 *  2. STRING word
 */

fn designerPdfViewer(h: &[i32], word: &str) -> i32 {
    let mut max_height = 0;
    for c in word.chars() {
        let index = (c as u8 - b'a') as usize; // Find the index for the character
        max_height = max_height.max(h[index]);
    }
    max_height * word.len() as i32
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let h: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let word = stdin_iterator.next().unwrap().unwrap();

    let result = designerPdfViewer(&h, &word);

    writeln!(&mut fptr, "{}", result).ok();
}


fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let h: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let word = stdin_iterator.next().unwrap().unwrap();

    let result = designerPdfViewer(&h, &word);

    writeln!(&mut fptr, "{}", result).ok();
}
