use std::io;

// Function to find prime numbers using Sieve of Eratosthenes
fn sieve_of_eratosthenes(limit: u64) -> Vec<bool> {
    let mut sieve = vec![true; (limit + 1) as usize];
    sieve[0] = false;
    sieve[1] = false;

    let sqrt_limit = (limit as f64).sqrt() as u64;
    for num in 2..=sqrt_limit {
        if sieve[num as usize] {
            for multiple in ((num * num)..=(limit)).step_by(num as usize) {
                sieve[multiple as usize] = false;
            }
        }
    }

    sieve
}

fn main() {
    println!("Prime Calculator using Sieve of Eratosthenes in Rust");
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

    let sieve = sieve_of_eratosthenes(num);
    if num >= 2 && sieve[num as usize] {
        println!("Prime");
    } else {
        println!("Not Prime");
    }
}
