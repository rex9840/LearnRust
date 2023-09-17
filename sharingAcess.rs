//RefCell is a container data structure
//RefCell is a type in Rust that provides interior mutability, allowing you to mutate data even when it's behind an immutable reference. 
//It is  primarily used in situations where you need to mutate data that is otherwise considered immutable according to Rust's ownership and borrowing rules.
//Only one mutable reference OR multiple immutable references, but not both or else it will go into panic 


use std::cell::RefCell;

struct Pie
{ 
    slices: u8
}

impl Pie
{ 
    fn eat(&mut self)
    { 
        println!("Eat in heap!");
        self.slices -= 1; 
    }
}


fn main()
{
    //Validation of mememory saftey at runtime using RefCell 
    let pie_cell  = RefCell::new(Pie{slices:8}); //pie_cell is not mutable 
    
    //defining a local scope 
    { 
        //burrow a mutable reference to the pie inside the RefCell 
        let mut mut_ref_pie = pie_cell.borrow_mut(); 
        mut_ref_pie.eat(); 
        mut_ref_pie.eat(); 

        //mut_ref_pie  is dropped
    }

    {
        let mut mut_borrow_ref_pie = pie_cell.borrow_mut(); 
        mut_borrow_ref_pie.eat(); 
    }


    let ref_pie  = pie_cell.borrow(); //burrowing the data form pie_cell
    println!("{0:?} slices left", ref_pie.slices);


}