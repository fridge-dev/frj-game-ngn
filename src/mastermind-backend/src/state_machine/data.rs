use crate::types::{PreparingBoard, PlayerSide, ActiveBoard, CompletedBoard};

// ---------------- struct ----------------

pub struct PregameData {
    pub left: PreparingBoard,
    pub right: PreparingBoard,
}

pub struct ActiveData {
    pub left: ActiveBoard,
    pub right: ActiveBoard,
}

pub struct LActiveRDoneData {
    pub left: ActiveBoard,
    pub right: CompletedBoard,
}

pub struct LDoneRActiveData {
    pub left: CompletedBoard,
    pub right: ActiveBoard,
}

pub struct DoneData {
    pub left: CompletedBoard,
    pub right: CompletedBoard,
}

// ---------------- impl ----------------

impl PregameData {
    pub fn new(num_colors: u8) -> Self {
        PregameData {
            left: PreparingBoard::new(num_colors),
            right: PreparingBoard::new(num_colors),
        }
    }

    pub fn my_board_mut(&mut self, me: PlayerSide) -> &mut PreparingBoard {
        match me {
            PlayerSide::Left => &mut self.left,
            PlayerSide::Right => &mut self.right,
        }
    }

    pub fn op_board(&self, me: PlayerSide) -> &PreparingBoard {
        match me {
            PlayerSide::Left => &self.right,
            PlayerSide::Right => &self.left,
        }
    }
}

impl ActiveData {
    pub fn my_board_mut(&mut self, me: PlayerSide) -> &mut ActiveBoard {
        match me {
            PlayerSide::Left => &mut self.left,
            PlayerSide::Right => &mut self.right,
        }
    }
}

impl DoneData {
    pub fn my_board(&self, me: PlayerSide) -> &CompletedBoard {
        match me {
            PlayerSide::Left => &self.left,
            PlayerSide::Right => &self.right,
        }
    }

    pub fn op_board(&self, me: PlayerSide) -> &CompletedBoard {
        match me {
            PlayerSide::Left => &self.right,
            PlayerSide::Right => &self.left,
        }
    }
}

mod type_converters {
    use crate::state_machine::data::{ActiveData, PregameData};
    use crate::types::ActiveBoard;

    impl From<PregameData> for ActiveData {
        fn from(data: PregameData) -> Self {
            ActiveData {
                // Left player gets right's password
                left: ActiveBoard::new(data.right.sparse_password),
                // Right player gets left's password
                right: ActiveBoard::new(data.left.sparse_password),
            }
        }
    }
}
