use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::io::Write;

fn main() {
    println!("Hello, world!");
    let number: i32 = rand::thread_rng().gen_range(0..100);

    loop {
        //user input
        let mut buffer: String = String::new();
        print!("please enter a number between 0 and 100: ");
        io::stdout().flush();
        io::stdin().read_line(&mut buffer);
        let get_user_input: i32 = buffer.trim().parse::<i32>().unwrap();

        match get_user_input.cmp(&number) {
            Ordering::Greater => println!("is lower"),
            Ordering::Less => println!("is upper"),
            Ordering::Equal => break,
        }
    }
    println!("You Won!");
}
