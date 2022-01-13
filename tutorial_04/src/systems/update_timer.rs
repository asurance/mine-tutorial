use amethyst::{
    core::Time,
    ecs::{Read, ReadExpect, System, WriteExpect},
};

pub struct UpdateTimerSystem;

impl<'s> System<'s> for UpdateTimerSystem {
    type SystemData = (
        ReadExpect<'s, crate::GameState>,
        WriteExpect<'s, crate::GameTimer>,
        Read<'s, Time>,
    );
    fn run(&mut self, (game_state, mut game_timer, time): Self::SystemData) {
        if *game_state == crate::GameState::PLAYING {
            game_timer.timer += time.delta_seconds()
        }
    }
}
