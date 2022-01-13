# 依然是静态画面
1. 添加资源(Resource)
   新建`data.rs`并添加
   ``` Rust
   #[derive(PartialEq)]
   pub enum GameState {
       READY,           // 等待开始
       PLAYING,         // 游戏中
       FINISH(bool),    // 游戏结束(bool表示成功失败)
   }

   pub struct RestMine {
       pub count: i32,  // 剩余雷数
   }

   pub struct GameTimer {
       pub timer: f32,  // 游戏时间
   }

   ```
2. 添加组件(Component)
   新建`components`目录并分别添加`reset_btn.rs`,`rest_mine_num.rs`,`timer_num.rs`,`cell.rs`和对应导出
   ``` Rust
   // reset_btn.rs
   pub struct ResetBtn {
       pub click_down: bool,  //是否点击中
   }

   impl Component for ResetBtn {
       type Storage = DenseVecStorage<Self>;
   }

   // reset_mine_num.rs
   pub struct RestMineNum {
       pub index: u32,  // 对应显示的位
   }

   impl Component for RestMineNum {
       type Storage = VecStorage<Self>;
   }

   // timer_num.rs
   pub struct TimerNum {
       pub index: u32,  // 对应显示的位
   }

   impl Component for TimerNum {
       type Storage = DenseVecStorage<Self>;
   }


   // cell.rs
   #[derive(PartialEq)]
   pub enum CellState {
       HIDE,  // 未显示
       SHOW,  // 显示
       FLAG,  // 插旗
   }
   pub struct Cell {
       pub has_mine: bool,            // 是否有雷
       pub state: CellState,          // 当前状态
       pub around: Vec<Entity>,       // 周围的格子
       pub click_down: bool,          // 是否点击中
       pub around_mine_count: usize,  // 缓存,周围雷的数量
   }

   impl Component for Cell {
       type Storage = VecStorage<Self>;
   }
   ```
   注意,其中`Component`的实现需要定义`Storage`的类型,对应的是`Component`在内存中的储存方式,具体可以参照[此处](https://book.amethyst.rs/book/stable/concepts/entity_and_component)
3. 添加系统(system)
   新建`systems`目录并分别添加`render_reset_btn.rs`,`render_rest_mine.rs`,`render_timer.rs`,`render_cell.rs`和对应导出
   ``` Rust
   // render_reset_btn.rs
   pub struct RenderResetBtnSystem;

   impl<'s> System<'s> for RenderResetBtnSystem {
       type SystemData = (
           ReadStorage<'s, crate::ResetBtn>,
           WriteStorage<'s, UiImage>,
           ReadExpect<'s, crate::GameState>,
           ReadExpect<'s, crate::BtnTextures>,
       );

       fn run(&mut self, (reset_btns, mut ui_images, game_state, btn_textures): Self::SystemData) {
           for (reset_btn, ui_image) in (&reset_btns, &mut ui_images).join() {
               if reset_btn.click_down {
                   *ui_image = UiImage::Texture(btn_textures.waiting.clone());
               } else if crate::GameState::FINISH(false) == *game_state {
                   *ui_image = UiImage::Texture(btn_textures.fail.clone());
               } else {
                   *ui_image = UiImage::Texture(btn_textures.ok.clone());
               }
           }
       }
   }

   // render_rest_mine.rs
   pub struct RenderRestMineSystem;

   impl<'s> System<'s> for RenderRestMineSystem {
       type SystemData = (
           ReadStorage<'s, crate::RestMineNum>,
           WriteStorage<'s, UiImage>,
           ReadExpect<'s, crate::RestMine>,
           ReadExpect<'s, crate::NumberTextures>,
       );

       fn run(
           &mut self,
           (rest_mine_nums, mut ui_images, rest_mine, number_textures): Self::SystemData,
       ) {
           let show_count = rest_mine.count.max(0).min(999) as u32;
           for (rest_mine_num, ui_image) in (&rest_mine_nums, &mut ui_images).join() {
               *ui_image = UiImage::Texture(
                   number_textures.textures
                       [(show_count / 10u32.pow(rest_mine_num.index) % 10) as usize]
                       .clone(),
               )
           }
       }
   }

   // render_timer.rs
   pub struct RenderTimerSystem;

   impl<'s> System<'s> for RenderTimerSystem {
       type SystemData = (
           ReadStorage<'s, crate::TimerNum>,
           WriteStorage<'s, UiImage>,
           ReadExpect<'s, crate::GameTimer>,
           ReadExpect<'s, crate::NumberTextures>,
       );

       fn run(&mut self, (timer_nums, mut ui_images, game_timer, number_textures): Self::SystemData) {
           let show_count = (game_timer.timer as u32).min(999);
           for (rest_mine_num, ui_image) in (&timer_nums, &mut ui_images).join() {
               *ui_image = UiImage::Texture(
                   number_textures.textures
                       [(show_count / 10u32.pow(rest_mine_num.index) % 10) as usize]
                       .clone(),
               )
           }
       }
   }

   // render_cell.rs
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

   ```
   其中`SystemData`表示系统需要访问的数据,其中`ReadStorage`/`WriteStorage`表示对组件(component)的只读/读写访问,`ReadExpect`/`WriteExpect`表示对资源(resource)的只读/读写访问(如果资源(resource)实现`Default`的话,可以使用`Read/Write`进行只读/读写访问)
   剩余的就是根据数据替换对应的图片
4. 初始化数据
   1. `lib.rs`中添加`pub const MINE_COUNT: u32 = 15;`表示游戏中生成雷的数量
   2. `mine.rs`中添加新增资源(resource)
      ``` Rust
      fn on_start(..){
          // ..

          world.insert(crate::GameState::READY);
          world.insert(crate::RestMine {
              count: crate::MINE_COUNT as i32,
          });
          world.insert(crate::RestArea { count: 0 });
          world.insert(crate::GameTimer { timer: 0.0 });

          // ..
      }
      ```
   3. `mine.rs`中添加新增组件(Component)
      ``` Rust
      // ..
      .with(UiTransform::new(
          "face-btn".to_string(),
          // ..
      ))
      // ..
      .with(crate::ResetBtn { click_down: false })
      .build();
      // ..
      for i in 0..3 {
        // ..
        .with(UiTransform::new(
            format!("mine-count-{}", i),
            // ..
        ))
        // ..
        .with(crate::RestMineNum { index: 2 - i })
        .build();
      }
      // ..
      for i in 0..3 {
        // ..
        .with(UiTransform::new(
            format!("time-count-{}", i),
            // ..
        ))
        // ..
        .with(crate::RestMineNum { index: 2 - i })
        .build();
      }
      // ..
      ```
      其中`Cell`的初始化比较复杂,因为需要储存周围的实体(entity),所以只能先创建实体后插入该组件
      ``` Rust
      // ..
      let mut cells = vec![];
      for i in 0..crate::CELL_ROW {
          let mut row = vec![];
          for j in 0..crate::CELL_COL {
              row.push(
                // ..
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
      ```

5. 程序初始化中添加系统
   ``` Rust
   // ..
   let game_data = GameDataBuilder::default()
       // ..
       .with(miner::RenderCellSystem, "render_cell_system", &[])
       .with(miner::RenderResetBtnSystem, "render_reset_btn_system", &[])
       .with(miner::RenderRestMineSystem, "render_rest_mine_system", &[])
       .with(miner::RenderTimerSystem, "render_timer_system", &[]);
   let mut game = Application::new(assets_root, miner::Mine, game_data)?;
   // ..
   ```
   其中`with`第一个参数是系统,第二个是系统名称,第三个是系统依赖,即在系统中调用顺寻,目前由于还没有修改数据的系统,所以依赖都为空

6. 完成
   这样就基本完成了,可以尝试下修改初始化数据来看不同的画面