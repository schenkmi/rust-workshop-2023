use std::io;

// Function to check if a number is prime
fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false;
    }
    
    for i in 2..=((num as f64).sqrt() as u64) {
        if num % i == 0 {
            return false;
        }
    }
    
    true
}

fn main() {
    println!("Prime Calculator in Rust");
    println!("Enter a positive integer:");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let num: u64 = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid input. Please enter a positive integer.");
            return;
        }
    };

    if is_prime(num) {
        println!("Prime");
    } else {
        println!("Not Prime");
    }
}
