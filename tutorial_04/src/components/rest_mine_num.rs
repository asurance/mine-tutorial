use amethyst::ecs::{Component, VecStorage};

pub struct RestMineNum {
    pub index: u32,
}

impl Component for RestMineNum {
    type Storage = VecStorage<Self>;
}
