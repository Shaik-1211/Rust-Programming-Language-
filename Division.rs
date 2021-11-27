//Problem:Perform a division only when the divisor given is a NonZero number 
//        using struct and methods implemented on the struct.
struct NeverZero(i32);

impl NeverZero
{
    fn new(i:i32)->Result<Self,String>
    {
        if i == 0
        {Err("Cannot be zero".to_string())}
        else
        {Ok(Self(i))}
    }
}
fn divide(a:i32,b:NeverZero)->i32
{   let b = b.0;
    a/b
}
fn main()
{
    match NeverZero::new(2)
    {
        Ok(x)=>println!("{:?}",divide(10,x)),
        Err(e)=>println!("{:?}",e),
    }
}