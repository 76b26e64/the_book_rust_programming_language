use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("guess the number between 0-99!");

    let secret_number: u32 = rand::thread_rng().gen_range(0..=99);
    
    loop {
        println!("Please input your guess.");
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(aaa) => aaa,
            Err(_) => continue,
        };
        println!("You guessed: {} ", guess);


        //println!("You guessed: {} secret_number: {}", guess, secret_number);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    } 
}
