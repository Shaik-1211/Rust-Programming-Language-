// A program to test a function.
fn capitalise(input : &str)->String
{
    input.to_uppercase()
}

fn main(){}
#[cfg(test)]
mod testing
{
    use crate:: *;//Uses the function out of this testing module's scope.
    #[test]
    fn checks(){
        let result = capitalise("environment");
        let expected  = String::from("ENVIRONMENT");
        assert_eq!(result,expected,"All the letters should be uppercased");
    }
}
