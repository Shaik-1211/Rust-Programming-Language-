// A program to print input entered by user.
use std::io;

fn get_input()->io::Result<String>
{
    let mut data = String::new();
    io::stdin().read_line(&mut data)?;
    Ok(data.trim().to_owned())
}

fn main()
{
    let mut input = vec![];
    let mut count = 0;

    while count<3
    {
        match get_input()
        {
            Ok(words)=>{input.push(words);
                        count += 1;       }

            Err(e)=>println!("Error : {:?}",e),
        }
    }
    for exp in input
    {
        println!("Original : {:?}\tCapitalized : {:?}",exp,exp.to_uppercase());
    }
}