


use voxtral_mini_realtime::EKO_X;


fn main() {
    // Statements here are executed when the compiled binary is called.

    // Print text to the console.
    let i = 123;
    let j = 456;
    let abc = "abc";
    EKO_X!();
    EKO_X!(i);
    EKO_X!("xyz");
    EKO_X!([i, j]);
    EKO_X!(abc);
    EKO_X!(abc, i);
    EKO_X!(abc, i, "toto", j);
    EKO_X!("Hello World!");
}
