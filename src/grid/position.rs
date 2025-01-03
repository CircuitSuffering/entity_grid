use bevy::prelude::*;

pub mod prelude {
    pub use super::GridPosition;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Component)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

impl GridPosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl From<IVec2> for GridPosition {
    fn from(vec: IVec2) -> Self {
        Self { x: vec.x, y: vec.y }
    }
}

impl std::ops::Add<GridPosition> for GridPosition {
    type Output = Self;

    fn add(self, rhs: GridPosition) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Add<IVec2> for GridPosition {
    type Output = Self;

    fn add(self, rhs: IVec2) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let pos1 = GridPosition::new(1, 2);
        let pos2 = GridPosition::new(3, 4);
        let result = pos1 + pos2;
        assert_eq!(result, GridPosition::new(4, 6));
    }

    #[test]
    fn test_add_ivec2() {
        let pos = GridPosition::new(1, 2);
        let vec = IVec2::new(3, 4);
        let result = pos + vec;
        assert_eq!(result, GridPosition::new(4, 6));
    }
}
