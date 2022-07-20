use std::io;
use std::process;

fn main() {
    loop {
        
       
        println!("Please enter a first name");
        let a = read_input_user();
        println!("Please enter a second name");
        let b =  read_input_user();
        let result = sum(a, b);
        println!("{} + {} = {}",a,b,result);
    }
    // println!("Hello, world!");
}

fn sum(a:u32,b:u32) -> u32 {
    a+b
}
fn read_input_user() -> u32 {
    let mut input=String::new();
    io::stdin().read_line(&mut input).unwrap();
        let  digit:u32 ;
        match input.trim().parse() {
            Ok(val) => {
                digit = val;
            },
            Err(_err) => {
                println!("this is not valid type number");
                process::exit(1);
            }
        }
        digit
}
