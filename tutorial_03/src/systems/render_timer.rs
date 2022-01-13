use amethyst::{
    ecs::{Join, ReadExpect, ReadStorage, System, WriteStorage},
    ui::UiImage,
};

pub struct RenderTimerSystem;

impl<'s> System<'s> for RenderTimerSystem {
    type SystemData = (
        ReadStorage<'s, crate::TimerNum>,
        WriteStorage<'s, UiImage>,
        ReadExpect<'s, crate::GameTimer>,
        ReadExpect<'s, crate::NumberTextures>,
    );

    fn run(&mut self, (timer_nums, mut ui_images, game_timer, number_textures): Self::SystemData) {
        let show_count = (game_timer.timer as u32).min(999);
        for (rest_mine_num, ui_image) in (&timer_nums, &mut ui_images).join() {
            *ui_image = UiImage::Texture(
                number_textures.textures
                    [(show_count / 10u32.pow(rest_mine_num.index) % 10) as usize]
                    .clone(),
            )
        }
    }
}
