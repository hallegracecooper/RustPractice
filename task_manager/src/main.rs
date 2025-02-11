// Function with immutable arguments and a return value
fn add(x: i32, y: i32) -> i32 {
    x + y
}

// Function with mutable arguments, demonstrates ownership transfer
fn change_value(x: &mut i32) {
    *x += 10; // Dereferencing to change the value
}

fn main() {
    // Call to add function
    let sum = add(5, 7);
    println!("Sum: {}", sum); // Should print: Sum: 12

    // Ownership demonstration with mutable variable
    let mut num = 5;
    println!("Before change: {}", num); // Should print: Before change: 5
    change_value(&mut num); // Borrowing mutable reference
    println!("After change: {}", num); // Should print: After change: 15

    // Another function with a return value
    let product = multiply(4, 3);
    println!("Product: {}", product); // Should print: Product: 12
}

// Function to multiply two numbers
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}






