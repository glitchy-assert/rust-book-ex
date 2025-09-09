fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!");
    println!("{s}");
    {
        // other scope
        let b = String::from("bye!");
        println!("{b}");
    }
    {
        let s1 = String::from("hello");
        let s2 = s1;
        println!("{s2}, world!");
    }
    {
        let mut s = String::from("hello");
        s = String::from("ahoy");
        println!("{s}, world!");
    }
    {
        let s1 = String::from("hello");
        let s2 = s1.clone();
        println!("s1 = {s1}, s2 = {s2}");
    }
    {
        let mut x = 5;
        let y = x; // it is on stack, so the value is copied

        x = 10;
        println!("x = {x}, y = {y}");
    }
    let s = String::from("helloooo");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);
    println!("x = {x}");
    // println!("s = {s}");
    let s1 = gives_ownership();
    println!("s1 = {s1}");

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("s3 = {s3}");

    let s1 = String::from("hello world");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
