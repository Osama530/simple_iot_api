#[derive(Serialize, Deserialize)]
pub struct Switch {
    pub name: String,
    pub timer: String,
    pub state: String,
    pub temp: u16,
    pub success: bool,
    
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

#[derive(FromFormValue, Serialize, Deserialize)]
pub enum Colour {
    Red,
    Green,
    Blue,
}

#[derive(FromForm, Serialize, Deserialize)]
pub struct Bulb {
    pub name: String,
    pub state: String,
    pub colour: Colour,
    pub success: bool,

}

#[derive(Serialize, Deserialize)]
pub struct Sensor {
    pub name: String,
    pub state: String,
    pub success: bool,
    
}


#[derive(Serialize, Deserialize)]
pub struct Lights {
    pub switch: SimpleSwitch,
    pub modes: Modes
}

#[derive(Serialize, Deserialize)]
pub enum Modes {
    Default(Default),
    Custom(Custom),
    Sleep,
}
// **************** using traits ***************//

#[derive(Serialize, Deserialize)]
pub struct Light {
    pub switch: SimpleSwitch,
    pub modes: Modes
}

#[derive(Serialize, Deserialize)]
pub struct Custom {
    pub name: String,
    pub state: String,
    pub colour: Colour
}

impl Custom {
    pub fn set_custom(name: String, state: String)-> Custom{
        Custom {
            name,
            state,
            colour: Colour::Green
        }
    }
}
#[derive(Serialize, Deserialize)]
pub struct Default {
    pub name: String,
    pub state: String,
    pub colour: Colour
}

impl Default {
    pub fn set_defaults()-> Default {
        Default {
            name: "Default mode".to_string(),
            state: "Active".to_string(),
            colour: Colour::Blue
        }
    }
}
