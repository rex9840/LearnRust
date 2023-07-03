#![allow(dead_code)]

/* 

A static lifetime is a memory resource that lasts indefinitely to the end of a program
that by the definition some static lifetime resources can be created at runtime.

 */

// Modifying static variables is inherently dangerous because they are globally accessible to be read from by anyone introducing the possibility of a data race 

//static is used for declaring global variables


static PI:f32 = 3.1415;

fn main()
{ 
//static variable can also be scoped 

static mut SECRET : &'static str ="swordfish";


//string literals has static life time 

let message : &'static str = "Hello World"; 


println!("{}",message); 
println!("{}",PI); 




unsafe { 

    println!("{}",SECRET);
}

}