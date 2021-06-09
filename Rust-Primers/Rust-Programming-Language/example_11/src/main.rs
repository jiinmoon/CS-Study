#![allow(dead_code)]

use std::fmt::Debug;


trait Animal {
    fn create(name: &'static str) -> Self where Self: Sized;
    fn get_name(&self) -> &'static str;
    // this would be default function
    fn introduction(&self) {
        println!("Hi, my name is {}.", self.get_name());
    }
}

struct Dog { name: &'static str }
struct Cat { name: &'static str }

impl Animal for Dog {
    fn create(name:&'static str) -> Dog { return Dog { name }; }

    fn get_name(&self) -> &'static str { self.name }

    // overriding default method
    fn introduction(&self) { println!("Hi, I am a dogo, {}!", self.name); }
}

impl Animal for Cat {
    fn create(name: &'static str) -> Cat { return Cat { name }; }

    fn get_name(&self) -> &'static str { self.name }

    fn introduction(&self) { println!("I'm {}. Meow!", self.name); }
}

trait Summable<T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        if self.is_empty() { return -1; }
        let first = self.get(0);
        let last = self.get(self.len() - 1);
        let mut result = 0;
        match (first, last) {
            (Some(x), Some(y)) => result = *x + *y,
            (_, _) => println!("Cannot do this")
        }
        return result * (self.len() as i32) / 2;
    }
}

fn traits() {
    let dog1 = Dog { name: "John" };
    dog1.introduction();

    let cat1 = Cat { name: "Quicy" };
    cat1.introduction();

    let dog2:Dog = Animal::create("Rocket");
    dog2.introduction();

    let cat2:Cat = Cat::create("Mesi");
    cat2.introduction();

    let mut vec1 = Vec::new();
    for val in 1..101 { vec1. push(val) };
    println!("Sum of {:?} is \n{}", vec1, vec1.sum());
}

// <--- Trait Parameters --->
// we can also pass traits as argument to function

trait Shape {
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Circle {
    radius : f64
}

#[derive(Debug)]
struct Square {
    side : f64
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        return self.radius * self.radius * std::f64::consts::PI;
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        return self.side * self.side;
    }
}

//fn print_info(shape: impl Shape + Debug) {
//fn print_info<T: Shape + Debug>(shape: T, shape2: T) {
fn print_info<T>(shape: T)
    where T: Shape + Debug
{
    println!("{:?}", shape);
    println!("The area of shape is {}.", shape.area());
}

fn trait_parameters() {
    let circle1 = Circle { radius: 1.0 };
    print_info(circle1);
    let square1 = Square { side: 2.2 };
    print_info(square1);
}

fn main() {
    //traits();
    trait_parameters();
}
