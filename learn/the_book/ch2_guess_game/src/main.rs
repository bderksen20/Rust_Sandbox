// billy derksen 7/21 - guessing game

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number...");

    let secret_num = rand::thread_rng().gen_range(1..101);   //-- uses rand gen seeded by os to gen rand in range 1-100

    //println!("Secret number is: {}", secret_num);
    loop{
        
        println!("Input guess: ");

        let mut guess = String::new();           //-- let mut creates mutable var (immutable default)
                                                 //-- ::new() - associated fxn of type String, not
                                                 //instance of string 

        io::stdin()
           . read_line(&mut guess)               //-- read input and append to & - reference to guess string -- returns: io::Result (Ok or Err) 
            .expect("Failed to read line");      //-- if Err returned, .expect crashes and reports error

        //let guess: u32 = guess.trim().parse().expect("Please type a number!");  //-- shadowing: allows reuse of "guess" name in type conversion
                                                                                  //-- let guess: u32 - specifies that we want an integer type
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,                          //-- "_" catchall that directs any errors to just continue (goto next loop iteration)
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_num){                           //-- .cmp returns Ordering variant, match decides what to do with it (basically enum switch statement)
            Ordering::Less => println!("Too small!\n"),
            Ordering::Greater => println!("Too big!\n"),
            Ordering::Equal => {
                println!("Correct, you win!\n");
                break;
            }
        }
    }
}
