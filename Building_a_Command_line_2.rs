//(This file is associated with Building_a_Command_line_1.)
use std::error::Error;
use std::fs;
use std::env;

//This function will check the condition and print the contents matched with the query provided. 
pub fn run(config:Config)->Result<(),Box<(dyn Error)>>
{
    let contents=fs::read_to_string(config.filename)?;//Read the entire contents of the file into a string.
    let results=if config.case_sensitive{
        search(&config.query,&contents)
    }else{
        search_case_insensitive(&config.query,&contents)
    };
    for line in results
    {println!("{}",line);}
    Ok(())
}


pub struct Config{
    pub query:String,
    pub filename:String,
    pub case_sensitive:bool,
}
impl Config
{
    pub fn new(args : &[String])->Result<Config,&str>
    {   if args.len()<3
        {return Err("not enough arguments")}
        let query=args[1].clone();
        let filename=args[2].clone();
        let case_sensitive=env::var("CASE_INSENSITIVE").is_err();
        
        Ok(Config{query,filename,case_sensitive})
    }
}

//Searches for the contents that match the query provided.
pub fn search<'a>(query:&str,contents:&'a str)->Vec<&'a str>
{
   let mut results=Vec::new();

   for line in contents.lines()
   {
       if line.contains(query)
       {results.push(line);}
   }
   results
}

//If the CASE_INSENSITIVE  in run returns true then this function will get executed using the query and contents provided.
fn search_case_insensitive<'a>(query:&str,contents:&'a str)->Vec<&'a str>
{   let query=query.to_lowercase();
    let mut results=Vec::new();
    for line in contents.lines()
    {
        if line.to_lowercase().contains(&query)
        {results.push(line);}
    }
    results
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn case_sensitive()
    {
        let query="safe";
        let contents="\
Rust:
safe,fast,productive.
Pick three.";

        assert_eq!(vec!["safe,fast,productive."],search(query,contents));
    }

    #[test]
    fn case_insensitive(){
        let query="rUsT";
        let contents="\
Rust:
safe,fast,productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:","Trust me."],search_case_insensitive(query,contents));
    }
}