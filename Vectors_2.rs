// A program to print 10,20,"Thirty",40 by using vectors and for-in keywords.
// Also print the length of the given vector.
fn main()
{
    let num =vec![10,20,30,40];
    for view in &num{
       match view{
           30 => println!("Thirty"),
           _ => println!("{}",view),
       }
    } 
    println!("The length of vector is {:?}",num.len());  
}
