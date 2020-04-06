use std::ops::Deref;
use std::path::Path;
use std::process::exit;

use rand::{distributions::{Distribution, Standard}, Rng};
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    timer::Timer,
};

use crate::engine::{render, text::cast_with_capacity, text::Text};
use crate::engine::render::Window;

use super::tetromino::{Rotation, Shape, Tetromino};

const GRAVITY: f32 = 50.0;
const WHITE: Color = Color::RGBA(255, 255,255,255);
const H_UI: i32 = 54;
const SZ_TILE: u32 = 18;

pub struct Field {
    pieces: Vec<Tetromino>,
    pocket: Option<Tetromino>,
    cursor: usize,
    pub pocketed: bool,
    pub level: u8,
    pub score: u16,
    pub time: u16
}

impl Field {
    /// Creates a new instance
    pub fn new() -> Self {
        let mut v = vec![Tetromino::new(rand::random()), Tetromino::new(rand::random())];
        v[0].set_default_pos();
        v[1].set_for_next();
        Field {
            pieces: v,
            pocket: None,
            pocketed: false,
            cursor: 0,
            level: 1,
            score: 0,
            time: 0
        }
    }

    /// Changes level and returns new G_AMPLIFIER value
    pub fn next_lvl(&mut self) -> f32 {
        self.level += 1;
        self.get_amplifier()
    }

    /// Returns new G_AMPLIFIER value
    pub fn get_amplifier(&self) -> f32 {
        match self.level {
            1 => 1.0,
            2 => 0.9,
            3 => 0.8,
            4 => 0.7,
            5 => 0.6,
            6 => 0.5,
            7 => 0.4,
            8 => 0.3,
            9 => 0.2,
            _ => 0.1,
        }
    }

    /// Returns a ref to the current piece
    pub fn current_piece(&mut self) -> &mut Tetromino {
        &mut self.pieces[self.cursor]
    }

    /// Handles spawning of the new piece
    pub fn next_piece(&mut self) {
        self.pieces.push(Tetromino::new(rand::random()));
        self.cursor += 1;
        self.pieces[self.cursor].set_default_pos();
        self.pieces[self.cursor+1].set_for_next();
    }

    /// Handles the logic of pocketing a piece
    pub fn pocket(&mut self) {
        if !self.pocketed {
            if self.pocket.is_some() {
                let next = self.pieces.pop().unwrap();
                let mut new = self.pieces.pop().unwrap();
                let mut piece = self.pocket.unwrap();

                piece.set_default_pos();
                new.set_to_pocket(); //Change pos

                self.pieces.push(piece);
                self.pieces.push(next);
                self.pocket = Some(new);
            } else {
                let mut cur = self.pieces.pop().unwrap();
                let mut new = Tetromino::new(rand::random());
                let mut poc = self.pieces.pop().unwrap(); // Store

                new.set_for_next();
                cur.set_default_pos();
                poc.set_to_pocket();

                self.pieces.push(cur);           // Swap new CURRENT
                self.pieces.push(new);           // Set new NEXT
                self.pocket = Some(poc);         // Set new POCKET
            }
            self.pocketed = true;
        }
    }

    /// Draws pieces on the screen
    pub fn draw(&mut self, window: &mut Window) {
        self.pieces.iter().for_each(|t| t.draw(window).unwrap());
        match self.pocket {
            Some(t) => t.draw(window).unwrap(),
            None => ()
        };
    }
}


pub fn run(window: &mut render::Window, event_pump: &mut sdl2::EventPump) -> Result<(), String> {
    let mut field = Field::new();
    let mut timer = 0.0;
    let mut g_amplifier = 1.0; // The less it becomes -- the faster pieces will fall
    let mut accelerated = false;
    let mut hard_drop = false;

    let ui_bottom_offset = (window.height - 54) as i32;
    let border_left: u32 = SZ_TILE*4-3;
    let border_right: u32 = window.width - SZ_TILE*4+4;
    let mut ui = vec!(Text::new("Score:", 10, 10, 15, Some(WHITE)),
                      Text::new("000000", 55, 11, 15, Some(WHITE)),
                      Text::new("Level:", 14, 30, 15, Some(WHITE)),
                      Text::new("01", 55, 31 , 15, Some(WHITE)),
                      Text::new("NEXT:", 165, 10, 15, Some(WHITE)),
                      Text::new("APM:", 15, ui_bottom_offset as u32 + 10, 15, Some(WHITE)),
                      Text::new("000", 55, ui_bottom_offset as u32 + 11, 15, Some(WHITE)),
                      Text::new("Lines:", 10, ui_bottom_offset as u32 + 30, 15, Some(WHITE)),
                      Text::new("000", 55, ui_bottom_offset as u32 + 31, 15, Some(WHITE)),
                      Text::new("Time:", 128, ui_bottom_offset as u32 + 10, 15, Some(WHITE)),
                      Text::new("000", 130, ui_bottom_offset as u32 + 31, 15, Some(WHITE)),
                      Text::new("Pocket:", 190, ui_bottom_offset as u32 + 10, 15, Some(WHITE)));

    //Todo miami mode
    'running: loop {
        window.draw_bg(Color::RGBA(0, 0, 0, 255));
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                    if !hard_drop {
                        field.current_piece().make_move(-1, 0);
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                    if !hard_drop {
                        field.current_piece().make_move(1, 0);
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
                    if !accelerated {
                        g_amplifier = (50.0*g_amplifier)/100.0;
                        accelerated = true;
                    }
                }
                Event::KeyUp { keycode: Some(Keycode::Down), ..} => {
                    if accelerated {
                        g_amplifier = (100.0*g_amplifier)/50.0;
                        accelerated = false;
                    }
                }
                Event::KeyUp { keycode: Some(Keycode::Up), ..} => {
                    if !hard_drop {
                        g_amplifier = 0.0;
                        hard_drop = true;
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::E), ..} => {
                    field.current_piece().rotate(Rotation::Right);
                },
                Event::KeyDown { keycode: Some(Keycode::Q), ..} => {
                    field.current_piece().rotate(Rotation::Left);
                }
                Event::KeyDown { keycode: Some(Keycode::R), ..} => {
                    field.pocket();
                }
                _ => {}
            }
        }
        // DRAW UI OUTLINES
        window.draw_line(WHITE, (0, H_UI), (window.width as i32, H_UI));
        window.draw_line(WHITE, (145,0), (145, H_UI));
        window.draw_line(WHITE, (border_left as i32, ui_bottom_offset), (border_left as i32, H_UI));
        window.draw_line(WHITE, (border_right as i32,ui_bottom_offset), (border_right as i32, H_UI));
        window.draw_line(WHITE, (120,ui_bottom_offset), (120, window.height as i32));
        window.draw_line(WHITE, (170,ui_bottom_offset), (170, window.height as i32));
        window.draw_line(WHITE, (0, ui_bottom_offset), (window.width as i32, ui_bottom_offset));

        ui[1].change_text(&cast_with_capacity(field.score, 6)); // UPDATE SCORE
        ui[3].change_text(&cast_with_capacity(field.level as u16, 2)); // UPDATE LEVEL
        ui[10].change_text(&cast_with_capacity(field.time, 3)); // UPDATE TIME
        window.draw_text(&ui, 0)?;            // DRAW USER INTERFACE
        field.draw(window); // DRAW PIECES

        window.present();
        ::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 500)); // 1 tick
        if timer >= GRAVITY * g_amplifier {
            field.current_piece().make_move(0, 1);
            if field.current_piece().stops_falling() {
                field.pocketed = false;
                field.next_piece();
                if hard_drop { hard_drop = false; g_amplifier = field.get_amplifier();}
            }
            timer = 0.0;
            field.time += 1;
        } else {timer += 1.0}
    }

    Ok(())
}