use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    prelude::*,
    renderer::{ImageFormat, Texture},
};
pub struct CellTextures {
    pub counts: Vec<Handle<Texture>>,
    pub normal: Handle<Texture>,
    pub blood: Handle<Texture>,
    pub error: Handle<Texture>,
    pub mine: Handle<Texture>,
    pub flag: Handle<Texture>,
}

pub fn load_cell_textures(world: &mut World) -> CellTextures {
    let loader = world.read_resource::<Loader>();
    let mut counts = vec![];
    for i in 0..=8 {
        counts.push(loader.load(
            format!("cell-{}.png", i),
            ImageFormat::default(),
            (),
            &world.read_resource::<AssetStorage<Texture>>(),
        ));
    }
    CellTextures {
        counts,
        normal: loader.load(
            "cell.png",
            ImageFormat::default(),
            (),
            &world.read_resource::<AssetStorage<Texture>>(),
        ),
        blood: loader.load(
            "cell-blood.png",
            ImageFormat::default(),
            (),
            &world.read_resource::<AssetStorage<Texture>>(),
        ),
        error: loader.load(
            "cell-error.png",
            ImageFormat::default(),
            (),
            &world.read_resource::<AssetStorage<Texture>>(),
        ),
        mine: loader.load(
            "cell-mine.png",
            ImageFormat::default(),
            (),
            &world.read_resource::<AssetStorage<Texture>>(),
        ),
        flag: loader.load(
            "cell-flag.png",
            ImageFormat::default(),
            (),
            &world.read_resource::<AssetStorage<Texture>>(),
        ),
    }
}
