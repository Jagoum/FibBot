mod fibbonacci_calculator;
use std::env;

use fibbonacci_calculator::fibonacci;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();

    let enable_fib = args.get(1).map_or(false, |arg| arg == "true");
    let max_threshold: u128 = args.get(3).and_then(|arg| arg.parse().ok()).unwrap_or(100);
    let users_input: u128 = args.get(2).and_then(|args| args.parse().ok()).unwrap_or(0);

    println!("The Fibbot Enable_Status: {}",enable_fib);
    println!("The maximum threshold Value is : {max_threshold}");
    println!("The users input is : {users_input}");

    if enable_fib && users_input <= max_threshold {
        println!("Printing fibonaci Series");
        for (_, element) in fibonacci(users_input).iter().enumerate(){
            print!("{} ",element);
        }
    }
}

