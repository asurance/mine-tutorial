use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    prelude::*,
    renderer::{ImageFormat, Texture},
};
pub struct NumberTextures {
    pub textures: Vec<Handle<Texture>>,
}

pub fn load_number_textures(world: &mut World) -> NumberTextures {
    let loader = world.read_resource::<Loader>();
    let mut textures = vec![];
    for i in 0..=9 {
        textures.push(loader.load(
            format!("number-{}.png", i),
            ImageFormat::default(),
            (),
            &world.read_resource::<AssetStorage<Texture>>(),
        ))
    }
    NumberTextures { textures }
}
