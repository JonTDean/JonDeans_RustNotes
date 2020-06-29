use std::io;

fn main() {

    let input = grab_input();
    // println!("{}", input);
    println!("The final number of given sequence {} is {}", input, fibonnaci_formula(input));
}

// Grabs the user input and converts to a number
fn grab_input() -> u128{
    // Creates a string to store the user Input
    let mut get_length = String::new();

    println!("How deep would you like to traverse the Fibonnaci Sequence?");

    // Grabs the user input and stores into a mutable reference
    io::stdin().read_line(&mut get_length)
        .expect("Failed to read line.");
    
    // Removes all whitespace and converts to an integer
    let get_length: u128 = match get_length.trim().parse(){
        Ok(num) => num,          // If able to convert to int, stores the user input as the value for get_Length
        Err(_) => grab_input(),  // If not correct recalls statement
    };

    return get_length;
}

// Speaks in the fibonnaci lemon language
fn fibonnaci_formula(num_length: u128) -> u128{
    let mut f1 = 0;             // Stores the first sequence of Fibo
    let mut f2 = 1;             // Stores the second sequence of Fibo

    // Iterates from 1 to the value of num_length
    for _x in 1..num_length+1{  // Add 1 since the for loop stops on the iteration before the correct one
        let temp = f2;          //  Stores the value of f2 into temp
        f2 += f1;               //  The value of f2 is equal to f1+f2
        f1 = temp;              //  The final value of the Fibonnaci Sequence is stored into f1
    }

    return f1;                  // Returns the value of Fibo
}


/*
Sources:

https://en.wikipedia.org/wiki/Fibonacci_number

*/