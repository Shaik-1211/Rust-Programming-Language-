//Problem: Triple the value of each item in the given vector.
//         Filter the data to only include values >10.
//         Print the elements in the modified vector.

fn main()
{
    let data = vec![1,2,3,4,5];

    //Triple the value of each item in the given vector
    let data2:Vec<_>= data
                        .iter()
                        .map(|num| num*3)
                        .collect();

    println!("The elements of the vector are");
    for var in &data2
    {
        print!("{:?} ",var);
    }
    println!("");

    // Filtering the items greater than 10 
    let data3:Vec<_>=data2
                        .iter()
                        .filter(|num| num>&&10)
                        .collect();
    println!("The elements of the vector greater than 10 are");
    for var in data3
    {
        print!("{:?} ",var);
    }
}