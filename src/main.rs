mod arrays;
mod conditional;
mod constant;
mod looping;
mod print;
mod strings;
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
}
