
fn main(){ 

    let string_literal:&'static str = "hello world 🦀";
    //The string_literal is a string slice (&str) with a 'static lifetime. The 'static lifetime means that the string slice has a static lifetime and will be valid for the entire duration of the program

    let another_string_literal:String = String::from("hello world 🦀👀");
    // The String type represents an owned, growable string in Rust. The lifetime of another_string_literal is determined by its ownership, and it will be valid as long as it is in scope and not dropped.
    println!("{}",string_literal);
    println!("{}",another_string_literal); 


    //for raw string we can use start with r#  and end with # 
    let raw_string:&'static str =r#"this is a raw string ////////:/00"#; 
    println!("{raw_string:?}");

    //use of macro for very large text string imported from file 

    let large_string = include_str!("notes.md"); 
    println!("\n\n{large_string:?}\n\n"); 

    //string slices 


    //let crab:&str= "ferris🦀"; vs let crab = "ferris🦀";
    /*So, the key difference is that in the first statement, the type is explicitly declared as a string slice (&str), while in the second statement, the type is inferred as &'static str based on the fact that it is a string literal. 
    Both statements represent a reference to the same string literal, but the first statement provides more explicit type information.*/


    //str slice, must also be valid utf-8.
    let crab:&str= "ferris🦀";

    // Common methods of &str:

    // 1. len gets the length of the string literal in bytes (not number of characters).
    // 2. starts_with/ends_with for basic testing.
    // 3. is_empty returns true if zero length.
    // 4. find returns an Option<usize> of the first position of some te



    println!("{0:?}",crab.len()); 
    let first_word= &crab[0..6];
    let second_word = &crab[6..10];
    println!("{0:?} {1:?}",first_word,second_word);
    
    //let crab:&str= "ferris🦀"; vs let crab: String = String::from("ferris🦀");


    /*The key difference is that in the first statement, crab is a reference to a string slice, while in the second statement, crab is an owned String object. 
    The first statement is useful when you want to work with string data without taking ownership, while the second statement is useful when you need to own and manipulate the string data. */


    //chars in string 

    let message = "hello world".chars().collect::<Vec<char>>();
    //collecting the characters into the vector 
    println!("{0:?}",message.len());
    for i in message.iter(){ 
        println!("{0:?}",i); 
    }

    //string concatination 

    let mut hello_world = String::from("hello");
    hello_world.push_str("world");
    hello_world  = hello_world + "🦀";

    println!("{hello_world:?}");  

    let upper_case = hello_world.trim().to_uppercase(); 
    println!("{upper_case:?}");

    //borrowing the values to &str 

    say_example("hello world");
    say_example(&String::from("Example"));


    //Building the string 

    let building = ["h","i","ferris"];
    let concat = building.concat();
    let join = building.join(",");

    println!("{0:?}\n {1:?}",concat,join);

    // string conversion 

    let number =128;
    let number_to_string = number.to_string(); 
    let parsed_number = string_to_int(&number_to_string).unwrap();
    println!("string:{0:?},numeric:{1:?}",number_to_string,parsed_number); 


}


fn say_example(msg:&str){ 
    println!("{0:?}",msg.to_uppercase())
}

fn string_to_int(string:&str)->Result<i32, std::num::ParseIntError>{ 
    return string.parse::<i32>();
}