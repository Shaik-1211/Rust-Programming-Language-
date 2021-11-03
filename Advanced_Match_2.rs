// Print out a list of tickets and their information for an event.
// Tickets can be backstage,VIP,standard.
// Backstage and VIP tickets include ticket holder's name.
enum Tickets{
    Backstage(i32,String),
    VIP(i32,String),
    Standard(i32),
}



fn main()
{
    let tickets =vec!
    [
        Tickets :: Backstage(800,"Alice".to_owned()),
        Tickets:: Standard(500),
        Tickets:: VIP(1000,"Bob".to_owned()),
    ];   

    for someval in tickets
    {
        match someval
        {
            Tickets :: Backstage(price,holder)=>{
            println!("Ticket type is Backstage ,Holder's name :{:?} , price : {:?}",holder,price)}
            Tickets :: VIP(price,holder)=>
            {println!("Ticket type is VIP ,Holder's name :{:?} , price : {:?}",holder,price)}
            Tickets :: Standard(price)=>
            println!("Ticket type is Standard , price : {:?}",price),
        }
    }
        

}