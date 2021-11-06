// A program that uses closures to perform addition
fn add(a:i32,b:i32) -> i32
{
    a+b
}
fn main()
{
    let sum = add(21,53);
    println!("Sum is {:?}",sum);
    // we can use closures to perform above function as follows
    let total = |a,b| a+b;
    print!("Sum is {}",total(12,37));

}