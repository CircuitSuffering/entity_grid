pub mod prelude {
    pub use super::Rotation;
}

pub const EMPTY: Rotation = Rotation::Up;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Rotation {
    Up,
    Down,
    Left,
    Right,
}

impl Default for Rotation {
    fn default() -> Self {
        Self::Up
    }
}

impl Rotation {
    pub fn next(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    pub fn previous(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }

    pub fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    pub fn to_angle(&self) -> f32 {
        match self {
            Self::Up => 0.0,
            Self::Right => 1.5707964,
            Self::Down => 3.1415927,
            Self::Left => 4.712389,
        }
    }

    pub fn random() -> Self {
        match rand::random::<u8>() % 4 {
            0 => Self::Up,
            1 => Self::Right,
            2 => Self::Down,
            3 => Self::Left,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_rotation_next() {
        assert_eq!(Rotation::Up.next(), Rotation::Right);
        assert_eq!(Rotation::Right.next(), Rotation::Down);
        assert_eq!(Rotation::Down.next(), Rotation::Left);
        assert_eq!(Rotation::Left.next(), Rotation::Up);
    }

    #[test]
    fn test_rotation_previous() {
        assert_eq!(Rotation::Up.previous(), Rotation::Left);
        assert_eq!(Rotation::Right.previous(), Rotation::Up);
        assert_eq!(Rotation::Down.previous(), Rotation::Right);
        assert_eq!(Rotation::Left.previous(), Rotation::Down);
    }

    #[test]
    fn test_rotation_opposite() {
        assert_eq!(Rotation::Up.opposite(), Rotation::Down);
        assert_eq!(Rotation::Right.opposite(), Rotation::Left);
        assert_eq!(Rotation::Down.opposite(), Rotation::Up);
        assert_eq!(Rotation::Left.opposite(), Rotation::Right);
    }

    #[test]
    fn test_rotation_to_angle() {
        assert_eq!(Rotation::Up.to_angle(), 0.0);
        assert_eq!(Rotation::Right.to_angle(), 1.5707964);
        assert_eq!(Rotation::Down.to_angle(), 3.1415927);
        assert_eq!(Rotation::Left.to_angle(), 4.712389);
    }
}
