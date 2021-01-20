#[derive(Serialize, Deserialize)]
pub struct Sensor {
    pub name: String,
    pub state: String,
    pub success: bool,
    
}