fn main() {
    const MAX: u32 = 1000;

    let primes = (2..=MAX)
        .filter(|number| (2..*number).all(|divisor| (number % divisor) != 0))
        .collect::<Vec<_>>();

    println!("Prime numbers up to {MAX}:");
    for number in primes {
        println!("{number}");
    }
}
