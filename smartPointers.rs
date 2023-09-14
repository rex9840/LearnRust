//Raw Pointer 
//refreences can be changed into primitive pointers called raw pointers

//A raw pointer in Rust is similar to a pointer in C but with much more compile time restrictions on how it can be stored and moved around 

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

    let ref_ref_ref_a: &&&f64 = &&&a;
    let ref_a: &f64 = **ref_ref_ref_a;
    let c: f64 = *ref_a;
    println!("reference c :\t{0:?}", c)




}