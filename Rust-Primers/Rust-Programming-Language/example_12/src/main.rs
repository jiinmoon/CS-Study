use std::ops::{Add, AddAssign};

struct Human { name : String }

impl Human
{
//    fn create(name: &str) -> Human { Human { name: name.to_string() } }
    // generic way to create constructor
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

fn main()
{
    //into_example();
    //drop_example();
    ops_overloading();
}

