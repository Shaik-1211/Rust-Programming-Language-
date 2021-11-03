// A Program to match variables of enum and fields of structs.
enum Planets{
    mercury(i32),
    venus(i32),
    earth(i32),
    mars(i32),
    jupiter(i32),
}

struct Toys{
    toy_name : String,
    price : i32,
    total_items : i32,
}

fn main()
{
    let a = 5;
    match a {
        5 => println!("The value entered is Five"),
        other => println!("The number is {:?}",other),
    }
    
    //Matching a variable of enum.
    let object = Planets::mercury(1);
    match object
    {
        Planets :: mercury(1) => println!("It's Mercury at position 1"),
        Planets :: mercury(other) => println!("No planet matched "),
        other =>(),
    }
    let object = Planets::mars(4);
    match object
    {
        Planets :: mars(4) => println!("It's Mars at position 4"),
        Planets :: mars(other) => println!("No planet matched "),
        other =>(),
    }

    //Matching a field of struct.
    let selected_toy = Toys
    {
        toy_name : "Lion".to_owned(),
        price : 50,
        total_items : 2, 
    };
    match selected_toy {
        Toys{toy_name : Lion ,price,total_items}=>
         println!("The Toy Lion's price is {:?} ,total_items taken is {:?}",price,total_items),
         Toys{toy_name,..}=> println!("{:?}",toy_name),
    }
}