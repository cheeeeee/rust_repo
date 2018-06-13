use std::io;

fn main() {

    println!("Enter fib number!");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read input!");

   
    let num = input.trim().parse::<f64>().unwrap();

    let fib = ((1.0_f64 + 5.0_f64.sqrt()).powf(num) - (1.0_f64 - 5.0_f64.sqrt()).powf(num)) / (2.0_f64.powf(num) * 5.0_f64.sqrt());

    let output = fib.round() as u64;

    println!("{}", output);
}