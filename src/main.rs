use std::io;

fn main() {
    const PRIME_P: u32 = 31;

    println!("Prime p is {}", PRIME_P);

    add_numbers(6, 5);

    let number = {
        let x = 3; //this is the statement
        x+1 //this turns number into expression
    };
    println!("Number is {}", add_numbers(5, 6));
}

fn add_numbers(x: i32, y: i32) -> i64 {
    return (x + y) as i64;
}

//fn for key generation

//fn for generating subgroups

//fn for applying CRT

//fn for comparing decrypted messages