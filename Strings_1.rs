//A program to print a receipt of grocery using Vectors and Strings.
struct Grocery{
    Name : String,
    Number_of_items : i32,
}
fn print_item(data : &str)
{
    println!("Name of the item : {:?} ",data);
}
fn main()
{   let receipt = vec!
    {
        Grocery
        {
            Name : "Sugar".to_owned(),
            Number_of_items : 5,
        },
        Grocery
        {
            Name : "Salt packet".to_owned(),
            Number_of_items : 1,
        },
    };
    for item in receipt
    {   print_item(&item.Name);
        println!("Number of items : {:?} ",item.Number_of_items);
    }

}
