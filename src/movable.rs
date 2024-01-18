use std::ops::{AddAssign, SubAssign};

use crate::{Direction, Position};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Movable<T: Clone + AddAssign + SubAssign> {
    pub position: Position<T>,
    pub direction: Direction,
}

impl<T: Clone + AddAssign + SubAssign> Movable<T> {
    pub fn new(position: Position<T>, direction: Direction) -> Self {
        Self {
            position,
            direction,
        }
    }

    pub fn turn_to(&mut self, direction: &Direction) -> &mut Self {
        self.direction = *direction;
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

    pub fn move_forward(&mut self, steps: T) -> &mut Self {
        self.position.move_to(&self.direction, steps);
        self
    }

    pub fn move_left(&mut self, steps: T) -> &mut Self {
        self.turn_left().move_forward(steps)
    }

    pub fn move_right(&mut self, steps: T) -> &mut Self {
        self.turn_right().move_forward(steps)
    }

    pub fn move_back(&mut self, steps: T) -> &mut Self {
        self.turn_back().move_forward(steps)
    }
}
