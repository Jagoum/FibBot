mod extract_nums;
mod fibbonacci_calculator;
mod get_pull_request;
mod push_comment;
use extract_nums::extract_nums;
use get_pull_request::get_pr;
use std::env;

use fibbonacci_calculator::fibonacci;
use push_comment::post_github_comment;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {


    //.. let repo = en.llv::var("GITHUB_REPOSITORY").expect("GITHUB_REPOSITORY not set");
    // println!("Hello, world!\n{:?}", extract_nums("12.9we 12.0 ,90 72"));

    let args: Vec<String> = env::args().collect();

    let github_token = args.get(1).and_then(|token| Some(token.as_str())).unwrap_or("default");
    let enable_fib = args.get(2).map_or(true, |arg| arg == "true");
    let users_input: u128 = args.get(3).and_then(|args| args.parse().ok()).unwrap_or(0);
    let max_threshold: u128 = args.get(4).and_then(|arg| arg.parse().ok()).unwrap_or(187);

    let owner = "Jagoum"; let repo = "FibBot";
    // let pr_number = args.get(5).and_then(|new| new.parse().ok()).unwrap_or(1);
    let pr_number = args.get(5).and_then(|new| new.parse().ok()).unwrap_or(3);
    
    // let max_threshold = extract_nums("Hello I will 23.8 like to give you 50.0 thousand");
    // let max_threshold =======



        // Here am passing the string as parameter into this funcition that posts to github 

        //This string contains the results of our fibo sequence of the numbers we collected


            

    

    
//= max_threshold[0] as u128;


    

        
    println!("The Fibbot Enable_Status: {}",enable_fib);
    println!("The maximum threshold Value is : {}",max_threshold);
    println!("The users input is : {users_input}");


    // user input is almost useless just for fun

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
           
            println!("Content to be Posted:");

            for &num in &response{
                
                let fib = fibonacci(num);
                string.push_str(format!("- Fibonacci({}) = {:?}\n", num, fib).as_str());
                
            }
            
            println!("{}",string);
            let posted_content = post_github_comment(github_token,owner,repo,pr_number,&string).await;
            match posted_content {
                Ok(_) => {println!("Successufully Posted Content\n")},
                Err(err) => {eprintln!("Failed to Post Content\n{}",err)},
            }
    
        // Here am passing the string as parameter into this funcition that posts to github 
        //This string contains the results of our fibo sequence of the numbers we collected
            
    }


Ok(())
}