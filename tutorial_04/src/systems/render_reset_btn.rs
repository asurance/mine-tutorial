use amethyst::{
    ecs::{Join, ReadExpect, ReadStorage, System, WriteStorage},
    ui::UiImage,
};

pub struct RenderResetBtnSystem;

impl<'s> System<'s> for RenderResetBtnSystem {
    type SystemData = (
        ReadStorage<'s, crate::ResetBtn>,
        WriteStorage<'s, UiImage>,
        ReadExpect<'s, crate::GameState>,
        ReadExpect<'s, crate::BtnTextures>,
    );

    fn run(&mut self, (reset_btns, mut ui_images, game_state, btn_textures): Self::SystemData) {
        for (reset_btn, ui_image) in (&reset_btns, &mut ui_images).join() {
            if reset_btn.click_down {
                *ui_image = UiImage::Texture(btn_textures.waiting.clone());
            } else if crate::GameState::FINISH(false) == *game_state {
                *ui_image = UiImage::Texture(btn_textures.fail.clone());
            } else {
                *ui_image = UiImage::Texture(btn_textures.ok.clone());
            }
        }
    }
}
