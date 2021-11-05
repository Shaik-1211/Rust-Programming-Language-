// Print the name and number of items in stock for a furnuture store using HashMap.
// If the number of items is 0,print "Out of stock" instead of 0.
// Print total number of items in stock. 

use std::collections::HashMap;
fn main()
{
    let mut furniture = HashMap::new();
    furniture.insert("Chairs",5);
    furniture.insert("Bed",3);
    furniture.insert("Table",2);
    furniture.insert("Couch",0);

    let mut total_stock=0;
    for (item,qty) in furniture.iter()
    {
        total_stock=total_stock+qty ;
        let x =if qty == &0
        {
            "Out of stock".to_owned()
        }
        else
        {
            format!("{:?}",qty)
        };
        println!("Item name : {:?}\nNumber of items in stock : {:?}",item,x);
    }
    println!("Total number of items in stock : {:?}",total_stock)
}