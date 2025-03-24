// References and Borrowing
// Safety and Performance
// Borrowing and references are powerful concepts

// Why safety is and why it is important?
// Safety refers to the prevention of bugs and errors (null pointer dereferencing, dangling pointers, buffer overflow, d pointer etc.)

// ================================
// Undersanding References
// Borrow values without transferring ownership
// Cant have multiple owners of the same value
// Immutable references: allows you to read the value of a variable without changing it
// Mutable references: allows you to read and write the value of a variable 
// Create references using & and *

// ================================
// Example: Immutable references
// fn main() {
//     let _x: i32 = 5;
//     // create an immutable reference to x
//     let _r: &i32 = &_x;
//     println!("x: {}", _x);
//     println!("r: {}", _r);
// }

// ================================
// Example: Mutable references
// fn main() {
//    let mut _x: i32 =  5;
//    let _r: &mut i32 = &mut _x;
//    // * is used to dereference the reference and get the value it points to
//    *_r += 1;
//    *_r -= 3;
//    println!("x: {}", _x);
// }

// You can have multiple immutable references to the same value
// But you can only have one mutable reference to the same value

// ================================
// Struct: A data structure that allows you to group multiple fields together under one name

// Demonstration on one mutable and multiple immutable references
fn main() {
    let mut account = BankAccount {
        owner: "Alice".to_string(),
        balance: 1005.55,
    };

    // Immutable borrow to check the balance
    account.check_balance();
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    // In the withdrawal function: we cannot have simultaneously have mutable access to the account to update the balance,
    // and immutable access for reading the owner's name for example
    fn withdraw(&mut self, amount: f64){
        println!("Withdrawing {} from {}", amount, self.owner);
        self.balance -= amount;
    }

    // In the check_balance function: while we are checking the balance, which as immutable access,
    // we cannot have a mutable access to the account to update the balance
    fn check_balance(&self){
        println!("{} has {} in their account", self.owner, self.balance);
    }
}