use bevy::prelude::*;

use crate::prelude::*;

pub mod prelude {
    pub use super::RadiusNeighbors;
}

/// The neighbors of a grid entity
#[derive(Debug, Clone, PartialEq)]
pub struct RadiusNeighbors {
    /// The neighbors
    pub neighbors: Vec<Neighbor>,
}

impl RadiusNeighbors {
    /// Create a new set of neighbors
    pub fn new() -> Self {
        RadiusNeighbors {
            neighbors: Vec::new(),
        }
    }

    /// Create a new set of neighbors with empty entities
    pub fn with_empty_entities(&self) -> Self {
        Self {
            neighbors: self
                .neighbors
                .iter()
                .map(|neighbor| {
                    Neighbor::new(neighbor.position, GridEntity {
                        entity: Entity::PLACEHOLDER,
                        rotation: neighbor.entry.rotation,
                    })
                })
                .collect(),
        }
    }
}

impl Grid {
    /// Get the neighbors of a grid entity
    /// This will return the neighbors of the entity at the given position
    /// This will return a square of neighbors with the given radius
    pub fn get_square_radius_neighbors(
        &self,
        position: GridPosition,
        radius: i32,
    ) -> RadiusNeighbors {
        let mut returned_neighbors: RadiusNeighbors = RadiusNeighbors::new();
        for x in -radius..=radius {
            for y in -radius..=radius {
                let neighbor_position = GridPosition {
                    x: position.x + x,
                    y: position.y + y,
                };

                if let Some(neighbor) = self.get(neighbor_position) {
                    returned_neighbors
                        .neighbors
                        .push(Neighbor::new(neighbor_position, neighbor));
                }
            }
        }
        returned_neighbors
    }

    // Get the neighbors of a position with a radius
    // This return should be rounded to only include neighbors that are X/radius from the position
    pub fn get_rounded_radius_neighbors(
        &self,
        position: GridPosition,
        radius: i32,
    ) -> RadiusNeighbors {
        let mut returned_neighbors: RadiusNeighbors = RadiusNeighbors::new();
        for x in -radius..=radius {
            for y in -radius..=radius {
                let neighbor_position = GridPosition {
                    x: position.x + x,
                    y: position.y + y,
                };

                if let Some(neighbor) = self.get(neighbor_position) {
                    returned_neighbors
                        .neighbors
                        .push(Neighbor::new(neighbor_position, neighbor));
                }
            }
        }

        returned_neighbors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radius_neighbors() {
        let mut app = App::new();
        setup_plugin(&mut app);
        RadiusNeighbors::seeded(1).into_app(&mut app).update();
        // Get the grid
        let grid = app
            .world()
            .get_resource::<EntityGridState>()
            .unwrap()
            .grid
            .clone();
        assert_eq!(
            grid.get_square_radius_neighbors(GridPosition::new(0, 0), 2)
                .with_empty_entities(),
            RadiusNeighbors::seeded(1)
        );
    }

    pub mod seed {
        use super::*;

        impl RadiusNeighbors {
            pub fn seeded(size: i32) -> Self {
                let mut neighbors: RadiusNeighbors = RadiusNeighbors::new();
                for x in -size..=size {
                    for y in -size..=size {
                        let position = GridPosition::new(x, y);
                        if x == 0 && y == 0 {
                            continue;
                        }
                        neighbors
                            .neighbors
                            .push(Neighbor::new(position, GridEntity {
                                entity: Entity::PLACEHOLDER,
                                rotation: Rotation::default(),
                            }));
                    }
                }
                neighbors
            }

            pub fn into_app(self, app: &mut App) -> &mut App {
                for entity in self.neighbors {
                    app.world_mut()
                        .get_resource_mut::<EntityGridState>()
                        .unwrap()
                        .spawn_rotation = entity.entry.rotation;
                    app.world_mut()
                        .spawn((Text2d::new("EMPTY"), entity.position));
                    app.update();
                }
                app
            }
        }
    }
}
