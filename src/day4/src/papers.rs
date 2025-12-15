use strum::IntoEnumIterator;
use crate::direction::Direction;

const ACCESSIBLE_LIMIT: usize = 4;
pub enum EntityType {
    Paper,
    Empty,
}

pub struct Entity {
    pub(crate) entity_type: EntityType,
    pub(crate) x: usize,
    pub(crate) y: usize,
}

impl Entity {
    pub(crate) fn is_empty(&self) -> bool {
        matches!(self.entity_type, EntityType::Empty)
    }

    pub fn check_freedom(&self, matrix:&Vec<Vec<Entity>>) -> bool {
        let count = Direction::iter().map(|dir| self.check_direction(dir, matrix))
            .filter(|&b| b)
            .count();
        count > ACCESSIBLE_LIMIT
    }

    fn check_direction(&self, direction:Direction, matrix:&Vec<Vec<Entity>>) -> bool {
        let (dx, dy) = direction.movement();
        let max_x = matrix[0].len() as isize;
        let max_y = matrix.len() as isize;
        let nx= self.x as isize + dx;
        let ny = self.y as isize + dy;
        let fx = match nx {
            ..=-1 => return true,
            n if n >= max_x => return true,
            _ => nx as usize,
        };
        let fy = match ny {
            ..=-1 => return true,
            n if n >= max_y => return true,
            _ => ny as usize,
        };
        matrix[fy][fx].is_empty()
    }
}


pub fn from_char(c: char, x:usize, y:usize) -> Entity {
    let entity_type = match c {
        '.' => EntityType::Empty,
        '@' => EntityType::Paper,
        _ => EntityType::Empty,
    };
    Entity { entity_type, x, y}
}


