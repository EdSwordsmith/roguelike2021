use bevy_ecs::prelude::*;
use rltk::{GameState, Rltk, VirtualKeyCode, RGB};
use std::cmp::{max, min};

struct Player;

struct Position {
    x: i32,
    y: i32,
}

struct Renderable {
    glyph: rltk::FontCharType,
    fg: RGB,
    bg: RGB,
}

struct State {
    ecs: World,
    schedule: Schedule,
}

fn handle_player_movement(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut query = ecs.query::<(&Player, &mut Position)>();
    for (_player, mut pos) in query.iter_mut(ecs) {
        pos.x = min(79, max(0, pos.x + delta_x));
        pos.y = min(49, max(0, pos.y + delta_y));
    }
}

fn handle_input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            // Player Movement
            VirtualKeyCode::Left => handle_player_movement(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => handle_player_movement(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => handle_player_movement(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => handle_player_movement(0, 1, &mut gs.ecs),
            // Exit when escape is pressed
            VirtualKeyCode::Escape => ctx.quit(),
            _ => {}
        },
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut rltk::Rltk) {
        ctx.cls();

        handle_input(self, ctx);

        let mut query = self.ecs.query::<(&Position, &Renderable)>();
        for (pos, render) in query.iter(&self.ecs) {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    let mut gs = State {
        ecs: World::new(),
        schedule: Schedule::default(),
    };

    gs.ecs
        .spawn()
        .insert(Player)
        .insert(Position { x: 40, y: 25 })
        .insert(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        });

    // let mut update = SystemStage::parallel();
    // gs.schedule.add_stage("update", update);

    rltk::main_loop(context, gs)
}
