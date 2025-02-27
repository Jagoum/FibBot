use regex::Regex;

pub fn extract_nums(string:&str) -> Vec<f64>{
    let nums = Regex::new(r"-?\d+(\.\d+)?").unwrap();
    
    let nums = nums.find_iter(string)
        .filter_map(|mat| mat.as_str().parse::<f64>().ok())
        .collect();
    nums
}