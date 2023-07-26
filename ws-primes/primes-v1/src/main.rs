fn is_prime(number: u32) -> bool {
    for x in 2..number {
        if number % x == 0 {
            return false;
        }
    }

    true
}

fn calculate_primes(max: u32) -> Vec<u32> {
    let mut numbers = Vec::new();
    for number in 2..max {
        if is_prime(number) {
            numbers.push(number);
        }
    }
    numbers
}

fn main() {
    const MAX: u32 = 1000;

    println!("Prime numbers up to {MAX}:");
    let primes = calculate_primes(MAX);
    for number in primes {
        println!("{number}");
    }
}
