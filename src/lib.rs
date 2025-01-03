pub mod grid;
pub mod plugin;

pub mod prelude {
    pub use super::grid::prelude::*;
    pub use crate::plugin::prelude::*;
    #[cfg(test)]
    pub use crate::test::*;
}

#[cfg(test)]
pub mod test {
    use crate::prelude::*;
    use bevy::prelude::*;

    pub fn setup_plugin(app: &mut App) -> &mut App {
        app.add_plugins(EntityGridPlugin {
            settings: EntityGridSettings {
                cell_size: 1.0,
                up_offset: 0.0,
            },
        });
        app
    }
}
