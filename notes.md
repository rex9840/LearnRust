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



