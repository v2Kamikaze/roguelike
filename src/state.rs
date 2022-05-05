use rltk::{GameState, Rltk};
use specs::prelude::*;


use crate::walker::*;
use crate::position::*;
use crate::renderable::*;
use crate::player::player_input;


pub struct State {
    pub ecs: World,
}

impl State {
    pub fn new(world: World) -> Self {
        Self { ecs: world }
    }

    fn run_systems(&mut self) {
        let mut lw = LeftWalker {};
        lw.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}
