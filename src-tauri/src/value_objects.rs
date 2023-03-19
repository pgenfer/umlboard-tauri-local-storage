use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Point {x: f32, y: f32}
#[derive(Debug, Serialize, Deserialize)]
pub struct Dimension {width: f32, y: f32}