
// various looping syntaxes

pub fn run() {

	let mut count = 5;
	loop 
	{
		println!("This is infinite loop if not careful.");
		count -= 1;
		if count == 0 { break };
	}

	count = 5;
	while count > 0 
	{
		println!("while looping");
		count -= 1; 
	}

	// for-loop which specifies range
	// iterables can also be enumerated
	for i in 1..=5 
	{
		println!("i = {}", i);
	}

	for (i, val) in (vec!["loop", "is", "fun"]).iter().enumerate()
	{
		println!("{} ==> {}", i, val);
	}

	let mut arr:[u8; 3]= [1, 2, 3];
	for val in arr.iter_mut()
	{
		*val *= 2;
	}
	
	println!("After doubling ==> {:?}", arr);
}