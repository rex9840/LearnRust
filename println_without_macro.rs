use std::io::{self,Write} ;//importing the io module and write trait 
// a trait is a collection of methods that can be shared across multiple types


fn main() -> io::Result<()>{ 
    let stdout = io::stdout(); //get stdout handel 
    let mut handle = stdout.lock(); // lock the handel so other cant access the resources 


    writeln!(handle, "Hello, world!")?; // write to stdout



    print!("hello");
    print!("world");

return Ok(()); 

}

