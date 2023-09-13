/* 

Due to the use of tratis assigned to the structs creates an unsized values  being stored.

Which is handelled into two ways: 
    -> generics : using parameterized types effectively create struct/functions known types and thus known sizes

    -> indirection : instances on the heap gives us a level of indirection that allow us to not have to worry about the size of the actual type and just store a pointer to it
                    i.e.(creating a pointer to heap and not worrying about the size of the actual type)

*/



struct Animal
{
    pub name: String,
    age: u8,
    catagoery: String
}

impl Animal
{   
    pub fn detail(&self)->String
    {
            let detail_info:String = format!("name: {0} age: {1} catagoery: {2} ",self.name,self.age,self.catagoery);
            return detail_info; 
    }
}


trait MakeNoise
{   
    fn make_noise(&self,noise:&str);
}



impl MakeNoise for Animal
{
    fn make_noise(&self,noise:&str)
    {
        println!("{0}",noise); 
    }
} 

//generic can also be called static dispath

fn generic_details<T>(animal: &T) where T: MakeNoise{
        animal.make_noise(&String::from("bark")); 
}



// generic dispatch can be written in short hand as 


fn generic_details_short(animal:&impl MakeNoise)
{
    animal.make_noise(&String::from("meow"));
}


//impl vs dyn : It's about performance (static dispatch, impl Trait) vs. code-size (dynamic dispatch, dyn Trait/Trait/Box<Trait>)
//in impl we have control over both types and tratis but in dyn we have control over only traits and not types.



//box  datasructure
//Box is a data structure that allows us to move our data from the stack to the heap.
//Box is a struct known as a smart pointer that holds the pointer to our data on the heap.
//it is often used as a way to store a reference to something in a struct that must know the size of its fields

struct Zoo 
{
    animals: Vec<Box<dyn MakeNoise>>
}




fn main()
{
        let dog = Animal{
            name: String::from("Kalu"),
            age: 3,
            catagoery: String::from("Dog") 
        };


        let cat = Animal{
            name: String::from("seti"),
            age: 2,
            catagoery: String::from("Cat") 
        };

        
        println!("{0:?}",dog.detail());
        dog.make_noise(&String::from("bark")); 
        println!("{0}",cat.detail()); 
        cat.make_noise(&String::from("meow"));
        println!("*******");
        generic_details(&dog);
        generic_details_short(&cat); 
        println!("*******");



        let zoo = Zoo {
            animals: vec![Box::new(dog),Box::new(cat)]
        };
        
        for animals in zoo.animals.iter()
        { 
            animals.make_noise(&String::from("pet me")); 
        }




}