/*

Rust lacks inheritance of data and behavior in any meaningful way.

- Structs cannot inherit fields from a parent struct.
- Structs cannot inherit functions from a parent struct.

 */


// first parameter of any method must be a reference to the instance associated with the method call 

/* 
Rust uses:

- &self - Immutable reference to the instance.
- &mut self - Mutable reference to the instance.


Methods are defined within an implementation block with keyword "impl:"
 */



//Abstraction With Selective Exposure by the use of "pub" keyword 



struct Animal{ 
    pub name: String,
    sound:String,

}


impl Animal{ 
    fn get_sound(&self)->&str{
        return &self.sound;
}

}


fn main(){ 
    let sound = "woof WOOF";
    let name ="Tiger"; 
    let dog = Animal{ 
        name: String::from(name), 
        sound:String::from(sound).to_lowercase(),
    };

    println!("{{\nname:{1:?},\nsound:{0:?}\n}}",dog.get_sound(),dog.name);

}


