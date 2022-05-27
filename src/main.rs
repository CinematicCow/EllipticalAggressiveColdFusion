use std::io;

fn fizzbuzz(n: i32) -> Vec<String> {
    (1..=n)
        .map(|x| match x {
            x if x % 15 == 0 => String::from("FizzBuzz"),
            x if x % 3 == 0 => String::from("Fizz"),
            x if x % 5 == 0 => String::from("Buzz"),
            _ => x.to_string(),
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
        Ok(i) => println!("{:?}", fizzbuzz(i)),
        Err(..) => println!("this was not an integer: {}", trimmed),
    }

}
