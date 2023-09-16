struct Pie; 

impl Pie 
{
    fn eat(&self)
    { 
        println!(" Heap is delicious! ")
    }
}



fn main()
{ 
    let headp_pie = Box::new(Pie); 
    headp_pie.eat(); 
    println!("SIZE OF POINTED VALUE : {:?} ", std::mem::size_of_val(&headp_pie)); 
}