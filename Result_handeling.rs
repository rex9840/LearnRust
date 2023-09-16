fn divide(number:i32 , divisior:i32 )-> Result<i32, String>
{ 
    if divisior==0{return Err(String::from("divided by zero"));}
    else{
        Ok(number/divisior)
    }
}


fn main(){ 

let result1  = divide(6,0);
let result2  = divide(6,3);

match result1 { 
    Ok(v)=>println!("{v:?}"),
    Err(e)=>println!("{e:?}"),
}


match result2 { 
    Ok(v)=>println!("{v:?}"),
    Err(e)=>println!("{e:?}"),
}


//creating a panic and returning the error message 

let result  = divide(100,0).unwrap();
println!("{:?}",result); 

}

