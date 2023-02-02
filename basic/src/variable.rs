// by default variables are immutable
pub fn run() {
    println!("===== How to Write Variable =====");

    // let
    let name: &str = "this is pointer of string";
    let age = 25;
    let mut some = "horse";
    println!("some: {} before mutation", some);
    some = "camel";
    println!("some: {} after mutation", some);

    println!("name: {name}, age: {age}"); // using named argument macro println

    println!();
}
