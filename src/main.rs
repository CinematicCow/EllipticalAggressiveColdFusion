use std::io;

static FIZZ: &'static str = "FIZZ";
static BUZZ: &'static str = "BUZZ";
static FIZZBUZZ: &'static str = "FIZZBUZZ";

fn fizzbuzz(n: i32) {
    (1..=n)
        .map(|x| match x {
            x if x % 15 == 0 => println!("{}", FIZZBUZZ),
            x if x % 3 == 0 => println!("{}", FIZZ),
            x if x % 5 == 0 => println!("{}", BUZZ),
            _ => println!("{:?}", x),
        })
        .collect()
}

fn main() {
    println!("Enter the limit: ");
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from std input");

    let trimmed = input_text.trim();
    match trimmed.parse::<i32>() {
        Ok(i) => {
            print!("\x1B[2J");
            fizzbuzz(i)
        }
        Err(..) => println!("this was not an integer: {}", trimmed),
    }
}
