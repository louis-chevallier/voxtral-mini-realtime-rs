
use crate::macros::mac;


fn main() {
    // Statements here are executed when the compiled binary is called.

    // Print text to the console.
    let i = 123;
    let abc = "abc";
    EKO_X!();
    EKO_X!(i);
    EKO_X!(abc);
    println!("Hello World!");
}
