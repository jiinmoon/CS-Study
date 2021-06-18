use std::mem::size_of_val;

pub fn run()
{
	// tuples
	// tuples are collection of different types that can be max 12 elements
	let people = ("John", 42, "Nate");

	println!("{:?} || {} {} {}", people, people.0, people.1, people.2);

	// arrays
	// like in C, this is a fixed length of memory segments
	// hence, the size must be known at the compile time
	let cats: [&str; 3] = ["John", "Quin", "Nabi"];
	println!("{:?}", cats);

	// can instantiate with default values as well
	let mut nums: [u8; 10] = [11; 10];
	println!("{:?}", nums);
	nums[9] = 20;
	println!("{:?}", nums);

	// array is "stack" allocated
	println!("Array is {} bytes on stack. Each is {} byte of u8 type.", 
		size_of_val(&nums),
		size_of_val(&nums[0]));
	
	// like Python, we can "slice" the array
	let nums_slice: &[u8] = &nums[2..5];
	println!("Slice: {:?}", nums_slice);

	// vectors
	// like an array, it supports many of array like operations
	// however it is not fixed in size
	let mut vec1 = vec![1, 2, 3];
	println!("{:?}", vec1);
	vec1.push(4);
	println!("{:?}", vec1);
	println!("{} is removed now.", vec1.pop().unwrap());

	// similar to python's `enumerate(iterable)`
	for (i, element) in vec1.iter().enumerate() {
		// note that we cannot alter the values within the vector this way
		// we need & mutable values
		//*element *= 4;
		println!("{} ==> {}", i, element);
	}

	for element in vec1.iter_mut() {
		*element *= 2;
	}
	println!("{:?}", vec1);
}