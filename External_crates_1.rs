//A program using human time crate.
// This program will print the provided time in human readable manner.
use std::time::Duration;
use humantime::format_duration;

fn main() {
   let time1 = Duration::new(9420,0);
   println!("{:?}",format_duration(time1).to_string());
   let time2 = Duration::new(0,32_000_00);
   println!("{:?}",format_duration(time2).to_string());

}
