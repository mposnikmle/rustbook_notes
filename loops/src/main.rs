fn main() {
    // infinite loop
    // loop {
    //     println!("Again!");
    // }
    
    // number starts at 3, while number doesn't equal 0
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }
    println!("LIFTOFF!!");

    while_loop();

    for_loop();

    print_range_rev();
}

// error prone, because index condition has to match elements in arr
fn while_loop() {
    let arr = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", arr[index]);

        index = index + 1;
    }
}

// this fn is faster and more safe
fn for_loop() {
    let arr = [10, 20, 30, 40, 50];

    for element in arr.iter() {
        println!("The value is: {}", element);
    }
}

// fn that prints a range of numbers in reverse
fn print_range_rev() {
    // first number inclusive, second number exclusive
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("TAKE IT TO THE MOON TAKE IT TO THE STARS!")
}