mod fibbonacci_calculator;
use std::env;

use fibbonacci_calculator::fibonacci;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();

    let enable_fib = args.get(1).map_or(false, |arg| arg == "true");
    let max_threshold: u128 = args.get(2).and_then(|arg| arg.parse().ok()).unwrap_or(100);

    println!("The Fibbot Enable_Status: {}",enable_fib);
    println!("The maximum threshold Value is : {max_threshold}");

    if enable_fib {
        println!("Printing fibonaci Series");
        for (_, element) in fibonacci(max_threshold).iter().enumerate(){
            print!("{} ",element);
        }
    }
}

