use std::collections::HashMap;
fn main()
{
    let text ="Give a man a fish, you feed him a day.Teach him how to fish , you feed him for a lifetime.";
    let mut map=HashMap::new();
    for word in text.split_whitespace()
    {
        let count=map.entry(word).or_insert(0);
        *count =*count+1;
    }
    println!("{:?} \n",map);//prints the number of times a word repeated in the given text.

    let mut map1=HashMap::new();
    for ch in text.chars()
    {
        let count=map1.entry(ch).or_insert(0);
        *count =*count+1;
    }
    println!("{:?}\t",map1);//prints number of times each character has repeated in the given text.
}