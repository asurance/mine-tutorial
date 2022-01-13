use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    prelude::*,
    renderer::{ImageFormat, Texture},
};
pub struct BtnTextures {
    pub ok: Handle<Texture>,
    pub fail: Handle<Texture>,
    pub waiting: Handle<Texture>,
}

pub fn load_btn_textures(world: &mut World) -> BtnTextures {
    let loader = world.read_resource::<Loader>();
    BtnTextures {
        ok: loader.load(
            "btn-ok.png",
            ImageFormat::default(),
            (),
            &world.read_resource::<AssetStorage<Texture>>(),
        ),
        fail: loader.load(
            "btn-fail.png",
            ImageFormat::default(),
            (),
            &world.read_resource::<AssetStorage<Texture>>(),
        ),
        waiting: loader.load(
            "btn-waiting.png",
            ImageFormat::default(),
            (),
            &world.read_resource::<AssetStorage<Texture>>(),
        ),
    }
}
