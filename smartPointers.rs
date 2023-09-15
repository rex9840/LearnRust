//Raw Pointer 
//refreences can be changed into primitive pointers called raw pointers

//A raw pointer in Rust is similar to a pointer in C but with much more compile time restrictions on how it can be stored and moved around 

//smart pointets

use std::ops::{Deref,DerefMut}; // for derefrencing the pointer 


//generic struct
struct Message<T>
{
    content:T
}


//implimentation of dreferencing for the struct
// defing the generic Type T for the Traits 

impl <T> Deref for Message<T>
{
    //In traits, type is used to declare an associated type:
    //An associated type is a placeholder for a type that is part of another type.
    /* example : type Meter= u32; 
                 let m : Meter =3 ; 
                this means we are associating a type Meter with u32
                it is similar to typedef in c 
    */

    type Target = T;
    fn deref(&self) ->&T
    {
        println!("{0:?} was used",std::any::type_name::<T>());
        &self.content
    }
}


fn type_def <T> (_:T,var:&str)
{
    println!("TYPE of {1:?} : {0:?}", std::any::type_name::<T>(),var);
}



fn main()
{ 

//we can use as keyword for defining the types
    let a:f64= (1000 as i32) as f64; 
    let memory_location_hex = &a as *const f64;  // type : raw pointer 
    let memory_location_int = memory_location_hex as usize; 
    //usize is a built-in unsigned integer type that is used to represent the size of memory in bytes on the current platform 
    //i.e. unsigned integer representation 
    let b:i32 = 32;
    let pointer:&i32= &b; 

    println!("HEX:\t{:?}",memory_location_hex);
    println!("INT:\t{:?}",memory_location_int); 

    let arr:[u8;4] = [1,2,3,4];
    //raw pointer to the arrray 
    let  arr_ptr = &arr as *const [u8;4]; 

    println!("ADDR HEX ARR : {0:?}",arr_ptr);
    println!("ADDR INT ARR : {0:?}",arr_ptr as usize); 
    type_def(arr_ptr,format!("{0:?}",arr_ptr).as_str());
    println!("VALUE ARR : {0:?}",unsafe{*arr_ptr});

    

    // derefrencing using * operator 

    //since raw pointers are unsafe to defrence we have to use unsafe block 
    
    //Unsafe code behaves exactly like normal Rust with the exception of a few abilities that the Rust compiler is unable to make guarantees about.

    // turning it into a representation of data you can use (i.e. *const f64 into f64). 
    //Rust has no way to keep track of the meaning of every byte that gets written to memory. 
    //Because Rust can't make guarantees about what exists at an arbitrary number used as a raw pointer,
    unsafe
    { 
    // This is unsafe because we are telling the compiler
    // to assume our pointer is a valid f64 and

    println!("VALUE:\t{:?}",*memory_location_hex);
    } 

    println!("VALUE b :\t{:?}",*pointer);

    // derefencing the pointer using * operator 

    let ref_ref_ref_a: &&&f64 = &&&a;
    let ref_a: &f64 = **ref_ref_ref_a;
    let c: f64 = *ref_a;
    println!("reference c :\t{0:?}", c);
    
    //use of smart pointer i.e.  creating a reference like structs
    // works on the basics of the internal logic that a programmer writes 
    //smart pointers implement Deref, DerefMut, and Drop traits 
    //to specify the logic of what should happen when the structure is dereferenced with * and . operators.

    let message = Message { content: "Hello world" }; 
    //derefencing occurs here 
    println!("{0:?}",message.len()); 
    //println!("{}",message); cause error as Display is not defined to be printed by the macro  
    println!("{0:?}",*message); 



}