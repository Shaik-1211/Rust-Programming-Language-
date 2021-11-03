// A Program to print about name,favourite fruit,price of fruit using derive functionality.
#[derive(Debug,Clone,Copy)]
enum Fruits{
    apple,
    guava,
    orange,
    banana,
}
#[derive(Debug,Clone,Copy)]
enum Names{
    Jack,
    Tarun,
    Nawaz,
    Nancy
}
#[derive(Debug,Clone,Copy)]
struct List{
    name : Names, 
    item : Fruits,
    price : i32,
}

fn display(data : List)
{
    println!("The favourite fruit of {:?} is {:?}",data.name,data.item);
    println!("The price of favourite fruit of {:?} is {:?} Rupees",data.name,data.price);
}
fn main()
{
    let person1 = List
    {   name : Names :: Jack,
        item : Fruits :: apple,
        price : 50,
    };
    let person2 = List
    {   name : Names :: Tarun,
        item : Fruits :: orange,
        price : 65,
    };
    let person3 = List
    {   name : Names :: Nancy,
        item : Fruits :: guava,
        price : 35,
    };
    
  /*  println!("The favourite fruit of person1 is {:?}",person1.item);//Item is printed by derive(Debug).
    println!("The price of favourite fruit of person1 is {:?}",person1.price);*/

    //we can print the above info by using a function not passing ownership to it
    //by using clone ,copy
    display(person1);
    display(person2);
    display(person3);

}