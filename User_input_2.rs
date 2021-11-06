// A program to verify user input against pre-defined keywords.
// Display the action choosen by the user if input is in pre-defined keywords.
// Display an appropriate error message if input is not from pre-defined keywords.
use std::io;
enum Powerstate
{
    Off,
    Shutdown,
    Sleep,
    Restart,
}

fn new_powerstate(input : &str)->Option<Powerstate>
{
    let input : String = input.trim().to_lowercase();

    match input.as_str()
    {
        "off"=>Some(Powerstate::Off),
        "shutdown"=>Some(Powerstate::Shutdown),
        "sleep"=>Some(Powerstate::Sleep),
        "restart"=>Some(Powerstate::Restart),
        _=>None
    }
}
fn print_powerstate(input : Powerstate)
{
    use Powerstate::*;
    match input
    {   Off=>println!("Turning off..."),
        Shutdown=>println!("Shutting down..."),
        Sleep=>println!("Sleeping..."),
        Restart=>println!("Restarting..."),
    }
}

 fn main()
 {
     let mut data =String::new();
     println!("Enter the action to be performed from the following: ");
     println!("  *Off  *Shutdown  *Sleep  *Restart  ");
     let status = io::stdin().read_line(&mut data);
    
     if status.is_ok()
     {
         match new_powerstate(&data)
         {  Some(var)=>print_powerstate(var),
            None=>println!("invalid powerstate"),
         }
     }
     else
     {
         println!("invalid input entered");
     }
 }

