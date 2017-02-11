#[macro_use]
extern crate serde_derive;

/// Describes an obstacle to be added to the pathfinding algorithm.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Obstacle {
    pub width: f64,
    pub length: f64,
    #[serde(rename = "coordinateX")]
    pub coordinate_x: f64,
    #[serde(rename = "coordinateY")]
    pub coordinate_y: f64,
}
