mod extract_nums;
mod fibbonacci_calculator;
use std::env;
use reqwest::Client;
use extract_nums::extract_nums;
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

    let nums = get_pr();
    // Here am converting the output of the fibonacci of those multiple numbers into a string 
    // This is so that i can parse it to my post comment which takes an &str
    // So here i use nested for loops which is not really the best
    let string: String = {
        let mut  str = String::from("Result Of Fibonacci Sequence pr_content: ");
        for i in nums.await.iter(){
        let fib = fibonacci(*i);
        for i in fib {

            str.push((i as u8) as char );
        }
    };
    str
};

// Here am passing the string as parameter into this funcition that posts to github 
//This string contains the results of our fibo sequence of the numbers we collected
    let _posted_content = post_comment(string.as_str());
    println!("Content to be Posted\n{}",string);
    
Ok(())
}



/// This function get the content from a pull request and then parse it to extract numbers
async fn get_pr() -> Vec<u128>{

    let files = octocrab::instance().pulls("Jagoum", "FibBot").list_files(1).await;
    let files = files.unwrap().items.first().unwrap().patch.clone().unwrap();
    println!("Pull Resquest Contents:\n{}",files);
    let nums = extract_nums(&files.as_str());
    println!("Collected Nums: {nums:?}");
    nums
}


 
 pub async fn post_comment(pr_content: &str) -> Result<(), reqwest::Error> {
     let repo = env::var("GITHUB_REPOSITORY").expect("GITHUB_REPOSITORY not set");
     let pr_number = env::var("PR_NUMBER")
         .expect("PR_NUMBER not set")
         .parse::<u32>()
         .expect("Invalid PR_NUMBER");
        
     let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
     
     let url = format!(
         "https://api.github.com/repos/{}/issues/{}/comments",
         repo, pr_number
        );
        
        let client = Client::new();
        let response = client
         .post(&url)
         .header("Authorization", format!("Bearer {}", github_token))
         .header("User-Agent", "FibBot")
         .header("Accept", "application/vnd.github.full+json")
         .json(&serde_json::json!({ "body": pr_content }))
         .send()
         .await?;
 
     if response.status().is_success() {
         println!("✅ Comment posted successfully.");
     } else {
         eprintln!("❌ Failed to post comment: {:?}", response.text().await?);
     }
 
     Ok(())
    }



    //  async fn run() -> octocrab::Result<Comment> {
    //     let octocrab = octocrab::Octocrab::default();
    //     let _ = octocrab.pulls("Jagoum", "fibbot").comment(CommentId(1)).delete();
    //     let _ = octocrab.pulls("Jagoum", "fibbot").comment(CommentId(1)).update("Added A new comment Locally");
    //     let comment = octocrab.pulls("Jagoum", "fibbot").comment(CommentId(1)).get().await;
    
    //     comment
    //  }