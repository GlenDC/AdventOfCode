use std::io;

fn main() {
    // fetch first and only expected line of input
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("expecting digits as input");

    // we expect only 0-9 digits as input, hence radix of 10
    let radix = 10;

    // get first valid digit
    let mut it = input.chars().skip_while(|x| !x.is_digit(radix));
    let first = it.next().expect("invalid input")
        .to_digit(radix).unwrap();

    // go through all other chars,
    // and increase the counter (by x) of the current
    // was equal to the previous
    let (mut previous, mut total) = (first, 0);
    for c in it {
        match c.to_digit(radix) {
            Some(x) => {
                if previous == x {
                    total += x;
                }
                previous = x;
            },
            None => {},
        }
    }
    // make sure to compare the last against the first,
    // as the input is to be concidered as a circular list
    if previous == first {
        total += previous;
    }

    // print the result (which can be 0)
    println!("{}", total);
}
