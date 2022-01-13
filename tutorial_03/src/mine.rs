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
            .with(crate::ResetBtn { click_down: false })
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
                .with(crate::RestMineNum { index: 2 - i })
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
                .with(crate::TimerNum { index: i })
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

        let mut cells = vec![];
        for i in 0..crate::CELL_ROW {
            let mut row = vec![];
            for j in 0..crate::CELL_COL {
                row.push(
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
                        .build(),
                );
            }
            cells.push(row);
        }
        {
            let mut cell_components = world.write_storage::<crate::Cell>();
            for i in 0..crate::CELL_ROW {
                for j in 0..crate::CELL_COL {
                    let mut around = vec![];
                    let start_i = if i > 0 { i - 1 } else { 0 };
                    let end_i = if i < crate::CELL_ROW - 1 {
                        i + 1
                    } else {
                        crate::CELL_ROW - 1
                    };
                    let start_j = if j > 0 { j - 1 } else { 0 };
                    let end_j = if j < crate::CELL_COL - 1 {
                        j + 1
                    } else {
                        crate::CELL_COL - 1
                    };
                    for around_i in start_i..=end_i {
                        for around_j in start_j..=end_j {
                            if around_i != i || around_j != j {
                                around.push(cells[around_i][around_j])
                            }
                        }
                    }
                    cell_components
                        .insert(
                            cells[i][j],
                            crate::Cell {
                                has_mine: false,
                                state: crate::CellState::HIDE,
                                around,
                                click_down: false,
                                around_mine_count: 0,
                            },
                        )
                        .unwrap();
                }
            }
        }
        world.insert(btn_textures);
        world.insert(cell_textures);
        world.insert(number_textures);
        world.insert(crate::GameState::READY);
        world.insert(crate::RestMine {
            count: crate::MINE_COUNT as i32,
        });
        world.insert(crate::GameTimer { timer: 0.0 });
    }
}
