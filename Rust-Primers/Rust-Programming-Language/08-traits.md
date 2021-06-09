# Traits

Traits are construct that defines implementations - a bit like inheritance of
sort.

```rust
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
```

## Trait Parameters

```rust
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
```

## `into` trait

```rust
struct Human { name : String }

impl Human
{
    // typical way to create Human
    // fn create(name: &str) -> Human { Human { name: name.to_string() } }

    // generic way to create constructor would be:
    // "Into" specifies that whatever the type is used to construct,
    // it has to be able to convert INTO String
    // conversion happens when we call upon name which is of generic type T into String
    fn create<T: Into<String>>(name: T) -> Human { Human { name: name.into() } }

    // different way to achieve same is using "where" clause
    // This is saying that whatever type T is passed in, it has to impl Into<String>
    fn create2<T>(name: T) -> Human
        where T: Into<String>
    {
        Human { name: name.into() }
    }
}

fn into_example()
{
    // passing String and &str
    // both work fine with "into" converting whatever passed into String type
    let human1 = Human::create2("Micky".to_string());
    let name1 = "Paul";
    let human2 = Human::create2(name1);

    println!("{} and {}.", human1.name, human2.name);
}
```

## `drop` trait

Clean up trait

```rust
// <-- DROP Trait -->

struct Monster { name: String }

impl Monster
{
    fn new(name: &str) -> Monster
    {
        println!("New Monster: {} has entered the game.", name);
        Monster { name: name.into() }
    }
}

// DROP specifies what should happen when struct is being "dropped"
impl Drop for Monster
{
    fn drop(&mut self) {
        println!("New Monster: {} is now dead.", self.name);
    }
}

fn drop_example()
{
    let monster1 = Monster::new("Orc");

    // note that we cannot call drop() destructor explicitly ourselves
    // monster1.drop() is an invalid
    // since it is possible that someone may try to access monster1 after drop has happened
    // but we can drop like so
    drop(monster1);
    // note: now when we try to access the monster1 like following
    // monster1.name;
    // it will give an error: using moved variable.
}
```

## Operator Overloading

```rust

// <-- Operator Overloading -->

// note the Copy and Clone traits are required to implement as we have overloaded Add
// as part of its operation, it moves the lhs and rhs;
// instead of implementing Copy/Clone ourselves, compiler can dervie it for us
#[derive(Debug, Copy, Clone)]
struct ComplexNumber<T>
{
    real: T,
    imaginary: T
}

impl<T> ComplexNumber<T> {
    fn new(real: T, imaginary: T) -> ComplexNumber<T>
    {
        ComplexNumber::<T> { real, imaginary }
    }
}

// implements std::ops::Add;
// this is for specific i32 type
//impl Add for ComplexNumber<i32> {
//    type Output = ComplexNumber<i32>;
//
//    fn add(self, rhs: Self) -> Self::Output {
//        ComplexNumber::new(
//            self.real + rhs.real,
//            self.imaginary + rhs.imaginary
//        )
//    }
//}

// generic implementation
// note that we do not know whether T implements "+"
// thus, we specify with "where"
impl<T> Add for ComplexNumber<T>
    // specifies T implements Add which returns output of type T
    where T: Add<Output = T>
{
    type Output = ComplexNumber<T>;

    // if i32 type is passed in, it would use "+" as defined on i32
    fn add(self, rhs: Self) -> Self::Output {
        ComplexNumber::new(
            self.real + rhs.real,
            self.imaginary + rhs.imaginary
        )
    }
}

// we are borrowing whatever generic T has implemented for AddAssign
impl<T> AddAssign for ComplexNumber<T>
    where T: AddAssign<T>
{
    fn add_assign(&mut self, rhs: Self) {
        self.real += rhs.real;
        self.imaginary += rhs.imaginary;
    }
}

fn ops_overloading()
{
    let mut complex1 = ComplexNumber { real: 1, imaginary: 2 };
    let mut complex2 = ComplexNumber { real: 3, imaginary: 4 };
    let mut complex3 = ComplexNumber { real: 1.0, imaginary: 2.0 };
    let mut complex4 = ComplexNumber { real: 3.0, imaginary: 4.0 };

    println!("{:?}", complex1);
    println!("{:?}", complex2);

    // how should we perform complex1 + complex2?
    // overload Add just like in Python how we would overload __add__()
    // std::ops::Add trait has to be implemented
    println!("{:?}", complex1 + complex2);
    println!("{:?}", complex3 + complex4);

    // how about +=?
    // this is an another trait called std::ops::AddAssign
    println!("{:?} + {:?} is...", complex1, complex2);
    complex1 += complex2;
    println!("{:?}", complex1);
}
```
