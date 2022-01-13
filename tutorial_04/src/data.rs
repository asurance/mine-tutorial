#[derive(PartialEq, Clone, Copy)]
pub enum GameState {
    READY,
    PLAYING,
    FINISH(bool),
}

pub struct RestMine {
    pub count: i32,
}

pub struct GameTimer {
    pub timer: f32,
}
