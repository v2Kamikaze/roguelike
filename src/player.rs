use rltk::{VirtualKeyCode, Rltk};
use specs::prelude::*;
use specs_derive::Component;

use crate::position::*;
use crate::state::*;

#[derive(Component, Debug)]
pub struct Player;

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        pos.x = std::cmp::min(79 , std::cmp::max(0, pos.x + delta_x));
        pos.y = std::cmp::min(49, std::cmp::max(0, pos.y + delta_y));
    }
}

pub fn player_input(game_state: &mut State, ctx: &mut Rltk) {
    // Player movement
    match ctx.key {
        None => {} // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut game_state.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut game_state.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut game_state.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut game_state.ecs),
            _ => {}
        },
    }
}