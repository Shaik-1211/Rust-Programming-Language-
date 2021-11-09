// A Program to handle errors while opening and creating a file.
use std::fs::File;
use std::io::ErrorKind;

fn main()
{
    let f =File::open("hello.txt");//Attempts to open a file hello.txt
    //As file is a result type of openning a file we have to handle ok and error
    let f = match f 
    {
        Ok(file)=>file,//If file exists it will be opened
        Err(error)=>match error.kind()//we match the error kind and create a file if it does not exists
        {
        ErrorKind::NotFound=>match File::create("hello.txt"){//Creating a file
            Ok(fc)=>fc,
            Err(e)=>panic!("Problem creating file : {:?}",e),//Handling error while creating a file
        },
        // If error kind is other than not found we will handle the error as follows
        other_error=>{
        panic!("Problem opening the file : {:?}",other_error)}
        }
    };

    //We can also perform the above function as follows
    let f = File::open("hello.txt").unwrap_or_else(|error|{
       if error.kind() == ErrorKind::NotFound{
        File::create("hello.txt").unwrap_or_else(|error|{
            panic!("Problem creating a file{:?}",error);
        })
    }
        else
        {
            panic!("Problem opening the file");
        }
        
    });
}
    


