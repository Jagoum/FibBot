use std::env;

use octocrab::Octocrab;

/// This function get the content from a pull request and then parse it to extract numbers
/// This function posts a comment to github
pub async fn post_comment(
    pr_number: u32,
    pr_content: &str,
    github_token: &str,
) -> Result<(), reqwest::Error> {
    // let repo = env::var("GITHUB_REPOSITORY").expect("GITHUB_REPOSITORY not set");
    // let pr_number = env::var("PR_NUMBER")
    //     .expect("PR_NUMBER not set")
    //     .parse::<u32>()
    //     .expect("Invalid PR_NUMBER"); this is not necessary for now

    // let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set"); // Finally decided to use args instead of env

    // let _url = format!(
    //     "https://api.github.com/repos/{}/issues/{}/comments",
    //     repo, pr_number
    //    ); This one is when we are using client instead of octocrab

    let client = Octocrab::builder()
        .personal_token(github_token)
        .build()
        .unwrap();
    let response = client
        .issues("Jagoum", "FibBot")
        .create_comment(pr_number.into(), &pr_content)
        .await;

    if response.is_ok() {
        println!("✅ Comment posted successfully.");
    } else {
        // eprintln!("❌ Failed to post comment {:?}",response.unwrap());
        println!("❌ Failed to post a comment")
    }
    Ok(())
}
