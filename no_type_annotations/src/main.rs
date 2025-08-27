use std::io;

fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The guess is: {guess}");

    let x = 2.0; // f64
    let y: f32 = 3.0;// f32

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let ramainder = 43 % 5;

    let t = true;
    let f: bool = false;

    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 3);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    

    // array
    // allocated on stack
    // fixed length
    let a = [1,2,3,4,5,6,7,8,9,10];
    let months = ["January", "February", "March", "April",
                  "May", "June", "July", "August",
                  "September", "October", "November", "December"];
    
    //     [type; length]
    let a: [i32; 5] = [1,2,3,4,5];

    let first = a[0];
    let second = a[1];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    println!("The value of the element at index {index} is: {element}");

}
