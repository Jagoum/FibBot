mod extract_nums;
mod get_pull_request;
mod push_comment;
mod fibbonacci_calculator;
use std::env;
use get_pull_request::get_pr;
use extract_nums::extract_nums;

use fibbonacci_calculator::fibonacci;
use push_comment::post_comment;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {


    // let repo = env::var("GITHUB_REPOSITORY").expect("GITHUB_REPOSITORY not set");
    
    println!("Hello, world!\n{:?}", extract_nums("12.9we 12.0 ,90 72"));
    
    let args: Vec<String> = env::args().collect();
    let pr_number = args.get(4).and_then(|new| new.parse().ok()).unwrap_or(1);

    let enable_fib = args.get(1).map_or(true, |arg| arg == "false");
    let max_threshold: u128 = args.get(3).and_then(|arg| arg.parse().ok()).unwrap_or(187);
    let users_input: u128 = args.get(2).and_then(|args| args.parse().ok()).unwrap_or(0);
    // let max_threshold = extract_nums("Hello I will 23.8 like to give you 50.0 thousand");

    // let max_threshold = max_threshold[0] as u128;
    
    println!("The Fibbot Enable_Status: {}",enable_fib);
    println!("The maximum threshold Value is : {}",max_threshold);
    println!("The users input is : {users_input}");


    if enable_fib {
        // println!("Testing \nPrinting fibonaci Series up to {users_input}th index");
        // for (_, element) in fibonacci(max_threshold).iter().enumerate(){
        //     print!("{} ",element);
        // }
        println!();
            // let nums = get_pr(pr_number);
            // Here am converting the output of the fibonacci of those multiple numbers into a string 
            // This is so that i can parse it to my post comment which takes an &str
            // So here i use nested for loops which is not really the best
            let mut string: String = String::from("##Pull Content: ");
            let response = get_pr(pr_number).await;
           
            for &num in &response{
                let fib = fibonacci(num);
                string.push_str(format!("- Fibonacci({}) = {:?}\n", num, fib).as_str());
            }
            
        
        // Here am passing the string as parameter into this funcition that posts to github 
        //This string contains the results of our fibo sequence of the numbers we collected
            let posted_content = post_comment(pr_number.try_into().unwrap(),string.as_str());
        
        
            println!("Content to be Posted\n{:?}",posted_content.await.unwrap());
    }
        
    
Ok(())
}