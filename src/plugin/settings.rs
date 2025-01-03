pub mod prelude {
    pub use super::EntityGridSettings;
}

/// The settings for the grid
#[derive(Debug, Clone, PartialEq)]
pub struct EntityGridSettings {
    /// The size of the cell in the grid
    pub cell_size: f32,
    /// the up offset of the grid
    pub up_offset: f32,
}
