// A program to add two numbers taken by the user as an input.
use std::io;

fn main()
{   println!("enter a ");
    let mut x = String::new();
    io::stdin()
        .read_line(&mut x)
        .expect("failed to read input");
    
    println!("enter b");
    let mut y = String::new();
    io::stdin()
        .read_line(&mut y)
        .expect("failed to read input");

    let a: i32 = x.trim().parse().expect("invalid input");
    let b: i32 = y.trim().parse().expect("invalid input");

    println!("sum = {} ",a+b);
    }
    