# 加载图片和显示静态画面
1. 准备资源
   自己绘制或是使用[我项目中的资源](https://github.com/asurance/mine-tutorial/tree/master/tutorial_02/assets)
2. 加载图片
   1. 添加`textures`目录,添加对应`btn.rs`,`cell.rs`,`number.rs`,此处以`cell.rs`为例
      ``` Rust  
      // 定义一个struct作为Resource存放加载图片的句柄
      pub struct CellTextures {
         // 不同数字的图片直接用Vec存方便到时候取
         pub counts: Vec<Handle<Texture>>,
         pub normal: Handle<Texture>,
         pub blood: Handle<Texture>,
         pub error: Handle<Texture>,
         pub mine: Handle<Texture>,
         pub flag: Handle<Texture>,
      }

      pub fn load_cell_textures(world: &mut World) {
         // 获取资源Loader后,根据路径逐个加载资源即可
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
      ```
      用类似的办法添加`btn.rs`和`number.rs`,这边就不再赘述了
   2. `textures`下添加`mod.rs`,方便之后引用
      ``` Rust
      mod btn;
      mod cell;
      mod number;

      pub use btn::{load_btn_textures, BtnTextures};
      pub use cell::{load_cell_textures, CellTextures};
      pub use number::{load_number_textures, NumberTextures};
      ```
   3. 添加`lib.rs`
      ``` Rust
      mod textures;

      pub use textures::*;
      ``` 
   4. 将原来的Mine状态迁移至新的`mine.rs`,并添加on_start方法的实现
      ``` Rust
      impl SimpleState for Mine {
         fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
            let world = data.world;
            // 调用之前写好
            let btn_textures = crate::load_btn_textures(world);
            let cell_textures = crate::load_cell_textures(world);
            let number_textures = crate::load_number_textures(world);
            // 将resource放入world
            world.insert(btn_textures);
            world.insert(cell_textures);
            world.insert(number_textures);
         }
      }
      ```
   5. 在`lib.rs`中补充`Mine`相关引用,将`main.rs`中`Mine`改为`miner::Mine`
   6. 这时候运行控制台没报错,则说明图面加载路径没问题
3. 显示静态画面