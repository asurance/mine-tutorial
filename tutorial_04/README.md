# 添加交互成为游戏
1. 添加游戏时间更新系统
   `systems`目录下添加`update_timer.rs`和对应导出
   ``` Rust
   pub struct UpdateTimerSystem;

   impl<'s> System<'s> for UpdateTimerSystem {
       type SystemData = (
           ReadExpect<'s, crate::GameState>,
           WriteExpect<'s, crate::GameTimer>,
           Read<'s, Time>,
        );
       fn run(&mut self, (game_state, mut game_timer, time): Self::SystemData) {
           if *game_state == crate::GameState::PLAYING {
               game_timer.timer += time.delta_seconds()
           }
       }
   }
   ```
2. 添加游戏重置系统
   `systems`目录下添加`reset.rs`和对应导出
   ``` Rust
   pub struct ResetSystem {
       last_state: crate::GameState,
   }

   impl ResetSystem {
       pub fn new() -> Self {
           ResetSystem {
               last_state: crate::GameState::READY,
           }
       }
   }
   impl<'s> System<'s> for ResetSystem {
       type SystemData = (
           ReadExpect<'s, crate::GameState>,
           WriteExpect<'s, crate::GameTimer>,
           WriteExpect<'s, crate::RestMine>,
           WriteStorage<'s, crate::Cell>,
       );
       fn run(&mut self, (game_state, mut game_timer, mut rest_mine, mut cells): Self::SystemData) {
           if *game_state != self.last_state {
               if *game_state == crate::GameState::READY {
                   game_timer.timer = 0.0;
                   rest_mine.count = crate::MINE_COUNT as i32;
                   for cell in (&mut cells).join() {
                       cell.state = crate::CellState::HIDE;
                   }
               }
               self.last_state = *game_state;
           }
       }
   }

   ```
2. 取消点击遮挡
   `mine.rs`中组初始化`UiTransform`中添加`into_transparent`使点击事件能正确传递到对应实体上
   ``` Rust
   // ..
   UiTransform::new(
       "header-group".to_string(),
       // ..
   )
   .into_transparent()
   // ..
   UiTransform::new(
       "cell-group".to_string(),
       // ..
   )
   .into_transparent()
   // ..
   ```
3. 添加重置按钮点击系统
   `systems`目录下添加`click_reset_btn.rs`和对应导出
   ``` Rust
   pub struct ClickResetBtnSystem {
       event_id: ReaderId<UiEvent>,
   }

   impl ClickResetBtnSystem {
       fn new(event_id: ReaderId<UiEvent>) -> Self {
           ClickResetBtnSystem { event_id }
       }
   }

   impl<'s> System<'s> for ClickResetBtnSystem {
       type SystemData = (
           Read<'s, EventChannel<UiEvent>>,
           WriteStorage<'s, crate::ResetBtn>,
           WriteExpect<'s, crate::GameState>,
       );

       fn run(&mut self, (event_channel, mut reset_btns, mut game_state): Self::SystemData) {
           for event in event_channel.read(&mut self.event_id) {
               if let Some(reset_btn) = reset_btns.get_mut(event.target) {
                   match event.event_type {
                       UiEventType::ClickStart => {
                           reset_btn.click_down = true;
                       }
                       UiEventType::ClickStop => {
                           reset_btn.click_down = false;
                           *game_state = crate::GameState::READY;
                       }
                       _ => (),
                   }
               }
           }
       }
   }

   pub struct ClickResetBtnSystemDesc;
   impl<'a, 'b> SystemDesc<'a, 'b, ClickResetBtnSystem> for ClickResetBtnSystemDesc {
       fn build(self, world: &mut World) -> ClickResetBtnSystem {
           let mut event_channel = world.fetch_mut::<EventChannel<UiEvent>>();
           ClickResetBtnSystem::new(event_channel.register_reader())
       }
   }

   ```
   其中Ui事件需要通过`EventChannel`获取,而初始化应用(即main函数)时拿不到,所以需要用`SystemDesc`包装`System`,main中使用的也是对应`SystemDesc`
4. 最复杂的格子点击系统
   1. 添加随机库
      `Cargo.toml`中添加
      ``` toml     
      [dependencies]
      rand = "0.8"
      ```
   2. `systems`目录下添加`click_cell.rs`和对应导出
      ``` Rust
      pub struct ClickCellSystem {
          event_id: ReaderId<UiEvent>,
      }

      impl ClickCellSystem {
          fn new(event_id: ReaderId<UiEvent>) -> Self {
              ClickCellSystem { event_id }
          }
      }

      impl<'s> System<'s> for ClickCellSystem {
          type SystemData = ( );
          fn run(&mut self, (): Self::SystemData) { }
      }

      pub struct ClickCellSystemDesc;
      impl<'a, 'b> SystemDesc<'a, 'b, ClickCellSystem> for ClickCellSystemDesc {
          fn build(self, world: &mut World) -> ClickCellSystem {
              let mut event_channel = world.fetch_mut::<EventChannel<UiEvent>>();
              ClickCellSystem::new(event_channel.register_reader())
          }
      }

      ```
   3. 简单点击效果 
      ``` Rust
      type SystemData = (
          Read<'s, EventChannel<UiEvent>>,
          WriteStorage<'s, crate::Cell>,
          WriteExpect<'s, crate::GameState>,
      );
      fn run(&mut self, (event_channel, mut cells, mut game_state): Self::SystemData) {
          if let crate::GameState::FINISH(_) = *game_state {
              return;
          }
          for event in event_channel.read(&mut self.event_id) {
              if let Some(cell) = cells.get_mut(event.target) {
                  if cell.state == crate::CellState::SHOW {
                      return;
                  }
                  match event.event_type {
                      UiEventType::ClickStart => {
                          cell.click_down = true;
                      }
                      UiEventType::ClickStop => {
                          if *game_state == crate::GameState::READY {
                              // 初始化格子信息
                              *game_state = crate::GameState::PLAYING;
                              cell.has_mine = false;
                              let mut mine = crate::MINE_COUNT;
                              let mut all = (crate::CELL_ROW * crate::CELL_COL) as u32 - 1;
                              self.rest_cell = all - mine + 1;
                              let mut rng = rand::thread_rng();
                              let mut set = HashSet::new();
                              set.insert(event.target);
                              let mut stack = cell.around.clone();
                              for entity in stack.iter() {
                                  set.insert(*entity);
                              }
                              while let Some(next) = stack.pop() {
                                  let next_cell = cells.get_mut(next).unwrap();
                                  if rng.gen_range(0..all) < mine {
                                      next_cell.has_mine = true;
                                      mine -= 1;
                                  } else {
                                      next_cell.has_mine = false;
                                  }
                                  all -= 1;
                                  for entity in &next_cell.around {
                                      if !set.contains(entity) {
                                          set.insert(*entity);
                                          stack.push(*entity);
                                      }
                                  }
                              }
                          }
                          let cell = cells.get_mut(event.target).unwrap();
                          if cell.has_mine {
                              *game_state = crate::GameState::FINISH(false);
                          } else {
                              let mut around_count = 0;
                              let arounds = cell.clone();
                              for around in arounds {
                                  let around_cell = cells.get(around).unwrap();
                                  if around_cell.has_mine {
                                      around_count += 1;
                                  }
                              }
                              let cell = cells.get_mut(event.target).unwrap();
                              cell.around_mine_count = around_count;
                              cell.click_down = false;
                              cell.state = crate::CellState::SHOW;
                              self.rest_cell -= 1;
                              if self.rest_cell == 0 {
                                  *game_state = crate::GameState::FINISH(true);
                                  for cell in (&mut cells).join() {
                                      if cell.state == crate::CellState::HIDE {
                                          cell.state = crate::CellState::FLAG
                                      }
                                  }
                              }
                          }
                      }
                      _ => (),
                  }
              }
          }
      }
      ```
   4. 点击0格子自动扩展到周围
      ``` Rust
      // ..
      UiEventType::ClickStop => {
        if *game_state == crate::GameState::READY {
            // ..
        }
        let cell = cells.get_mut(event.target).unwrap();
        if cell.has_mine {
            *game_state = crate::GameState::FINISH(false);
        } else {
            cell.click_down = false;
            let mut stack = vec![event.target];
            let mut set = HashSet::new();
            set.insert(event.target);
            while let Some(next_entity) = stack.pop() {
                let next_cell = cells.get_mut(next_entity).unwrap();
                let mut around_count = 0;
                let arounds = next_cell.around.clone();
                for around in arounds.iter() {
                    let around_cell = cells.get(*around).unwrap();
                    if around_cell.has_mine {
                        around_count += 1;
                    }
                }
                let next_cell = cells.get_mut(next_entity).unwrap();
                next_cell.around_mine_count = around_count;
                next_cell.state = crate::CellState::SHOW;
                if around_count == 0 {
                    for around in arounds.iter() {
                        if !set.contains(around) {
                            stack.push(*around);
                            set.insert(*around);
                        }
                    }
                }
            }
            // ..
        }
        // ..
    }
      ```
5. 初始化时添加System并设定好依赖关系
6. 这样,一款简易版的扫雷就完成了,让我们祈祷它不要出bug吧...S