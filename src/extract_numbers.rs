
use crate::fibbonacci_calculator::fibonacci;
///- This function uses string parsing methods to get the numbers
/// > This function is limited in that it can only get numbers that are not attarched to any thing like hanging numbers in a string
pub  fn extract_nums(string:&str) -> Vec<u128>{

    let string: Vec<String> = string.split_whitespace().map(String::from).collect();
    let mut  arr: Vec<u128> = Vec::new();
    
    for  i in string{
    let num: u128 = match i.trim().parse() {
        Ok(num) => {num},
        Err(_) => {continue;},
    };
    arr.push(num);
    }
    arr
}

pub fn printing_fibbo(){
    let string1  = "Hello Bro I want to give you 10 thousand by Jesus want to give you 100 million";
    let string2 = "Please take that 100 million and come let share";
    let nums1 =   extract_nums(string1) ;
    let _nums2 =  extract_nums(string2) ;

    for i in nums1{
        fibonacci(i as i128);
    }  
    // println!();
    // for i in nums2{
    //     fibonacci(i);
    // }
}

#[test]
fn test_extract_nums(){
    let numbers =  { extract_nums("Hello this is my number 1 test for this program, hello Mr 9") };
    assert_eq!(*numbers.get(0).unwrap(),1);
    assert_eq!(*numbers.get(1).unwrap(),9);
    assert!(numbers.get(3).is_none())
}

#[test]
fn test_wrong_nums_extracted(){
    let wrong_nums = extract_nums("4 plus 4 is 8 but you said 44 ");
    assert_ne!(*wrong_nums.get(0).unwrap(),8);
    assert_ne!(*wrong_nums.get(1).unwrap(), 44);
    assert_ne!(*wrong_nums.get(3).unwrap(), 4);
}