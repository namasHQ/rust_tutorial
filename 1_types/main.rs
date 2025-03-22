// Rust's Type System
// This file demonstrates Rust's primitive data types and their characteristics
// Rust is a statically typed language, meaning all types are checked at compile time

// Integer Types
// Rust provides both signed (can be negative) and unsigned (only positive) integers
// The number in the type (e.g., i32) represents the number of bits used to store the value
// Signed types (i8, i16, i32, i64, i128): Can store both positive and negative numbers
// Unsigned types (u8, u16, u32, u64, u128): Can only store positive numbers
// Default integer type is i32 (most efficient for modern processors)

fn main(){
    // Basic integer examples
    let x: i32 = -42;  // Signed 32-bit integer
    let y: u64 = 100;  // Unsigned 64-bit integer
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);

    // Integer size limits
    // i32 range: -2^31 to 2^31 - 1 (-2,147,483,648 to 2,147,483,647)
    // i64 range: -2^63 to 2^63 - 1 (-9,223,372,036,854,775,808 to 9,223,372,036,854,775,807)
    let max_i32: i32 = 2_147_483_647;
    let max_i64: i64 = 9_223_372_036_854_775_807;
    // Increasing these numbers will give error
    println!("i32 max value: {}", max_i32);
    println!("i64 max value: {}", max_i64);

    // Floating-Point Types
    // f32: 32-bit float (single precision)
    // f64: 64-bit float (double precision) - default float type in Rust
    let pi: f64 = 3.14;
    println!("pi value: {}", pi);

    // Boolean Type
    // Only two possible values: true or false
    // Takes 1 byte of memory
    let is_snowing: bool = true;
    println!("is it snowing? {}", is_snowing);

    // Character Type (char)
    // Represents a Unicode scalar value (4 bytes)
    // Can store any Unicode character, not just ASCII
    let letter: char = 'a';
    println!("what is first letter? {}", letter);
}
