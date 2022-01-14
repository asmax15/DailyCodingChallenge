use std::io;

fn main() {
    let mut num1 = String::new();
    let mut num2 = String::new();

    println!("Type a Number");

    io::stdin()
    .read_line(&mut num1)
    .expect("Failed to read line");

    println!("Type a number");

    io::stdin()
    .read_line(&mut num2)
    .expect("Failed to read line");

    //let num1: u32 = num1.parse::<u32>().unwrap();
    //let num2: u32 = num2.parse::<u32>().unwrap();

    let num1: i32 = match num1.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    let num2: i32 = match num2.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    let result: i32 = &num1 + &num2;

    println!("Result: {}", result);
}
