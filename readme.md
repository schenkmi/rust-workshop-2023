# Rust Workshop 2023

source "$HOME/.cargo/env"



cargo new hello
cargo run


Bibliothek
cargo new --lib example

Execute unit tests
cargo test

# Cargo.toml

## Dependencies

[dependencies]
chrono = "0.4"
regex = "1.9.1"

Number can be incomplete `regex = "1.9"`

# Variables

let a = 128u8;
let b = "Hello";
let d = [0i32, 1i32, 2i32];

let a: u8 = 123;
let b: &str = "Hello";
const PI: f32 = 3.14159265;



fn get_value() -> i32 {
    5000
}
let a = get_value();


fn print_my_name(name: &str) {
    println!("My name is {name}");
}

Last is return and have no ";"
fn square(input: i64) -> i64 {
    input * input
}

fn division(x: u32, y: u32) -> u32 {
    if y == 0 {
        0
    }

    x / y
}





fn division(x: u32, y: u32) -> u32 {
    if y == 0 {
        0
    } else {
        x / y
    }
}

fn main() {
    println!("Hello World");
    
    let g: u32 = division(10, 10);
}



Unused variables
use "_" like _test

Output variable for debuging
let a = [1, 2, 3];
dbg!(a);

Output var
let a = 10
println!("Value {a}");
println!("Value {}", a);
println!("Item {} of {}", index + 1, items.len());


cargo build --example discover

cargo build --example discover







