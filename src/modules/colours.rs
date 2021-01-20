#[derive(Serialize, Deserialize)]
pub struct Colour {
    content: [u8; 3]
}

impl Colour {
    pub fn black()-> Colour{
        Colour {
            content: [0,0,0]
        }

    }

    pub fn white()-> Colour{
        Colour {
            content: [255,255,255]
        }

    }

    pub fn define(r: u8, g: u8, b: u8)-> Colour {
        Colour {
            content: [r,g,b]
        }
    }
}