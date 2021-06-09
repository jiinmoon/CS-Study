pub fn run() 
{
    // print to console
    println!("Hello! from \"print.rs\"");

    // Basic Print Formatting
    println!("{} is the meaning of {}.", 42, "life");

    // Positional Arguments Formatting
    println!("{0} comes before {1} and {1} after {0}",
        "FIRST", "SECOND");

    // Named Arguments Formatting
    println!("Welcome! {first} {last}",
        first = "Reimu",
        last = "Hakurei");
    
    // <--- Start Placeholder Traits --->

    // Number Formatting
    println!(
        "Binary: {:b}\nHex: {:x}\nOctal: {:o}",
        11, 11, 11);
        
    // Memory Address
    let val = 42;
    println!("Memory Address = {:p}", &val);

    // Debug Trait (has to impl Debug)
    println!("{:?}", (42, "is meaning of life?", true));
    
    // <-- End Placeholder Traits -->

    // Basic Arithmetic OPs
    println!("{} + {} = {}", 10, 10, 10 + 10);
}