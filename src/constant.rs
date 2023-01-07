pub fn run() {
    println!("===== Constant =====");
    // Naming convention constant is screaming snake case ex: MY_NAME, YOUR_NAME, PHI

    // Keyword const
    // the data type must explicit
    // const has no memory address which is certain, and every time it is used, a copy value process occurs.
    const LABEL: &str = "Value of PHI is:";
    const PHI: f32 = 22.0 / 7.0;
    println!("{} {}", LABEL, PHI);

    // Keyword static
    // the data type must explicit
    // static has fixed / definite memory address.
    static NUMBER: i32 = 18;
    println!("Value of constant using static keyword {}", NUMBER);

    println!();
}
