use std::io;

fn main() {
    // fetch first and only expected line of input
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("expecting digits as input");

    // collect all valid digits
    let vec = input.chars()
        .map(|c| c.to_digit(10).unwrap_or(0))
        .filter(|d| *d != 0)
        .collect::<Vec<u32>>();
    let length = vec.len();
    if length > 0 && length % 2 != 0 {
        panic!("expecting an non-empty even vector");
    }
    // compute steps
    let steps = length/2;

    // compute captcha
    let mut total = 0;
    for i in 0..length {
        if vec[i] == vec[(i+steps)%length] {
            total += vec[i];
        }
    }

    // print the result (which can be 0)
    println!("{}", total);
}
