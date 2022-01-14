use std::collections::HashSet;

use amethyst::{
    ecs::{Join, Read, System, WriteExpect, WriteStorage},
    prelude::*,
    shrev::{EventChannel, ReaderId},
    ui::{UiEvent, UiEventType},
};
use rand::Rng;

pub struct ClickCellSystem {
    event_id: ReaderId<UiEvent>,
    rest_cell: u32,
}

impl ClickCellSystem {
    fn new(event_id: ReaderId<UiEvent>) -> Self {
        ClickCellSystem {
            event_id,
            rest_cell: 0,
        }
    }
}

impl<'s> System<'s> for ClickCellSystem {
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
                            show_cell(event.target, &mut cells);
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
}

fn show_cell(
    entity: Entity,
    cells: &mut amethyst::ecs::Storage<
        crate::Cell,
        amethyst::shred::FetchMut<amethyst::ecs::storage::MaskedStorage<crate::Cell>>,
    >,
) {
    let mut stack = vec![entity];
    let mut set = HashSet::new();
    set.insert(entity);
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
}

pub struct ClickCellSystemDesc;
impl<'a, 'b> SystemDesc<'a, 'b, ClickCellSystem> for ClickCellSystemDesc {
    fn build(self, world: &mut World) -> ClickCellSystem {
        let mut event_channel = world.fetch_mut::<EventChannel<UiEvent>>();
        ClickCellSystem::new(event_channel.register_reader())
    }
}
