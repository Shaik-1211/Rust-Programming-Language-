//A Program to print the color of the shirt,pant,shoes using enum and structs (for each type separately).
#[derive(Debug)]
enum Color
{
    red,
    blue,
    orange,
    yellow,
    black,
    violet,
    browm,
}
#[derive(Debug)]
struct ShoeColor(Color);
impl ShoeColor
{
    fn new(a:Color)->Self
    {Self(a)}
}
#[derive(Debug)]
struct ShirtColor(Color);
impl ShirtColor
{
    fn new(b:Color)->Self
    {Self(b)}
}
#[derive(Debug)]
struct PantColor(Color);
impl PantColor
{
    fn new(c:Color)->Self
    {Self(c)}
}

fn print_shirt_color(shirt:ShirtColor)
{
    println!("Color of the shirt = {:?}",shirt.0);
}

fn print_pant_color(pant:PantColor)
{
    println!("Color of the pant = {:?}",pant.0);
}

fn print_shoe_color(shoe:ShoeColor)
{
    println!("Color of the shoe = {:?}",shoe.0);
}
fn main()
{
    let shirt_color = ShirtColor::new(Color::black);
    let pant_color = PantColor::new(Color::blue);
    let shoe_color = ShoeColor::new(Color::browm);

    print_shirt_color(shirt_color);
    print_pant_color(pant_color);
    print_shoe_color(shoe_color);

}