fn capitalise(input : &str)->String
{
    input.to_uppercase()
}

fn main(){}
#[cfg(test)]
mod testing
{
    use crate:: *;
    #[test]
    fn checks(){
        let result = capitalise("environment");
        let expected  = String::from("ENVIRONMENT");
        assert_eq!(result,expected,"All the letters should be uppercased");
    }
}