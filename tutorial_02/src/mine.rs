use amethyst::{
    core::Parent,
    prelude::*,
    ui::{Anchor, UiImage, UiTransform},
    window::ScreenDimensions,
};
pub struct Mine;

impl SimpleState for Mine {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let btn_textures = crate::load_btn_textures(world);
        let cell_textures = crate::load_cell_textures(world);
        let number_textures = crate::load_number_textures(world);
        let dpi = {
            let dimensions = world.read_resource::<ScreenDimensions>();
            dimensions.hidpi_factor()
        } as f32;
        let cell_width = crate::CELL_WIDTH as f32 * dpi;
        let cell_height = crate::CELL_HEIGHT as f32 * dpi;
        let header_height = crate::HEADER_HEIGHT as f32 * dpi;
        let btn_width = crate::BTN_WIDTH as f32 * dpi;
        let btn_height = crate::BTN_HEIGHT as f32 * dpi;
        let number_width = crate::NUMBER_WIDTH as f32 * dpi;
        let number_height = crate::NUMBER_HEIGHT as f32 * dpi;
        let header_group = world
            .create_entity()
            .with(UiTransform::new(
                "header-group".to_string(),
                Anchor::BottomLeft,
                Anchor::BottomLeft,
                0.0,
                cell_height * crate::CELL_ROW as f32,
                0.0,
                cell_width * crate::CELL_COL as f32,
                header_height,
            ))
            .build();
        world
            .create_entity()
            .with(UiTransform::new(
                "face-btn".to_string(),
                Anchor::Middle,
                Anchor::Middle,
                0.0,
                0.0,
                0.0,
                btn_width,
                btn_height,
            ))
            .with(UiImage::Texture(btn_textures.ok.clone()))
            .with(Parent {
                entity: header_group,
            })
            .build();
        for i in 0..3 {
            world
                .create_entity()
                .with(UiTransform::new(
                    format!("mine-count-{}", i),
                    Anchor::MiddleLeft,
                    Anchor::MiddleLeft,
                    i as f32 * number_width,
                    0.0,
                    0.0,
                    number_width,
                    number_height,
                ))
                .with(UiImage::Texture(number_textures.textures[0usize].clone()))
                .with(Parent {
                    entity: header_group,
                })
                .build();
        }
        for i in 0..3 {
            world
                .create_entity()
                .with(UiTransform::new(
                    format!("time-count-{}", i),
                    Anchor::MiddleRight,
                    Anchor::MiddleRight,
                    i as f32 * -number_width,
                    0.0,
                    0.0,
                    number_width,
                    number_height,
                ))
                .with(UiImage::Texture(number_textures.textures[0usize].clone()))
                .with(Parent {
                    entity: header_group,
                })
                .build();
        }
        let cell_group = world
            .create_entity()
            .with(UiTransform::new(
                "cell-group".to_string(),
                Anchor::BottomLeft,
                Anchor::BottomLeft,
                0.0,
                0.0,
                0.0,
                cell_width * crate::CELL_COL as f32,
                cell_height * crate::CELL_ROW as f32,
            ))
            .build();

        for i in 0..crate::CELL_ROW {
            for j in 0..crate::CELL_COL {
                world
                    .create_entity()
                    .with(UiTransform::new(
                        format!("cell-{}-{}", i, j),
                        Anchor::BottomLeft,
                        Anchor::BottomLeft,
                        j as f32 * cell_width,
                        i as f32 * cell_height,
                        0.0,
                        cell_width,
                        cell_height,
                    ))
                    .with(UiImage::Texture(cell_textures.normal.clone()))
                    .with(Parent { entity: cell_group })
                    .build();
            }
        }
        world.insert(btn_textures);
        world.insert(cell_textures);
        world.insert(number_textures);
    }
}
