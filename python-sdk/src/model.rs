use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicKey {
    x: String,
    y: String,
}



impl PublicKey {
    pub fn new(x: String, y: String) -> Self {
        Self{x,y}
    }
}