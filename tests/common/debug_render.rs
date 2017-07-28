// extern crate rex;
// extern crate font_types as font;
// #[macro_use]
// extern crate serde_derive;

use font::FontUnit;
use rex::parser::color::RGBA;
use rex::render::{Renderer, RenderSettings, Cursor};
use std::rc::Rc;
use std::cell::Cell;

type Objects = Vec<Object>;

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, PartialEq)]
pub struct Equation {
    pub tex: String,
    pub description: String,
    pub width: FontUnit,
    pub height: FontUnit,
    pub render: Objects,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Symbol(DebugSymbol),
    Rule(DebugRule),
    Color(RGBA, Vec<Object>),
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DebugSymbol {
    pub scale: f64,
    pub codepoint: u32,
    pub x: FontUnit,
    pub y: FontUnit,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DebugRule {
    pub width: FontUnit,
    pub height: FontUnit,
    pub x: FontUnit,
    pub y: FontUnit,
}

#[derive(Clone, Default)]
pub struct DebugRenderer {
    settings: RenderSettings,
    pub width: Cell<FontUnit>,
    pub height: Cell<FontUnit>,
}

impl Renderer for DebugRenderer {
    type Out = Objects;

    fn settings(&self) -> &RenderSettings {
        &self.settings
    }

    fn prepare(&self, _: &mut Objects, width: FontUnit, height: FontUnit) {
        self.width.set(width);
        self.height.set(height);
    }

    fn symbol(&self, out: &mut Objects, pos: Cursor, symbol: u32, scale: f64) {
        out.push(Object::Symbol(DebugSymbol {
                                    codepoint: symbol,
                                    scale: scale,
                                    x: pos.x,
                                    y: pos.y,
                                }));
    }

    fn rule(&self, out: &mut Objects, pos: Cursor, width: FontUnit, height: FontUnit) {
        out.push(Object::Rule(DebugRule {
                                  width: width,
                                  height: height,
                                  x: pos.x,
                                  y: pos.y,
                              }));
    }

    fn color<F>(&self, out: &mut Objects, color: RGBA, mut contents: F)
        where F: FnMut(&Self, &mut Objects)
    {
        let mut inner = Objects::default();
        contents(self, &mut inner);
        out.push(Object::Color(color, inner));
    }
}