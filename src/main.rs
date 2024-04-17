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

    //loop and break after meet condition
    let mut counter = 0;
    let result = loop {
      counter += 1;
      if counter == 10 {
        break counter * 2;
      }  
    };

    println!("The result is {}", result);

    //loop label and multiple break nested loop
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; // break specific loop label
            }
            remaining -= 1;
        }

        count += 1;
    }

    // while
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // for
    let a = [10, 20, 30, 40, 50];
    // try using while, you need to specify index which can cause panic if index out of range
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
    // using for but same approach, you don't need to specify index cause it will be automatic increment collection of element
    for element in a {
        println!("the value is: {}", element);        
    }

    // for range rev(), this can be used for reverse loop
    for number_range in (1..4).rev() { // range from 1 to 3
        println!("{}!", number_range);
    }
    println!("LIFTOFF!!!");
}
