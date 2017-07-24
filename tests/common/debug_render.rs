// extern crate rex;
// extern crate font_types as font;
// #[macro_use]
// extern crate serde_derive;

use font::FontUnit;
use rex::parser::color::RGBA;
use rex::render::{Renderer, RenderSettings, Cursor};

type Objects = Vec<Object>;

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct Equation {
    pub tex: String,
    pub description: String,
    pub render: Objects
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub enum Object {
    Symbol(DebugSymbol),
    Rule(DebugRule),
    Color(RGBA, Vec<Object>),
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, Copy)]
pub struct DebugSymbol {
    scale: f64,
    codepoint: u32,
    x: FontUnit,
    y: FontUnit,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, Copy)]
pub struct DebugRule {
    width: FontUnit,
    height: FontUnit,
    x: FontUnit,
    y: FontUnit
}

#[derive(Clone, Default)]
pub struct DebugRenderer {
    settings: RenderSettings,
}

impl Renderer for DebugRenderer {
    type Out = Objects;

    fn settings(&self) -> &RenderSettings {
        &self.settings
    }

    fn symbol(&self, out: &mut Objects, pos: Cursor, symbol: u32, scale: f64) {
        out.push(
            Object::Symbol(
                DebugSymbol {
                    codepoint: symbol,
                    scale: scale,
                    x: pos.x,
                    y: pos.y,
                }
            ));
    }

    fn rule(&self, out: &mut Objects, pos: Cursor, width: FontUnit, height: FontUnit) {
        out.push(
            Object::Rule(
                DebugRule {
                    width: width,
                    height: height,
                    x: pos.x,
                    y: pos.y,
                }
            ));
    }

    fn color<F>(&self, out: &mut Objects, color: RGBA, mut contents: F)
        where F: FnMut(&Self, &mut Objects)
    {
        let mut inner = Objects::default();
        contents(self, &mut inner);
        out.push(Object::Color(color, inner));
    }
}