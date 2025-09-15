fn main() {
    let s = String::from("Hello darkness my old friend");
    let index = first_word(&s);
    let word = first_word(&s);
    println!("First word ends at {index} position");
    // s.clear();
    // println!("{s.t}");
    println!("{word}");
    let word = first_word(&s[0..6]);
    println!("#{word}");
    // &s[..2] == &s[0..2]
    // &s[3..len] == &s[3..]
    // entire strring = &s[..] == &s[0..len]
    let other_slice = [1, 2, 3, 4, 5];
    let slice = &other_slice[1..3];
    assert_eq!(slice, &[2, 3]);
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }
// fn first_word(s: &String) -> &str {
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
