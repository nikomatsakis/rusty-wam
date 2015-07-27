extern crate lalrpop_intern as intern;

#[macro_use]
mod ast;

mod interpret;
mod functor;
mod machine;

fn main() {
    println!("Hello, world!");
}
