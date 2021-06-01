#![allow(dead_code)]

use std::mem;

struct Point {
    x: i32,
    y: i32
}

struct Line {
    start: Point,
    end: Point
}

fn structures() {
    let p1 = Point { x: 0, y: 4 };
    let p2 = Point { x: 4, y: -10 };
    //let line1 = Line { start: p1, end: p2 };
    println!("p1 is at ({}, {})", p2.x, p1.y);
}

enum Color {
    R, G, B,
    RGBColor(u8, u8, u8),   // tuple
    CMYK {
        cyan: u8, magenta: u8, yellow: u8, black: u8
    }                       // struct
}

fn enumerations() {
    //let color:Color = Color::RGBColor(0, 199, 255);
    let color:Color = Color::CMYK { cyan: 10, magenta: 20, yellow: 30, black: 255 };
    match color {
        Color::R => println!("RED"),
        Color::G => println!("GREEN"),
        Color::B => println!("BLUE"),
        Color::RGBColor(0,0,0) |
        Color::CMYK{cyan:_, magenta:_, yellow:_, black:255} => println!("BLACK"),
        Color::RGBColor(r,g,b) => println!("{} {} {}", r, g, b),
        Color::CMYK{cyan:c, magenta: m, yellow: y, black:b} => println!("{} {} {} {}", c, m, y, b),
        _ => println!("Invalid")
    }
}

union Number {
    i: i32,
    f: f32
}

fn identify_value(num: Number) {
    unsafe {
        match num {
            Number { i: 222 } => { println!("222") },
            // this will print garbage!
            //Number { i } => { println!("Some int {}", i) },
            Number { f: 3.14 } => { println!("PI") },
            Number { i } => { println!("Some int {}", i) },
            Number { f } => { println!("Some float {}", f) }
        }
    }
}

fn unions() {
    let mut num = Number { i : 111 };
    num.i = 222;

    // notice that we require unsafe block
    let x = unsafe { num.i };
    // below will fail to compile: accessing to union field
    //let y = num.i;

    println!("{}", x);
    num.f = 3.14;
    // i will now contain garbage!
    unsafe { println!("{} {}", num.i, num.f); }
    identify_value(num);
    identify_value(Number {i : 7});
}

fn options() {
    let x = 3.14;
    let y = 2.0;

    // Option -> Some(V) | None
    let result =
        if y != 0.0 { Some(x / y) }
        else { None };

    match result {
        Some(z) => println!("{}/{} = {}", x, y, z),
        None => println!("Divide by zero")
    }

    // checks for None
    if let Some(z) = result {
        println!("result = {}", z);
    }
}

fn arrays() {
    // array is a data structure where you know the size beforehand
    let mut arr1:[i32;5] = [1, 2, 3, 4, 5];

    // rust arrays support typical behaviors of an array
    println!("arr1 length : {}", arr1.len());
    println!("arr1[0] : {}", arr1[0]);

    arr1[0] = 999;
    println!("{:?}", arr1);

    if arr1 == [999, 2, 3, 4, 5] {
        println!("Yes");
    }

    // create array of size 10 initialized with 1
    let arr2 = [1; 10];
    println!("{:?}", arr2);
    for i in 0..arr2.len() {
        print!("{} ", arr2[i]);
    }

    // arr2 occupies 40 bytes = i32 is 4 bytes and arr2 length is 10
    println!("arr2 has taken {} bytes", mem::size_of_val(&arr2));

    // nested arrays; matrix
    let matrix:[[f32;2]; 3] = [
        [0.0, 1.0],
        [2.0, 3.0],
        [4.0, 5.0]
    ];

    println!("{:?}", matrix);

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            println!("({}, {}) = {}", i, j, matrix[i][j]);
        }
    }
}

// notice how slice can be mutable
fn use_slice(slice: &mut[i32]) {
    slice[0] = 10;
    println!("slice is : {:?}", slice);
}

fn slices() {
    let mut arr = [1,2,3,4,5];

    // slice is akin to python slices
    // taking the range of elements in the array
    use_slice(&mut arr[1..4]);
    use_slice(&mut arr);
    println!("{:?}", arr);
}

// this functions returns a tuple
// unlike array, the types can be mismatched
fn sum_and_product(x: i32, y: i32) -> (i32, f64) {
    return (x + y, (x * y) as f64);
}

fn tuples() {
    let x = 1;
    let y = 2;
    let mut result = sum_and_product(x, y);
    // result.0, result.1
    result.0 = 10;
    println!("{:?}", result);

    // destructuring (unpacking of elements in tuple)
    let (a,b) = result;
    println!("{}, {}", a, b);

    // tuples can be nested
    let result2 = sum_and_product(4, 5);
    let result3 = (result, result2);
    println!("{:?} {}", result3, (result3.1).1);
}

fn fizzBuzz(i: i32) -> &'static str {
    let x = i % 3;
    let y = i % 5;
    match (x, y) {
        (0, 0) => "Fizz Buzz",
        (0, _) => "Fizz",
        (_, 0) => "Buzz",
        (_, _) => ""
    }
}

fn pattern_matching() {
    for i in 0..100 {
        println!("{} : {}", i, fizzBuzz(i));
    }
}

// point in 3 dimensions (x, y, z)
// but x, y, z can be of any type (Option<T>)
struct Point3D<T> {
    x: T, y: T, z: T
}

// can also have different types as well
struct Point2D<T, V> {
    x: T, y: V
}

struct Line2D<T> {
    start: Point2D<T, T>,
    end: Point2D<T, T>
}

struct Line3D<T, V> {
    start: Point3D<T>,
    end: Point3D<V>
}

fn generics() {
    let point1:Point3D<u8> = Point3D { x: 0, y: 0, z: 0 };
    let point2:Point3D<f32> = Point3D { x: 1.0, y: 9.0, z: -12.2 };
    let point3:Point2D<u8, u16> = Point2D { x: 16, y: 255 };
    let point4:Point2D<f32, f32> = Point2D { x: 0.0, y: 0.0 };
    let line1:Line2D<f32> = Line2D {
        start: point4,
        end: Point2D { x: 8.9, y: 9.9 }
    };
    let line2 = Line3D {
        start: point1,
        end: point2
    };
}

fn main() {
    //structures();
    //enumerations();
    //unions();
    //options();
    //arrays();
    //slices();
    //tuples();
    //pattern_matching();
    generics();
}
