use std::time::{Duration, Instant};

// Function to measure the execution time of a closure
fn measure_execution_time<F, R>(function: F) -> Duration
where
    F: FnOnce() -> R,
{
    let start_time = Instant::now();
    function();
    let end_time = Instant::now();

    end_time - start_time
}

fn is_prime_1(number: u32) -> bool {
    if number <= 1 {
        return false;
    }
    for a in 2..number {
        if number % a == 0 {
            return false; // if it is not the last statement you need to use `return`
        }
    }
    true // last value to return
}

fn is_prime_2(number: u32) -> bool {
    if number <= 1 {
        return false;
    }

    for i in 2..=((number as f32).sqrt() as u32) {
        if number % i == 0 {
            return false;
        }
    }

    true
}

// Function to find prime numbers using Sieve of Eratosthenes
fn is_prime_3(number: u32) -> Vec<bool> {
    let mut sieve = vec![true; (number + 1) as usize];
    sieve[0] = false;
    sieve[1] = false;

    let sqrt_limit = (number as f32).sqrt() as u32;
    for num in 2..=sqrt_limit {
        if sieve[num as usize] {
            for multiple in ((num * num)..=(number)).step_by(num as usize) {
                sieve[multiple as usize] = false;
            }
        }
    }

    sieve
}

fn main() {
    const MAX: u32 = 1000;
    let mut prime_vector = Vec::new();
    println!("Prime numbers:");

    let start = std::time::Instant::now();

    for number in 2..MAX {
        if is_prime_1(number) == true {
            prime_vector.push(number);
            //println!("{}", number);
        }
    }
    println!(
        "Duration: {}us",
        (std::time::Instant::now() - start).as_micros()
    );

    //for x in &prime_vector {
    //    println!("{x}");
    //}

    // square root
    let start = std::time::Instant::now();

    for number in 2..MAX {
        if is_prime_2(number) == true {
            prime_vector.push(number);
        }
    }
    println!(
        "Duration sqrt: {}us",
        (std::time::Instant::now() - start).as_micros()
    );

    // square root
    let start = std::time::Instant::now();

    let sieve = is_prime_3(MAX);
    println!(
        "Duration sieve of Eratosthenes: {}us",
        (std::time::Instant::now() - start).as_micros()
    );
    let mut cnt = 0;
    for x in sieve {
        if x == true {
            println!("{}", cnt);
        }
        cnt += 1;
    }


    // Example usage: Create a lambda that calculates the factorial of a number and measure its execution time.
    let factorial = |num: u32| {
        let mut result: u32 = 1;
        for i in 1..=num {
            result *= i;
        }
        println!("Factorial of {} is {}", num, result);
    };

    let num_to_calculate = 10;
    let execution_time = measure_execution_time(|| factorial(num_to_calculate));
    println!("Execution time: {:?}", execution_time);


    //let prime_calc = |num: u32, vec: Vec<u32>| {
    //    for number in 2..num {
    //        if is_prime_1(number) == true {
    //            vec.push(number);
    //        }
    //    }
    //}

    //let execution_time = measure_execution_time(|| prime_calc(MAX, prime_vector));
    //println!("Execution time: {:?}", execution_time);


}
