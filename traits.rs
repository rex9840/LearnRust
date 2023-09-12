#![allow(dead_code)]

struct SeaCreature
{ 
    pub name:String,
    noise:String
}

impl SeaCreature
{ 
    pub fn get_sound(&self) -> &str
    {
            return &self.noise; 
    } 
}


trait NoiseMaker  //interface and aggregrate 
{ 
    fn make_noise(&self);
    
    fn make_alot_of_noise(&self)
    { 
        self.make_noise();
        self.make_noise();

    } 
}


impl NoiseMaker for SeaCreature
{
    fn make_noise(&self)
    {
        println!("{0:?}",&self.get_sound());
    }

    fn make_alot_of_noise(&self)
    {
            for _ in 0..3
            {
                self.make_noise();
            }
    }
    
}

// static and dyanmic dispatch where dynamic dispactch dosent know the  instances we are using so we use  traits objects.
//use of dyn keyword.

fn static_make_noise(creature: &SeaCreature)
{
    creature.make_noise(); 
}

fn dynamic_make_noise(noise_maker : &dyn NoiseMaker)
{
    noise_maker.make_alot_of_noise(); 
}


fn main()
{ 
    let  creature = SeaCreature{
        name:String::from("ferris"),
        noise:String::from("bulb")
    };
    println!("making small noise :");
    creature.make_noise();
    println!("make more noise :");
    creature.make_alot_of_noise();
    println!("*************");
    static_make_noise(&creature);
    dynamic_make_noise(&creature); 

}
