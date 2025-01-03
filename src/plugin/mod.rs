pub mod settings;
pub mod state;

use bevy::prelude::*;

pub mod prelude {
    pub use super::EntityGridPlugin;
    pub use super::settings::prelude::*;
    pub use super::state::prelude::*;
}

use crate::prelude::*;

/// The plugin for the entity grid
#[derive(Debug, Clone)]
pub struct EntityGridPlugin {
    /// The settings for the grid
    pub settings: EntityGridSettings,
}

/// Plugin implementation
impl Plugin for EntityGridPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EntityGridState {
            settings: self.settings.clone(),
            grid: Grid::default(),
            spawn_rotation: Rotation::default(),
        });

        app.add_systems(
            Update,
            |added_entities: Query<Entity, Added<GridPosition>>,
             mut query_common: Query<(&mut Transform, &GridPosition)>,
             mut state: ResMut<EntityGridState>| {
                added_entities.iter().for_each(|incoming_entity| {
                    // Get the position of the entity
                    let mut scoped_query = match query_common.get_mut(incoming_entity) {
                        Ok(transform) => transform,
                        Err(_) => return,
                    };
                    // Spawn rotation
                    let spawn_rotation = state.spawn_rotation;

                    // Set the translation of the entity based on the position of the position
                    scoped_query.0.translation = Vec3::new(
                        scoped_query.1.x as f32 * state.settings.cell_size,
                        state.settings.up_offset,
                        scoped_query.1.y as f32 * state.settings.cell_size,
                    );
                    // Check if the position is already in the grid
                    match state.grid.data.get_mut(scoped_query.1) {
                        // If the position is already in the grid, update the entity
                        Some(entity) => {
                            // Set the rotation of the entity based on the rotation of the position
                            scoped_query.0.rotation =
                                Quat::from_rotation_y(entity.rotation.to_angle());
                            *entity = GridEntity {
                                entity: incoming_entity,
                                rotation: spawn_rotation,
                            };
                        }
                        // If the position is not in the grid, insert the entity
                        None => {
                            // Set the rotation of the entity based on the rotation of the position
                            scoped_query.0.rotation =
                                Quat::from_rotation_y(state.spawn_rotation.to_angle());
                            state.grid.data.insert(scoped_query.1.clone(), GridEntity {
                                entity: incoming_entity,
                                rotation: spawn_rotation,
                            });
                        }
                    }
                });
            },
        );
    }
}
