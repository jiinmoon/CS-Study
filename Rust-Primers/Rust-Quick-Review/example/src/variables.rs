/* Variables
 * 
 * Variables hold primitive data or "refereces" to data.
 * In Rust, variables are "immutable" by default.
 * 
 * Note that Rust is block scoped and block is defined by "{...}".
 */

pub fn run() 
{
    let name = "John";
    let age = 29;
    // following will be an error: cannot assign to immutable var
    //age = 30;
    println!("{} is {} years old.", name, age);

    // "mut" keyword makes the variable as mutable
    let mut mutable_age = 30;
    mutable_age = 99;
    println!("{} is {} years old.", name, mutable_age);

    // we may also define constant variables
    // compiler will simply replace all const vars with that value on compile time
    // note that you have to specify type for the const
    const PI: f32 = 3.14;
    println!("PI = {}", PI);

    // multiple assignment in single line
    let (dog_name, dog_age) = ("Luke", 12);
    println!("{} is a good dog and is {} years old.", dog_name, dog_age);
}