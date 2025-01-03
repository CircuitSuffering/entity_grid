use crate::prelude::*;

pub mod cardinal;
pub mod ordinal;
pub mod radius;

pub mod prelude {
    pub use super::Neighbor;
    pub use super::cardinal::prelude::*;
    pub use super::ordinal::prelude::*;
    pub use super::radius::prelude::*;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Neighbor {
    pub position: GridPosition,
    pub entry: GridEntity,
}

impl Neighbor {
    pub fn new(position: GridPosition, entry: GridEntity) -> Self {
        Self { position, entry }
    }
}
