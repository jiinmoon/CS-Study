use std::fmt;

// structs are very similar to class
// it has members, and functions
// it allows for custom data types

// Traditional Struct
#[derive(Debug)]
struct Color
{
	r: u8, g: u8, b: u8
}

// Tuple Struct
#[derive(Debug)]
struct Color2(u8, u8, u8);

// "impl" implements specifics about struct
// can derive from other definitions and override as well

struct Dog {
	name: String,
	age: u16
}

impl Dog 
{
	// constructor
	pub fn new(name: &str, age: u16) -> Dog {
		Dog {
			name: name.to_string(),
			age: age
		}
	}

	// note the &self reference
	// refers to itself (just like Python class -> self)
	pub fn bark(&self) {
		println!("Bark! I am {}!", self.name);
	}

	pub fn set_name(&mut self, new_name: &str) {
		self.name = new_name.to_string();
	}
}

// Debug is a Trait that can be implemented
impl fmt::Debug for Dog {
	fn fmt(&self, f: &mut fmt::Formatter<
	'_>) -> fmt::Result {
		f.debug_struct("Nice Doggy")
			.field("Dogo's name", &self.name)
			.field("Dogo's age", &self.age)
			.finish()
	}
}

pub fn run()
{
	let mut color = Color {
		r: 0, g: 0, b: 0
	};

	println!("Black is {:?}", color);
	println!("Black is {} {} {}", color.r, color.g, color.b);
	
	color.r = 255;

	let color2 = Color2(0, 0, 0);

	println!("{:?}", color2);
	println!("{} {} {}", color2.0, color2.1, color2.2);

	let mut dog1 = Dog::new("John", 7);
	println!("{:?}", dog1);
	dog1.bark();

	dog1.set_name("Dodo");
	println!("{:?}", dog1);
}