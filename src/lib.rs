#[macro_use]
extern crate serde_derive;

/// Describes an obstacle to be added to the pathfinding algorithm.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Obstacle {
    pub coordinate: [f64; 2],
    pub dimensions: [f64; 2],
}

/// Describes the robot which will be pathfinding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Robot {
    pub coordinate: [f64; 2],
    pub radius: f64,
}
