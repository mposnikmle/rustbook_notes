fn main() {
    // Variables
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    
    // Character type
    let c = 'c';
    let d:char = 'd';
    println!("{}",c);
    println!("{}",d);

    // Tuple
    let tup: (i32, f64, u8) = (300, 7.3, 1);
    let first_index = tup.0; // prints 300
    println!("{}",first_index);

    // Array
    let arr = [1,2,3,4,5];
    let first_arr = arr[0]; // prints 1
    println!("{}", first_arr);
}
