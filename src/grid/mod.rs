pub mod entity;
pub mod position;

use bevy::{prelude::*, utils::HashMap};

use crate::prelude::*;

pub mod prelude {
    pub use super::Grid;
    pub use super::entity::prelude::*;
    pub use super::position::prelude::*;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid {
    pub data: HashMap<GridPosition, GridEntity>,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            data: HashMap::default(),
        }
    }
}

impl Grid {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, position: GridPosition, entity: Entity, rotation: Rotation) {
        self.data.insert(position, GridEntity { entity, rotation });
    }

    pub fn remove(&mut self, position: GridPosition) -> Option<GridEntity> {
        self.data.remove(&position)
    }

    pub fn get(&self, position: GridPosition) -> Option<GridEntity> {
        eprintln!("GET position: {:?}", position);
        eprintln!("GET data: {:?}", self.data);

        let data = self.data.get(&position).copied();
        eprintln!("GET NEW data: {:?}", data);
        data
    }

    pub fn get_mut(&mut self, position: GridPosition) -> Option<&mut GridEntity> {
        self.data.get_mut(&position)
    }

    pub fn contains(&self, position: GridPosition) -> bool {
        self.data.contains_key(&position)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&GridPosition, &GridEntity)> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&GridPosition, &mut GridEntity)> {
        self.data.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn fill(&mut self, entity: Entity, rotation: Rotation, radius: i32) {
        let radius = radius % 2;

        for x in -radius..radius {
            for y in -radius..radius {
                let position = GridPosition::new(x, y);
                self.insert(position, entity, rotation);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid() {
        let mut grid = Grid::new();

        let position = GridPosition::new(0, 0);
        let entity = Entity::PLACEHOLDER;
        let rotation = Rotation::default();

        grid.insert(position, entity, rotation);

        assert_eq!(grid.get(position), Some(GridEntity { entity, rotation }));
        assert_eq!(grid.contains(position), true);
        assert_eq!(grid.len(), 1);

        grid.remove(position);

        assert_eq!(grid.get(position), None);
        assert_eq!(grid.contains(position), false);
        assert_eq!(grid.len(), 0);
    }

    #[test]
    fn test_grid_get_mut() {
        let mut grid = Grid::new();

        let position = GridPosition::new(0, 0);
        let entity = Entity::PLACEHOLDER;
        let rotation = Rotation::default();

        grid.insert(position, entity, rotation);

        let grid_entity = grid.get_mut(position).unwrap();
        grid_entity.rotation = Rotation::Right;

        assert_eq!(
            grid.get(position),
            Some(GridEntity {
                entity,
                rotation: Rotation::Right
            })
        );
    }

    #[test]
    fn test_grid_contains() {
        let mut grid = Grid::new();

        let position = GridPosition::new(0, 0);
        let entity = Entity::PLACEHOLDER;
        let rotation = Rotation::default();

        grid.insert(position, entity, rotation);

        assert_eq!(grid.contains(position), true);
    }

    #[test]
    fn test_grid_len() {
        let mut grid = Grid::new();

        let position1 = GridPosition::new(0, 0);
        let entity1 = Entity::PLACEHOLDER;
        let rotation1 = Rotation::default();

        let position2 = GridPosition::new(1, 1);
        let entity2 = Entity::PLACEHOLDER;
        let rotation2 = Rotation::default();

        grid.insert(position1, entity1, rotation1);
        grid.insert(position2, entity2, rotation2);

        assert_eq!(grid.len(), 2);
    }

    #[test]
    fn test_grid_get() {
        let mut grid = Grid::new();

        let position = GridPosition::new(0, 0);
        let entity = Entity::PLACEHOLDER;
        let rotation = Rotation::default();

        grid.insert(position, entity, rotation);

        assert_eq!(grid.get(position), Some(GridEntity { entity, rotation }));
    }

    #[test]
    fn test_grid_insert() {
        let mut grid = Grid::new();

        let position = GridPosition::new(0, 0);
        let entity = Entity::PLACEHOLDER;
        let rotation = Rotation::default();

        grid.insert(position, entity, rotation);

        assert_eq!(grid.get(position), Some(GridEntity { entity, rotation }));
    }

    #[test]
    fn test_grid_remove() {
        let mut grid = Grid::new();

        let position = GridPosition::new(0, 0);
        let entity = Entity::PLACEHOLDER;
        let rotation = Rotation::default();

        grid.insert(position, entity, rotation);

        assert_eq!(grid.remove(position), Some(GridEntity { entity, rotation }));
    }
}
