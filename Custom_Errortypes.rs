//A program that displays the Custom Error types occured while 
//running the functions pick_menu,divide,run.
use thiserror::Error;

#[derive(Debug,Error)]
enum ProgramError
{
    #[error("Menu Error")]
    Menu(#[from]MenuError),

    #[error("Math Error")]
    Math(#[from]MathError),
}

#[derive(Debug,Error)]
enum MenuError
{
    #[error("Not Found")]
    NotFound,
}

#[derive(Debug,Error)]
enum MathError
{
    #[error("divide by zero error")]
    DividebyZero,
}

fn pick_menu(choice:&str)->Result<i32,MenuError>
{
    match choice
    {
        "1"=>Ok(1),
        "2"=>Ok(2),
        "3"=>Ok(3),
        _=>Err(MenuError::NotFound),
    }
}

fn divide(a:i32,b:i32)->Result<i32,MathError>
{
    if b!=0
    {Ok(a/b)}
    else
    {Err(MathError::DividebyZero)}
}

fn run(step:i32)->Result<(),ProgramError>
{
    if step==1
    {
        pick_menu("4")?;//As pick choice doesn't have '4' pick choice will return menu error
    }
    else if step==2
    {
        divide(1,0)?;//As 'b' is zero divide will return a Math error
    }
    Ok(())
}
fn main()
{
    println!("{:?}",run(1));
    println!("{:?}",run(2));

}