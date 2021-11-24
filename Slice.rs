//Some Methods implemented on Slices
fn main()
{
    //Slicing a vector
    let a=vec![1,2,3,4,5];
    let int_slice=&a[..];
    for i in &int_slice[..]
    {
        println!("Element is {}",i);//prints all the elements in a
    }
    let str_slice:&[&str]=&["one","two","three"];
    for i in &str_slice[..]
    {
        println!("The subslice is {}",i);//prints all the elements in str_slice
    }

    //Mutable slices
    let mut x=[99,98,78];
    let x = &mut x[..];//Take a full slice of x
    x[1]=56;
    println!("The elelments in x are");
    for i in &mut x[..]
    {
        print!("{} ",i);
    }
    println!("");

    //Checks if the slice is empty if it is not empty prints the elements in the slice
    let a=["Apple","Banana","Cherry"];  
    if a.is_empty()
    {println!("The provided slice is empty");}
    else
    {
        println!("The elements in the provided slice are");
        for i in &a[..]
        {print!("{} ",i);}
    }
    println!("");

    //Returns first element of the slice or None if the slice is empty
    let a:&[i32]=&[9,8,7];
    println!("The first element is {:?}",a.first());

    //Get method 
    let v=[10,11,12,13,14,15];
    println!("The elements in the given range are\n{:?}",v.get(0..3));//prints the elements in the given range
    println!("The element at given index is {:?}",v.get(1));

    //Get Mutable
    let x=&mut [45,65,23];
    println!("The elements in x before appplying get_mut are ");
    for i in &*x
    {
        print!("{:?} ",i);
    }
    println!("");
    if let Some(element)=x.get_mut(2){*element=78;}//changing the element at the given index
    println!("The elements in x after appplying get_mut are ");
    for i in &*x
    {
        print!("{:?} ",i);
    }
    println!("");
}