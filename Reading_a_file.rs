//A Program that prints the contents of the provided file 
use std::env;
use std::fs;

fn main() {
    let args:Vec<String>=env::args().collect();
    println!("{:?}",args);

    let query=&args[1];
    let filename=&args[2];
    println!("Searching for query {:?}",query);
    println!("Searching for file {:?}",filename);

    let contents=fs::read_to_string("poem.txt")//Read the entire contents of the file into a string.
                .expect("Something went wrong in reading the file");
    println!("With text :\n{}",contents);//prints the contents in the file.dbg!
}
