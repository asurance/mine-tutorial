use amethyst::{
    renderer::{types::DefaultBackend, RenderToWindow, RenderingBundle},
    window::DisplayConfig,
    Application, GameDataBuilder, SimpleState,
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let app_root = amethyst::utils::application_root_dir()?;
    let assets_root = app_root.join("assets");
    let game_data = GameDataBuilder::default().with_bundle(
        RenderingBundle::<DefaultBackend>::new().with_plugin(
            RenderToWindow::from_config(DisplayConfig {
                title: "扫雷".to_string(),
                dimensions: Some((800, 600)),
                ..Default::default()
            })
            .with_clear([0.0, 0.0, 0.0, 1.0]),
        ),
    )?;
    let mut game = Application::new(assets_root, Mine, game_data)?;
    game.run();
    Ok(())
}

struct Mine;

impl SimpleState for Mine {}
