extern crate rand;
use rand::Rng;
use std::io;

fn main()
{
    println!("Guess a number > ");
    let secret = rand::thread_rng().gen_range(1 .. 10);

    loop
    {
        println!("Input your guess");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed");
        let guess:i32 = guess.trim().parse().expect("Failed");

        if guess == secret
        {
            println!("You win!");
            break;
        }
        else
        {
            println!("Try again!");

            if guess > secret
            {
                println!("You guessed higher than the secret number");
            }
            else
            {
                println!("You guessed lower than the secret number");
            }
        }
    }
}