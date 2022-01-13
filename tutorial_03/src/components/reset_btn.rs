use amethyst::ecs::{Component, DenseVecStorage};

pub struct ResetBtn {
    pub click_down: bool,
}

impl Component for ResetBtn {
    type Storage = DenseVecStorage<Self>;
}
