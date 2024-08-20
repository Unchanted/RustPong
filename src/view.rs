use crate::entity::{Ball, GameState, Racket};
use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::ttf::Font;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::gfx::primitives::DrawRenderer;
use std::path::Path;

const HALFWAY_LINE_DASHES: i32 = 20;

/// Draw a racket object on the screen.
fn draw_racket(racket: &Racket, canvas: &mut Canvas<Window>) {
    let rectangle = Rect::new(racket.pos_x, racket.pos_y, racket.width, racket.height);
    canvas.set_draw_color(racket.color);
    canvas.fill_rect(rectangle).expect("Failed to draw racket");
}

/// Draw a ball object on the screen.
fn draw_ball(ball: &Ball, canvas: &mut Canvas<Window>) {
    canvas.filled_circle(ball.pos_x as i16, ball.pos_y as i16, ball.radius as i16, ball.color)
        .expect("Failed to draw ball");
}

/// Draw the line separating the two players at the middle of the screen.
pub fn draw_halfway_line(canvas: &mut Canvas<Window>) {
    let middle_x = (WINDOW_WIDTH / 2) as i32 - 2;
    let dash_length = WINDOW_HEIGHT as i32 / (HALFWAY_LINE_DASHES * 2);
    let margin_top = dash_length / 2;
    canvas.set_draw_color(Color::WHITE);

    for i in (0..HALFWAY_LINE_DASHES * 2).step_by(2) {
        let start_point = Point::new(middle_x, margin_top + i * dash_length);
        let end_point = Point::new(middle_x, margin_top + i * dash_length + dash_length);
        canvas.draw_line(start_point, end_point)
            .expect("Failed to draw halfway line dash");
    }
}

/// Draw the score for the two players.
pub fn draw_score(gs: &GameState, canvas: &mut Canvas<Window>) {
    let ttf_context = sdl2::ttf::init().expect("SDL TTF initialization failed");
    let texture_creator = canvas.texture_creator();
    let font = load_font(&ttf_context, "font/schluber/Schluber.ttf", 128);
    
    let rect_width = WINDOW_WIDTH / 12;
    let rect_height = WINDOW_HEIGHT / 10;
    let font_rect_p1 = Rect::new(
        (WINDOW_WIDTH / 4 - rect_width / 2) as i32,
        rect_height as i32,
        rect_width,
        rect_height,
    );
    let font_rect_p2 = Rect::new(
        (WINDOW_WIDTH * 3 / 4 - rect_width / 2) as i32,
        rect_height as i32,
        rect_width,
        rect_height,
    );

    draw_text(canvas, &texture_creator, &font, gs.score_p1, font_rect_p1);
    draw_text(canvas, &texture_creator, &font, gs.score_p2, font_rect_p2);
}

/// Draw text on the screen.
fn draw_text(
    canvas: &mut Canvas<Window>,
    texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    font: &Font,
    score: i32,
    rect: Rect,
) {
    let surface = font.render(&format!("{}", score))
        .blended(Color::WHITE)
        .expect("Failed to render text surface");
    let texture = texture_creator.create_texture_from_surface(&surface)
        .expect("Failed to create texture from surface");
    canvas.copy(&texture, None, rect).expect("Failed to draw text");
}

/// Load a font from the specified path.
fn load_font(ttf_context: &sdl2::ttf::Sdl2TtfContext, path: &str, size: u16) -> Font {
    ttf_context.load_font(Path::new(path), size)
        .expect("Failed to load font")
}

/// Draw all elements for the game.
pub fn draw_game(gs: &GameState, canvas: &mut Canvas<Window>) {
    draw_halfway_line(canvas);
    draw_score(gs, canvas);
    draw_ball(&gs.ball, canvas);
    draw_racket(&gs.racket_1, canvas);
    draw_racket(&gs.racket_2, canvas);
}
