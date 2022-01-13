use amethyst::{
    ecs::{Read, System, WriteExpect, WriteStorage},
    prelude::*,
    shrev::{EventChannel, ReaderId},
    ui::{UiEvent, UiEventType},
};

pub struct ClickResetBtnSystem {
    event_id: ReaderId<UiEvent>,
}

impl ClickResetBtnSystem {
    fn new(event_id: ReaderId<UiEvent>) -> Self {
        ClickResetBtnSystem { event_id }
    }
}

impl<'s> System<'s> for ClickResetBtnSystem {
    type SystemData = (
        Read<'s, EventChannel<UiEvent>>,
        WriteStorage<'s, crate::ResetBtn>,
        WriteExpect<'s, crate::GameState>,
    );

    fn run(&mut self, (event_channel, mut reset_btns, mut game_state): Self::SystemData) {
        for event in event_channel.read(&mut self.event_id) {
            if let Some(reset_btn) = reset_btns.get_mut(event.target) {
                match event.event_type {
                    UiEventType::ClickStart => {
                        reset_btn.click_down = true;
                    }
                    UiEventType::ClickStop => {
                        reset_btn.click_down = false;
                        *game_state = crate::GameState::READY;
                    }
                    _ => (),
                }
            }
        }
    }
}

pub struct ClickResetBtnSystemDesc;
impl<'a, 'b> SystemDesc<'a, 'b, ClickResetBtnSystem> for ClickResetBtnSystemDesc {
    fn build(self, world: &mut World) -> ClickResetBtnSystem {
        let mut event_channel = world.fetch_mut::<EventChannel<UiEvent>>();
        ClickResetBtnSystem::new(event_channel.register_reader())
    }
}
