
// conditionals statements (if-else)

pub fn run() {
	let budget = 100;

	// evalutes true/false
	if budget > 50 {
		println!("Enough to buy");
	} else {
		println!("Not enough.");
	}

	// single line
	// note, in this case if-else should return same types; it will be checked
	let result = if budget > 50 { "Enough to buy." } else { "Not enough." };
	println!("{} is {}", budget, result);
}