// // fn main() {
// //     let x = 5;
// //     let x = x + 1;          // new x = 6, old x discarded

// //     {
// //         let x = x * 2;       // inner-scope x = 12, only exists inside {}
// //         println!("The value of x in the inner scope is: {x}");
// //     }

// //     println!("The value of x is: {x}");   // back to outer x = 6
// // }

// // fn main() {
// //     let x = 5;
// //     println!("The value of x is: {x}");
// //     x = 6;
// //     println!("The value of x is: {x}");
// // }

// /// we cannot do it as we can not assign a immutable variable twice

// // fn main() {
// //     let mut x = 5;
// //     println!("The value of x is: {x}");
// //     x = 6;
// //     println!("The value of x is: {x}");
// // }

// // we define variable with mut which means mutable

// // fn main() {
// //     let x = 5;
// //     let x = x + 1;          // new x = 6, old x discarded

// //     {
// //         let x = x * 2;       // inner-scope x = 12, only exists inside {}
// //         println!("The value of x in the inner scope is: {x}");
// //     }

// //     println!("The value of x is: {x}");   // back to outer x = 6
// // }

// /// The value of x in the inner scope is: 12
// /// The value of x is: 6

// // fn main() {
// //     let spaces = "  ";
// //     let spaces = spaces.len();
// //     println!("Number of spaces: {spaces}")
// // }

// /// Output : Number of spaces : 2

// // fn main() {
// //     let mut spaces = "  ";
// //     spaces = spaces.len();
// //     println!("Number of spaces: {spaces}")
// // }

// //error occured as we try to assign a value of different type to a variable

// // fn main() {
// //     let guess = "42".parse().expect("Not a number!");
// //     println!("{guess}");
// // }

// //error[E0284]: type annotations needed
// //type must be known at this point
// //help: consider giving `guess` an explicit type

// // fn main() {
// //     let guess: u32 = "42".parse().expect("Not a number!");
// //     println!("{guess}");

// // //1. Unsigned integers (u) -  Unsigned means only positive numbers and zero.
// // //2.Signed integers (i) - integer with a sign, so it can store negative and positive values.
// // // f is used for floating-point numbers

// // let a: u32 = "42".parse().unwrap();
// // println!("{a}");

// // let b: i32 = "42".parse().unwrap();
// // println!("{b}");

// // let c: f64 = "42".parse().unwrap();
// // println!("{c}");

// // }

// // use std::env::args;

// // fn main() {
// //     let args: Vec<String> = std::env::args().collect(); // call args(), collect into a real Vec
// //     let x: u8 = args[1].parse().unwrap();                // .parse() with parentheses
// //     let y = x + 1;
// //     println!("{y}");
// // }

// //attempt to add with overflow

// // fn main() {
// //     let tup: (i32, f64, u8) = (500, 6.4, 1);
// //     let (x, y, z) = tup;
// //     println!("{y}");
// //     let five_hundred = tup.0;         // direct index access with .N
// //     let six_point_four = tup.1;
// //     let one = tup.2;

// //     println!("{five_hundred}");
// //     println!("{six_point_four}");
// //     println!("{one}");

// // }

// // fn main() {
// //     let a = [1, 2, 3, 4, 5];
// //     let element = a[2];
// //     println!("The value of element is: {}", element);
// // }

// // use std::io;

// // fn main() {
// //     let a = [1, 2, 3, 4, 5];

// //     println!("Please enter an array index.");

// //     let mut index = String::new();

// //     io::stdin()
// //         .read_line(&mut index)
// //         .expect("Failed to read line");

// //     let index: usize = index
// //         .trim()
// //         .parse()
// //         .expect("Index entered was not a number");

// //     let element = a[index];

// //     println!("The value of the element at index {index} is: {element}");
// // }

// // fn main() {
// //     println!("Hello, world!");

// //     another_function();
// // }

// // fn another_function() {
// //     println!("Another function.");
// // }

// // calling a plain function

// // fn main() {
// //     another_function(5);
// // }

// // fn another_function(x: i32) {
// //     println!("The value of x is: {x}");
// // }

// // fn main() {
// //     print_labeled_measurement(5, 'h');
// // }

// // fn print_labeled_measurement(value: i32, unit_label: char) {
// //     println!("The measurement is: {value}{unit_label}");
// // }

// // fn five() -> i32 {
// //     5
// // }

// // fn main() {
// //     let x = five();

// //     println!("The value of x is: {x}");
// // }

// // fn main() {
// //     let x = plus_one(5);

// //     println!("The value of x is: {x}");
// // }

// // fn plus_one(x: i32) -> i32 {
// //     x + 1
// // }

// // // function with return variables

// // fn main() {
// //     let number = 3;

// //     if number < 5 {
// //         println!("condition was true");
// //     } else {
// //         println!("condition was false");
// //     }
// // }

// // fn main() {
// //     let mut count = 0;
// //     'counting_up: loop {
// //         println!("count = {count}");
// //         let mut remaining = 10;

// //         loop {
// //             println!("remaining = {remaining}");
// //             if remaining == 9 {
// //                 break;
// //             }
// //             if count == 2 {
// //                 break 'counting_up;
// //             }
// //             remaining -= 1;
// //         }

// //         count += 1;
// //     }
// //     println!("End count = {count}");

// // fn main() {
// //     let a = [10, 20, 30, 40, 50];

// //     for element in a {
// //         println!("the value is: {element}");
// //     }
// // }

// // fn main() {
// // //     let s1 = String::from("hello");
// // //     let s2 = s1;
// // //     println!("{s1}"); // ❌ compile error: value borrowed after move
// //     let s1 = String::from("hello");
// //     let s2 = s1.clone();
// //     println!("{s1}, {s2}"); // both valid — separate heap allocations
// // }

// // fn main() {
// //     let s = String::from("hello");
// //     takes_ownership(s);       // s moves in — no longer usable here
// //     // println!("{s}");          // ❌ compile error: value borrowed after move
// //     let x = 5;
// //     makes_copy(x);             // x is Copy — still usable after
// //     println!("{x}");  // no error
// // }
// // fn takes_ownership(some_string: String) { println!("{some_string}"); } // dropped at end
// // fn makes_copy(some_integer: i32) { println!("{some_integer}"); }        // nothing special happens

// // fn calculate_length(s: &String) -> usize {
// //     s.len()
// // }

// // fn main() {
// //     let s1 = String::from("hello");

// //     let len = calculate_length(&s1); //The & is the important part.

// //     println!("String: {s1}");
// //     println!("Length: {len}");
// // }

// // fn change(some_string: &String) {
// //     some_string.push_str(", world");
// // }

// // fn main() {
// //     let s = String::from("hello");

// //     change(&s);

// //     println!("{s}");
// // }

// // // &s means immutable reference.

// // now lets make the reference mutable

// // fn change (some_string: &mut String){
// //     some_string.push_str(",world");
// // }

// // fn main() {
// //     let mut s =  String::from("hello");

// //     change(&mut s);
// //             // ^ creates a mutable reference
// //     println!("{s}");
// // }

// // fn main() {
// //     let mut s = String::from("hello");

// //     let r1 = &mut s;
// //     let r2 = &mut s;

// //     println!("{r1}, {r2}");
// // }

// // //Rust doesn't allow both to exist at the same time

// // mutiple immutable reference

// // fn main() {
// //     let s = String::from("hello");

// //     let r1 = &s;
// //     let r2 = &s;

// //     println!("{r1}, {r2}");

// // }

// // // /Both are only reading, so there's no conflict.

// // fn main() {
// //     let mut s = String::from("hello");

// //     let r1 = &s;
// //     let r2 = &s;

// //     let r3 = &mut s;

// //     println!("{r1}, {r2}, {r3}");
// // }

// // r1 ──┐
// //      ├──→ s
// // r2 ──┘

// // r3 ───→ s
// //        ↑
// //    wants to modify

// //Rust says: people are currently reading this value, so you can't modify it at the same time.

// // fn main() {
// //     let mut s = String::from("hello");

// //     {
// //         let r1 = &mut s;
// //         r1.push_str(" world");
// //         println!("{r1}");
// //     } // r1 ends here

// //     let r2 = &mut s;
// //     r2.push_str("!");
// //     println!("{r2}");
// // }

// // //Mutable references one after another

// // fn main() {
// //     let mut s = String::from("hello");
// //     let r1 = &s;
// //     let r2 = &s;
// //     println!("{r1} and {r2}"); // r1, r2 last used here — their scope ends now

// //     let r3 = &mut s; // ✅ fine — no overlap with r1/r2
// //     println!("{r3}");
// // }

// // //reference is valid from where it's created until the last point it's actually used — not necessarily until the end of the block. This is called non-lexical lifetimes

// // fn dangle() -> &String {
// //     let s = String::from("hello");
// //     &s   ❌ reference to local variable

// // }

// // fn main() {
// //     let r = dangle();
// //     println!("{r}");
// // }

// // dangle()
// //    │
// //    ├── s → "hello"
// //    │
// //    └── &s → reference to "hello"

// // } // s is dropped here

// // fn no_dangle() -> String {
// //     let s = String::from("hello");
// //     s  // // ✅ ownership moves out
// // }

// // fn main() {
// //     let r = no_dangle();

// //     println!("{r}");
// // }

// //The Slice Type

// // let s = String::from("hello");

// // println!("{}", s[0]); // ❌ Error

// // fn first_word(s: &String) -> usize{
// //     let bytes = s.as_bytes();

// //     for(i,&item) in bytes.iter().enumerate() {
// //         if item == b' '{
// //             return i;
// //         }
// //     }

// //     s.len()
// // }

// // fn main() {
// //     let mut s = String::from("hello world");
// //     let word = first_word(&s);
// //     println!("word index: {word}");
// //     s.clear();
// //     println!("string: '{s}'");
// //     println!("word index: {word}");
// // }

// // fn main() {
// //     let s = String::from("hello world");

// //     let hello = &s[0..5];
// //     let world = &s[6..11];
// //     println!("{hello} {world}");
// // }

// // s
// // │
// // └──────────────────────────► h e l l o   w o r l d
// //                               0 1 2 3 4 5 6 7 8 9 10
// //                               └─────────┘
// //                                   hello

// // s
// // │
// // └──────────────────────────────► h e l l o   w o r l d
// //                                   0 1 2 3 4 5 6 7 8 9 10
// //                                             └─────────────┘
// //                                                  world

// // Both hello and world are simply views into s

// // fn first_word(s: &String) -> &str {
// //     let bytes = s.as_bytes();
// //     for (i, &item) in bytes.iter().enumerate() {
// //         if item == b' ' {
// //             return &s[0..i];
// //         }
// //     }
// //     &s[..]
// // }

// // fn main() {
// //     let mut s = String::from("hello world");
// //     let word = first_word(&s);
// //     s.clear();                    // ❌ error!
// //     println!("{word}");
// // }

// // struct User {
// //     active: bool,
// //     username: String,
// //     email: String,
// //     sign_in_count: u64,
// // }

// // fn main() {
// //     // let user1 = User {
// //     //     active: true,
// //     //     username: String::from("someusername123"),
// //     //     email: String::from("someone@example.com"),
// //     //     sign_in_count: 1,
// //     // };
// //     // user1.email = String::from("anotheremail@example.com");
// //     // error cause user1 is not mutable

// //     // let mut user1 = User {
// //     //     active: true,
// //     //     username: String::from("someusername123"),
// //     //     email: String::from("someone@example.com"),
// //     //     sign_in_count: 1,
// //     // };
// //     // user1.email = String::from("anotheremail@example.com");
// //     // println!("{}",user1.email);

// //     // verbose
// //     let user2 = User {
// //         active: user1.active,
// //         username: user1.username,
// //         email: String::from("another@example.com"),
// //         sign_in_count: user1.sign_in_count,
// //     };

// //     // with struct update syntax
// //     let user2 = User {
// //         email: String::from("another@example.com"),
// //         ..user1
// //     };
// // }

// // fn build_user(email: String, username: String) -> User {
// //     User {
// //         active: true,
// //         username: username,
// //         email: email,
// //         sign_in_count: 1,
// //     }
// // }

// // #[derive(Debug)]
// // struct Rectangle {
// //     width: u32,
// //     height: u32,
// // }

// // fn main() {
// //     let rect1 = Rectangle {
// //         width: 30,
// //         height: 50,
// //     };

// //     println!("rect1 is {rect1:?}"); // ❌ still fails — needs Debug too

// //     println!(
// //         "The area of the rectangle is {} square pixels.",
// //         area(&rect1)
// //     );
// // }

// // fn area(rectangle: &Rectangle) -> u32 {
// //     rectangle.width * rectangle.height
// // }

// // #[derive(Debug)]
// // struct Rectangle {
// //     width: u32,
// //     height: u32,
// // }

// // fn main() {
// //     let scale = 2;
// //     let rect1 = Rectangle {
// //         width: dbg!(30 * scale),
// //         height: 50,
// //     };

// //     dbg!(&rect1);
// // }

// // dbg! returns ownership of the expression’s value, the width field will get the same value as if we didn’t have the dbg! call there

// // 1. What a method is

// // Definition: A method is like a function, but it's defined inside an impl block for a specific type, and its first parameter is always self — the instance the method is called on.

// // impl Rectangle {
// //     fn area(&self) -> u32 {
// //         self.width * self.height
// //     }
// // }

// // rect1.area() // method syntax: instance.method_name()

// // // &self — borrows immutably (just reading). Most common.
// // // &mut self — borrows mutably (method changes the instance).
// // // self — takes ownership (rare; used when the method transforms self into something else and the original shouldn't be usable afterward).

// // impl Rectangle {
// //     fn width(&self) -> bool {
// //         self.width > 0
// //     }
// // }

// // fn main() {
// //     let rect1 = Rectangle {
// //         width: 30,
// //         height: 50,
// //     };

// //     if rect1.width() {
// //         println!("The rectangle has a nonzero width; it is {}", rect1.width);
// //     }
// // }

// // struct Rectangle {
// //     width: u32,
// //     height: u32,
// // }

// // impl Rectangle {
// //     fn width(&self) -> bool {
// //         self.width > 0
// //     }
// // }

// // fn main() {
// //     let rect1 = Rectangle {
// //         width: 30,
// //         height: 50,
// //     };

// //     println!("Field: {}", rect1.width);
// //     println!("Method: {}", rect1.width());
// // }

// //enum

// //n enum lets you say a value is one of a fixed set of possibilities

// //defining a enum
// // enum Payment {
// //     Cash,
// //     Card,
// //     UPI,
// // }

// // fn process_payment(payment: Payment) {
// //     match payment {
// //         Payment::Cash => {
// //             println!("Paid using cash");
// //         }

// //         Payment::Card => {
// //             println!("Paid using card");
// //         }

// //         Payment::UPI => {
// //             println!("Paid using UPI");
// //         }
// //     }
// // }

// // fn main() {
// //     let payment = Payment::Card;

// //     process_payment(payment);
// // }

// //enum with data
// // enum Payment {
// //     Cash,
// //     Card(String),
// //     UPI(String),
// // }

// // fn process_payment(payment:Payment){
// //     match payment {
// //         Payment::Cash => {
// //             println!("Paid using cash");
// //         }
// //         Payment::Card(number) => {
// //             println!("Card Number: {number}");
// //         }
// //         Payment::UPI(upi) => {
// //             println!("UPI Id: {upi}");
// //         }
// //     }
// // }

// // fn main() {
// //     let payment1 = Payment::Cash;
// //     let payment2 = Payment::Card("1234567890".to_string());
// //     let payment3 = Payment::UPI("1234567890".to_string());

// //     process_payment(payment1);
// //     process_payment(payment2);
// //     process_payment(payment3);
// // }

// // enum Coin {
// //     Penny,
// //     Nickel,
// //     Dime,
// //     Quarter,
// // }

// // fn value(coin: Coin) -> u8 {
// //     match coin {
// //         Coin::Penny => 1,
// //         Coin::Nickel => 5,
// //         Coin::Dime => 10,
// //         Coin::Quarter => 25,
// //     }
// // }

// // fn main() {
// //     let coin = Coin::Quarter;

// //     let money = value(coin);

// //     println!("{money}");
// // }

// // 1. What is a crate?
// // A crate is the smallest unit of code that the Rust compiler compiles.

// //             Crate
// //               │
// //        ┌──────┴──────┐
// //        ↓             ↓
// //    Binary          Library

// // Binary crate
// // A binary crate creates an executable program.
// // For eg
// // fn main() {
// //     println!("Hello");
// // }

// //Library Crate
// // A library crate doesn't have a main() function. Instead, it provides functionality that other programs can use.

// // what is package
// // A package is a bundle of one or more crates.

// // What does Cargo.toml do?
// // Cargo.toml describes the package and tells Cargo how to build it.

//     //              PACKAGE
//     //                 │
//     //          Cargo.toml
//     //                 │
//     //       ┌─────────┴─────────┐
//     //       │                   │
//     //   BINARY CRATE         LIBRARY CRATE
//     //   src/main.rs           src/lib.rs
//     //       │                   │
//     //  executable          reusable code

// // 1. What is a module?
// // A module is a way to organize related code.

// // mod front_of_house {
// //     mod hosting {
// //         fn add_to_waitlist() {}
// //         fn seat_at_table() {}
// //     }

// //     mod serving {
// //         fn take_order() {}
// //         fn serve_order() {}
// //         fn take_payment() {}
// //     }
// // }

// // mod front_of_house {
// //     mod hosting {
// //         fn add_to_waitlist() {}
// //     }
// // }

// // crate
// // │
// // └── front_of_house
// //        │
// //        └── hosting
// //               │
// //               └── add_to_waitlist()

// // crate represents the root of your current crate.

// // Making something public with pub

// // A path tells Rust where something is located.

// // Paths can be absolute or relative.

// // 12. What does use do?
// // Suppose you have this long path:
// // crate::garden::vegetables::Asparagus
// // Instead of repeatedly writing:
// // let a = crate::garden::vegetables::Asparagus {};
// // let b = crate::garden::vegetables::Asparagus {};

// // you can write:
// // use crate::garden::vegetables::Asparagus;
// // Then:
// // let a = Asparagus {};
// // So:
// // use crate::garden::vegetables::Asparagus;
// // basically means:
// // "Bring Asparagus into my current scope so I can use its short name."

// // mod
// //  ↓
// // Create/declare a module

// // pub
// //  ↓
// // Make something public

// // ::
// //  ↓
// // Navigate through a path

// // use
// //  ↓
// // Create a shortcut to a path

// // crate
// //  ↓
// // Root of the current crate

// // parent
// //  ↓
// // Module containing another module

// // child
// //  ↓
// // Module inside another module

// // siblings
// //  ↓
// // Modules at the same level

// // Absolute path
// // An absolute path starts from the root.

// // Relative path
// // A relative path starts from where you currently are

// // Syntax	Meaning
// // use crate::foo;	Create shortcut foo
// // use crate::foo::bar;	Import bar
// // use x as y;	Rename x to y
// // pub use x;	Import + make it publicly available
// // use std::{A, B};	Import multiple items
// // use std::io::{self, Write};	Import module + item
// // use std::*;	Import all public items

// // vector normally stores values of the same type.
// // let numbers = vec![1, 2, 3, 4];       // Vec<i32>
// // let names = vec!["Arnav", "Rahul"];   // Vec<&str>

// // You can't normally do:
// // let v = vec![10, "hello", 20]; // ❌

// //creating a vector

// // fn main(){
//     // let mut v = Vec::new();

//     // v.push(10);
//     // v.push(20);
//     // v.push(30);

//     // println!("{:?}", v);

//     // let v = vec![10, 20, 30, 40];

//     // // let x = v[2];
//     // // println!("{x}");

//     // let x = v.get(1);
//     // println!("{:?}", x);

//     //example of borrowing

//     // let mut v = vec![1,2,3,4,5];
//     // let first = &v[0];
//     // v.push(6);
//     // println!("the first element is: {first}");

//     // //error
//     // //A vector stores its elements next to each other in memory
//     // //Sometimes when you push() a new element, Rust may need to move the entire vector to a new memory location if the old location doesn't have enough space.

//     // let v = vec![100, 32, 57];

//     // for i in &v {
//     //     println!("{i}");
//     // }

// //     let mut v = vec![100, 32, 57];

// //     for i in &mut v {
// //         *i += 50;
// //     }

// //     println!("{:?}", v);
// //  //   *i means:the actual value being referenced

// // }

// // let name = "Arnav";
// // Here "Arnav" is a &str

// // let name = String::from("Arnav");
// // This is an owned, growable string.

// // fn main() {
// //     let s = "नमस्ते";

// //     for c in s.chars() {
// //         println!("{c}");
// //     }
// // }

// // fn main() {

// //     use std::collections::HashMap;

// //     let mut scores = HashMap::new();

// //     scores.insert(String::from("Blue"), 10);
// //     scores.insert(String::from("Yellow"), 50);

// //     // let score = scores.get("Blue");
// //     // println!("{:?}",score);
// //     // //Some(10)

// //     // let score = scores.get("Blue").copied().unwrap_or(0);
// //     // println!("{:?}",score);

// //     //update and overwrite
// //     //scores.insert(String::from("Blue"), 25);

// //     //check if key exists and only then insert
// //     scores.entry(String::from("Blue")).or_insert(50);
// //     let score = scores.get("Blue").copied().unwrap_or(0);
// //     println!("{:?}",score);
// //     //it will not print 50 cause blue already exist

// // }

// // fn main() {
// //     use std::collections::HashMap;

// //     let text = "hello world hello";

// //     let mut map = HashMap::new();

// //     for word in text.split_whitespace() {
// //         let count = map.entry(word).or_insert(0);
// //         *count += 1;
// //     }

// //     println!("{map:?}");
// // }

// // enum Result<T, E> {
// //     Ok(T),
// //     Err(E),
// // }

// // fn divide(a: i32, b: i32) -> Result<i32, String> {
// //     if b == 0 {
// //         Err(String::from("Cannot divide by zero"))
// //     } else {
// //         Ok(a / b)
// //     }
// // }

// // fn main() {
// //     // let result = divide(10, 2);
// //     // println!("{:?}", result); // Ok(5)
// //     match divide(10, 0) {
// //     Ok(value) => println!("Result: {value}"),
// //     Err(error) => println!("Error: {error}"),
// //     }
// //     //Error: Cannot divide by zero

// // }

// // use std::fs::File;
// // use std::io::ErrorKind;

// // let file = match File::open("hello.txt") {
// //     Ok(file) => file,

// //     Err(error) => match error.kind() {
// //         ErrorKind::NotFound => {
// //             File::create("hello.txt").unwrap()
// //         }

// //         _ => {
// //             panic!("Something went wrong: {error:?}");
// //         }
// //     },
// // };

// // Try opening file
// //       ↓
// //     Ok?
// //    /   \
// //  Yes    No
// //  ↓       ↓
// // Use    Why?
// // file     ↓
// //        NotFound?
// //        /      \
// //      Yes       No
// //       ↓         ↓
// //    Create     panic
// //    file

// // fn divide(a: i32, b: i32) -> Result<i32, String> {
// //     if b == 0 {
// //         Err(String::from("Cannot divide by zero"))
// //     } else {
// //         Ok(a / b)
// //     }
// // }

// // use std::fs::File;

// // fn main() {
// //     let result = File::open("hello.txt");
// //     match result {
// //     Ok(file) => println!("File opened!"),
// //     Err(error) => println!("Could not open file: {error}"),
// //     }
// // }

// // use std::fs::File;
// // use std::io::ErrorKind;
// // fn main() {

// //     let file = match File::open("hello.txt") {
// //         Ok(file) => file,

// //         Err(error) => match error.kind() {
// //             ErrorKind::NotFound => {
// //                 File::create("hello.txt").unwrap()
// //             }

// //             _ => {
// //                 panic!("Something went wrong: {error:?}");
// //             }
// //         },
// //     };
// // }

// // use std::fs::File;
// // use std::io;
// // use std::io::Read;

// // fn main() {
// //     let mut file = match File::open("hello.txt") {
// //         Ok(file) => file,
// //         Err(e) => {
// //             println!("Error opening file: {e}");
// //             return;
// //         }
// //     };

// //     let mut username = String::new();

// //     match file.read_to_string(&mut username) {
// //         Ok(_) => println!("{username}"),
// //         Err(e) => println!("Error reading file: {e}"),
// //     };
// // }

// use std::cmp::PartialOrd;

// fn largest<T: PartialOrd>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&number_list);
//     println!("The largest number is {result}");

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest(&char_list);
//     println!("The largest char is {result}");
// }

// Because largest is generic over T, it doesn't care what type it's working with — it only cares that T implements PartialOrd (so > is legal) and that values can be moved/copied around.

// struct Point<T, U> {   // two type params — x and y can differ
//     x: T,
//     y: U,
// }

// fn main() {
//     let both_integer = Point { x: 5, y: 10 };       // T = i32, U = i32
//     let both_float = Point { x: 1.0, y: 4.0 };      // T = f64, U = f64
//     let integer_and_float = Point { x: 5, y: 4.0 }; // T = i32, U = f64
// }

// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// fn main() {
//     let p = Point { x: 5, y: 10 };

//     println!("p.x = {}", p.x());
// }

//writing test

// pub fn add(left: u64 , right: u64)->u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2,2);
//         assert_eq!(result,4);
//     }
// }

// #[test]
// fn check_bool() {
//     assert!(5 > 3);              // pass if true
// }

// #[test]
// fn check_eq() {
//     assert_eq!(add(2, 2), 4);    // pass if equal, prints both values on failure
// }

// #[test]
// fn check_ne() {
//     assert_ne!(add(2, 2), 5);    // pass if NOT equal
// }

// #[test]
// fn greeting_contains_name() {
//     let result = greeting("Carol");
//     assert!(
//         result.contains("Carol"),
//         "Greeting did not contain name, value was `{result}`"
//     );
// }

// // fn greeting(name: &str) -> String {
// //     format!("Hello, {name}!")
// // }//passes

// fn greeting(name: &str) -> String {
//     String::from("Hello!")
// }

// testing panic

// pub struct Guess { value: i32 }

// impl Guess {
//     pub fn new(value: i32) -> Guess {
//         if value < 1 || value > 100 {
//             panic!("Guess value must be between 1 and 100, got {value}.");
//         }
//         Guess { value }
//     }
// }

// #[test]
// #[should_panic(expected = "less than or equal to 100")]
// fn greater_than_100() {
//     Guess::new(200);
// }

// test result

// #[test]
// fn it_works() -> Result<(),String> {
//     if add(2,2) == 4{
//         Ok(())
//     }else {
//         Err(String::from("two plus two does not equal to four"))
//     }
// }

// #[derive(Debug)]

// enum Shape {
//     Circle(f64),
//     Rectangle(f64,f64),
// }
// const PI:f64 = 3.14;
// impl Shape {
//     fn new_circle(r:f64)->Self{
//         Self::Circle(r)
//     }
//     fn new_rectange(l:f64,b:f64)->Self{
//         Self::Rectangle(l,b)
//     }

//     fn area(&self){
//         match self{
//             Shape::Circle(r)=>println!("Area of the circle:{}",PI*r*r),
//             Shape::Rectangle(l,b)=>println!("Area of the rectangle:{}",l*b),
//         }
//     }
// }

// fn main() {
//     let circle: Shape = Shape::new_circle(5.0);
//     println!("Circle:{:?}",circle);
//     let reactange: Shape = Shape::new_rectange(5.0,4.0);
//     println!("Rectange:{:?}",reactange);
//     circle.area();
//     reactange.area();
// }

// fn main() {
//     let add_one = |x:i32|x + 1;
//     println!("{}",add_one(5));
// }

// fn main() {
//     let mut counter = 0;
//     let mut increase_counter = ||{
//         counter = counter +1 ;
//         println!("{}",counter);
//     };

//     increase_counter();
//     increase_counter();
//     increase_counter();
//     increase_counter();
//     println!("{}",counter);
// }

// fn main() {
//     // let x: String = String::from("hello");
//     // let consume_and_return_x = || x;
//     // // println!("{}",x);
//     // // println!("{}",consume_and_return_x());
//     // let y = consume_and_return_x();
//     // println!("{}",y);

//     let x: String = String::from("hello");
//     let consume_and_return_x = || &x;
//     // println!("{}",x);
//     let y = consume_and_return_x();
//     println!("{}",y);
//     println!("{}",consume_and_return_x());

//     // in the above i was tranfering the ownership from x to y thats when when i was using x i was not able to cause i tranffered but in the second i borrowed x

// }

// fn main() {
//     let vec = vec![3,2,3,4];

//     // let double_vec:Vec<i32> = vec.into_iter().map(|x|x*2).collect();
//     // println!("{:?}",double_vec);

//     let double_vec:Vec<i32> = vec.iter().map(|x| *x * 2).collect();
//     println!("{:?}",double_vec);

//     let even_vec: Vec<&i32> = vec.iter().filter(|x| *x%2 == 0).collect();
//     println!("{:?}",even_vec);

//     match vec.into_iter().reduce(|acc,item|acc+item){
//         Some(sum)=>println!("Sum of the vector: {sum}"),
//         None=>println!("Vector is empty"),
//     }

//     //pseudo code
//     // acc = 1(first array element)
//     // acc = acc+item ( till the vector get exhausted)

// }

// pub fn add(a: i32, b: i32) -> i32 {
//     a + b
// }

// pub fn divide(a: i32, b: i32) -> i32 {
//     if b == 0 {
//         panic!("cannot divide by zero");
//     }

//     a / b
// }

// enum List {
//     Cons(i32,Box<List>),
//     Nil,
// }

// fn main() {
//     let list = List::Cons(1,Box::new(List::Cons(2,Box::new(List::Nil))));

//     let x = 5;
//     let boxed = Box::new(x);
//     println!("{}",*boxed);
// }

// use std::rc::Rc;

// fn main() {
//     let a = Rc::new(String::from("hello"));
//     println!("count after creating a = {}", Rc::strong_count(&a)); // 1

//     let b = Rc::clone(&a); // just increments the counter, no deep copy
//     println!("count after creating b = {}", Rc::strong_count(&a)); // 2

//     {
//         let c = Rc::clone(&a);
//         println!("count after creating c = {}", Rc::strong_count(&a)); // 3
//     } // c dropped here

//     println!("count after c is dropped = {}", Rc::strong_count(&a)); // 2
// }

// use std::cell::RefCell;

// fn main() {
//     let data = RefCell::new(5);

//     *data.borrow_mut() += 10; // mutable borrow, checked at runtime
//     println!("{}",data.borrow());

// }

// use std::rc::Rc;
// use std::cell::RefCell;

// #[derive(Debug)]
// struct Counter {
//     value:i32,
// }

// fn main() {
//     let shared = Rc::new(RefCell::new(Counter {value:0}));

//     let a = Rc::clone(&shared);
//     let b = Rc::clone(&shared);

//     a.borrow_mut().value += 1;
//     b.borrow_mut().value += 5;

//     println!("{}", shared.borrow().value);
// }

// use std::sync::{Arc, Mutex};  //Arc = Atomic Reference Counting
// Rc  → single-threaded
// Arc → multi-threaded
// Mutex = Mutual Exclusion
// It ensures that only one thread can access the protected data at a time
// use std::thread;

// fn main() {
//     let counter = Arc::new(Mutex::new(0));
// //         ┌── Thread 1
// //         │
// //         ├── Thread 2
// // Arc ────┼── Thread 3
// //         │
// //         └── Thread 10
// //              │
// //              ▼
// //           Mutex
// //              │
// //              ▼
// //              0
//     let mut handles =  vec![];

//     for _ in 0..10 {
//         let counter = Arc::clone(&counter);
//         handles.push(thread::spawn(move ||{
//             let mut num = counter.lock().unwrap();
//             *num += 1;
//         }))
//     }
//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("Result: {}", *counter.lock().unwrap());
// }

// use std::thread;
// use std::time::Duration;

// // fn main() {
// //     let handle = thread::spawn(||{ // this creates a new thread
// //         for i in 1..5 {
// //             println!("spawned thread:{}",i);
// //             thread::sleep(Duration::from_millis(1));
// //         }
// //     });

// //     for i in 1..5 {
// //         println!("main thread: {}",i);
// //         thread::sleep(Duration::from_millis(1));
// //     }

// //     handle.join().unwrap();
// // }

// fn main() {
//     let v = vec![1,2,3];

//     let handle = thread::spawn(move ||{
//         //"Move the variables that this closure uses into the closure."
//         println!("Here is a vector: {:?}",v);
//         // v is moved into the closure
//     });

//     // println!("{:?}",v); // this will not work cause v is moved into the closure
//     //"You don't own v anymore."
//     handle.join().unwrap();

// //     Before:

// // Main thread
// //     |
// //     v
// //     ↓
// //  [1,2,3]

// // After move:

// // Main thread                    Spawned thread
// //                                |
// //                                v
// //                             [1,2,3]

// }

// use std::sync::mpsc;
// use std::thread;

// fn main() {
//     let (tx, rx) = mpsc::channel(); // multi-producer, single-consumer
//     //tx means transmitter → sender
//     //rx means receiver → receives messages.

//     thread::spawn(move || {
//         //The move is important because we're going to use tx inside the new thread:

//         let vals = vec!["hi", "from", "the", "thread"];
//         for val in vals {
//             tx.send(val.to_string()).unwrap();
//             //val is a &str.
//             //.to_string() converts it into a String:
//             //Spawned thread
//             // "hi"
//             //  ↓
//             // tx.send("hi")
//             //  ↓
//             // CHANNEL
//             //  ↓
//             // rx
//             //What does unwrap() do?
//             //send() returns a Result
//             thread::sleep(std::time::Duration::from_millis(100));
//         }
//         // tx dropped here when thread ends
//     });

//     // rx acts as an iterator — blocks waiting for each message
//     for received in rx {
//         println!("Got: {}", received);
//     }
//     // loop ends automatically when tx is dropped and channel closes
// }

//              CHANNEL
//     ┌────────────────────────┐
//     │                        │
//     │  hi                    │
//     │  from                  │
//     │  the                   │
//     │  thread                │
//     │                        │
//     └────────────────────────┘
//          ↑              ↓
//          │              │
//         tx             rx
//          │              │
//    Spawned thread    Main thread
//          │              │
//     tx.send(...)    for received in rx
//          │              │
//          │          println!("Got...")
//          │
//     sleep 100ms
//          │
//     send next...
//          │
//          ↓
//     thread finishes
//          │
//       tx dropped
//          │
//    channel closes
//          │
//          └────────────→ rx loop ends

// SPAWNED THREAD                    MAIN THREAD
//  │                                 │
//  │ tx.send("hi")                   │
//  ├────────────→ CHANNEL ──────────→│
//  │                                 │ prints "hi"
//  │                                 │
//  │ tx.send("from")                 │
//  ├────────────→ CHANNEL ──────────→│
//  │                                 │ prints "from"
//  │                                 │
//  │ ...                             │
//  │                                 │
//  │ tx dropped                      │
//  └────────────── CHANNEL CLOSED ──→│
//                                    │
//                                   STOP

// use std::sync::Mutex;

// fn main() {
//     let m = Mutex::new(5);

//     {
//         let mut num = m.lock().unwrap(); // blocks until lock is free
//         *num = 6;
//     } // lock released automatically here

//     println!("m = {:?}", m);
// }

// use std::sync::{Arc, Mutex};
// use std::thread;

// fn main(){
//     let counter = Arc::new(Mutex::new(0));//The 0 is protected by the Mutex.
//     // puts that Mutex inside an Arc:
//     //    Arc
//     //     ↓
//     //   Mutex
//     //     ↓
//     //     0
//     // Why do we need both?
//     // Because we have 10 threads that all need to access the same counter.
//     //          counter
//     //             ↓
//     //        Arc<Mutex<0>>
//     //       /      |      \
//     //      /       |       \
//     // Thread 1  Thread 2  Thread 3 ...
//     // Arc solves:
//     // How can multiple threads own/access the same data?
//     // Mutex solves:
//     // How can we make sure they don't modify it simultaneously?

//     let mut handles = vec![];
//     //It will store the JoinHandle of every spawned thread
//     //     handles = [
//     //     handle1,
//     //     handle2,
//     //     handle3,
//     //     ...
//     //     handle10
//     // ]

//     for _ in 0..10{
//         let counter = Arc::clone(&counter);
//         //We're not cloning the counter itself.
//         //We're cloning the Arc pointer/reference count.
//         // After
//         //                counter  ──┐
//         //                           ↓
//         //                          Arc ──→ Mutex ──→ 0
//         //                           ↑
//         //                counter  ──┘
//         // Original
//         // counter
//         //   ↓
//         //  Arc
//         //   ↓
//         // Mutex
//         //   ↓
//         //   0
//         let handle = thread::spawn(move||{
//             let mut num = counter.lock().unwrap(); // 2. wait and lock
//             //   Thread 1
//             //     ↓
//             //    lock()
//             //     ↓
//             //  🔒 Mutex
//             //     ↓
//             //     0
//             // num points to the 0
//             *num += 1; // change 0 → 1
//         });//lock released when num goes out of scope here

//                 //       lock()
//                 //         ↓
//                 //     MutexGuard
//                 //         ↓
//                 //     modify data
//                 //         ↓
//                 // num goes out of scope
//                 //         ↓
//                 //  🔓 Mutex unlocked

//         handles.push(handle);

//     }
//     for handle in handles{
//         handle.join().unwrap();
//         //Wait until this thread finishes
//     //     Main thread
//             //  |
//             //  | join thread 1
//             //  ↓
//             // WAIT
//             //  |
//             //  | thread 1 finishes
//             //  ↓
//             //  |
//             //  | join thread 2
//             //  ↓
//             // WAIT
//             //  |
//             //  ...

//     }

//     println!("Result: {}", *counter.lock().unwrap());

// }

// async fn get_data() -> String {
//     // some async operation
//     "Data".to_string()
// }

// async fn main_task() {
//     let data = get_data().await;
//     println!("{data}");
// }

// async fn hello() {
//     println!("Hello from async function!");
// }

// fn main() {
//     println!("Program started");

//     let future = hello().await;

//     println!("Future created");

//     // We haven't run the future yet.
// }

// async fn hello() {
//     println!("Hello from async function!");
// }

// fn main() {
//     println!("Program started");

//     trpl::block_on(async {
//         hello().await;
//     });

//     println!("Future created");
// }

// main()
//   ↓
// block_on()
//   ↓
// async block
//   ↓
// hello()
//   ↓
// Future
//   ↓
// .await
//   ↓
// Future completes
//   ↓
// "Hello from async function!"

// you need an async runtime as tokio as your main function cannot be async
// #[tokio::main]
// async fn main() {
//     let num = get_number().await;
//     let result: u8 = smol::block_on(future:num);
//     println!("{:?}", num);
// }

// async fn get_number() -> u8 {
//     return 8;
// }

//program 1
// async fn get_number() -> u8 {
//     return 8;
// }

// fn main() {
//     let future = get_number();
//     let result = trpl::block_on(future);
//     println!("{}", result);
// }

//program 2 sequential await function

// async fn task1() {
//     println!("Task 1");
// }

// async fn task2() {
//     println!("Task 2");
// }

// async fn task3() {
//     println!("Task 3");
// }

// fn main() {
//     trpl::block_on(async {
//         task1().await;
//         task2().await;
//         task3().await;
//     });
// }

//program 3 make the actual function wait

// use std::time::Duration;

// async fn task1() {
//     println!("Task 1 is started ");

//     trpl::sleep(Duration::from_secs(3)).await;
//     println!("Task 1 is ended");
// }

// async fn task2() {
//     println!("Task 2 is started ");
//     trpl::sleep(Duration::from_secs(3)).await;
//     println!("Task 2 is ended");
// }

// async fn task3() {
//     println!("Task 3 is started ");
//     trpl::sleep(Duration::from_secs(3)).await;
//     println!("Task 3 is ended");
// }

// fn main() {
//     trpl::block_on(async {
//         task1().await;
//         task2().await;
//         task3().await;
//     });
// }

use std::time::Duration;

async fn task1() {
    println!("Task 1 is started ");

    trpl::sleep(Duration::from_secs(3)).await;
    println!("Task 1 is ended");
}

async fn task2() {
    println!("Task 2 is started ");
    trpl::sleep(Duration::from_secs(1)).await;
    println!("Task 2 is ended");
}

fn main() {
    trpl::run(async {
        let future1 = task1();
        let future2 = task2();

        trpl::select(future1, future2).await;
    });
}
