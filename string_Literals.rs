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



    let crab:&str= "ferris🦀";
    println!("{0:?}",crab.len()); 
    let first_word= &crab[0..6];
    let second_word = &crab[6..10];
    println!("{0:?} {1:?}",first_word,second_word);
    
    //let crab:&str= "ferris🦀"; vs let crab: String = String::from("ferris🦀");


    /*The key difference is that in the first statement, crab is a reference to a string slice, while in the second statement, crab is an owned String object. 
    The first statement is useful when you want to work with string data without taking ownership, while the second statement is useful when you need to own and manipulate the string data. */






}



