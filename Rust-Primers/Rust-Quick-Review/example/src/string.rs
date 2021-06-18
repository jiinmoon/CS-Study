// Note that there are different strings in Rust
// 1. Primitive &str is an Immutable fixed-length string in memory
// 2. String is an object, heap-allocated data structure

pub fn run() 
{
	let str1 = "This is immutable string on stack";
	let mut str2 = String::from("This is String object on heap");

	println!("str1 = {} | length = {}", str1, str1.len());
	println!("str2 = {} | length = {}",str2, str2.len());

	// there is very little that we can do with primitive str
	// String object offers more flexibility to manipualte it

	// append char at the end
	str2.push('!');
	// append str at the end
	str2.push_str(" This is exciting!");

	println!("{}", str2);

	// Read up on various String operations from Rust Doc
}