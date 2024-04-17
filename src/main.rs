fn main() {
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("number is not divisible by 4");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 }; // if expression returns a value and become statement
    // type cannot be different in if expression
    // for example number = if condition { 5 } else { "six" };
    println!("The value of number is: {}", number);
}
