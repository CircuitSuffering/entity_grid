use crate::prelude::*;
use bevy::prelude::*;

pub mod prelude {
    pub use super::OrdinalNeighbors;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrdinalNeighbors {
    pub north_west: Option<Neighbor>,
    pub north_east: Option<Neighbor>,
    pub south_east: Option<Neighbor>,
    pub south_west: Option<Neighbor>,
}

impl OrdinalNeighbors {
    pub fn new() -> Self {
        Self {
            north_west: None,
            north_east: None,
            south_east: None,
            south_west: None,
        }
    }

    pub fn with_empty_entities(&self) -> Self {
        Self {
            north_west: match self.north_west.clone() {
                Some(entity) => Some(Neighbor {
                    position: entity.position,
                    entry: GridEntity {
                        entity: Entity::PLACEHOLDER,
                        rotation: entity.entry.rotation,
                    },
                }),
                None => None,
            },
            north_east: match self.north_east.clone() {
                Some(entity) => Some(Neighbor {
                    position: entity.position,
                    entry: GridEntity {
                        entity: Entity::PLACEHOLDER,
                        rotation: entity.entry.rotation,
                    },
                }),
                None => None,
            },
            south_east: match self.south_east.clone() {
                Some(entity) => Some(Neighbor {
                    position: entity.position,
                    entry: GridEntity {
                        entity: Entity::PLACEHOLDER,
                        rotation: entity.entry.rotation,
                    },
                }),
                None => None,
            },
            south_west: match self.south_west.clone() {
                Some(entity) => Some(Neighbor {
                    position: entity.position,
                    entry: GridEntity {
                        entity: Entity::PLACEHOLDER,
                        rotation: entity.entry.rotation,
                    },
                }),
                None => None,
            },
        }
    }
}

impl Grid {
    pub fn get_ordinal_neighbors(&self, position: GridPosition) -> OrdinalNeighbors {
        OrdinalNeighbors {
            north_west: match self.get(position + IVec2::new(-1, 1)) {
                Some(entity) => Some(Neighbor {
                    position: GridPosition::from(position + IVec2::new(-1, 1)),
                    entry: entity,
                }),
                None => None,
            },
            north_east: match self.get(position + IVec2::new(1, 1)) {
                Some(entity) => Some(Neighbor {
                    position: GridPosition::from(position + IVec2::new(1, 1)),
                    entry: entity,
                }),
                None => None,
            },
            south_east: match self.get(position + IVec2::new(1, -1)) {
                Some(entity) => Some(Neighbor {
                    position: GridPosition::from(position + IVec2::new(1, -1)),
                    entry: entity,
                }),
                None => None,
            },
            south_west: match self.get(position + IVec2::new(-1, -1)) {
                Some(entity) => Some(Neighbor {
                    position: GridPosition::from(position + IVec2::new(-1, -1)),
                    entry: entity,
                }),
                None => None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordinal_neighbors() {
        // Setup
        let mut app = App::new();
        setup_plugin(&mut app);
        seed::ordinal(&mut app).update();
        // Get the grid post setup
        let grid = app
            .world()
            .get_resource::<EntityGridState>()
            .unwrap()
            .grid
            .clone();
        // assert equal
        assert_eq!(
            grid.get_ordinal_neighbors(GridPosition::from(IVec2::new(0, 0)))
                .with_empty_entities(),
            OrdinalNeighbors::seeded(),
        );
    }

    pub mod seed {
        use super::*;

        impl OrdinalNeighbors {
            pub fn seeded() -> Self {
                let mut neighbors: OrdinalNeighbors = OrdinalNeighbors::new();
                neighbors.north_west = Some(Neighbor {
                    position: GridPosition::new(-1, 1),
                    entry: GridEntity {
                        entity: Entity::PLACEHOLDER,
                        rotation: Rotation::Up,
                    },
                });
                neighbors.north_east = Some(Neighbor {
                    position: GridPosition::new(1, 1),
                    entry: GridEntity {
                        entity: Entity::PLACEHOLDER,
                        rotation: Rotation::Right,
                    },
                });
                neighbors.south_east = Some(Neighbor {
                    position: GridPosition::new(1, -1),
                    entry: GridEntity {
                        entity: Entity::PLACEHOLDER,
                        rotation: Rotation::Down,
                    },
                });
                neighbors.south_west = Some(Neighbor {
                    position: GridPosition::new(-1, -1),
                    entry: GridEntity {
                        entity: Entity::PLACEHOLDER,
                        rotation: Rotation::Left,
                    },
                });
                neighbors
            }

            pub fn into_app(self, app: &mut App) -> &mut App {
                // north west
                match self.north_west.clone() {
                    Some(entity) => {
                        app.world_mut()
                            .get_resource_mut::<EntityGridState>()
                            .unwrap()
                            .spawn_rotation = entity.entry.rotation;
                        app.world_mut()
                            .spawn((Text2d::new("EMPTY"), entity.position));
                        app.update();
                    }
                    None => {}
                }

                // north east
                match self.north_east.clone() {
                    Some(entity) => {
                        app.world_mut()
                            .get_resource_mut::<EntityGridState>()
                            .unwrap()
                            .spawn_rotation = entity.entry.rotation;
                        app.world_mut()
                            .spawn((Text2d::new("EMPTY"), entity.position));
                        app.update();
                    }
                    None => {}
                }

                // south east
                match self.south_east.clone() {
                    Some(entity) => {
                        app.world_mut()
                            .get_resource_mut::<EntityGridState>()
                            .unwrap()
                            .spawn_rotation = entity.entry.rotation;
                        app.world_mut()
                            .spawn((Text2d::new("EMPTY"), entity.position));
                        app.update();
                    }
                    None => {}
                }

                // south west
                match self.south_west.clone() {
                    Some(entity) => {
                        app.world_mut()
                            .get_resource_mut::<EntityGridState>()
                            .unwrap()
                            .spawn_rotation = entity.entry.rotation;
                        app.world_mut()
                            .spawn((Text2d::new("EMPTY"), entity.position));
                        app.update();
                    }
                    None => {}
                }
                app
            }
        }

        pub fn ordinal(app: &mut App) -> &mut App {
            app.world_mut()
                .spawn((Text2d::new("EMPTY"), GridPosition::new(0, 0)));
            app.update();
            // north west
            app.world_mut()
                .get_resource_mut::<EntityGridState>()
                .unwrap()
                .spawn_rotation = Rotation::Up;
            app.world_mut()
                .spawn((Text2d::new("EMPTY"), GridPosition::new(-1, 1)));
            app.update();
            // north east
            app.world_mut()
                .get_resource_mut::<EntityGridState>()
                .unwrap()
                .spawn_rotation = Rotation::Right;
            app.world_mut()
                .spawn((Text2d::new("EMPTY"), GridPosition::new(1, 1)));
            app.update();
            // south east
            app.world_mut()
                .get_resource_mut::<EntityGridState>()
                .unwrap()
                .spawn_rotation = Rotation::Down;
            app.world_mut()
                .spawn((Text2d::new("EMPTY"), GridPosition::new(1, -1)));
            app.update();
            // south west
            app.world_mut()
                .get_resource_mut::<EntityGridState>()
                .unwrap()
                .spawn_rotation = Rotation::Left;
            app.world_mut()
                .spawn((Text2d::new("EMPTY"), GridPosition::new(-1, -1)));
            app.update();
            app
        }
    }
}
