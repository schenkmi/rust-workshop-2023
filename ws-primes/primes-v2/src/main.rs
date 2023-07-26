fn is_prime(number: u32) -> bool {
    let mut divisor = 2;
    loop {
        if number / divisor < divisor {
            break true;
        }
        if number % divisor == 0 {
            break false;
        }
        divisor += 1;
    }
}

fn calculate_primes(max: u32) -> Vec<u32> {
    let mut numbers = Vec::with_capacity(max as usize);
    for number in 2..max {
        if is_prime(number) {
            numbers.push(number);
        }
    }
    numbers
}

fn measure_duration<T>(func: impl Fn() -> T) -> T {
    let start = std::time::Instant::now();
    let result = func();
    let end = std::time::Instant::now();

    println!("Duration: {} us", (end - start).as_micros());
    result
}

fn main() {
    const MAX: u32 = 1000;

    let primes = measure_duration(|| calculate_primes(MAX));

    println!("Prime numbers up to {MAX}:");
    for number in &primes {
        println!("{number}");
    }
}
