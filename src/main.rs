use std::io::{self, Write};

fn main() {
    // Prompt the user for the original salary
    print!("Enter your original salary: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let original_salary: f64 = input.trim().parse().expect("Invalid input");

    // Prompt the user for the increase amount
    print!("Enter your increase amount: ");
    io::stdout().flush().expect("Failed to flush stdout");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let increase_amount: f64 = input.trim().parse().expect("Invalid input");

    // Calculate the increased salary
    let increased_salary = original_salary + increase_amount;

    // Calculate the percentage increase
    let percentage_increase = (increase_amount / original_salary) * 100.0;

    // Print the result
    println!("Original Salary: {:.2}", original_salary);
    println!("Increase Amount: {:.2}", increase_amount);
    println!("Increased Salary: {:.2}", increased_salary);
    println!("Percentage Increase: {:.2}%", percentage_increase);
}
