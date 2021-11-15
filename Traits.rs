//A program to calculate the perimeter of a square and a triangle using traits.
trait Perimeter
{
    fn calculate_perimeter(&self);
}

struct Square{
    side :i32,
}
 impl Perimeter for Square
 {
     fn calculate_perimeter(&self)
     {
         println!("The perimeter of square with side {:?} is {:?}",self.side,self.side*4);
     }
 }

 struct Triangle
 {
     side1 :i32,
     side2 :i32,
     side3 :i32,

 }
 impl Perimeter for Triangle
 {
     fn calculate_perimeter(&self)
     {
         println!("The perimeter of the triangle is {:?}",self.side1+self.side2+self.side3);
     }
 }

 fn tally(input : impl Perimeter)
 {
     input.calculate_perimeter();
 }

 fn main()
 {
     tally(Square{side :13});
     tally(Triangle{side1:12,side2:14,side3:15});
 }