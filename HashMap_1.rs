// A program to print keys, values of a HashMap.
use std::collections::HashMap;
#[derive(Debug)]
struct Toys
{
    toy :String,
}
fn main()
{
    let mut things = HashMap::new();
    things.insert(1,Toys{ toy :"Ball".to_owned()});
    things.insert(2,Toys{ toy :"Bat".to_owned()});
    things.insert(3,Toys{ toy :"Doll".to_owned()});

    println!("The keys of the hashmap are :");
    for num in things.keys()
    {
        print!("{:?}  ",num);
    }
    println!("\nThe values of the hashmap are :");
    for num in things.values()
    {
        println!("{:?}",num);
    }
    println!("The hashmap is :");
    for num in things.iter()
    {
        println!("{:?}",num);
    }
}