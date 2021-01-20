//index route for testing purpose only
#[get("/")]
pub fn index_page()-> String {
    "well come to IOT-API home page".to_string()
}
//****************************************************//
use rocket_contrib::json::Json;

use crate::modules::bulb::*;
use crate::modules::sensors::*;

#[get("/bulb?<state>")]
pub fn bulb(state: String)-> Json<Bulb> {
    Json(
        Bulb {
            name: "bulb 1".to_string(),
            state,
            success: true

        }
    )
}

#[get("/sensor?<state>")]
pub fn sensor(state: String)-> Option<Json<Sensor>> {
    if state == 
    Some(Json (
        Sensor {
            name: "sensor 1".to_string(),
            state,
            success: true
        }
    ))

}

