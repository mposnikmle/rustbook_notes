// using rand crate
extern crate rand;

// “To obtain user input and then print the result as output, we need to bring the io (input/output) library into scope”
use std::io;
// used for comparing 2 values
use std::cmp::Ordering;
//add use statement for rand as well
use rand::Rng;

// entry point
fn main() {
    // printing the game prompt
    println!("Guess the number!");

    // rand::thread_rng() is the random number generator, .gen_range is the range
    // the range is inclusive on the lower bound but exclusive on the upper bound
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        // storing user input in a string type
        let mut guess = String::new();

        // handling user input
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // guess is shadowing the previous value of guess with this new one
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };
        // prints out string that users input is in
        println!("You guessed: {}", guess);

        // comparing guess and secret_number
        // match expression made up of arms & patterns
        match guess.cmp(&secret_number) {
            Ordering::Less      => println!("Too small!"),
            Ordering::Greater   => println!("Too big!"),
            Ordering::Equal     => {
                println!("You win!");
                break;
            }
        }
    }
}


// My notes
    // ! About Rust Variables
        // * immutable by default
        // * mut changes it to mutable
        // * string type provided by the standard library - growable UTF-8 encoded bit of text
    // ! About Associated Functions & References
    // * double colon '::' indicates an associated function
    // * associated functions also called static methods in other langs
    // ? .readline(&mut guess) “The '&' indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times" (also immutable by default)
    // * readline takes the input and puts it into string form and stores it in reference of guess
    // * .expect is a Result type which is an enum with the variants Ok or Err
    // ! Other Methods & Keywords
    // * trim method on a string will eliminate any whitespace at the beginning and end
    // * parse method on a string parses a string into some kind of number
    // * loop keyword gives an infinite loop
    // * break keyword exits a loop
    // * '_' is used as a catchall value