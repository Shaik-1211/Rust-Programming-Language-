// Functions written using Iterators.
fn main()
{
    let numbers1 = vec![1,2,3,4,5];

    //Creating a new vector in which each of the element is the double of elements in numbers1 vector 

    /*let mut numbers2 = vec![];
    for num in numbers1
    {
        numbers2.push(num*2)
     }*/
    
    /***The above program can be modified as follows***/
    let  numbers2 :Vec<_>= numbers1
                                    .iter()
                                    .map(|num| num*2)
                                    .collect();

    println!("Elements in numbers2 vector are ");
    for num in &numbers2
    {
        print!("{:?} ",num);
    }
    println!("");

    //Creating a new vector named "new_numbers" by filtering the numbers2 vector for elements >=6
    let new_numbers :Vec<_>=numbers2
                            .iter()
                            .filter(|var| var>=&&6)
                            .collect();

    println!("Elements in new_numbers vector are ");
    for data in new_numbers//printing the elements in new_numbers vector
    {
        print!("{:?} ",data);
    }
    println!("");
    
    //Finding a number in number2 vector
    let find_number = numbers2
                        .iter()
                        .find(|num| num ==&&4);

    println!("Element found : {:?}",find_number);

    //Counting total number of elements in number2 vector
    let count = numbers2
                    .iter()
                    .count();
    println!("Total number of elements in numbers2 vector are {:?}",count);

    //Finding and printing the last element in numbers1 vector
    let last :Option<&i32>= numbers1
                            .iter()
                            .last();

    println!("The last element in the  vector numbers1 is {:?}",last);

    //Finding and printing the smallest element in numbers2 vector
    let min :Option<&i32> = numbers2
                                    .iter()
                                    .min();
    println!("Smallest element in numbers2 vector is {:?}",min);

    //Finding and printing the largest element in numbers2 vector
    let max :Option<&i32> = numbers2
                                .iter()
                                .max();
    println!("Largest element in numbers2 vector is {:?}",max);

}