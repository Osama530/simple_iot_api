#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use rocket_contrib::json::Json;

#[get("/")]
fn index_page()-> String {
    "well come to IOT-API home page".to_string()
}

#[derive(Serialize, Deserialize)]
struct Switch {
    name: String,
    timer: String,
    state: String,
    temp: u16,
    success: bool,
    
}

#[derive(FromFormValue, Serialize, Deserialize)]
enum Colour {
    Red,
    Green,
    Blue,
}

#[derive(FromForm, Serialize, Deserialize)]
struct Bulb {
    name: String,
    state: String,
    colour: Colour,
    success: bool,

}

#[get("/switch?<state>&<timer>")]
fn switch(state: String, timer: String) -> Json<Switch> {
    Json(
        Switch {
            name: "switch 1".to_string(),
            timer,
            state,
            temp: 27,
            success: true, 
        }
    )
}

#[get("/bulb?<state>&<colour>")]
fn bulb(state: String, colour: Colour)-> Json<Bulb> {
    Json(
        Bulb {
            name: "bulb 1".to_string(),
            state,
            colour,
            success: true

        }
    )
}

#[derive(Serialize, Deserialize)]
struct Sensor {
    name: String,
    state: String,
    success: bool,
    
}

#[get("/sensor?<state>")]
fn sensor(state: String)-> Json<Sensor> {
    Json (
        Sensor {
            name: "sensor 1".to_string(),
            state,
            success: true
        }
    )
}

fn main() {
    rocket::ignite().mount("/myapi", routes![switch, bulb, sensor, index_page]).launch();
}
