#![allow(non_snake_case)]

use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::ttf::Font;

// 画面にテキストを描画
pub fn textOut<'a>(
    canvas: &mut Canvas<Window>,
    font: &Font,
    texture_creator: &'a TextureCreator<WindowContext>,
    text: &str,
    r: u8, g: u8, b: u8,
    x: i32, y: i32,
    w: u32, h: u32  // 1文字分の幅と高さ
) {
    let surface = font.render(text).blended(Color::RGB(r, g, b)).expect("failed to get texture surface");
    let rendered_text = texture_creator.create_texture_from_surface(&surface).ok().expect("failed to render text");
    let l = text.len();
    canvas.copy(&rendered_text, None, Some(Rect::new(x, y, w*(l as u32), h))).expect("failed to copy text");
}