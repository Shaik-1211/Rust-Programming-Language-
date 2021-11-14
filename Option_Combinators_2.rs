#[derive(Debug,Eq,PartialEq)]
enum Access{
    Admin,
    User,
    Guest,
}

fn may_be_access(name :&str)->Option<Access>
{
    match name{
        "admin"=>Some(Access::Admin),
        "gary"=>Some(Access::User),
        _ =>None,
    }
}

fn root()->Option<Access>
{
    Some(Access::Admin)
}

fn part_1()->bool
{
    //We are checking whether or not this particular user
    //has an access level.The "admin" user does have an access level.
    may_be_access("admin").is_some()
}

fn part_2()->Option<Access>
{
    //"Root" is equivalent to Access::Admin, but it is not listed in the 
    //may_be_access function.So,root is added to the list.
    may_be_access("root").or_else(|| root())
}

fn part_3()->Access
{
    //"Alice"is not a listed user ,so she will be a guest.
    may_be_access("Alice").unwrap_or_else(|| Access::Guest)
}

fn main(){}

#[cfg(test)]
mod test
{
    use crate::*;
    #[test]
    fn check_part_1()
    {
        assert_eq!(part_1(),true,"result should be true");
    }

    #[test]
    fn check_part_2()
    {
        assert_eq!(part_2(),Some(Access::Admin),"Root users have admin access");
    }

    #[test]
    fn check_part_3()
    {
        assert_eq!(part_3(),Access::Guest,"Alice ia a guest");
    }
}
