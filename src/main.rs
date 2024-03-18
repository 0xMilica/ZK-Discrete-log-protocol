use std::io;

fn main() {
    const PRIME_P: u32 = 31;

    println!("Prime p is {}", PRIME_P);

    let tup : (i32, char) = (1, '1');
    println!("Prime tup is {}", tup.0);

    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("Prime arr is {}", arr[2]);

    let cond = (2 as f32) < 3.5;

    let cond2 = !(cond && false || cond);

    println!("Condition is {}", cond2);

    let day = "Friday";
    let food = "Najlepse zelje";

    if day == "Friday" {
        println!("Food is {}", food);
    }
    else { println!("Danas je dijeta, nista od slatkog :D"); }

    add_numbers(6, 5);

    let number = {
        let x = 3;
        x+1
    };
    println!("Number is {}", number);
}

fn add_numbers(x: i32, y: i32) {
    println!("The sum is {}", x+y);
}
