// Compound Data Types in Rust
// This file demonstrates how to work with collections of values in Rust
// Compound types allow you to group multiple values into one type

// Arrays
fn main(){
    // Arrays
    // Fixed-size collection of elements of the same type
    // Size must be known at compile time
    // Stored on the stack
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Number Array: {:?}", numbers);

    // Arrays must contain elements of the same type
    // This would cause a compile error:
    // let mix = [1, 2, "apple", true];  // ❌ Different types not allowed

    // Array of string slices (&str)
    // &str is a string slice type, which is a reference to string data
    let fruits: [&str; 3] = ["apple", "banana", "orange"];
    println!("Fruits Array 1st: {}", fruits[0]);
    println!("Fruits Array 2nd: {}", fruits[1]);
    println!("Fruits Array 3rd: {}", fruits[2]);

    // Tuples
    // Fixed-size collection of elements of different types
    // Useful for returning multiple values from a function
    let person: (String, i32, bool) = ("John".to_string(), 25, true);
    println!("Person Tuple: {:?}", person);
    
    // Tuples can contain other compound types
    let mix_tuple: (i32, bool, char, [i32; 5]) = (1, true, 'a', [1, 2, 3, 4, 5]);
    println!("Mix Tuple: {:?}", mix_tuple);
    
    // Type inference: Rust can infer tuple types
    let mix_tuple2 = (1, true, 'a', [1, 2, 3, 4, 5]);
    println!("Mix Tuple 2: {:?}", mix_tuple2);

    // Slices
    // A reference to a contiguous sequence of elements in a collection
    // Zero-cost abstraction: no runtime overhead
    // Useful for borrowing parts of collections without copying
    let number_slices: &[i32] = &[1, 2, 3, 4, 5];
    println!("Slice: {:?}", number_slices);

    // String slices (&str)
    // Immutable view into string data
    // More efficient than String for read-only operations
    let animal_slices: &[&str] = &["dog", "cat", "bird"];
    println!("Animal Slices: {:?}", animal_slices);

    // String Type (String)
    // Growable, heap-allocated string
    // Owned type (has ownership of the data)
    let mut stone_cold: String = String::from("Hell, ");
    println!("Stone Cold Says: {}", stone_cold);
    stone_cold.push_str("Yeah!");  // Append string slice
    println!("Stone Cold Says: {}", stone_cold);

    // String Slicing
    // Create a view into part of a String
    // Syntax: &string[start..end] (end is exclusive)
    let string: String = String::from("Hello, world!");
    let slice: &str = &string[0..5];  // "Hello"
    println!("Slice: {}", slice);
}

// Function demonstrating string slice parameter
// Using &str is more flexible than String as it can accept both String and &str
fn print(slice: &str) {
    println!("Slice: {}", slice);
}