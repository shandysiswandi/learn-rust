mod arrays;
mod conditional;
mod constant;
mod looping;
mod print;
mod slices;
mod strings;
mod tuples;
mod types;
mod variable;

fn main() {
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
}
