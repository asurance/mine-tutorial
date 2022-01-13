use amethyst::ecs::{Component, Entity, VecStorage};

#[derive(PartialEq)]
pub enum CellState {
    HIDE,
    SHOW,
    FLAG,
}
pub struct Cell {
    pub has_mine: bool,
    pub state: CellState,
    pub around: Vec<Entity>,
    pub click_down: bool,
    pub around_mine_count: usize,
}

impl Component for Cell {
    type Storage = VecStorage<Self>;
}
