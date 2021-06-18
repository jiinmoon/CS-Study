
// like many languages, enums define types that have definitive values

enum Direction
{
	// Variants
	NORTH, EAST, WEST, SOUTH
}


fn move_to_direction(dir: &Direction) {
	match dir {
		Direction::NORTH => println!("Moving North"),
		Direction::EAST => println!("Moving East"),
		Direction::WEST => println!("Moving West"),
		Direction::SOUTH => println!("Moving South")
	}
}

pub fn run()
{
	for dir in &[Direction::NORTH, Direction::SOUTH] {
		move_to_direction(dir);
	}
}