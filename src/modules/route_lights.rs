use rocket_contrib::json::Json;
use rocket::response::Redirect;
use crate::modules::lights::*;


//route to light setting to default colour mode
#[get("/lights")] 
pub fn light_default()-> Json<Light> {
    Json(
        Light {
            name: "balcony".to_string(),
            switch: SimpleSwitch::create("lamp switch".to_string(), "On".to_string()),
            modes: Modes::set_defaults()
        }
    )
}

#[get("/lights?<r>&<g>&<b>")] //Custom mode
pub fn light_costom(r: u8, g: u8, b:u8)-> Json<Light> {
    Json(
        Light {
            name: "room 1 light".to_string(),
            switch: SimpleSwitch::create("on off".to_string() , "active".to_string()),
            modes: Modes::set_custom(r, g, b)
        }
    )
}

//route to light setting to night colour mode (lights off mode)
#[get("/lights?<mode>", rank = 1)] //Custom mode
pub fn light_sleep(mode: String)-> Json<Light> {
    if mode == "sleep".to_string() {
        Json(
            Light {
                name: "sleep mode".to_string(),
                switch: SimpleSwitch::create("lamp switch".to_string(), "On".to_string()),
                modes: Modes::set_sleep()
            }
        )
    }
    else {
        Json(
            Light {
                name: "balcony".to_string(),
                switch: SimpleSwitch::create("lamp switch".to_string(), "On".to_string()),
                modes: Modes::set_defaults()
            }
        )
    }
}