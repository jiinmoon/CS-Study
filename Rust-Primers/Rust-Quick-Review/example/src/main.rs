#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(unused_variables)]

mod print;
mod variables;
mod types;
mod string;
mod array;
mod conditionals;
mod loops;
mod functions;
mod pointer_ref;
mod structs;
mod enums;
mod cli;

// another way to import
use print::run as hello;
   
pub(crate) fn main() {
    //hello();
    //print::run();
    //variables::run();
    //types::run();
    //string::run();
    //array::run();
    //conditionals::run();
    //loops::run();
    //functions::run();
    //pointer_ref::run();
    //structs::run();
    //enums::run();
    cli::run();
}