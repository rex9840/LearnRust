use std::time::SystemTime; 
struct Student{ 
    name: String, 
    roll_no : i32,
    faculty : String ,
    examination_date : String,
}

fn main(){
    
    let ram = Student{
        name: String::from("Ram"),
        roll_no : 20 ,
        faculty : String::from("science") ,
        examination_date : String::from("2078/06/09") ,
    };
    
    println!("{{name:{} , roll_no:{} , faculty:{} , examination_date:{}}}",ram.name,ram.roll_no,ram.faculty,ram.examination_date); 
    
    let now = SystemTime::now(); 
    println!("time{:#?}",now )
}