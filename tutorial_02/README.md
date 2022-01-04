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
   1. 在`lib.rs`中添加常量
      ``` Rust
      // 背景颜色
      pub const BACKGROUND_COLOR: [f32; 4] = [117.0 / 255.0, 117.0 / 255.0, 117.0 / 255.0, 1.0];
      // 格子行数,列数,宽高
      pub const CELL_ROW: usize = 15;
      pub const CELL_COL: usize = 10;
      pub const CELL_WIDTH: u32 = 25;
      pub const CELL_HEIGHT: u32 = 25;
      // 顶上UI区域高度
      pub const HEADER_HEIGHT: u32 = 40;
      // 笑脸按钮宽高
      pub const BTN_WIDTH: u32 = 20;
      pub const BTN_HEIGHT: u32 = 20;
      // 数字宽高
      pub const NUMBER_WIDTH: u32 = 15;
      pub const NUMBER_HEIGHT: u32 = 25;
      ```
   2. 修改`main.rs`中system设置
      ``` Rust
      // ...
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
         .with_bundle(UiBundle::<StringBindings>::new())?;
      // ...
      ```
      本项目中图片使用UiImage显示,所以需要添加UI相关内容,同时显示也需要将窗口大小锁死
   3. 最后在`mine.rs`的`on_start`中添加数据即可
      ``` Rust
      // ...
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
      // ...
      ```
      其中dpi对应的windows系统中的显示的缩放比例,其他系统的暂时没有考证.
      至于UiTransform,首先坐标系是以右为x正方向,上为y正方向,第一个锚点指的父节点(没有则为整个屏幕),第二个锚点指的自身,可以理解为父节点锚点计算出的位置+自身的x,y = 自身锚点的位置