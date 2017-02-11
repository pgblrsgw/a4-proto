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

/// Describes the goal which the robot will pathfind to.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    pub point: Point,
}

/// Describes the boundary in which the pathfinding algorithm will search for a path.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Boundary {
    pub width: f64,
    pub length: f64,
    pub point: Point,
}

/// Describes the path of the rover.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Path {
    pub points: Vec<Point>,
}
