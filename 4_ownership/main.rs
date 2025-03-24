// C, C++ -> Manual Memory Management Control issue
// Java -> Garbage Collector -> Slow
// Rust -> Ownership -> Fast

// Ownership Rules -> Memory Safety
// 1. Each value in Rust has a variable that is called its owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.

// ================================
// Example: Each value has one owner
// fn main() {
//     let s1 = String::from("RUST"); // s1 is the owner of the value
//     let len = calculate_length(&s1); // s1 is borrowed
//     println!("Length of '{}' is {}", s1, len);
// }
// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// ================================
// Example: There can only be one owner at a time
// fn main() {
//     let s1 = String::from("RUST"); // s1 is the owner of the value
//     let s2 = s1; // s1 is moved to s2, s1 is no longer the owner of the value
//     // println!("s1: {}", s1); // This will cause an error because s1 is no longer the owner of the value
//     println!("s2: {}", s2); // This will work because s2 is the owner of the value
// }

// ================================
// Example: When a variable goes out of scope, the value will be dropped
// fn main() {
//     let s1 = String::from("RUST");
//     let len =  calculate_length(&s1);
//     println!("Length of '{}' is {}", s1, len);
// }
// s1 goes out of scope and the value will be dropped
// fn calculate_length(s: &String) -> usize {
//     s.len()
// }
// fn printLost(s: &String) {
//     println!("{}", &s1);
// }
// This will cause an error because s1 is no longer the owner of the value
