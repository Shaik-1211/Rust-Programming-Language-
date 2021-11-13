fn clamp(n:i32,lower:i32,upper:i32)->i32
{
    if n<lower
    {lower}
    else if n>upper
    {upper}
    else 
    {n}
}

fn divide(a:i32,b:i32)->Option<i32>
{   if b==0
    { return None}
    else
    {Some(a/b)}
    
}

fn connect(first : &str, second : &str)->String
{
    format!("{} {}",first,second)
}

#[cfg(test)]
mod test
{
    use crate :: *;
    #[test]
    fn check_clamp()
    {
        let result = clamp(3,4,5);
        let expected = 4;
        assert_eq!(result,expected,"should be 4");
    }
    #[test]
    fn check_divide()
    {
        let result = divide(15,3);
        let expected = Some(5);
        assert_eq!(result,expected,"should be 5");
    }
    #[test]
    fn check_connect()
    {
        let result = connect("wonderful","world of wisdom");
        let expected =String::from( "wonderful world of wisdom");
        assert_eq!(result,expected,"check the input");
    }
}