use std::io;
use std::process;
fn main() {
    // println!("Hello, world!");
    let mut first = String::new();
    let mut second = String::new();
    println!("Please enter a first name");
    io::stdin().read_line(&mut first);
    let mut a:u32 = 0;
    match first.trim().parse() {
        Ok(val) => {
            a = val;
        },
        Err(_err) => {
            println!("this is not valid type number");
            process::exit(1);
        }
    }
    println!("Please enter a second name");
    io::stdin().read_line(&mut second);
    let mut b:u32 = 0;
    match second.trim().parse() {
        Ok(val) => {
            b=val;
        },
        Err(_err) => {
            println!("this is not second valid type number");
            process::exit(1);
        }
    }
    let result = sum(a,b);
    println!("{} + {} = {}",a,b,result);
}

fn sum(a:u32,b:u32) -> u32 {
    a+b
}
