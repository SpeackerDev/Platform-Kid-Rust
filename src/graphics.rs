use sdl2::image::*;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::surface::Surface;
use sdl2::ttf::Font;
use sdl2::video::Window;
use sdl2::*;

use std::mem;
use std::rc::Rc;

use std::borrow::Borrow;
use std::collections::HashMap;
use std::ops::Deref;

pub struct Graphics {
    _renderer: Canvas<Window>,
}

/*pub fn loadImage(filePath: &str) -> &Surface;

pub fn loadText(font: &Font, text: &char, color: Color) -> &Surface;

pub fn blitSurface(source: &Texture, sourceRect: &Rect, destRect: &Rect);

pub fn flip();

pub fn clear();

pub fn getCanvas() -> &Canvas;*/

impl Graphics {
    pub fn new(name: &str, context: Sdl, video: VideoSubsystem) -> Self {
        let window: Window = video
            .window("Aaa", 1280, 720)
            .position_centered()
            .build()
            .unwrap();
        let canvas: Canvas<Window> = window.into_canvas().build().unwrap();
        return Self { _renderer: canvas };
    }

    pub fn loadImage<'a>(filepath: &str, this: &mut Self) -> Result<Surface<'a>, String> {
        return LoadSurface::from_file(filepath);
    }

    pub fn loadText<'a>(font: &Font, text: &str, color: Color) -> Result<Surface<'a>, String> {
        return font.render(text).blended(color).map_err(|e| e.to_string());
    }

    pub fn blitSurface(source: &Texture, sourceRect: &Rect, destRect: &Rect, this: &mut Self) {
        let _ = this._renderer.copy(&source, Some(*sourceRect), Some(*destRect));
    }

    pub fn flip(this: &mut Self) {
        this._renderer.present();
    }

    pub fn clear(this: &mut Self) {
        this._renderer.clear();
    }

    pub fn getCanvas(this: Self) -> Canvas<Window> {
        return this._renderer;
    }
}
