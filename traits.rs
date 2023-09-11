
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
                &self.make_noise();
            }
    }
    
}



fn main()
{ 
    let  creature = SeaCreature{
        name:String::from("ferris"),
        noise:String::from("bulb")
    };
    creature.make_noise();
    creature.make_alot_of_noise();
}
