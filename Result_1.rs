// A Program to print items selected by Teena by using "Result". 
#[derive(Debug)]
enum Available_items
{
    Coffee,
    Tea,
    Milkshake,
    Orange_juice,
}

fn get_choice(input : &str)->Result<Available_items,String>
{
    match input
    {
        "Coffee"=>Ok(Available_items::Coffee),
        "Tea"=>Ok(Available_items::Tea),
        "Milkshake"=>Ok(Available_items::Milkshake),
        "Orange_juice"=>Ok(Available_items::Orange_juice),
        _ =>Err("Not available".to_owned()),
    }
}
fn print_choice(input : &Available_items)
{
    println!("The item selected by Teena is {:?}",input);
}
fn pick_choice(input : &str)->Result<(),String>
{
    let choice :Available_items = get_choice(input)?;
    print_choice(&choice);
    Ok(())
}
fn main()
{
    pick_choice("Coffee");
    pick_choice("Milkshake");

}