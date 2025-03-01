use crate::extract_nums;

pub async fn get_pr(pr_num:u32) -> Vec<u128>{

    let files = octocrab::instance().pulls("Jagoum", "FibBot").list_files(pr_num.into()).await;
    let files = files.unwrap().items.first().unwrap().patch.clone().unwrap();
    println!("Pull Resquest Contents:\n{}",files);
    let nums = extract_nums(&files.as_str());
    println!("Collected Nums: {nums:?}");
    nums
    
}