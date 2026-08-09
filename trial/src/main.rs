// fn main() {
//     let x = 5;
//     let x = x + 1;          // new x = 6, old x discarded

//     {
//         let x = x * 2;       // inner-scope x = 12, only exists inside {}
//         println!("The value of x in the inner scope is: {x}");
//     }

//     println!("The value of x is: {x}");   // back to outer x = 6
// }

// fn main() {
//     let x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }

/// we cannot do it as we can not assign a immutable variable twice 


// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }


// we define variable with mut which means mutable 


// fn main() {
//     let x = 5;
//     let x = x + 1;          // new x = 6, old x discarded

//     {
//         let x = x * 2;       // inner-scope x = 12, only exists inside {}
//         println!("The value of x in the inner scope is: {x}");
//     }

//     println!("The value of x is: {x}");   // back to outer x = 6
// }

/// The value of x in the inner scope is: 12
/// The value of x is: 6


// fn main() {
//     let spaces = "  ";
//     let spaces = spaces.len();
//     println!("Number of spaces: {spaces}")
// }

/// Output : Number of spaces : 2


// fn main() {
//     let mut spaces = "  ";
//     spaces = spaces.len();
//     println!("Number of spaces: {spaces}")
// }


//error occured as we try to assign a value of different type to a variable 


// fn main() {
//     let guess = "42".parse().expect("Not a number!");
//     println!("{guess}");
// }

//error[E0284]: type annotations needed
//type must be known at this point
//help: consider giving `guess` an explicit type


// fn main() {
//     let guess: u32 = "42".parse().expect("Not a number!");
//     println!("{guess}");


// //1. Unsigned integers (u) -  Unsigned means only positive numbers and zero.
// //2.Signed integers (i) - integer with a sign, so it can store negative and positive values.
// // f is used for floating-point numbers

// let a: u32 = "42".parse().unwrap();
// println!("{a}");

// let b: i32 = "42".parse().unwrap();
// println!("{b}");

// let c: f64 = "42".parse().unwrap();
// println!("{c}");

// }

// use std::env::args;

// fn main() {
//     let args: Vec<String> = std::env::args().collect(); // call args(), collect into a real Vec
//     let x: u8 = args[1].parse().unwrap();                // .parse() with parentheses
//     let y = x + 1;
//     println!("{y}");
// }

//attempt to add with overflow



// fn main() {
//     let tup: (i32, f64, u8) = (500, 6.4, 1);
//     let (x, y, z) = tup;              
//     println!("{y}");
//     let five_hundred = tup.0;         // direct index access with .N
//     let six_point_four = tup.1;
//     let one = tup.2;

//     println!("{five_hundred}");
//     println!("{six_point_four}");
//     println!("{one}");

// }


// fn main() {
//     let a = [1, 2, 3, 4, 5];
//     let element = a[2];
//     println!("The value of element is: {}", element);
// }


use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}