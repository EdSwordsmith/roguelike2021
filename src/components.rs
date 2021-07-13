use rltk::RGB;
use specs::prelude::*;
use specs_derive::*;

#[derive(Component, Debug)]
pub struct Player {}

#[derive(Component, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Debug)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}
