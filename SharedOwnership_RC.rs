use std::rc::Rc ;

//smat pointer which moves data from stack to heap 
//Rc smart pointers that all have the ability to immutably borrow the data that was put on the heap.
//Rc stands for "Reference Counting" in Rust, and it's a type that provides shared ownership of a value across multiple parts of your program. 
struct Person;


impl Person
{ 
    fn task(&self)
    { 
        println!("Heap task  is performed");
    }
}



fn main()
{ 
    let heap_person  = Rc::new(Person); 
    let heap_person_clone = heap_person.clone();
    let heap_person_clone2 = heap_person_clone.clone();

    heap_person.task();
    heap_person_clone.task(); 
    heap_person_clone2.task(); 


//refrences are dropped and heap data is deallocated

}