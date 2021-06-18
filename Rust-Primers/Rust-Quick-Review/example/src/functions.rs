// simply, functions is a reusable block of code.

pub fn run() {
	println!("Calling another function!");
	say_something("HAHAHA", 3);

	// Closure
	let add_two_nums = |x: i32, y: i32| -> i32 {
		x + y
	};

	println!("{} + {} = {}", 4, 5, add_two_nums(4, 5));
	println!("Passing function pointers");
	println!("{} + {} = {}", 10, 11, apply_closure(add_two_nums, 10, 11));
}

fn apply_closure(f: fn(i32, i32) -> i32, x: i32, y: i32) -> i32 {
	f(x, y)
}

fn say_something(something: &str, count: i32)
{
	for _ in 0..count {
		println!("{}", something);
	}
}