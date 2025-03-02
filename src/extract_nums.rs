/// Here we are going to be using regex to extract the numbers
/// This is good in that it can get all numbers
use regex::Regex;

pub fn extract_nums(string:&str) -> Vec<u128>{
    let nums = Regex::new(r"[0-9]+").unwrap();
    let nums = nums.find_iter(string)
        .filter_map(|mat| mat.as_str().parse::<u128>().ok())
        .collect();
    nums
}