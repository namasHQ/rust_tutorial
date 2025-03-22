# Rust Programming Tutorial

This tutorial provides a structured approach to learning Rust programming language, from basic concepts to more advanced topics.

## Prerequisites

- Rust installed on your system (see [Installation Guide](https://www.rust-lang.org/tools/install))
- Basic understanding of programming concepts
- A terminal/command prompt

## Basic Rust Concepts

### What is Rust?
Rust is a systems programming language that focuses on:
- Memory safety without garbage collection
- Thread safety without data races
- Zero-cost abstractions
- Modern tooling and package management

### Key Features
- **Ownership System**: Unique memory management model
- **Type System**: Strong, static typing with type inference
- **Cargo**: Package manager and build system
- **Zero-cost Abstractions**: High-level features with no runtime overhead

## Using the Rust Compiler (rustc)

### Basic Commands
```bash
# Compile a single file
rustc main.rs

# Run the compiled program
./main

# Check code without producing an executable
rustc --check main.rs

# Show compiler version
rustc --version
```

### Common Compiler Flags
- `-O` or `--opt-level`: Optimization level (0-3)
- `-g` or `--debug-info`: Include debug information
- `--test`: Compile and run tests
- `--crate-type`: Specify output type (bin, lib, dylib, etc.)

## Tutorial Structure

### 1. Types (`1_types/`)
- Primitive data types
- Integer types (signed and unsigned)
- Floating-point numbers
- Boolean values
- Character type
- Type inference

### 2. Compound Data Types (`2_compound_data_types/`)
- Arrays
- Tuples
- Slices
- Strings and string slices
- Ownership and borrowing basics

### 3. Functions (`3_functions/`)
- Function definition and calling
- Parameters and return values
- Expressions vs Statements
- Block expressions
- Function overloading

## Best Practices

1. **Naming Conventions**
   - Use snake_case for functions and variables
   - Use PascalCase for types and traits
   - Use SCREAMING_SNAKE_CASE for constants

2. **Code Organization**
   - One concept per file
   - Clear module structure
   - Proper documentation

3. **Error Handling**
   - Use Result and Option types
   - Handle errors explicitly
   - Use ? operator for propagation

## Getting Started

1. Clone this repository
2. Navigate to each tutorial directory
3. Read the comments in the code
4. Try modifying the examples
5. Compile and run the code

## Additional Resources

- [Rust Official Documentation](https://www.rust-lang.org/learn)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Reference](https://doc.rust-lang.org/reference/)
- [Rust Playground](https://play.rust-lang.org/)

## Contributing

Feel free to contribute to this tutorial by:
1. Adding more examples
2. Improving documentation
3. Fixing errors
4. Adding new topics

## License

This tutorial is open source and available under the MIT License. 