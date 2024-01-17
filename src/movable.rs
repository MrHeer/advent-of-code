use crate::{Direction, Position};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Movable {
    pub position: Position,
    pub direction: Direction,
}

impl Movable {
    pub fn new(position: Position, direction: Direction) -> Self {
        Self {
            position,
            direction,
        }
    }

    pub fn turn_to(&mut self, direction: Direction) -> &Self {
        self.direction = direction;
        self
    }

    pub fn turn_left(&mut self) -> &mut Self {
        self.direction = self.direction.turn_left();
        self
    }

    pub fn turn_right(&mut self) -> &mut Self {
        self.direction = self.direction.turn_right();
        self
    }

    pub fn turn_back(&mut self) -> &mut Self {
        self.direction = self.direction.turn_back();
        self
    }

    pub fn move_forward(&mut self) -> &mut Self {
        self.position.move_to(&self.direction);
        self
    }

    pub fn move_left(&mut self) -> &mut Self {
        self.turn_left().move_forward()
    }

    pub fn move_right(&mut self) -> &mut Self {
        self.turn_right().move_forward()
    }

    pub fn move_back(&mut self) -> &mut Self {
        self.turn_back().move_forward()
    }
}
