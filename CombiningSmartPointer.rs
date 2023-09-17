use std::cell::RefCell;
use std::rc::Rc; 



struct Person
{ 
    tasks: u8
}



impl Person
{ 
    fn work(&mut self,name :&str)
    { 
        println!("{0:?} is working",name);
        self.tasks -= 1;  
    }
}


struct Worker
{
    name:String,
    person:Rc<RefCell<Person>>
}

//Rc(refernce counter) smart pointer for collective ownership 
//RefCell is a datastructure of smart pointer which gives us the mutable access to imuutable data 


impl Worker
{ 
    fn task(&self)
    {
        let mut person = self.person.borrow_mut();
        person.work(&self.name); 
    }
}

fn main()
{
    let person = Rc::new(RefCell::new(Person{tasks:4}));
    let hr= Worker {name:String::from("Ram"),person:person.clone()};

    let manager= Worker {name:String::from("Shyam"),person:person.clone()} ;

    manager.task(); 
    hr.task(); 

    let borrow_person = person.borrow();
    println!("Remaining task : {0:?}",borrow_person.tasks); 



}

