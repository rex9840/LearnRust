use std::alloc::{alloc , Layout};
use std:: ops::Deref; 


struct Pie 
{ 
    recipie: usize //unsigned int repr for bytes 
}


impl Pie
{ 
    //creating a new isntances 
    fn new()->Self
    {   
        /*Defining a memory Layout: 
            allocation 4bytes of memory with 1 byte alignment 
            allignment is used for defining the size of the memory block or establishing some boundary on the block of contiguous memory
            the allocation of 4 bytes with an alignment of 1 byte is a simple memory allocation that doesn't impose any special alignment requirements. 
            It's suitable for storing basic data types like u8, i8, or small integer types where strict alignment may not be critical.

                allign : must not be zero and must be in the power of 2 
         */
        let layout =Layout::from_size_align(4,1).unwrap();  
        //The unwrap function is used to get the value inside the Result object returned by from_size_align. 
        //If from_size_align fails to create a layout, it will return an Err variant of Result and the unwrap function will panic.
        unsafe
        { 
            let ptr = alloc(layout) as *mut u8;  //Allocate memory with the global allocator. where ptr is a pointer which points toward the allocated memory
            
            ptr.write(86);
            ptr.add(1).write(14); 
            ptr.add(2).write(73);
            ptr.add(3).write(64);

            /*  The code uses pointer arithmetic to write specific u8 values (86, 14, 73, 64) to consecutive memory locations starting from the address pointed to by ptr.
                ptr.write(10) writes the value 10 to the memory location pointed to by ptr.
                ptr.add(1) calculates a new pointer that is one byte ahead of the current pointer, and .write(87) writes the value 87 to that memory location. This pattern continues for the other values. */

            let pie = Pie{recipie: ptr as usize};
            return pie;
        }
    }
}


impl  Deref for Pie
{
    type Target =f32; 
    fn deref (&self)->&Self::Target
    {
        let pointer = self.recipie as *const Self::Target; 
        unsafe{ return &*pointer;}
    }
}


fn main()
{ 
    let  p =   Pie::new(); 
    println!("SIZE : {0:?}", std::mem::size_of_val(&p));
    println!("DEREFERENCE : {0:?}", *p);
}