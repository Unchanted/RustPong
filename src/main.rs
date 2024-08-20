extern crate sdl2;

use crate::entity::*;
use crate::view::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::EventPump;

mod entity;
mod view;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const WINDOW_TITLE: &str = "pong";
const SCREEN_MARGIN: i32 = 10;
const FRAME_DURATION: u32 = 50;

struct FrameEvent;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    game_loop(&sdl_context, &mut canvas);
}

/// Contains the main loop of the game.
fn game_loop(context: &sdl2::Sdl, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    let mut gs = GameState::new();
    let mut event_pump = context.event_pump().unwrap();
    let ev = context.event().unwrap();
    ev.register_custom_event::<FrameEvent>().unwrap();

    let timer_subsystem = context.timer().unwrap();
    let _timer = timer_subsystem.add_timer(
        FRAME_DURATION,
        Box::new(|| {
            ev.push_custom_event(FrameEvent).unwrap();
            FRAME_DURATION
        }),
    );

    'game_loop: loop {
        while !gs.is_game_over && !gs.is_game_restarted {
            handle_game_events(&mut gs, &mut event_pump, canvas);
            handle_ball_out_of_border(&mut gs);
        }
        if !gs.is_game_restarted {
            break 'game_loop;
        }
        gs = GameState::new();
    }
}

/// Handles game events, rendering, and updates.
fn handle_game_events(
    gs: &mut GameState,
    event_pump: &mut EventPump,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
) {
    let event = event_pump.wait_event();

    if event.is_user_event() {
        handle_collisions(gs);
        gs.ball.update_position();
        update_cpu_racket(gs);

        render_game(canvas, gs);
    } else {
        handle_key_events(gs, &event);
    }
}

/// Handles key events like moving the paddle or restarting the game.
fn handle_key_events(gs: &mut GameState, event: &Event) {
    match event {
        Event::Quit { .. }
        | Event::KeyDown {
            keycode: Some(Keycode::Escape),
            ..
        } => {
            gs.is_game_over = true;
        }
        Event::KeyDown {
            keycode: Some(Keycode::Up),
            ..
        } => {
            gs.racket_1.move_up();
        }
        Event::KeyDown {
            keycode: Some(Keycode::Down),
            ..
        } => {
            gs.racket_1.move_down();
        }
        Event::KeyDown {
            keycode: Some(Keycode::Space),
            ..
        } => {
            gs.is_game_restarted = true;
        }
        _ => {}
    }
}

/// Handles collisions of the ball with paddles and screen edges.
fn handle_collisions(gs: &mut GameState) {
    let ball = &mut gs.ball;

    if ball.has_collision_with(&gs.racket_1) {
        adjust_ball_direction(ball, gs.racket_1.collision_point_with(ball), Direction::EAST);
    }

    if ball.has_collision_with(&gs.racket_2) {
        adjust_ball_direction(ball, gs.racket_2.collision_point_with(ball), Direction::WEST);
    }

    if ball.has_collision_with_ceiling() {
        ball.direction = if ball.direction == Direction::NORTHWEST {
            Direction::SOUTHWEST
        } else {
            Direction::SOUTHEAST
        };
    }

    if ball.has_collision_with_floor() {
        ball.direction = if ball.direction == Direction::SOUTHWEST {
            Direction::NORTHWEST
        } else {
            Direction::NORTHEAST
        };
    }
}

/// Adjusts the ball direction after a collision.
fn adjust_ball_direction(ball: &mut Ball, collision_point: i32, base_direction: Direction) {
    ball.direction = match collision_point {
        0 => base_direction,
        cp if cp > 0 => match base_direction {
            Direction::EAST => Direction::SOUTHEAST,
            Direction::WEST => Direction::SOUTHWEST,
            _ => base_direction,
        },
        _ => match base_direction {
            Direction::EAST => Direction::NORTHEAST,
            Direction::WEST => Direction::NORTHWEST,
            _ => base_direction,
        },
    };
    ball.increase_speed();
}

/// Check if the ball scores and update the corresponding player's score.
fn handle_ball_out_of_border(gs: &mut GameState) {
    if gs.ball.pos_x < 0 {
        gs.score_p2 += 1;
        println!("p2 scored!, total: {}-{}", gs.score_p1, gs.score_p2);
        gs.reset_positions();
    } else if gs.ball.pos_x > WINDOW_WIDTH as i32 {
        gs.score_p1 += 1;
        println!("p1 score!, total: {}-{}", gs.score_p1, gs.score_p2);
        gs.reset_positions();
    }
}

/// Manage the movement of the AI racket depending on the game state.
fn update_cpu_racket(gs: &mut GameState) {
    let racket_2 = &mut gs.racket_2;
    let ball = &gs.ball;

    if matches!(
        ball.direction,
        Direction::SOUTH | Direction::SOUTHWEST | Direction::SOUTHEAST
    ) {
        racket_2.pos_y += racket_2.speed;
    } else if matches!(
        ball.direction,
        Direction::NORTH | Direction::NORTHEAST | Direction::NORTHWEST
    ) {
        racket_2.pos_y -= racket_2.speed;
    }
}

/// Renders the game state to the canvas.
fn render_game(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, gs: &GameState) {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    draw_game(gs, canvas);
    canvas.present();
}
