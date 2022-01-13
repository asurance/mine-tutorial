use amethyst::{
    ecs::{Join, ReadExpect, ReadStorage, System, WriteStorage},
    ui::UiImage,
};
pub struct RenderRestMineSystem;

impl<'s> System<'s> for RenderRestMineSystem {
    type SystemData = (
        ReadStorage<'s, crate::RestMineNum>,
        WriteStorage<'s, UiImage>,
        ReadExpect<'s, crate::RestMine>,
        ReadExpect<'s, crate::NumberTextures>,
    );

    fn run(
        &mut self,
        (rest_mine_nums, mut ui_images, rest_mine, number_textures): Self::SystemData,
    ) {
        let show_count = rest_mine.count.max(0).min(999) as u32;
        for (rest_mine_num, ui_image) in (&rest_mine_nums, &mut ui_images).join() {
            *ui_image = UiImage::Texture(
                number_textures.textures
                    [(show_count / 10u32.pow(rest_mine_num.index) % 10) as usize]
                    .clone(),
            )
        }
    }
}
