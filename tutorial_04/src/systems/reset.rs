use amethyst::ecs::{Join, ReadExpect, System, WriteExpect, WriteStorage};

pub struct ResetSystem {
    last_state: crate::GameState,
}

impl ResetSystem {
    pub fn new() -> Self {
        ResetSystem {
            last_state: crate::GameState::READY,
        }
    }
}
impl<'s> System<'s> for ResetSystem {
    type SystemData = (
        ReadExpect<'s, crate::GameState>,
        WriteExpect<'s, crate::GameTimer>,
        WriteExpect<'s, crate::RestMine>,
        WriteStorage<'s, crate::Cell>,
    );
    fn run(&mut self, (game_state, mut game_timer, mut rest_mine, mut cells): Self::SystemData) {
        if *game_state != self.last_state {
            if *game_state == crate::GameState::READY {
                game_timer.timer = 0.0;
                rest_mine.count = crate::MINE_COUNT as i32;
                for cell in (&mut cells).join() {
                    cell.state = crate::CellState::HIDE;
                    cell.click_down = false;
                }
            }
            self.last_state = *game_state;
        }
    }
}
