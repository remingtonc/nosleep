#[macro_use] extern crate log;
use ::nosleep;

fn main() {
    println!("Preventing sleep!");
    nosleep::prevent_sleep();
}