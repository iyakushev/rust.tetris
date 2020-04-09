use rand::{distributions::{Distribution, Standard}, Rng};
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    timer::Timer,
};

use crate::engine::{render::Window, text::cast_with_capacity, text::Text};

use super::field::Field;
use super::tetromino::Rotation;

//TODO GAMEOVER SCREEN
//TODO JOYSTICK
//TODO GAME SETTINGS BEFORE START
//TODO ROTATION COLLISION


// CONSTS
pub const GRAVITY: f32 = 50.0;
pub const WHITE: Color = Color::RGBA(255, 255,255,255);
pub const H_UI: i32 = 54;
pub const SZ_TILE: u32 = 18;
pub const W_FIELD: usize = 10;
pub const H_FIELD: usize = 20;


fn check_rotation_collision(field: &mut Field, direction: Rotation, bl: u32, br: u32, f: u32) {
    if field.has_collision() {
        // CHECK IF ROTATION IS EVEN POSSIBLE
        let mut columns = 0;
        let mut pos_y = 0;
        for tile in field.current_piece().get_tiles_pos().iter() {
            if field.tiles.contains(tile) && columns < 2 {
                if pos_y != tile.1 {columns+=1; pos_y = tile.1;}
            } else {
                println!("HOLD ON {}", columns);
                match direction {
                    Rotation::Right => field.current_piece().rotate(Rotation::Left, bl, br, f),
                    Rotation::Left => field.current_piece().rotate(Rotation::Right, bl, br, f),
                }
            }
        }
    }
}


pub fn run(window: &mut Window, event_pump: &mut sdl2::EventPump) -> Result<(), String> {
    let mut field = Field::new(W_FIELD, H_FIELD);
    let mut ticks = 0;
    let mut g_amplifier = 1.0; // The less it becomes -- the faster pieces will fall
    let mut accelerated = false;
    let mut hard_drop = false;

    let ui_bottom_offset = (window.height - SZ_TILE * 3 + 4) as i32;
    let border_left: u32 = SZ_TILE * 3 + 16; // 3 -- random offset
    let border_right: u32 = border_left + W_FIELD as u32 * SZ_TILE;
    let mut ui = vec!(Text::new("Score:", 10, 10, 15, Some(WHITE)),
                      Text::new("000000", 55, 11, 15, Some(WHITE)),
                      Text::new("Level:", 14, 30, 15, Some(WHITE)),
                      Text::new("01", 55, 31, 15, Some(WHITE)),
                      Text::new("NEXT:", 165, 10, 15, Some(WHITE)),
                      Text::new("APM:", 15, ui_bottom_offset as u32 + 10, 15, Some(WHITE)),
                      Text::new("000", 55, ui_bottom_offset as u32 + 11, 15, Some(WHITE)),
                      Text::new("Lines:", 10, ui_bottom_offset as u32 + 30, 15, Some(WHITE)),
                      Text::new("000", 55, ui_bottom_offset as u32 + 31, 15, Some(WHITE)),
                      Text::new("Time:", 128, ui_bottom_offset as u32 + 10, 15, Some(WHITE)),
                      Text::new("000", 130, ui_bottom_offset as u32 + 31, 15, Some(WHITE)),
                      Text::new("Hold:", 180, ui_bottom_offset as u32 + 10, 15, Some(WHITE)));

    //Todo miami mode
    'running: loop {
        window.draw_bg(Color::RGBA(0, 0, 0, 255));
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    field.current_piece().make_move(1, -1, 0, border_left, border_right, ui_bottom_offset as u32);
                    if field.has_collision() {
                        field.current_piece().make_move(1, 1, 0, border_left, border_right, ui_bottom_offset as u32);
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    field.current_piece().make_move(1, 1, 0, border_left, border_right, ui_bottom_offset as u32);
                    if field.has_collision() {
                        field.current_piece().make_move(1, -1, 0, border_left, border_right, ui_bottom_offset as u32);
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    if !accelerated {
                        g_amplifier = (10.0 * g_amplifier) / 100.0;
                        accelerated = true;
                    }
                }
                Event::KeyUp { keycode: Some(Keycode::Down), .. } => {
                    if accelerated {
                        g_amplifier = (100.0 * g_amplifier) / 10.0;
                        accelerated = false;
                    }
                }
                Event::KeyUp { keycode: Some(Keycode::Up), .. } => {
                    if !hard_drop {
                        g_amplifier = 0.0;
                        hard_drop = true;
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::E), .. } => {
                    field.current_piece().rotate(Rotation::Right,
                                                 border_left,
                                                 border_right,
                                                 ui_bottom_offset as u32);
                    check_rotation_collision(&mut field, Rotation::Left, border_left, border_right, ui_bottom_offset as u32);
                },
                Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
                    field.current_piece().rotate(Rotation::Left,
                                                 border_left,
                                                 border_right,
                                                 ui_bottom_offset as u32);
                    check_rotation_collision(&mut field, Rotation::Left, border_left, border_right, ui_bottom_offset as u32);
                }
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    field.pocket();
                }
                _ => {}
            }
        }
        // DRAW UI OUTLINES
        window.draw_line(WHITE, (0, H_UI), (window.width as i32, H_UI));
        window.draw_line(WHITE, (145, 0), (145, H_UI));
        window.draw_line(WHITE, (border_left as i32, ui_bottom_offset), (border_left as i32, H_UI));
        window.draw_line(WHITE, (border_right as i32, ui_bottom_offset), (border_right as i32, H_UI));
        window.draw_line(WHITE, (120, ui_bottom_offset), (120, window.height as i32));
        window.draw_line(WHITE, (170, ui_bottom_offset), (170, window.height as i32));
        window.draw_line(WHITE, (0, ui_bottom_offset), (window.width as i32, ui_bottom_offset));

        window.draw_text(&ui, 0)?; // DRAW USER INTERFACE
        field.draw(window);        // DRAW PIECES

        window.present();          // PRESENT BUFFER TO THE SCREEN
        ::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 500)); // 1 tick

        // ON PIECE UPDATE
        if ticks >= (GRAVITY * g_amplifier) as u32 {
            ui[10].change_text(&cast_with_capacity(field.time.elapsed().as_secs() as u16, 3)); // UPDATE TIME
            field.current_piece().make_move(1, 1, 1, border_left, border_right, ui_bottom_offset as u32);
            if field.has_collision() {
                field.current_piece().make_move(1, -1, 1, border_left, border_right, ui_bottom_offset as u32);
                field.current_piece().deactivate();
            }
            if !field.current_piece().is_active() {
                if field.game_over() {break 'running;}

                ui[1].change_text(&cast_with_capacity(field.score, 6)); // UPDATE SCORE
                ui[3].change_text(&cast_with_capacity(field.level as u16, 2)); // UPDATE LEVEL

                field.pocketed = false;
                field.next_piece();
                if hard_drop {
                    hard_drop = false;
                    g_amplifier = field.get_amplifier();
                }

            }
            ticks = 0;
        }
        else { ticks += 1 }
    }

    Ok(())
}
