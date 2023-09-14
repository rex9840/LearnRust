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
    fn deref(&self) -> &Self::Target
    {
        println!("{0:?} was used",std::any::type_name::<T>());
        &self.content
    }
}




fn main()
{ 

//we can use as keyword for defining the types
    let a:f64= (1000 as i32) as f64; 
    let memory_location_hex = &a as *const f64;  // type : raw pointer 
    let memory_location_int = memory_location_hex as usize;  
    let b:i32 = 32;
    let pointer:&i32= &b; 
    println!("HEX:\t{:?}",memory_location_hex);
    println!("INT:\t{:?}",memory_location_int); 


    // derefrencing using * operator 

    //since raw pointers are unsafe to defrence we have to use unsafe block 
    unsafe
    { 
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
 //   println!("{}",message); cause error as Display is not defined to be printed by the macro  
    println!("{0:?}",*message); 



}