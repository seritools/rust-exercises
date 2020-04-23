// Get this generic function to work, with all builtin primitive number types.
// You can use https://docs.rs/num-traits to make your life easier :)

// original question by abledbody#0001 on discord

fn modulo<T: std::ops::Add + std::ops::Rem + std::cmp::PartialOrd>(a: T, b: T) -> T {
    // We're going to exploit the property that, where a is negative, a % b == a mod b - b
    let rem_error = if a < 0 { b } else { 0 };
    a % b + rem_error
}

fn main() {
    println!("{}", modulo(1u8, 22u8));
    println!("{}", modulo(1i8, 22i8));
    println!("{}", modulo(24i64, 22i64));
}
