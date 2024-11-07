use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'formingMagicSquare' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts 2D_INTEGER_ARRAY s as parameter.
 */

fn formingMagicSquare(s: &[Vec<i32>]) -> i32 {
    let magic_squares = vec![
        vec![8, 1, 6, 3, 5, 7, 4, 9, 2],
        vec![6, 1, 8, 7, 5, 3, 2, 9, 4],
        vec![4, 9, 2, 3, 5, 7, 8, 1, 6],
        vec![2, 9, 4, 7, 5, 3, 6, 1, 8],
        vec![8, 3, 4, 1, 5, 9, 6, 7, 2],
        vec![4, 3, 8, 9, 5, 1, 2, 7, 6],
        vec![6, 7, 2, 1, 5, 9, 8, 3, 4],
        vec![2, 7, 6, 9, 5, 1, 4, 3, 8],
    ];

    let flat_s = s.iter().flatten().cloned().collect::<Vec<i32>>();
    magic_squares
        .into_iter()
        .map(|magic| magic.iter().zip(&flat_s).map(|(a, b)| (a - b).abs()).sum())
        .min()
        .unwrap()
}


fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let mut s: Vec<Vec<i32>> = Vec::with_capacity(3_usize);

    for i in 0..3_usize {
        s.push(Vec::with_capacity(3_usize));

        s[i] = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    let result = formingMagicSquare(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
