pub fn fibonacci(num: u128) -> Vec<u128>{
    let mut elements:Vec<u128> = vec![0,1];
    
    let mut fib1 = 0;
    let mut fib2 = 1;
    let mut fibn ;

    if num > 1 {
        for _ in 2..=num{
            
            fibn = fib1 + fib2;
            fib1 = fib2;
            fib2 = fibn;
            elements.push(fibn);

        }
        for i in elements.clone(){
            print!("{} ",i);
        }
        println!();
        elements
    }
    else if num == 0{
        return vec![0];
    }
    else if num == 1 {
        return elements;
    }
    else {
        return vec![];
    }
 
}
#[test]
fn works_correctly(){
    let fib = fibonacci(4);
    assert_eq!(*fib.get(3).unwrap(), 2);
    assert_eq!(*fib.get(0).unwrap(), 0);
    assert_eq!(*fib.get(1).unwrap(),1);
}
#[test]
fn it_might_work(){
    let might_work = fibonacci(1);
    assert_eq!(*might_work.get(0).unwrap(),0);
 assert_eq!(*might_work.get(1).unwrap(),1);
}