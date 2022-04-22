use std::collections::VecDeque;

use crate::board::Position;

#[derive(Debug)]
pub struct Snake {
    pub segments: VecDeque<Position>,
}

impl Default for Snake {
    fn default() -> Self {
        Self {
            segments: VecDeque::from([
                Position { x: 4, y: 4 },
                Position { x: 3, y: 4 },
            ]),
        }
    }
}
