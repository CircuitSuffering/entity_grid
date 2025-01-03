use bevy::prelude::*;

pub mod prelude {
    pub use super::EntityGridState;
}

use crate::prelude::*;

/// The state of the grid
#[derive(Debug, Clone, Resource)]
pub struct EntityGridState {
    /// The grid
    pub grid: Grid,
    /// The settings for the grid
    pub settings: EntityGridSettings,
    /// Spawn Rotation
    pub spawn_rotation: Rotation,
}
