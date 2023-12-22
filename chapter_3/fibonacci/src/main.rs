use std::io;

fn main() {
    // Prompt the user for input
    println!("Enter the number of Fibonacci numbers to generate:");

    // Read user input
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");

    // Convert the input to an unsigned integer
    let user_input: u32 = match user_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid non-negative integer");
            return;
        }
    };

    // Generate and print the Fibonacci sequence
    let mut fib_sequence = Vec::new(); // storing output in a vector
    for i in 0..user_input {
        fib_sequence.push(fibonacci(i));
    }

    println!("Fibonacci sequence up to {}:", user_input);
    println!("{:?}", fib_sequence);
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}
