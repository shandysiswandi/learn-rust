pub fn run() {
    println!("===== String Type =====");

    // string literal (&str)
    let name = "John Doe";
    println!("My name is {}", name);

    // escape characters
    let paragraf = "This is a \"Rust\" programming language";
    println!("{}", paragraf);

    // multiline of strings
    let article = "one line
    two line
    three line";
    println!("{}", article);

    // raw string
    let raw = r#"{ "key":"value" }"#;
    println!("{}", raw);

    println!();
}
