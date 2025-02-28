mod extract_numbers;
mod get_input_on_github;
mod fibbonacci_calculator;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use fibbonacci_calculator::fibonacci;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // println!("Hello, world!");
    
    let args: Vec<String> = env::args().collect();

    let enable_fib = args.get(1).map_or(false, |arg| arg == "true");
    let max_threshold: u128 = args.get(3).and_then(|arg| arg.parse().ok()).unwrap_or(100);
    let users_input: u128 = args.get(2).and_then(|args| args.parse().ok()).unwrap_or(0);
    // let max_threshold = extract_nums("Hello I will 23.8 like to give you 50.0 thousand");

    // let max_threshold = max_threshold[0] as u128;
    
    println!("The Fibbot Enable_Status: {}",enable_fib);
    println!("The maximum threshold Value is : {}",max_threshold);
    println!("The users input is : {users_input}");


    if enable_fib && users_input <= max_threshold {
        println!("Printing fibonaci Series up to {users_input}th index");
        for (_, element) in fibonacci(users_input.try_into().unwrap()).iter().enumerate(){
            print!("{} ",element);
        }
    }
    println!();
    
    let files = octocrab::instance().pulls("Jagoum", "FibBot").list_files(1).await?;
    println!("{:?}",files);
    Ok(())

    
    
    

}

use octocrab::models::CommentId;
use octocrab::models::pulls::Comment;
use reqwest::Url;

 async fn run() -> octocrab::Result<Comment> {
    let octocrab = octocrab::Octocrab::default();
    let _ = octocrab.pulls("Jagoum", "fibbot").comment(CommentId(21)).delete();
    let _ = octocrab.pulls("Jagoum", "fibbot").comment(CommentId(42)).update("new comment");
    let comment = octocrab.pulls("Jagoum", "fibbot").comment(CommentId(42)).get().await;

    comment
 }
