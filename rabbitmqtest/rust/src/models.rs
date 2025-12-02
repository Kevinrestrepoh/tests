use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Item {
    pub id: u32,
    pub quantity: u32,
}
