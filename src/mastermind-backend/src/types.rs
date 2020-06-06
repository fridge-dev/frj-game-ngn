use std::time::{Instant, Duration};

/// Allow variable-number of active colors in a game, up to 256.
/// Reserve 0x00 for Option::None.
pub type Color = u8;
pub const NO_COLOR: u8 = 0;
pub const NUM_PEGS_PER_ROW: usize = 4; // TODO:2 variable peg size

// ------------- struct -------------

#[derive(Default)]
pub struct ResultPegs {
    pub correct: u8,
    pub correct_color_wrong_slot: u8,
}

#[derive(Clone)]
pub struct Row {
    pegs: [Color; NUM_PEGS_PER_ROW],
    // TODO:2 move out
    max_color: Color,
}

pub struct PreparingBoard {
    // Will potentially have pegs set to `None`
    pub sparse_password: Row,
    pub ready: bool,
}

pub struct ActiveBoard {
    // Head => first guess
    // Tail => recent guess
    pub completed_rows: Vec<(Row, ResultPegs)>,
    pub current_guess: Row,
    pub password_to_guess: Row,
    start_time: Instant,
}

pub struct CompletedBoard {
    // Head => first guess
    // Tail => correct guess
    completed_rows: Vec<(Row, ResultPegs)>,
    completion_timing: Duration,
}

pub struct Players {
    pub ids: (String, String),
    pub as_vec: Vec<String>,
}

#[derive(Copy, Clone, PartialEq)]
pub enum PlayerSide {
    Left,
    Right,
}

pub enum AppError {
    InvalidInput( /* message */ &'static str),
}

// ------------- impl -------------

impl Row {
    pub fn new(num_colors: u8) -> Self {
        Row {
            pegs: [NO_COLOR, NO_COLOR, NO_COLOR, NO_COLOR],
            max_color: num_colors,
        }
    }

    pub fn try_set(&mut self, peg: usize, color: Color) -> Result<(), AppError> {
        if peg > NUM_PEGS_PER_ROW {
            return Err(AppError::InvalidInput("Peg is out of bounds"));
        }

        if color > self.max_color {
            return Err(AppError::InvalidInput("Color is out of bounds"));
        }

        self.pegs[peg] = color;
        Ok(())
    }

    pub fn is_complete(&self) -> bool {
        for peg in 0..NUM_PEGS_PER_ROW {
            if self.pegs[peg] == NO_COLOR {
                return false;
            }
        }
        true
    }

    pub fn len(&self) -> usize {
        self.pegs.len()
    }

    pub fn peg(&self, index: usize) -> Color {
        self.pegs[index]
    }
}

impl PreparingBoard {
    pub fn new(num_colors: u8) -> Self {
        PreparingBoard {
            sparse_password: Row::new(num_colors),
            ready: false
        }
    }
}

impl ActiveBoard {
    pub fn new(password: Row) -> Self {
        ActiveBoard {
            completed_rows: Vec::new(),
            current_guess: Row::new(password.max_color),
            password_to_guess: password,
            start_time: Instant::now(),
        }
    }
}

impl CompletedBoard {
    pub fn new(completed_rows: Vec<(Row, ResultPegs)>, completion_timing: Duration) -> Self {
        CompletedBoard {
            completed_rows,
            completion_timing,
        }
    }
}

impl From<ActiveBoard> for CompletedBoard {
    fn from(active_board: ActiveBoard) -> Self {
        CompletedBoard::new(
            active_board.completed_rows,
            Instant::now().saturating_duration_since(active_board.start_time),
        )
    }
}

impl Players {
    pub fn new(player_id_1: String, player_id_2: String) -> Self {
        Players {
            ids: (player_id_1.clone(), player_id_2.clone()),
            as_vec: vec![player_id_1, player_id_2],
        }
    }

    pub fn get_side(&self, client_player_id: &String) -> Option<PlayerSide> {
        if client_player_id == &self.ids.0 {
            Some(PlayerSide::Left)
        } else if client_player_id == &self.ids.1 {
            Some(PlayerSide::Right)
        } else {
            None
        }
    }
}