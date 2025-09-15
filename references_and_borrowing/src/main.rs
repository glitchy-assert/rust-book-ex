// The rules of references
// 1. At any given time, you can have either one mutable reference or any
// number of immutable references
// 2. References must always be valid
fn main() {
    let mut s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
    change(&mut s1);
    println!("{s1}");

    let r1 = &s1;
    let r2 = &s1;
    // let r3 = &mut s1;

    println!("{r1}, {r2}");
    let r3 = &mut s1;
    println!("{r3}");
    let a = no_dangle();
    println!("{a}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
