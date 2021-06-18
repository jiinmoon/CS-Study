/* Primitive Types:
 *
 * Numeric Types involve Ints/Floats 
 * u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
 * f32, f64
 * 
 * Boolean Type
 * bool
 * 
 * Characters Type
 * char
 *
 * Array Types
 * tuples, arrays 
 */


// Note that Rust is strong, statically typed.
// This implies that the compiler requires us to define the types for all the
// variables at the compile time. But there is a flexibility in that the
// compiler can also "infer" types based on the usage of the values.
pub fn run()
{
    // example of inferred types
    // by default, x is of type i32 and y is of type f64
    let x = 1;
    let y = 3.14;

    // explict type declaration
    let z: u8 = 1;

    // max/min values for the types
    println!("max i32 = {} and min i32 = {}.", i32::max_value(), i32::min_value());

    // boolean
    let is_smaller = x < y as i32;

    // char literal in Rust is Unicode (4 byes)
    let char1: char = 'a';
    let char2: char = '\u{1F600}';
    println!("{:?}", (char1, char2));
}