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

    println!("🚀 Sending request to: {}", url);

    let body = json!({ "body": pr_content });

    let response = client
        .post(&url)
        .header("Authorization", format!("token {}", github_token))
        .header("User-Agent", "reqwest")
        .json(&body)
        .send()
        .await?;

    let status = response.status();
    let text = response.text().await.unwrap_or_else(|_| "Failed to read response".to_string());

    println!("📡 Response Status: {}", status);
    println!("📩 Raw Response: {}", text);

    if status.is_success() {
        println!("✅ Comment posted successfully!");
    } else {
        eprintln!("❌ Failed to post comment: {}", text);
    }

    Ok(())
}
