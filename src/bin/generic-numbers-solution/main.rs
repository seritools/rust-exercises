fn modulo<T: Copy + num_traits::Num + std::cmp::PartialOrd>(a: T, b: T) -> T {
    // We're going to exploit the property that, where a is negative, a % b == a mod b - b
    let rem_error = if a < T::zero() { b } else { T::zero() };
    a % b + rem_error
}

fn main() {
    println!("{}", modulo(1u8, 22u8));
    println!("{}", modulo(1i8, 22i8));
    println!("{}", modulo(24i64, 22i64));
}
