use rocket::request::Form;
use crate::modules::colours;

#[derive(Serialize, Deserialize)]
pub struct Lights {
    pub light: Light
}

#[derive(Serialize, Deserialize)]
pub struct Light {
    pub name: String,
    pub switch: SimpleSwitch,
    pub modes: Modes
}

impl Light {
    pub fn new_light(name: String, switch: SimpleSwitch, mode: Modes)-> Light{
        Light {
            name,
            switch,
            modes: mode
        }
    }
}

impl Lights {
    fn create(name: String, switch: SimpleSwitch, mode: Modes)-> Lights {
        Lights {
            light: Light::new_light(name, switch, mode), 
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SimpleSwitch{
    pub name: String,
    pub state: String,
}
impl SimpleSwitch {
    pub fn create(name: String, state: String)-> SimpleSwitch{
        SimpleSwitch {
            name,
            state,
        }
    }
}


#[derive(Serialize, Deserialize)]
pub struct Modes {
    pub name: String,
    pub colour: colours::Colour
}

//initializing traits
pub trait api {
    //default trait implimentation
    fn set_defaults()-> Modes {
        Modes {
            name: "Default mode".to_string(),
            colour: colours::Colour::white()
        }
    }
    fn set_custom(r: u8, g: u8, b: u8)-> Modes;
    fn set_sleep()-> Modes;
}

impl api for Modes {
    fn set_custom(r: u8, g: u8, b: u8)-> Modes {
        Modes {
            name: "User defined".to_string(),
            colour: colours::Colour::define(r, g, b)
        }
    }
    
    fn set_sleep()-> Modes {
        Modes {
            name: "Sleep mode".to_string(),
            colour: colours::Colour::black()
        }
    }
}