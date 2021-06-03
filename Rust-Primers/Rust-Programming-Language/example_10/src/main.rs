fn print_value(x: i32) {
    println!("Passed in value is {}", x);
}

// Rust is pass by value
// to modify the actual value, we must pass the reference
// then, dereference to modify the value stored within
fn increment_by_one(x: &mut i32) { *x += 1; }

fn product(x: i32, y: i32) -> i32 {
    return x * y;
}

fn functions() {
    print_value(22);

    let mut x = 0;
    println!("Before x was {}", x);
    increment_by_one(&mut x);
    println!("After x is {}", x);

    let mut y = 10;
    let mut z = 11;
    let result = product(y, z);
    println!("{} * {} = {}", y, z, result);
}

fn methods() {

}

fn main() {
    //functions();
    methods();
}
