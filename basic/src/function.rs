#![allow(clippy::all)]

pub fn run() {
  println!("===== Function =====");

  // private function
  greet();

  // function with params
  with_param(1);

  // function with return value
  let val1 = return_value(1, 2, 3);
  println!("function with return value: {val1}");

  // function with return value using naked
  let val2 = naked_return_value(1, 2, 3);
  println!("function with naked return value: {val2}");

  // function with return value using statement
  let val3 = statement_return_value(1, 2, 3);
  println!("function with statement return value: {val3}");

  // std macro format to format string and return `String` not `str` or `&str`
  let msg = format!("the box volume is {}", val3);
  println!("function std format: {msg}");

  // special return type of &str
  let special = special_return_type();
  println!("special return type: {:?}", special);

  // Default return value
  let def = default_return_value();
  println!("default return value: {:?}", def);

  println!();
}

// this is private function, it only can access from this file
fn greet() {
  println!("private function");
}

// this is private function with param, in rust naming convention is using `_`
fn with_param(param_a: i32) {
  println!("function with param {param_a}");
}

// this is private function with param and return value
fn return_value(width: i32, height: i32, length: i32) -> i32 {
  let volume = width * height * length;
  return volume;
}

// this is private function with param and return value using naked return
// no semicolun at the end
// no return keyword
// at the end of the code block
fn naked_return_value(width: i32, height: i32, length: i32) -> i32 {
  let volume = width * height * length;
  volume
}

// this is private function with param and return value using statement return
// no semicolun at the end
// no return keyword
// at the end of the code block
fn statement_return_value(width: i32, height: i32, length: i32) -> i32 {
  width * height * length
}

// Especially for some types of data types, such as &str,
// if it is used as a return value data type, the function must add the static keyword by writing &'static str
fn special_return_type() -> &'static str {
  return "you got a perfect score!";
}

// All those functions have a return value, yep all of them.
// For functions whose return value is not defined, the return value is an empty tuple `()`
fn default_return_value() {
  print!("default_return_value ");
}
