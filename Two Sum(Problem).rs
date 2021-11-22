// Problem : Given an array of integers num1 and 
//           a integer target ,return indices of 
//           the two numbers such that they add 
//           up to target.
fn main()
{
    let num1=vec![1,3,5,7];
    let mut num2=Vec::new();
    let target=6;
    let size1=num1.len();
    for x in 0..size1
    {
        for y in (x+1)..size1
        {
            if num1[x]+num1[y]==target
            {
                num2.push(x);
                num2.push(y);
                break;
            }
        }
    }

    let size2=num2.iter().count();
    for a in 0..size2
    {
        print!("{} ",num2[a]);
    }
}