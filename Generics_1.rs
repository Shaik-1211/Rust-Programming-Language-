// A program to find the largest number or largest char in the given vector using Generics.
fn get_largest<T : PartialOrd + Copy>(input : Vec<T>)-> T //"T" is a generic type 
{
    let mut largest = input[0] ;
    for data in input
    {
       if  data > largest
        {largest = data;}
    }
    largest
}
fn main()
{
    let char_list = vec!['a' , 'b','s','y','d' ];
    let number_list = vec![12,34,76,85,54,190,3];
    let largest_char= get_largest(char_list);
    // let largest_num = get_largest_num(number_list);//used with function without generics
    let largest_num= get_largest(number_list);
    println!("The largest char in the given vector is {:?}",largest_char);
    println!("The largest number in the given vector is {:?}",largest_num);

}
//Without using generics we can perform the above program using following functions

/*fn get_largest(input : Vec<char>)->char 
{
    let mut largest = input[0] ;
    for data in input
    {
       if  data > largest
        {largest = data;}
    }
    largest
}
fn get_largest_num(input : Vec<i32>)->i32
{
    let mut largest = input[0] ;
    for data in input
    {
       if  data > largest
        {largest = data;}
    }
    largest
}*/