pub mod neighbor;
pub mod rotation;

use crate::prelude::*;

use bevy::prelude::*;

pub mod prelude {
    pub use super::GridEntity;
    pub use super::neighbor::prelude::*;
    pub use super::rotation::prelude::*;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct GridEntity {
    pub entity: Entity,
    pub rotation: Rotation,
}

impl GridEntity {
    pub fn new(entity: Entity, rotation: Rotation) -> Self {
        Self { entity, rotation }
    }

    #[cfg(test)]
    pub fn test() -> Self {
        Self {
            entity: Entity::PLACEHOLDER,
            rotation: Rotation::default(),
        }
    }
}
