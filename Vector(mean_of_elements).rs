//Calculate the mean of the elements of a vector.
fn main()
{
    let a=vec![11,24,11,36,45,52];
    let mean;
    let size=a.len();
    println!("{}",size);
    let mut total=0;
    for i in &a
    {
        total=total+i;
    }
    println!("{}",total);
    mean=total/size;
    println!("mean is : {}",mean);
}