use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Point {pub x: f32, pub y: f32}
#[derive(Debug, Serialize, Deserialize)]
pub struct Dimension {pub width: f32, pub y: f32}