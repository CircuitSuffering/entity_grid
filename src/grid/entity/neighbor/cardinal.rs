use bevy::prelude::*;

use crate::prelude::*;

pub mod prelude {
    pub use super::CardinalNeighbors;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CardinalNeighbors {
    pub north: Option<Neighbor>,
    pub east: Option<Neighbor>,
    pub south: Option<Neighbor>,
    pub west: Option<Neighbor>,
}

impl CardinalNeighbors {
    pub fn new() -> Self {
        Self {
            north: None,
            east: None,
            south: None,
            west: None,
        }
    }

    pub fn with_empty_entities(&self) -> Self {
        CardinalNeighbors {
            north: match self.north.clone() {
                Some(entity) => Some(Neighbor {
                    position: entity.position,
                    entry: GridEntity {
                        entity: Entity::PLACEHOLDER,
                        rotation: entity.entry.rotation,
                    },
                }),
                None => None,
            },
            east: match self.east.clone() {
                Some(entity) => Some(Neighbor {
                    position: entity.position,
                    entry: GridEntity {
                        entity: Entity::PLACEHOLDER,
                        rotation: entity.entry.rotation,
                    },
                }),
                None => None,
            },
            south: match self.south.clone() {
                Some(entity) => Some(Neighbor {
                    position: entity.position,
                    entry: GridEntity {
                        entity: Entity::PLACEHOLDER,
                        rotation: entity.entry.rotation,
                    },
                }),
                None => None,
            },
            west: match self.west.clone() {
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
    pub fn get_cardinal_neighbors(&self, position: GridPosition) -> CardinalNeighbors {
        CardinalNeighbors {
            north: self
                .get(GridPosition::new(position.x, position.y + 1))
                .map(|entry| Neighbor {
                    position: GridPosition::new(position.x, position.y + 1),
                    entry,
                }),
            east: self
                .get(GridPosition::new(position.x + 1, position.y))
                .map(|entry| Neighbor {
                    position: GridPosition::new(position.x + 1, position.y),
                    entry,
                }),
            south: self
                .get(GridPosition::new(position.x, position.y - 1))
                .map(|entry| Neighbor {
                    position: GridPosition::new(position.x, position.y - 1),
                    entry,
                }),
            west: self
                .get(GridPosition::new(position.x - 1, position.y))
                .map(|entry| Neighbor {
                    position: GridPosition::new(position.x - 1, position.y),
                    entry,
                }),
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_cardinal_neighbors() {
        let mut app = App::new();
        setup_plugin(&mut app);
        seed::cardinal(&mut app).update();
        // Get the grid
        let grid = app
            .world()
            .get_resource::<EntityGridState>()
            .unwrap()
            .grid
            .clone();
        // assert equal
        assert_eq!(
            grid.get_cardinal_neighbors(GridPosition::from(IVec2::new(0, 0)))
                .with_empty_entities(),
            CardinalNeighbors::seeded()
        );
    }

    pub mod seed {
        use super::*;

        impl CardinalNeighbors {
            pub fn seeded() -> Self {
                let mut neighbors: CardinalNeighbors = CardinalNeighbors::new();
                neighbors.north = Some(Neighbor {
                    position: GridPosition::new(0, 1),
                    entry: GridEntity {
                        entity: Entity::PLACEHOLDER,
                        rotation: Rotation::Up,
                    },
                });
                neighbors.east = Some(Neighbor {
                    position: GridPosition::new(1, 0),
                    entry: GridEntity {
                        entity: Entity::PLACEHOLDER,
                        rotation: Rotation::Right,
                    },
                });
                neighbors.south = Some(Neighbor {
                    position: GridPosition::new(0, -1),
                    entry: GridEntity {
                        entity: Entity::PLACEHOLDER,
                        rotation: Rotation::Down,
                    },
                });
                neighbors.west = Some(Neighbor {
                    position: GridPosition::new(-1, 0),
                    entry: GridEntity {
                        entity: Entity::PLACEHOLDER,
                        rotation: Rotation::Left,
                    },
                });
                neighbors
            }

            pub fn into_app(self, app: &mut App) -> &mut App {
                match self.north {
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
                match self.east {
                    Some(entity) => {
                        app.world_mut()
                            .get_resource_mut::<EntityGridState>()
                            .unwrap()
                            .spawn_rotation = entity.entry.rotation;
                        app.world_mut()
                            .spawn((Text2d::new("EMPTY"), GridPosition::new(1, 0)));
                        app.update();
                    }
                    None => {}
                }
                match self.south {
                    Some(entity) => {
                        app.world_mut()
                            .get_resource_mut::<EntityGridState>()
                            .unwrap()
                            .spawn_rotation = entity.entry.rotation;
                        app.world_mut()
                            .spawn((Text2d::new("EMPTY"), GridPosition::new(0, -1)));
                        app.update();
                    }
                    None => {}
                }
                match self.west {
                    Some(entity) => {
                        app.world_mut()
                            .get_resource_mut::<EntityGridState>()
                            .unwrap()
                            .spawn_rotation = entity.entry.rotation;
                        app.world_mut()
                            .spawn((Text2d::new("EMPTY"), GridPosition::new(-1, 0)));
                        app.update();
                    }
                    None => {}
                }
                app
            }
        }

        pub fn cardinal(app: &mut App) -> &mut App {
            app.world_mut()
                .spawn((Text2d::new("EMPTY"), GridPosition::new(0, 0)));
            app.update();
            // north
            app.world_mut()
                .get_resource_mut::<EntityGridState>()
                .unwrap()
                .spawn_rotation = Rotation::Up;
            app.world_mut()
                .spawn((Text2d::new("EMPTY"), GridPosition::new(0, 1)));
            app.update();
            // east
            app.world_mut()
                .get_resource_mut::<EntityGridState>()
                .unwrap()
                .spawn_rotation = Rotation::Right;
            app.world_mut()
                .spawn((Text2d::new("EMPTY"), GridPosition::new(1, 0)));
            app.update();
            // south
            app.world_mut()
                .get_resource_mut::<EntityGridState>()
                .unwrap()
                .spawn_rotation = Rotation::Down;
            app.world_mut()
                .spawn((Text2d::new("EMPTY"), GridPosition::new(0, -1)));
            app.update();
            // west
            app.world_mut()
                .get_resource_mut::<EntityGridState>()
                .unwrap()
                .spawn_rotation = Rotation::Left;
            app.world_mut()
                .spawn((Text2d::new("EMPTY"), GridPosition::new(-1, 0)));
            app.update();
            app
        }
    }
}
