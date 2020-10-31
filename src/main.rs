use std::io;

fn main() {
    println!("\n[#] - Calc on Rust\n");

    let mut first_input = String::new();
    let mut second_input = String::new();

    println!("[!] ~> Type the first number");
    io::stdin().read_line(&mut first_input).ok().expect("Couldn't get the first number");

    println!("\n[!] ~> Type the second number");
    io::stdin().read_line(&mut second_input).ok().expect("Couldn't get the second number");

    let first_number: u32 = first_input.trim().parse().expect("Wanted a number");
    let second_number: u32 = second_input.trim().parse().expect("Wanted a number");

    let add = first_number + second_number;
    let sub = first_number - second_number;
    let div = first_number / second_number;
    let mul = first_number * second_number;

    println!("\n- Addition: {} \n- Subtraction: {}  \n- Division: {} \n- Multiplication: {}", add, sub, div, mul);

}
