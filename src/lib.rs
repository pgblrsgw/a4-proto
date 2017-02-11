#[macro_use]
extern crate serde_derive;

/// Describes an obstacle to be added to the pathfinding algorithm.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Obstacle {
    pub width: f64,
    pub length: f64,
    pub coordinateX: f64,
    pub coordinateY: f64,
}
