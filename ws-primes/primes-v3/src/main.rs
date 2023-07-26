fn calculate_primes(max: u32) -> Vec<u32> {
    let mut candidates = Vec::new();
    candidates.resize(max as usize + 1, true);

    let mut primes: Vec<u32> = Vec::with_capacity(max as usize);

    for number in 2..max {
        if candidates[number as usize] {
            primes.push(number);

            let mut multiple = number;
            loop {
                multiple += number;
                if multiple >= candidates.len() as u32 {
                    break;
                }
                candidates[multiple as usize] = false;
            }
        }
    }
    primes
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
    for number in primes {
        println!("{number}");
    }
}
