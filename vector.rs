#![allow(dead_code)]






fn main() { 

    //explicit vector defination 


    let mut i32_vec = Vec::<i32>::new();

    i32_vec.push(1); 
    i32_vec.push(2); 
    i32_vec.push(3);


//implicit vector defination 
    
       let mut float_vec = Vec::new();
       float_vec.push([1.0,2.0,3.0]); 
       




//use of macros vector defination

    let string_vec = vec![String::from("hello"),String::from("world"),String::from("!")]; 
    

    for word in string_vec.iter(){ 
        println!("{:#?}",word);
    }
    
//using from 


let vec = vec!([1,2,3,String::from("hello")]);






    println!("i32 vector :  {:?}" ,i32_vec);  
    println!("string vector :  {:?}" ,string_vec);
    println!("float vector : {:#?}",float_vec);
    println!("mixed vector : {}",vec);
}