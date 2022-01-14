use amethyst::{
    core::Time,
    ecs::{Entity, Join, Read, System, WriteExpect, WriteStorage},
    prelude::*,
    shrev::{EventChannel, ReaderId},
    ui::{UiEvent, UiEventType},
};
use rand::Rng;
use std::collections::HashSet;

pub struct ClickCellSystem {
    event_id: ReaderId<UiEvent>,
    rest_cell: u32,
    timer: f32,
    current_click: Option<Entity>,
}

impl ClickCellSystem {
    fn new(event_id: ReaderId<UiEvent>) -> Self {
        ClickCellSystem {
            event_id,
            rest_cell: 0,
            timer: 0.0,
            current_click: None,
        }
    }
}

impl<'s> System<'s> for ClickCellSystem {
    type SystemData = (
        Read<'s, EventChannel<UiEvent>>,
        Read<'s, Time>,
        WriteExpect<'s, crate::RestMine>,
        WriteStorage<'s, crate::Cell>,
        WriteExpect<'s, crate::GameState>,
    );
    fn run(
        &mut self,
        (event_channel, time, mut rest_mine, mut cells, mut game_state): Self::SystemData,
    ) {
        if let crate::GameState::FINISH(_) = *game_state {
            return;
        }
        if let Some(current) = self.current_click {
            self.timer += time.delta_seconds();
            if self.timer >= crate::FLAG_INTERVAL {
                let cell = cells.get_mut(current).unwrap();
                cell.click_down = false;
                if cell.state == crate::CellState::HIDE {
                    cell.state = crate::CellState::FLAG;
                    rest_mine.count -= 1;
                } else {
                    cell.state = crate::CellState::HIDE;
                    rest_mine.count += 1;
                }
                self.current_click = None;
            }
        }
        for event in event_channel.read(&mut self.event_id) {
            if let Some(cell) = cells.get_mut(event.target) {
                if cell.state == crate::CellState::SHOW {
                    return;
                }
                match event.event_type {
                    UiEventType::ClickStart => {
                        if cell.state == crate::CellState::HIDE {
                            cell.click_down = true
                        }
                        if *game_state == crate::GameState::PLAYING {
                            self.timer = 0.0;
                            self.current_click = Some(event.target);
                        }
                    }
                    UiEventType::ClickStop => {
                        if *game_state == crate::GameState::PLAYING && self.current_click == None {
                            continue;
                        }
                        self.current_click = None;
                        if *game_state == crate::GameState::READY {
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
                            cell.click_down = false;
                            let mut stack = vec![event.target];
                            let mut set = HashSet::new();
                            set.insert(event.target);
                            while let Some(next_entity) = stack.pop() {
                                let next_cell = cells.get_mut(next_entity).unwrap();
                                if next_cell.state != crate::CellState::HIDE {
                                    continue;
                                }
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
                                self.rest_cell -= 1;
                                if around_count == 0 {
                                    for around in arounds.iter() {
                                        if !set.contains(around) {
                                            stack.push(*around);
                                            set.insert(*around);
                                        }
                                    }
                                }
                            }
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
}

pub struct ClickCellSystemDesc;
impl<'a, 'b> SystemDesc<'a, 'b, ClickCellSystem> for ClickCellSystemDesc {
    fn build(self, world: &mut World) -> ClickCellSystem {
        let mut event_channel = world.fetch_mut::<EventChannel<UiEvent>>();
        ClickCellSystem::new(event_channel.register_reader())
    }
}
