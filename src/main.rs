use std::io;

fn main() {
    const PRIME_P: u32 = 31;

    println!("Prime p is {}", PRIME_P);

    let tup : (i32, char) = (1, '1');
    println!("Prime tup is {}", tup.0);

    add_numbers(6, 5);

    let number = {
        let x = 3; //this is the statement
        x+1 //this turns number into expression
    };
    println!("Number is {}", number);
}

fn add_numbers(x: i32, y: i32) {
    println!("The sum is {}", x+y);
}
