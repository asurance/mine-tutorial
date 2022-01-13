use amethyst::{
    ecs::{Join, ReadExpect, ReadStorage, System, WriteStorage},
    ui::UiImage,
};

pub struct RenderCellSystem;

impl<'s> System<'s> for RenderCellSystem {
    type SystemData = (
        ReadStorage<'s, crate::Cell>,
        WriteStorage<'s, UiImage>,
        ReadExpect<'s, crate::GameState>,
        ReadExpect<'s, crate::CellTextures>,
    );

    fn run(&mut self, (cells, mut ui_images, game_state, cell_textures): Self::SystemData) {
        if crate::GameState::FINISH(false) == *game_state {
            for (cell, ui_image) in (&cells, &mut ui_images).join() {
                if cell.click_down {
                    *ui_image = UiImage::Texture(cell_textures.blood.clone());
                } else {
                    *ui_image = UiImage::Texture(match cell.state {
                        crate::CellState::HIDE => {
                            if cell.has_mine {
                                cell_textures.mine.clone()
                            } else {
                                cell_textures.normal.clone()
                            }
                        }
                        crate::CellState::SHOW => {
                            cell_textures.counts[cell.around_mine_count].clone()
                        }
                        crate::CellState::FLAG => {
                            if cell.has_mine {
                                cell_textures.flag.clone()
                            } else {
                                cell_textures.error.clone()
                            }
                        }
                    })
                }
            }
        } else {
            for (cell, ui_image) in (&cells, &mut ui_images).join() {
                if cell.click_down {
                    *ui_image = UiImage::Texture(cell_textures.counts[0].clone());
                } else {
                    *ui_image = UiImage::Texture(match cell.state {
                        crate::CellState::HIDE => cell_textures.normal.clone(),
                        crate::CellState::SHOW => {
                            cell_textures.counts[cell.around_mine_count].clone()
                        }
                        crate::CellState::FLAG => cell_textures.flag.clone(),
                    })
                }
            }
        }
    }
}
