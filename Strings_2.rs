// A program to print the name,age,favourite colour of people of age 10 and under 10.
struct Information{
    name : String,
    age : i32,
    colour : String,
}
fn print_it(data :&str,data1 : &str)
{
    println!("Name : {:?}",data);
    println!("Favourite colour is {:?}",data1);

}

fn main()
{
    let info = vec!
    {
        Information
        {
            name : String::from("Jack"),
            colour : String::from("Red"),
            age : 7,
        },
        Information
        {
            name : String::from("Jill"),
            colour : String::from("Black"),
            age : 10,
        },
        Information
        {
            name : String::from("Rosy"),
            colour : String::from("Yellow"),
            age : 6,
        },
        Information
        {
            name : String::from("Emanuel"),
            colour : String::from("Violet"),
            age : 17,
        },
    };
    for person in info
    {   if person.age <=10
        {print_it(&person.name,&person.colour);
        println!("The age of {:?} is {:?}",person.name,person.age);}
        else 
        {
            println!("The age of the {:?} is above {:?}",person.name,person.age);
        }
    }
}