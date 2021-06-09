#![allow(dead_code)]

mod print;
mod variables;

use print::run as hello;

pub(crate) fn main() {
    //hello();
    //print::run();
    variables::run();
}