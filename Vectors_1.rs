//A program to print test scores.
struct Test{
    score :i32
}
fn main()
{
   /* //This method of creating a vector uses a struct .
   let my_marks = vec!{
        Test{score : 99},
        Test{score : 65},
        Test{score : 36},
        Test{score : 55},
        Test{score : 87},
    };
    for num in my_marks{
        println!("score : {:?} ",num.score);
    }*/
/***********************************************/
    // we can also create a vector as follows
    let v = vec![99,65,36,55,87];
    for i in &v
    {
        println!("{:?}",i);
    }
    // To access an element from a vector
    // Method : 1
    let third :&i32 =  &v[2];
    println!("The element at index 3 of vector my_marks is {:?}",third);
    //Method :2
    match v.get(2)
    {
        Some(third) => println!("The element at index three in vector my_marks is {:?}",third),
        None => println!("There is no element at given index in vector my_marks"),
    }
}