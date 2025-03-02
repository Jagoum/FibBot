use reqwest::Client;
use serde_json::{json, Value};
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

    // Fetch existing comments to avoid duplicates
    let existing_comments: Vec<Value> = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", github_token))
        .header("User-Agent", "MyGitHubBot")
        .send()
        .await?
        .json()
        .await?;

    if existing_comments.iter().any(|comment| {
        comment["body"].as_str().map_or(false, |body| body == pr_content)
    }) {
        println!(" ⚠️ Comment already exists, skipping...");
        return Ok(());
    }

    // Post the comment
    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", github_token))
        .header("User-Agent", "MyGitHubBot")
        .json(&json!({ "body": pr_content }))
        .send()
        .await?;

    if response.status().is_success() {
        println!(" ✅ Comment posted successfully");
    } else {
        eprintln!(" ❌ Failed to post comment: {:?}", response.text().await?);
    }

    Ok(())
}
