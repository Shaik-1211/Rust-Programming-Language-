//(main.rs)A Program that prints the contents of the provided file matched with the query provided in command line.
use std::env;
use std::process;
use minigrep::Config;

pub fn main() {
    let args:Vec<String>=env::args().collect();
    eprintln!("{:?}",args);

    let config=Config::new(&args).unwrap_or_else(|err|
    {println!("Problem parsing arguments : {}",err);
     process::exit(1);});  
     
     
    println!("Searching for query {:?}",config.query);
    println!("Searching for file {:?}",config.filename);


    if let Err(e)=minigrep::run(config){
        eprintln!("Application error : {}",e);
        process::exit(1);
    }
}

