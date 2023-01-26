pub fn run() {
    println!("===== Array =====");

    // how to write array
    let mut numbers = [24, 12, 32, 7];
    println!("array {:?}, array index 0 is {:?}", numbers, numbers[0]);

    // how to modify array mutability
    numbers[0] = 16;
    println!("array {numbers:?} size {}", numbers.len());

    // loop using `for in`
    // for number in numbers {
    //     println!("{number}");
    // }

    // loop using `for in` and tuple
    // for (i_num, number) in numbers.iter().enumerate() {
    //     println!("array index {i_num}: {number}");
    // }

    println!();
}
