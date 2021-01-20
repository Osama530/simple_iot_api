#[derive(FromForm, Serialize, Deserialize)]
pub struct Bulb {
    pub name: String,
    pub state: String,
    pub success: bool,

}



