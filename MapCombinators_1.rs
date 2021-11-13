// Usage of Map Combinators

//This function may either return Some integer or None.
fn may_be_num()->Option<i32>
{
   Some(7)
   
}

//This function may either return Some word or None.
fn may_be_word()->Option<String>
{
   Some("pineapple".to_owned())
}

fn main()
{   
    let data1 = match may_be_num()
    {
        Some(num)=>Some(num+1),//increasing the acquired result by 1
        None=>None,
    };

    match data1
    {
        Some(var)=>println!("The integer after increament is {:?}",var),
        None=>println!("There is no integer"),
    }
    /****The above program can be modified by using mapcombinators as follows****/
    let data1 = may_be_num().map(|num| num+1);
    match data1
    {
        Some(var)=>println!("The value of integer after increament is {:?}",var),
        None=>println!("There is no integer"),
    }

    let word_length = may_be_word()
                        .map(|word| word.len());
                        // .map(|word| word);//This will print the word provided
    match word_length
    {
        Some(var)=>println!("The length of the provided word is  {:?}",var),
        None=>println!("There is no word"),
    }

}
    