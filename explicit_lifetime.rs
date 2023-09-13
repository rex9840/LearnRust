#![allow(dead_code)]

/* Rust's borrow checker uses these annotations to ensure that a program does not have any invalid memory references, which can cause a range of issues from null pointer dereferences to data races. 
By specifying the lifetimes of references, Rust can guarantee that a compiled program will be safe and free of these common issues. 
Explicit lifetime annotations are especially important when working with structs that contain references, as their lifetimes must be carefully managed to prevent dangling pointers and other unsafe conditions. */


struct Foo { 
    x: i32,

}

//life time of the datatypes 
//defining a struct called LifeTime 
struct LifeTime<'a>{
    x:&'a i32 
}

//single life time example 
fn do_something_single<'a>(foo: &'a Foo) -> &'a i32 { 
    return &foo.x;
}

//multiple life time example 

// foo_b and the return value share the same lifetime
// foo_a has an unrelated lifetime

fn do_something_multiple<'a,'b>(foo_a: &'a Foo, foo_b: &'b Foo)-> &'b i32 { 
    println!("{}",foo_a.x);
    println!("{}",foo_b.x); 
    return &foo_b.x; 
//lifetime 'b bounds both do_something_multiple and foo_b which means foo_b exits until do_something_multiple exits 

}


fn main()
{ 
    println!("---single lifetime---");

    let mut foo = Foo { x: 42 }; 
    let x = &mut foo.x; 
    *x = 13; 
    let y= do_something_single(&foo);
    println!("{}",y);


    //multiple lifetime 
    let foo_a = Foo {x:42};
    let foo_b = Foo {x:43}; 
    println!("---multiple lifetime---");
    let x =do_something_multiple(&foo_a, &foo_b); 
    // foo_a is dropped here because only foo_b's lifetime exist beyond here
    
    println!("{}",x);
    
     // x is dropped here
    // foo_b is dropped here



    //life time of a datatype 
    println!("---lifetime of a datatype---"); 
    let x = 12; 

    let datatype_lifetime_a = LifeTime{
        x :&x
    };

    println!("{}",datatype_lifetime_a.x); 

}

