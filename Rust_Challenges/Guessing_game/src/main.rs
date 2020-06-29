use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){

    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1, 10);
    // println!("The secret number is: {}!", secret_number);

    // If the number grabbed isn't correct then the Input fetch is looped until the user inputs the Correct number
    loop{   
        println!("Please input your guess.");

        //  Creates a Mutable Variable bound to a new Instance of an Empty String.
        let mut guess = String::new();

        //  Also able to write the io::stdin as std::io::stdin as a direct reference to the Class Function instead of a lazy reference.
        io::stdin().read_line(&mut guess)   //  Reads incoming data and mutates the -reference of (&mut)- var::guess as the incoming input.  
            .expect("Failed to read line"); //  If the line isn't readable then an error message is thrown.

        // Strips the spaces from the input grabbed on line 19 and then converts that input into a Number
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,                 // If a number is found then it gets returned into the match sequence on line 32
            Err(_) => continue,             // If no number is found then nothing happens
        };   
        
        println!("You guessed: {}", guess); //  Prints out the current value of var::guess

        // Compares the input to the randomly generated number

        // Done Via If Statement:
        // if guess == secret_number{
        //     println!("Correct Guess!");
        //     break;
        // }else if guess != secret_number{
        //     println!("Incorrect Number!")
        // }

        // Done Via Match
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("Correct!");
                break;                      // Exits out of the loop if the user enters the correct answer
            }
        }

    }
}