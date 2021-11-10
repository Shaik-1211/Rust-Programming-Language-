// A program to mixup two different datatypes using Generics.
struct Point <T,U>
{
    x : T,
    y : U,
}
impl<T,U> Point<T,U>
{
    fn mixup<V,W>(self,other : Point<V,W>)->Point<T,V>
    // Considering the upcoming generics of p3 as V and W and returning the p3 with respective parameters
    { 
        Point
        {
          x : self.x,//Datatype of x is 'T'
          y : other.x,//Datatype of y is 'V'
        }
    }
}
fn main()
{
    let p1 = Point{x : 23 , y : 43};
    let p2 = Point{x : "Hi" , y : 'a'};  
    let p3 = p1.mixup(p2);
    println!("p3.x = {:?} and p3.y = {:?}",p3.x,p3.y);
}