use rltk::{GameState, Rltk, RltkBuilder, VirtualKeyCode, RGB};
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Component)]
struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

struct State {
    pub ecs: World,
}

impl State {
    pub fn new(world: World) -> Self {
        Self { ecs: world }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

fn main() -> rltk::BError {
    let context = RltkBuilder::simple80x50().with_title("Roguelike").build()?;
    let world = World::new();
    let mut game_state = State::new(world);
    game_state.ecs.register::<Position>();
    game_state.ecs.register::<Renderable>();

    // Criando uma entidade.
    game_state
        .ecs
        .create_entity()
        .with(Position::new(40, 25))
        .with(Renderable {
            glyph: rltk::to_cp437('♦'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .build();

    rltk::main_loop(context, game_state)
}
