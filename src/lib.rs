#[macro_use]
extern crate serde_derive;

/// An arbitrary point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// Describes an obstacle to be added to the pathfinding algorithm.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Obstacle {
    pub length: f64,
    pub width: f64,
    pub point: Point,
}

/// Describes the robot which will be pathfinding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Robot {
    pub point: Point,
    pub radius: f64,
}
