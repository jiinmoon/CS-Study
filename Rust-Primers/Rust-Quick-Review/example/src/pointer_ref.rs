// pointers
// Simply variables that holds the memory address for the value stored in the
// memory. Hence, they are "pointing" to the actual values.
pub fn run() 
{
	// array is an example of pointer
	// in fact, all it is that it points to the first value of the array.
	let arr1 = [1, 2, 3];
	let arr2 = arr1;

	println!("{:?}", (arr1, arr2));
	println!("&arr1: {:p} \t &arr1[0]: {:p}", &arr1, &arr1[0]);

	// so far this is true with "primitives"

	// With non-primitive types, if you assign another variable to a piece of data, the first variable no longer hold that value - so you need to use & reference to point to the resource

	// for example, vector is non-primitive type
	let vec1 = vec![1, 2, 3];
	let vec2 = vec1;

	// note that below will not run

	// println!("Values are {:?}", (vec1, vec2));

	// specifically, it will say "use of moved value: vec1"
	// the value (memory addr of vec1) is moved over to the vec2
	// and vec1 is now unavailable to use

	// so instead, we need to use the reference
	// what vec3 has is now the pointer to the vec2
	let vec3:&Vec<i32> = &vec2;
	println!("Values are {:?}", (&vec2, vec3));
}