use amethyst::{
    core::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{types::DefaultBackend, RenderToWindow, RenderingBundle},
    ui::{RenderUi, UiBundle},
    window::DisplayConfig,
    GameDataBuilder,
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let app_root = amethyst::utils::application_root_dir()?;
    let assets_root = app_root.join("assets");
    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config(DisplayConfig {
                        title: "扫雷".to_string(),
                        dimensions: Some((
                            miner::CELL_WIDTH * miner::CELL_COL as u32,
                            miner::CELL_HEIGHT * miner::CELL_ROW as u32 + miner::HEADER_HEIGHT,
                        )),
                        resizable: false,
                        ..Default::default()
                    })
                    .with_clear(miner::BACKGROUND_COLOR),
                )
                .with_plugin(RenderUi::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<StringBindings>::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with(miner::RenderCellSystem, "render_cell_system", &[])
        .with(miner::RenderResetBtnSystem, "render_reset_btn_system", &[])
        .with(miner::RenderRestMineSystem, "render_rest_mine_system", &[])
        .with(miner::RenderTimerSystem, "render_timer_system", &[]);
    let mut game = Application::new(assets_root, miner::Mine, game_data)?;
    game.run();
    Ok(())
}
