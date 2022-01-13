use amethyst::ecs::{Component, DenseVecStorage};

pub struct TimerNum {
    pub index: u32,
}

impl Component for TimerNum {
    type Storage = DenseVecStorage<Self>;
}
