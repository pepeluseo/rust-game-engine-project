use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct SpriteData {
    pub b: i32,
    pub g: i32,
    pub height: i32,
    pub r: i32,
    pub width: i32,
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
pub enum NetworkCommand {
    FetchSprite,
    Quit,
}
