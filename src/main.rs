use std::io;

fn test(_tt: i32) {
    // take two integer inputs from user
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut iter = input.split_whitespace();
    // parse int from the received string
    let l = iter.next().unwrap().to_string();
    let r = iter.next().unwrap().to_string();
    let l: i32 = l.trim().parse().expect("Please type a number!");
    let r: i32 = r.trim().parse().expect("Please type a number!");

    if (r - l) >= 100 {
        for i in l..r+1{
            if i%100==90{
                println!("{}", i);
                break;
            }
        }
    } else {
        let mut mlh = 0;
        let mut value = l;
        for i in l..r + 1 {
            let mut hi = 0;
            let mut lo = 9;
            let mut ic = i;
            while ic > 0 {
                hi = i32::max(hi, ic % 10);
                lo = i32::min(lo, ic % 10);
                if mlh < i32::max(mlh, hi - lo) {
                    mlh = i32::max(mlh, hi - lo);
                    value = i;
                }
                ic = ic / 10;
            }
        }
        println!("{}", value);
    }
}

fn main() {
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // parse int from the received string
    let mut guess: i32 = guess.trim().parse().expect("Please type a number!");

    while guess > 0 {
        test(guess);
        guess = guess - 1;
    }
}
