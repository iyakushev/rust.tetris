use std::collections::HashSet;
use std::path::Path;
use std::process::exit;
use std::time::Instant;

use crate::engine::render::Window;

use super::tetromino::{Rotation, Shape, Tetromino};

//TODO implement multiple scoring rules

pub struct Field {
    pub tiles: HashSet<(u32,u32)>,
    pieces: Vec<Tetromino>,
    pocket: Option<Tetromino>,
    cursor: usize,
    width: usize,
    height: usize,
    pub pocketed: bool,
    pub level: u8,
    pub score: u16,
    pub time: Instant
}

impl Field {
    /// Creates a new instance
    pub fn new(width: usize, height: usize) -> Self {
        let mut v = vec![Tetromino::new(rand::random()), Tetromino::new(rand::random())];
        v[0].set_default_pos();
        v[1].set_for_next();
        Field {
            tiles: HashSet::new(),
            pieces: v,
            width,
            height,
            pocket: None,
            pocketed: false,
            cursor: 0,
            level: 1,
            score: 0,
            time: Instant::now()
        }
    }

    /// Modifies the score where n is a number of rows deleted
    pub fn inc_score(&mut self, n: u16) {
        self.score = match n {
            1 => 40*(self.level + 1) as u16,
            2 => 100*(self.level + 1)as u16,
            3 => 300*(self.level + 1)as u16,
            _ => 1200*(self.level + 1)as u16,
        };
    }

    /// Changes level and returns new G_AMPLIFIER value
    pub fn next_lvl(&mut self) -> f32 {
        self.level += 1;
        self.get_amplifier()
    }

    /// Checks if any tile Y-value is low enough to consider it a game_over
    pub fn game_over(&mut self) -> bool {
        self.pieces[self.cursor].get_tiles_pos().iter().any(|c| c.1 <= 1) // ANY TILE.Y IS TOO HIGH ON THE STACK
    }

    fn lower_pieces(&mut self, n: u16, lb: u32, rb: u32, f: u32) {
        self.pieces.iter_mut().for_each(|p| p.make_move(n as i32, 1, 1, lb, rb, f));
    }


    pub fn check_lines(&mut self) {
        let pos = self.pieces[self.cursor].get_tiles_pos();
        for tile in pos {
            if self.tiles.iter().map(|t| t.1 == tile.1).count() == self.width {
                ()
            }
        }
    }

    /// Returns new G_AMPLIFIER value
    pub fn get_amplifier(&self) -> f32 {
        match self.level {
            1 => 1.0,  // G = 50 ticks
            2 => 0.9,  // G = 45 ticks
            3 => 0.8,  // G = 40 ticks
            4 => 0.7,  // G = 35 ticks
            5 => 0.6,  // G = 30 ticks
            6 => 0.5,  // G = 25 ticks
            7 => 0.4,  // G = 20 ticks
            8 => 0.3,  // G = 15 ticks
            9 => 0.2,  // G = 10 ticks
            _ => 0.1,  // G =  5 ticks
        }
    }

    /// Returns a ref to the current piece
    pub fn current_piece(&mut self) -> &mut Tetromino {
        &mut self.pieces[self.cursor]
    }

    /// Handles spawning of the new piece
    pub fn next_piece(&mut self) {

        self.current_piece().get_tiles_pos().iter().for_each(|c| {self.tiles.insert(*c);});

        self.pieces.push(Tetromino::new(rand::random()));
        self.cursor += 1;
        self.pieces[self.cursor].set_default_pos();
        self.pieces[self.cursor + 1].set_for_next();
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
        }
    }

    /// check pieces for collision
    pub fn has_collision(&mut self) -> bool {
        self.current_piece().get_tiles_pos().iter().any(|c| self.tiles.contains(c))
    }
}