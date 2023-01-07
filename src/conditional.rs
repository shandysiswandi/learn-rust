pub fn run() {
    println!("===== Conditional =====");

    // if statement
    let value_of_math = 75;
    if value_of_math <= 50 {
        println!("Your score is bad | {value_of_math}")
    } else {
        if value_of_math < 75 {
            println!("Your score is enough | {value_of_math}")
        } else if value_of_math < 85 {
            println!("Your score is good | {value_of_math}")
        } else {
            println!("Your score is excellent | {value_of_math}")
        }
    }

    // returning from if
    let score = 3;
    let result = if score == 2 { true } else { false };
    println!("Result of returning if is {result} because your score {score} is not equal 2");

    println!();
}
