// This program will display the current date and time .
use chrono::prelude::*;//This is a chrono crate(An External Crate)

fn main()
{
    let local :DateTime<Local> = Local::now();
    println!("{}",local.format("%Y-%m-%d %H:%M:%S").to_string());
}