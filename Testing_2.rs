// A program to test the result of the function clamp,function divide,function connect.
//Ensures that value of 'n' is between lower and upper. 
fn clamp(n:i32,lower:i32,upper:i32)->i32
{
    if n<lower
    {lower}
    else if n>upper
    {upper}
    else 
    {n}
}

//Divide 'a' by 'b'
fn divide(a:i32,b:i32)->Option<i32>
{   if b==0
    { return None}
    else
    {Some(a/b)}
    
}

//Joins two string slice and returns a string
fn connect(first : &str, second : &str)->String
{
    format!("{} {}",first,second)
}

#[cfg(test)]
mod test
{
    use crate :: *;//This will provide access to the functions out of test module
    #[test]
    fn check_clamp()
    {
        let result = clamp(3,4,5);//Result obtained by the clamp function
        let expected = 4;//Result expected by the clamp function
        assert_eq!(result,expected,"should be 4");
    }
    #[test]
    fn check_divide()
    {
        let result = divide(15,3);//Result obtained by the divide function 
        let expected = Some(5);//Result expected by the divide function 
        assert_eq!(result,expected,"should be 5");
    }
    #[test]
    fn check_connect()
    {
        let result = connect("wonderful","world of wisdom");//Result obtained by the connect function 
        let expected =String::from( "wonderful world of wisdom");//Result expected by the connect function
        assert_eq!(result,expected,"check the input");
    }
}
