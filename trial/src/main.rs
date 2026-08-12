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

fn first_word(s: &String) -> usize{
    let bytes = s.as_bytes();
}