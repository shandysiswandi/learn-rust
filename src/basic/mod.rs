mod arrays;
mod conditional;
mod constant;
mod function;
mod looping;
mod print;
mod slices;
mod strings;
mod tuples;
mod types;
mod variable;
mod vectors;

pub fn run() {
  print::run();
  variable::run();
  types::run();
  strings::run();
  constant::run();
  conditional::run();
  looping::run();
  arrays::run();
  slices::run();
  tuples::run();
  vectors::run();
  function::run();
}
