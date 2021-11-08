use std::io;
mod Matching_Words
{   pub enum colours
    {
        red,
        yellow,
        blue,
    }
    //Accept the colour entered by the user and matches it and returns a string associated with it. 
    pub fn procedure(input : &str)->Option<colours>
    {
        let input : String = input.trim().to_lowercase();
        match input.as_str()
        {
            "red"=>Some(colours::red),
            "yellow"=>Some(colours::yellow),
            "blue"=>Some(colours::blue),
            _ =>None,
        }
    }
    
    
}
//This is a function defined out of module Matching_Words and used in main function 
use Matching_Words::colours;
pub fn printword(input :colours)
{   use colours::*;
    match input
    {
        red=>println!("It's Red Rose"),
        yellow=>println!("It's Yellow Yacht"),
        blue=>println!("It's Blue Bracelet"),
    }

}

  fn main(){  
        let mut selected_colour = String::new();
        println!("Select a colour from the following ");
        println!("Red  Yellow  Blue ");
        println!("enter the colour : ");
         let a =io::stdin().read_line(&mut selected_colour);//stores the input in variable 'a' 
         use self::Matching_Words::procedure;
         
        if a.is_ok()     
        {  match  procedure(&selected_colour)
         {
             Some(var)=>printword(var),
             None=>println!("Not of the above colours"),
         }
        }
        else
        {
            println!("invalid input entered");
        }
    
  }