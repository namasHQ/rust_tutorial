// Functions in Rust
// This file demonstrates how to work with functions, expressions, and statements in Rust
// Functions are the primary way to organize code in Rust
// They can take parameters, return values, and be called from other functions

fn main() {
    // Function calls
    hello_world();
    tell_height(180);
    human_id("John", 20, 180.0);

    // Block expressions
    // A block can be used as an expression if it returns a value
    // The last expression in a block (without semicolon) becomes its return value
    let _x = {
        let price = 5;
        let quantity = 10;
        price * quantity  // No semicolon here - this is the return value
    };
    println!("The result of the expression is {}", _x);

    // Function with return value
    let y = add(5, 10);
    println!("The result of the addition is {}", y);
    println!("The result of the function is {}", add(5, 10));

    // BMI calculation example
    let weight = 70.0;
    let height = 180.0;
    let bmi = calculate_bmi(height, weight);
    println!("Your BMI is {:.2}", bmi);
}

// ================================
// Basic function without parameters
// Functions can be called before they are defined (hoisting)
fn hello_world() {
    println!("Hello, rust!");
}

// ================================
// Function with a single parameter
// Parameters must have their type explicitly declared
fn tell_height(height: u32) {
    println!("You are {}cm tall", height);
}

// ================================
// Function with multiple parameters of different types
// &str is a string slice (reference to string data)
fn human_id(name: &str, age: u32, height: f32) {
    println!("You are {} and {} years old and {}cm tall", name, age, height);
}

// ================================
// Function with return value
// The return type is specified after the arrow (->)
// The last expression (without semicolon) becomes the return value
fn add(a: i32, b: i32) -> i32 {
    a + b  // No semicolon here - this is the return value
}

// ================================
// Expressions vs Statements in Rust
// Expressions:
// - Return a value
// - Examples:
//   * Literals: 5, "hello", true
//   * Function calls: add(5, 10)
//   * Blocks: { let x = 5; x + 1 }
//   * Control flow: if condition { 5 } else { 10 }
//
// Statements:
// - Do not return a value
// - End with semicolon (;)
// - Examples:
//   * Variable declarations: let x = 5;
//   * Function definitions: fn foo() {}
//   * Control flow: if condition { ... }
//   * Expressions with semicolon: x + 1;

// ================================
// BMI Calculator Function
// Demonstrates:
// - Multiple parameters
// - Return value
// - Mathematical operations
// - Type conversion
fn calculate_bmi(height_cm: f64, weight_kg: f64) -> f64 {
    let height_m = height_cm / 100.0;  // Convert cm to meters
    weight_kg / height_m.powi(2)       // BMI formula: weight / height²
}


