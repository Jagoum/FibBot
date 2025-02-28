use std::env;

use crate::extract_nums;

pub async fn get_pr() -> Vec<u128>{


    let repo = env::var("GITHUB_REPOSITORY").expect("GITHUB_REPOSITORY not set");
    let pr_number = env::var("PR_NUMBER")
        .expect("PR_NUMBER not set")
        .parse::<u32>()
        .expect("Invalid PR_NUMBER");

    let files = octocrab::instance().pulls("Jagoum", "FibBot").list_files(pr_number.into()).await;
    let files = files.unwrap().items.first().unwrap().patch.clone().unwrap();
    println!("Pull Resquest Contents:\n{}",files);
    let nums = extract_nums(&files.as_str());
    println!("Collected Nums: {nums:?}");
    nums
    
}