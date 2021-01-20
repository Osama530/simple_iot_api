#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

mod modules;
use crate::modules::{routes, route_lights};

fn main() {
    rocket::ignite().mount("/myapi", routes![
        routes::bulb,
        routes::sensor,
        routes::index_page,
        route_lights::light_default,
        route_lights::light_sleep,
        route_lights::light_costom ]).launch();
}
