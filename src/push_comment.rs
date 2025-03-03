

pub async fn post_github_comment(github_token: &str, owner: &str, repo: &str, pr_number: u64, pr_content: &str) -> Result<(), Box<dyn Error>> {

use reqwest::Client;
use serde_json::json;
use std::error::Error;
pub async fn post_github_comment(
    github_token: &str,
    owner: &str,
    repo: &str,
    pr_number: u64,
    pr_content: &str,
) -> Result<(), Box<dyn Error>> {
  
    let client = Client::new();
    let url = format!(
        "https://api.github.com/repos/{}/{}/issues/{}/comments",
        owner, repo, pr_number
    );
    
    let body = json!({ "body": pr_content });
    

    println!("🚀 Sending request to: {}", url);
    let body = json!({ "body": pr_content });
// This particular method will do a post method to the gh api and the api will respond to the request
// The url is used to precise the action repository and user that will be posting to that repository
// Json here is just to format the content that will be posted on github or the input that it gets form the pr_conten=======



    let response = client
        .post(&url)
        .header("Authorization", format!("token {}", github_token))
        .header("User-Agent", "reqwest")
        .json(&body)
        .send()
        .await;
    
        match response {
            Ok(_) => println!("✅ Comment posted successfully."),
            Err(err) => eprintln!("❌ Failed to post comment: {:?}", err),
        }
    
    Ok(())
}

// use octocrab::Octocrab;

// /// This function get the content from a pull request and then parse it to extract numbers
// /// This function posts a comment to github
// pub async fn post_comment(
//     pr_number: u32,
//     pr_content:  &str,
//     github_token: &str,
// ) -> Result<String, reqwest::Error> {
//     // let repo = env::var("GITHUB_REPOSITORY").expect("GITHUB_REPOSITORY not set");
//     // let pr_number = env::var("PR_NUMBER")
//     //     .expect("PR_NUMBER not set")
//     //     .parse::<u32>()
//     //     .expect("Invalid PR_NUMBER"); this is not necessary for now

//     // let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set"); // Finally decided to use args instead of env

//     // let _url = format!(
//     //     "https://api.github.com/repos/{}/issues/{}/comments",
//     //     repo, pr_number
//     //    ); This one is when we are using client instead of octocrab

//     let client = Octocrab::builder()
//         .personal_token(github_token)
//         .build()
//         .unwrap();
//     let response = client
//         .issues("Jagoum", "FibBot")
//         .create_comment(pr_number.into(), pr_content)
//         .await;

//     if response.is_ok() {
//         println!("✅ Comment posted successfully.");
//     } else {
//         // eprintln!("❌ Failed to post comment {:?}",response.unwrap());
//         println!("❌ Failed to post a comment")
//     }
//     Ok(pr_content.into())
// }


