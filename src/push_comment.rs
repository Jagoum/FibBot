///This is bet
/// 

use reqwest::Client;
use serde_json::json;
use std::error::Error;

pub async fn post_github_comment(github_token: &str, owner: &str, repo: &str, pr_number: u64, pr_content: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let url = format!(
        "https://api.github.com/repos/{}/{}/issues/{}/comments",
        owner, repo, pr_number
    );
    
    let body = json!({ "body": pr_content });
    
    let response = client
        .post(&url)
        .header("Authorization", format!("token {}", github_token))
        .header("User-Agent", "reqwest")
        .json(&body)
        .send()
        .await?;
    
    if response.status().is_success() {
        println!(" ✅ Comment posted successfully");
    } else {
        println!(" ❌ Failed to post comment: {:?}", response.text().await?);
    }
    
    Ok(())
}
