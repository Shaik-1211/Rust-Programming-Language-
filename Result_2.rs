// Determine if a customer is able to make a restricted purchase by using Result.
// Restricted purchase require that the age of the customer is atleast 20.

fn try_purchase(input : i32)->Result<(),String>
{
    if input>20
    {
        Ok(())
    }
    else
    {
        Err("Customer must be atleast 20 years old".to_owned())
    }
}
fn main()
{
    let age_of_the_customer = 25;
    let a = try_purchase(age_of_the_customer);
    println!("{:?}",a);
}