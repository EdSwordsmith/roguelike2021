mod components;
mod map;
mod rect;

use rltk::{GameState, Rltk, VirtualKeyCode, RGB};
use specs::prelude::*;
use std::cmp::{max, min};

use components::*;
use map::*;

struct State {
    ecs: World,
}

fn handle_player_movement(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let players = ecs.read_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, pos) in (&players, &mut positions).join() {
        let destination_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map[destination_idx] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));
        }
    }
}

fn handle_input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            // Player Movement
            VirtualKeyCode::Left | VirtualKeyCode::A => handle_player_movement(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right | VirtualKeyCode::D => handle_player_movement(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up | VirtualKeyCode::W => handle_player_movement(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down | VirtualKeyCode::S => handle_player_movement(0, 1, &mut gs.ecs),
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
        self.ecs.maintain();

        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    let mut gs = State { ecs: World::new() };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    let (rooms, map) = new_map();
    let (px, py) = rooms[0].center();

    gs.ecs.insert(map);

    gs.ecs
        .create_entity()
        .with(Position { x: px, y: py })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .build();

    rltk::main_loop(context, gs)
}
