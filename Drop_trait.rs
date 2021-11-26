//Drop Trait is associated with Smart Pointers in Rust.
struct System_Status{
    data : String,
}
impl Drop for System_Status
{
    fn drop(&mut self){
        println!("The status of the system is {}.",self.data);
    }
}
fn main()
{
    let status1=System_Status{
        data:String::from("Running")
    };
    
    let status2=System_Status{
        data:String::from("Restarting")
    };
    drop(status1);
    println!("The Drop trait has been implemented.");
}