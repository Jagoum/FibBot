pub fn fibonacci(num: u128) -> Vec<u128>{
    let mut elements:Vec<u128> = vec![0,1];
    
    let mut fib1 = 0;
    let mut fib2 = 1;
    let mut fibn = 0;

    if num > 1 {
        for _ in 2..=num{
            
            fibn = fib1 + fib2;
            fib1 = fib2;
            fib2 = fibn;
            elements.push(fibn);

        }
    }
    elements

}