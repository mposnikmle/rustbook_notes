fn main() {
    let number = 3;

    // checking if number is less than 5
    if number < 5 {                     // if branches also sometimes called arms
        println!("condition was true"); // condition is true so this will print
    } else {
        println!("condition was false");
    }

    println!("{}", number_divisible(6));

    checker();
}

// function that takes a number as a parameter and prints 1 of the 4 strings based on input
fn number_divisible(x: i32) -> &'static str {
    // after first true condition is met Rust will only execute that one and not check the rest
    if x % 4 == 0 {
        return "number is divisible by 4";
    } else if x % 3 == 0 {
        return "number is divisible by 3";
    } else if x % 2 == 0 {
        return "number is divisible by 2";
    } else {
        return "number is not divisible by 2, 3, or 4";
    }
}

// function with if expression assigned to a variable
fn checker() {
    let condition = true;
    let y = if condition {
        5
    } else {
        6
    };
    println!("The value of y is: {}", y);
}