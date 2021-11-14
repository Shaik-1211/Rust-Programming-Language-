// A program that uses some Option Combinators.

fn main()
{
    let a:Option<i32> = Some(6);
    //checks if there is some data in 'a'
    let a_is_some = a.is_some();
    dbg!(a_is_some);

    //checks if there is no data in 'a'
    let a_is_none = a.is_none();
    dbg!(a_is_none);

    //stores the value of a in num variable and multiply it with 2
    let a_mapped = a.map(|num| num*2);
    dbg!(a_mapped);

    //a.filter matches the value of 'a' and if matched then stores the value in a_filtered 
    //else returns None
    let a_filtered = a.filter(|num| num==&5);
    dbg!(a_filtered);

    //a.or_else checks if there is some data in 'a' if it has some data the nothing will gets 
    //executed otherwise if there is no data in 'a' then it will store the provided value in a_or_else.
    let a_or_else = a.or_else(|| Some(9));
    dbg!(a_or_else);
    
    //The unwrap_or_else combinator checks if there is some data in 'a' if it has some data then 
    //the data in 'a' will be stored in a_unwrapped otherwise it will store the provided value 
    //in a_unwrapped
    let a_unwrapped = a.unwrap_or_else(|| 5);
    dbg!(a_unwrapped);
}