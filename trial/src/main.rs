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


// use std::io;

// fn main() {
//     let a = [1, 2, 3, 4, 5];

//     println!("Please enter an array index.");

//     let mut index = String::new();

//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read line");

//     let index: usize = index
//         .trim()
//         .parse()
//         .expect("Index entered was not a number");

//     let element = a[index];

//     println!("The value of the element at index {index} is: {element}");
// }


// fn main() {
//     println!("Hello, world!");

//     another_function();
// }

// fn another_function() {
//     println!("Another function.");
// }

// calling a plain function


// fn main() {
//     another_function(5);
// }

// fn another_function(x: i32) {
//     println!("The value of x is: {x}");
// }

// fn main() {
//     print_labeled_measurement(5, 'h');
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }


// fn five() -> i32 {
//     5
// }

// fn main() {
//     let x = five();

//     println!("The value of x is: {x}");
// }


// fn main() {
//     let x = plus_one(5);

//     println!("The value of x is: {x}");
// }

// fn plus_one(x: i32) -> i32 {
//     x + 1
// }

// // function with return variables



// fn main() {
//     let number = 3;

//     if number < 5 {
//         println!("condition was true");
//     } else {
//         println!("condition was false");
//     }
// }



// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }

//         count += 1;
//     }
//     println!("End count = {count}");


// fn main() {
//     let a = [10, 20, 30, 40, 50];

//     for element in a {
//         println!("the value is: {element}");
//     }
// }


// fn main() {
// //     let s1 = String::from("hello");
// //     let s2 = s1;
// //     println!("{s1}"); // ❌ compile error: value borrowed after move
//     let s1 = String::from("hello");
//     let s2 = s1.clone();
//     println!("{s1}, {s2}"); // both valid — separate heap allocations
// }


// fn main() {
//     let s = String::from("hello");
//     takes_ownership(s);       // s moves in — no longer usable here
//     // println!("{s}");          // ❌ compile error: value borrowed after move
//     let x = 5;
//     makes_copy(x);             // x is Copy — still usable after
//     println!("{x}");  // no error
// }
// fn takes_ownership(some_string: String) { println!("{some_string}"); } // dropped at end
// fn makes_copy(some_integer: i32) { println!("{some_integer}"); }        // nothing special happens


// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1); //The & is the important part.

//     println!("String: {s1}");
//     println!("Length: {len}");
// }


// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

// fn main() {
//     let s = String::from("hello");

//     change(&s);

//     println!("{s}");
// }

// // &s means immutable reference.


// now lets make the reference mutable 


// fn change (some_string: &mut String){
//     some_string.push_str(",world");
// }

// fn main() {
//     let mut s =  String::from("hello");

//     change(&mut s);
//             // ^ creates a mutable reference
//     println!("{s}");
// }



// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &mut s;
//     let r2 = &mut s;

//     println!("{r1}, {r2}");
// }

// //Rust doesn't allow both to exist at the same time


// mutiple immutable reference 


// fn main() {
//     let s = String::from("hello");

//     let r1 = &s;
//     let r2 = &s;

//     println!("{r1}, {r2}");

// }

// // /Both are only reading, so there's no conflict.


// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s;
//     let r2 = &s;

//     let r3 = &mut s;

//     println!("{r1}, {r2}, {r3}");
// }

// r1 ──┐
//      ├──→ s
// r2 ──┘

// r3 ───→ s
//        ↑
//    wants to modify

//Rust says: people are currently reading this value, so you can't modify it at the same time.



// fn main() {
//     let mut s = String::from("hello");

//     {
//         let r1 = &mut s;
//         r1.push_str(" world");
//         println!("{r1}");
//     } // r1 ends here

//     let r2 = &mut s;
//     r2.push_str("!");
//     println!("{r2}");
// }

// //Mutable references one after another


// fn main() {
//     let mut s = String::from("hello");
//     let r1 = &s;
//     let r2 = &s;
//     println!("{r1} and {r2}"); // r1, r2 last used here — their scope ends now

//     let r3 = &mut s; // ✅ fine — no overlap with r1/r2
//     println!("{r3}");
// }

// //reference is valid from where it's created until the last point it's actually used — not necessarily until the end of the block. This is called non-lexical lifetimes


// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s   ❌ reference to local variable

// }

// fn main() {
//     let r = dangle();
//     println!("{r}");
// }

// dangle()
//    │
//    ├── s → "hello"
//    │
//    └── &s → reference to "hello"

// } // s is dropped here

// fn no_dangle() -> String {
//     let s = String::from("hello");
//     s  // // ✅ ownership moves out
// }

// fn main() {
//     let r = no_dangle();

//     println!("{r}");
// }


//The Slice Type

// let s = String::from("hello");

// println!("{}", s[0]); // ❌ Error

// fn first_word(s: &String) -> usize{
//     let bytes = s.as_bytes();

//     for(i,&item) in bytes.iter().enumerate() {
//         if item == b' '{
//             return i;
//         }
//     }

//     s.len()
// }

// fn main() {
//     let mut s = String::from("hello world");
//     let word = first_word(&s);
//     println!("word index: {word}");
//     s.clear();
//     println!("string: '{s}'");
//     println!("word index: {word}");
// }


// fn main() {
//     let s = String::from("hello world");

//     let hello = &s[0..5];
//     let world = &s[6..11];
//     println!("{hello} {world}");
// }

// s
// │
// └──────────────────────────► h e l l o   w o r l d
//                               0 1 2 3 4 5 6 7 8 9 10
//                               └─────────┘
//                                   hello

// s
// │
// └──────────────────────────────► h e l l o   w o r l d
//                                   0 1 2 3 4 5 6 7 8 9 10
//                                             └─────────────┘
//                                                  world

// Both hello and world are simply views into s




// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
//     &s[..]
// }


// fn main() {
//     let mut s = String::from("hello world");
//     let word = first_word(&s);
//     s.clear();                    // ❌ error!
//     println!("{word}");
// }



// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }


// fn main() {
//     // let user1 = User {
//     //     active: true,
//     //     username: String::from("someusername123"),
//     //     email: String::from("someone@example.com"),
//     //     sign_in_count: 1,
//     // };
//     // user1.email = String::from("anotheremail@example.com");
//     // error cause user1 is not mutable 

//     // let mut user1 = User {
//     //     active: true,
//     //     username: String::from("someusername123"),
//     //     email: String::from("someone@example.com"),
//     //     sign_in_count: 1,
//     // };
//     // user1.email = String::from("anotheremail@example.com");
//     // println!("{}",user1.email);

//     // verbose
//     let user2 = User {
//         active: user1.active,
//         username: user1.username,
//         email: String::from("another@example.com"),
//         sign_in_count: user1.sign_in_count,
//     };

//     // with struct update syntax
//     let user2 = User {
//         email: String::from("another@example.com"),
//         ..user1
//     };
// }


// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username: username,
//         email: email,
//         sign_in_count: 1,
//     }
// }


// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!("rect1 is {rect1:?}"); // ❌ still fails — needs Debug too

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1)
//     );
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }


// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let scale = 2;
//     let rect1 = Rectangle {
//         width: dbg!(30 * scale),
//         height: 50,
//     };

//     dbg!(&rect1);
// }

// dbg! returns ownership of the expression’s value, the width field will get the same value as if we didn’t have the dbg! call there

// 1. What a method is

// Definition: A method is like a function, but it's defined inside an impl block for a specific type, and its first parameter is always self — the instance the method is called on.


// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// rect1.area() // method syntax: instance.method_name()

// // &self — borrows immutably (just reading). Most common.
// // &mut self — borrows mutably (method changes the instance).
// // self — takes ownership (rare; used when the method transforms self into something else and the original shouldn't be usable afterward).


// impl Rectangle {
//     fn width(&self) -> bool {
//         self.width > 0
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     if rect1.width() {
//         println!("The rectangle has a nonzero width; it is {}", rect1.width);
//     }
// }


// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn width(&self) -> bool {
//         self.width > 0
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!("Field: {}", rect1.width);
//     println!("Method: {}", rect1.width());
// }



//enum 


//n enum lets you say a value is one of a fixed set of possibilities


//defining a enum 
// enum Payment {
//     Cash,
//     Card,
//     UPI,
// }

// fn process_payment(payment: Payment) {
//     match payment {
//         Payment::Cash => {
//             println!("Paid using cash");
//         }

//         Payment::Card => {
//             println!("Paid using card");
//         }

//         Payment::UPI => {
//             println!("Paid using UPI");
//         }
//     }
// }


// fn main() {
//     let payment = Payment::Card;

//     process_payment(payment);
// }


//enum with data 
// enum Payment {
//     Cash,
//     Card(String),
//     UPI(String),
// }

// fn process_payment(payment:Payment){
//     match payment {
//         Payment::Cash => {
//             println!("Paid using cash");
//         }
//         Payment::Card(number) => {
//             println!("Card Number: {number}");
//         }
//         Payment::UPI(upi) => {
//             println!("UPI Id: {upi}");
//         }
//     }
// }

// fn main() {
//     let payment1 = Payment::Cash;
//     let payment2 = Payment::Card("1234567890".to_string());
//     let payment3 = Payment::UPI("1234567890".to_string());

//     process_payment(payment1);
//     process_payment(payment2);
//     process_payment(payment3);
// }


// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

// fn value(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }


// fn main() {
//     let coin = Coin::Quarter;

//     let money = value(coin);

//     println!("{money}");
// }


// 1. What is a crate?
// A crate is the smallest unit of code that the Rust compiler compiles.


//             Crate
//               │
//        ┌──────┴──────┐
//        ↓             ↓
//    Binary          Library

// Binary crate
// A binary crate creates an executable program.
// For eg 
// fn main() {
//     println!("Hello");
// }

//Library Crate 
// A library crate doesn't have a main() function. Instead, it provides functionality that other programs can use.



// what is package 
// A package is a bundle of one or more crates.


// What does Cargo.toml do?
// Cargo.toml describes the package and tells Cargo how to build it.

    //              PACKAGE
    //                 │
    //          Cargo.toml
    //                 │
    //       ┌─────────┴─────────┐
    //       │                   │
    //   BINARY CRATE         LIBRARY CRATE
    //   src/main.rs           src/lib.rs
    //       │                   │
    //  executable          reusable code


// 1. What is a module?
// A module is a way to organize related code.

// mod front_of_house {
//     mod hosting {
//         fn add_to_waitlist() {}
//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}
//         fn serve_order() {}
//         fn take_payment() {}
//     }
// }



// mod front_of_house {
//     mod hosting {
//         fn add_to_waitlist() {}
//     }
// }


// crate
// │
// └── front_of_house
//        │
//        └── hosting
//               │
//               └── add_to_waitlist()

// crate represents the root of your current crate.

// Making something public with pub

// A path tells Rust where something is located.

// Paths can be absolute or relative.




// 12. What does use do?
// Suppose you have this long path:
// crate::garden::vegetables::Asparagus
// Instead of repeatedly writing:
// let a = crate::garden::vegetables::Asparagus {};
// let b = crate::garden::vegetables::Asparagus {};

// you can write:
// use crate::garden::vegetables::Asparagus;
// Then:
// let a = Asparagus {};
// So:
// use crate::garden::vegetables::Asparagus;
// basically means:
// "Bring Asparagus into my current scope so I can use its short name."

// mod
//  ↓
// Create/declare a module

// pub
//  ↓
// Make something public

// ::
//  ↓
// Navigate through a path

// use
//  ↓
// Create a shortcut to a path

// crate
//  ↓
// Root of the current crate

// parent
//  ↓
// Module containing another module

// child
//  ↓
// Module inside another module

// siblings
//  ↓
// Modules at the same level


// Absolute path
// An absolute path starts from the root.

// Relative path
// A relative path starts from where you currently are



// Syntax	Meaning
// use crate::foo;	Create shortcut foo
// use crate::foo::bar;	Import bar
// use x as y;	Rename x to y
// pub use x;	Import + make it publicly available
// use std::{A, B};	Import multiple items
// use std::io::{self, Write};	Import module + item
// use std::*;	Import all public items