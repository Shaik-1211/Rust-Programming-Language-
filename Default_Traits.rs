struct Box
{
    length:f64,
    width:f64,
    height:f64,
}
impl Box
{
    fn new(l:f64,w:f64,h:f64)->Self
    {
        Self
        {
            length:l,
            width:w,
            height:h,
        }
    }
}
impl Default for Box//These dimensions will get implemented by default when no dimension values are provided
{
    fn default()->Self
    {
        Self
        {
            length:6.0,
            width:4.0,
            height:4.0,
        }
    }
}

fn main()
{
    let b = Box::default();
    println!("length {:?},height :{:?} ,width : {:?}",b.length,b.height,b.width);
}