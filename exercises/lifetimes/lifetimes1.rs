// lifetimes1.rs
//
// The Rust compiler needs to know how to check whether supplied references are
// valid, so that it can let the programmer know if a reference is at risk of
// going out of scope before it is used. Remember, references are borrows and do
// not own their own data. What if their owner goes out of scope?
//
// Execute `rustlings hint lifetimes1` or use the `hint` watch subcommand for a
// hint.


fn longest<'a, T: std::cmp::PartialOrd + std::fmt::Display>(x: &'a T, y: &'a T) -> &'a T {
    if x > y {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest(&string1, &string2);
    println!("The longest string is '{}'", result);
}
