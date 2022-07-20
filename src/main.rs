use std::io;
use std::process;
fn main() {
    loop {
        
        let mut first = String::new();
        let mut second = String::new();
        println!("Please enter a first name");
        io::stdin().read_line(&mut first).unwrap();
        let  a:u32 ;
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
        io::stdin().read_line(&mut second).unwrap();
        let  b:u32 ;
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
    // println!("Hello, world!");
}

fn sum(a:u32,b:u32) -> u32 {
    a+b
}
