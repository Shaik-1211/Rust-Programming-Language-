//Given a item name,create and print out a item struct if the item exists using Map Combinators.
#[derive(Debug)]
struct Furniture
{
    item_name : String,
    price : i32,
}

fn finding(name : &str)->Option<i32>
{
    let var = name.to_lowercase();
    match var.as_str()
    {
        "chair"=>Some(500),
        "table"=>Some(1000),
        "bed"=>Some(5000),
        _=>None,
    }
}

fn main()
{
    let item = "chair";
    let item_match = finding(item)
                        .map(|price|    { 
                            Furniture{
                                     price,
                                     item_name : item.to_owned(),                                                                      
                                     }
                                        }
                                    );
   
    match item_match
    {
        Some(data)=>println!("{:?}",data),
        None =>println!("Item not found"),
    }
}                                    