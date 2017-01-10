// Re-export the sub-libraries under the `units::` namespace
pub use units::drawing::{AsGame,AsTile,AsPixel};
pub use units::drawing::{Game,Tile,HalfTile,Pixel};

pub use units::physics::{Millis,Velocity,Acceleration};
pub use units::physics::{Direction, Degrees,AngularVelocity};

pub use units::physics::{Frame,Fps};

pub use units::physics::dt2ms;

// Load sub-libraries
pub mod drawing;
pub mod linear;
pub mod physics;
