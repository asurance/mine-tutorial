use amethyst::prelude::*;
pub struct Mine;

impl SimpleState for Mine {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let btn_textures = crate::load_btn_textures(world);
        let cell_textures = crate::load_cell_textures(world);
        let number_textures = crate::load_number_textures(world);
        world.insert(btn_textures);
        world.insert(cell_textures);
        world.insert(number_textures);
        println!("load finish")
    }
}
