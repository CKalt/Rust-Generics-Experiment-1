use std::ops::Add;

fn add<T: Add<T>>(a: T, b: T) -> T::Output {
    a + b
}

fn main() {
    println!("i32 result = {}", add(10i32,5i32));
    println!("f32 result = {}", add(10.3939f32,5.39393f32));
    println!("f64 result = {}", add(10.3923832839f64,5.39392238233f64));
}
