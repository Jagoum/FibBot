# This is my fibbot Program
# FibBot Project

This project is to learn how to  use github apis and to familiarise myself with github actions .

This project scans  a pull request for numbers and post the result as a comment on github . 

The numbers are parse into a fibbonacci calculator and then post the result of the fibbonacci calculator to this gh repo as a comment

I used some rust libaries like octocrab to get the content of a pull request

I used regex pattern matching to extract the the numbers from the pull request that has being parsed to a string

To parse the string i called some specail methods on the the content of the pull request.
This project is very fun

I Love this chanlenge


```rs

let files = files.unwrap().items.first().unwrap().patch.clone().unwrap();

```

<footer>
This project is build with rust
</footer>
