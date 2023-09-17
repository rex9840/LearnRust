use std::sync::Mutex; 
//mutex  means mutal exclusion which means giving resouces can be acessed by one thread at a time 

//Mutex takes in data and lets us borrow mutable and immutable references to the data 
//This prevents borrowing from being abused by having the operating system restrict only one CPU thread at time to have access to the data, blocking other threads until that original thread is done with its locked borrow.



struct Person; 


impl  Person
{ 
    fn task(&self)
    { 
        println!("initiating the task");
    }
}


fn main()
{ 
    let mutex_person = Mutex::new(Person); 
    //burrow the imutable reference to the person 
    let mutex_person_ref = mutex_person.lock().unwrap(); 
    //unwrap is used to handle the error during the burrow lock 
    
    /*let mutex_person_ref2 = mutex_person.lock().unwrap(); 
    mutex_person_ref2.task();*/

    //uncommenting and runnung the above section of code may result into dead lock 


    mutex_person_ref.task(); 


    //mutex_person_ref is dropped here and mutex_person is unlocked
}