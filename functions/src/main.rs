fn main() {
    another_function(5, 3);
    
    add_function();
    
    let j = five();
    println!("The value of j is: {}", j);
    
    let k = plus_one(6);
    println!("The value of k is: {}", k);
}

// function with 2 parameters
fn another_function(x: i32, y: i32) {
    println!("The value of x and y are: {}, {}", x,y);
}

// simple rust add function
fn add_function() {
    let a = 5;

    let b = {
        let a = 3;
        a + 1
    };

    println!("The value of b is: {}", b);
}

// function with a return value
fn five() -> i32 {
    5
}

// function with a parameter and a return value
fn plus_one(k: i32) -> i32 {
    k + 1
}