enum Shape {L, J, T, Z, S, I}

struct Tetromino {
    shape: Shape,
    pos_x: u32,
    pos_y: u32
}

impl Tetromino {
    pub fn new(shape: Shape) -> Self {
        Tetromino {
            shape: shape,
            pos_x: 0,
            pos_y: 0
        }
    }
}