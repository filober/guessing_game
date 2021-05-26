use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    //Use mut to make the variable mutable, otherwhise if you assign a new vatiable
    //you get a compiler error 
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line"); //in case return an error, print this

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {}", guess);
    
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
