// A Program to print about library members using option.
struct Librarians{
    name : Option<String>,
    age :  Option<i32>,
    is_member : Option<bool>
}
fn main()
{   let person1 = Librarians
    {
        name : Some("Harry".to_owned()),
        age : Some(20),
        is_member : Some(true),
    };
    let person2 = Librarians
    {
        name : Some("Haleema".to_owned()),
        age : Some(18),
        is_member : Some(false),
    };
    println!();
    // For first person
    match person1.name
    {
        Some(name)=>println!("The name of the member is {:?}",name),
        None =>println!("No informaation provided "),
    }
    match person1.age
    {
        Some(age)=>println!("The age of the member is {:?}",age),
        None =>println!("No informaation provided "),
    }
    match person1.is_member
    {
        Some(is_member)=>println!("Library membership : {:?}",is_member),
        None =>println!("No informaation provided "),
    }
    println!();
    //For second person
    match person2.name
    {
        Some(name)=>println!("The name of the member is {:?}",name),
        None =>println!("No informaation provided "),
    }
    match person2.age
    {
        Some(age)=>println!("The age of the member is {:?}",age),
        None =>println!("No informaation provided "),
    }
    match person2.is_member
    {
        Some(is_member)=>println!("Library membership : {:?}",is_member),
        None =>println!("No informaation provided "),
    }
}
    