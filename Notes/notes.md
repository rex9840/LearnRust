*Rust as a language aims to solve many of these common challenges in systems programming:*

- Unintentional modification of resources
- Forgetting to deconstruct resources
- Resources accidentally being deconstructed twice
- Using resources after they have been deconstructed
- Data races caused by writing to resources while others are reading from resources
- Seeing clearly areas of the code where the compiler can’t make guarantees

**One of the imp concepts of rust are**

- ownership
- borrowing
- lifetimes

**impl**-> implimentation
**traits** -> interfaces and aggrigate  for polymerphism and i heritances

**Dispatches in rust**

- **static dispatch** :  When the instance type is known, we have direct knowledge of what function to call.
- **dynamic dispatch** : When an instance type is not known, we must find out some way of calling the correct function , slower , must define the dynamic dispacthes ie refrences to a *trait object* using dyn keyword.

##Project Organization and Structure 

####Modules 

- Rust uses `crates` for rust program and libraries. 
- we must build module tree explicitly for rust as rust doesnt have 1 to 1 mapping 
- Rust  has a root  module in file `./src/main.rs`
- library  has a root module in file  `./<lib_name>/src/lib.rs`
- creating a new crate `cargo new <crate_name>` 
- creating a new library `cargo new --lib <crate_name>` this 
- There are two ways to create module in rust : 
    - filename `<modulename>.rs`
    - folder `<modulename>/mod.rs`
- `mod` keyword is used to define a module in rust 



**prelude**: 
- rust has a prelude which is imported by default in every rust program 
- prelude contains the most common used types,traits and functions
- prelude is imported by `use std::prelude::*` 
        -`std::prelude::*` is automatically available to every part of Rust. That is the case for Vec and Box but others as well (Option, Copy, etc.).

**Creating your own prelude:** 

- It's common for library to have its own prelude module as a starting point for where users should import all of the most common data structures for using your library (e.g `use my_library::prelude::*`). It doesn't automatically get used in programs/libraries that uses crates, but it's a good convention to follow so people can know where to start.



