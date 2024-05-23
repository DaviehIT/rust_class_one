use std::io;

fn main() {
    let mut num1_str = String::new();
    let mut num2_str = String::new();

    // Get first input
    println!("Enter the first number: ");
    io::stdin().read_line(&mut num1_str).expect("Failed to read input");

    // Get second input
    println!("Enter the second number: ");
    io::stdin().read_line(&mut num2_str).expect("Failed to read input");

    // Remove trailing newline from inputs (if any)
    let num1_str = num1_str.trim();
    let num2_str = num2_str.trim();

    // Parse inputs to numbers
    let num1: f64 = match num1_str.parse() {
        Ok(val) => val,
        Err(_) => {
            println!("Error: Invalid input for first number.");
            return;
        }
    };

    let num2: f64 = match num2_str.parse() {
        Ok(val) => val,
        Err(_) => {
            println!("Error: Invalid input for second number.");
            return;
        }
    };

    // Check for division by zero
    if num2 == 0.0 {
        println!("Error: Cannot divide by zero.");
    } else {
        let result = num1 / num2;
        println!("The result of {} / {} is: {}", num1, num2, result);
    }
}

