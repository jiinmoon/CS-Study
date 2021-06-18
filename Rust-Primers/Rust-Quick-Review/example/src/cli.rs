use std::env;

// interacting with command line interface
// i.e. dealing with passed arguments to the program

pub fn run() {

	// taking command line arguments like most langagues, first argument would
	// be file that is running which in this case would be
	// "target/debug/example"
	let args: Vec<String> = std::env::args().collect();

	for (i, arg) in args.iter().enumerate() {
		println!("Your arg {} is [{}]", i, arg);
	}
}