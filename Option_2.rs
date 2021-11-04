// Print out the details of a student's locker assignment using Option.

struct Details{
    
    name : Option<String>,
    locker_assigned : Option<String>,
    locker_number : Option<i32>
}
fn main()
{
    let person1 = Details
    {
        
        name : Some("Mary".to_owned()),
        locker_assigned : Some("Yes".to_owned()),
        locker_number : Some(1),
    };
    let person2 = Details
    {
        
        name : Some("Abraham".to_owned()),
        locker_assigned : Some("No".to_owned()),
        locker_number : None,
    };

    
    match person1.name
    {  
        Some(name)=>println!("Student name is \t\t\t{:?}",name),
        None => println!("No locker assigned"),
    }
    match person1.locker_assigned
    {  
        Some(locker_assigned)=>println!("Is locker assigned to the student ? \t{:?}",locker_assigned),
        None => println!("No locker assigned"),
    }
    match person1.locker_number
    {  
        Some(locker_number)=>println!("Locker number of the student is \t {:?}",locker_number),
        None => println!("No locker assigned"),
    }
    /*************/
    //For second person 
    match person2.name
    {  
        Some(name)=>println!("Student name is \t\t\t{:?}",name),
        None => println!("No locker assigned"),
    }
    match person2.locker_assigned
    {  
        Some(locker_assigned)=>println!("Is locker assigned to the student ? \t{:?}",locker_assigned),
        None => println!("No locker assigned"),
    }
    match person2.locker_number
    {  
        Some(locker_number)=>println!("Locker number of the student is \t {:?}",locker_number),
        None => println!("No locker assigned"),
    }
}