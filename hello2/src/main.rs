use std::io;

fn get_value() -> u32 {
    123
}

fn hello_michael<'life>() -> &'life str {
    return "Michael Schenk";
}

fn print_out(name: &str) {
    println!("Hello, world! {name}");
}

fn enter_name() -> String {
    let mut my_name = String::new();
    println!("Enter your name followed by <ENTER>");
    io::stdin()
        .read_line(&mut my_name)
        .expect("Failed to read line");
    my_name
}

fn main() {
    let a = get_value();
    let name = hello_michael();
    let name_len = name.len();

    println!("Hello, world! {a}");
    println!("Hello, world! {name} len {name_len}");

    print_out(name);

    let apples = 5; // immutable
                    //let mut bananas = 5; // mutable
    println!("apples = {apples} and apples + 2 = {}", apples + 2);

    let mut my_name = String::new();
    println!("Enter your name followed by <ENTER>");
    io::stdin()
        .read_line(&mut my_name)
        .expect("Failed to read line");
    println!("Your name is: {my_name}");

    println!("Enter your name followed by <ENTER>");
    let my_name2 = enter_name();
    println!("Your name is: {my_name2}");
}
