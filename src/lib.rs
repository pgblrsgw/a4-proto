#[macro_use]
extern crate serde_derive;

/// Describes the shape of an obstacle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Shape {
    Circular {
        /// The radius of the circular shape
        radius: f64,
    },
    Rectangular {
        /// The [width, height] of the rectangle
        dimensions: [f64; 2],
        /// An angle of 0 means the width is along the x-axis and height is along the y-axis.
        /// A left-handed coordinate system is used and `angle` is in radians.
        angle: f64,
    },
}

/// Describes an obstacle to be added to the pathfinding algorithm.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Obstacle {
    /// [x, y] position of the obstacle
    pub location: [f64; 2],
    /// The shape specification of the obstacle
    pub shape: Shape,
}
