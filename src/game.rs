//! Game logic

use gl;
use std;
use libc;
use glfw;
use render;
use std::rc::Rc;
use gl::types::{GLint, GLfloat};
use std::cell::RefCell;
use render::{Tex, Sprite, Engine};
use movement::{accel, accel_compute};

pub struct Game {
    state: GameState,
    engine: render::Engine,
    p1: Player,
    p2: Player,
}

struct PlayState;

impl PlayState {
    fn new() -> PlayState {
        PlayState
    }
}

pub struct Player{
    pub number: int,
    pub position: GLfloat,
    pub velocity: GLfloat,
    pub accel: GLfloat,
    pub accel_mod: int,
    pub rotation_velocity: GLfloat,
    pub rotation_accel: GLfloat,
    pub points: int,
    pub sprite: Sprite
}

enum Selected {
    Start,
    Leaderboard,
    Quit
}

pub enum GameState {
    MainMenu(Vec<Rc<RefCell<Sprite>>>, Selected),
    Playing(PlayState),
    LeaderboardState(Vec<Rc<RefCell<Sprite>>>),
}

impl Game {
    /// Initialize game state
    pub fn new(width: GLint, height: GLint) -> Game {
        let mut e = Engine::new(width, height);
        let start = e.load_texture("menu.start", "menu.start.png");
        let p1tex = e.load_texture("p1tex", "p1tex.png");
        let p2tex = e.load_texture("p2tex", "p2tex.png");

        let mut p1:Player = Player{number:1, position:-0.8, velocity:0.0, accel: 0.0, accel_mod:0,
                            rotation_velocity:0.0, rotation_accel:0.0, points:0, sprite:Sprite::new(0.0, 0.0, 20, 20, 0.0, p1tex)};

        let mut p2:Player = Player{number:2, position:0.8, velocity:0.0, accel: 0.0, accel_mod:0,
                            rotation_velocity:0.0, rotation_accel:0.0, points:0, sprite:Sprite::new(0.0, 0.0, 20, 20, 0.0, p2tex)};

        let hscores = e.load_texture("menu.highscore", "menu.highscore.png");
        let quit = e.load_texture("menu.quit", "menu.quit.png");

        let start = Sprite::new(-0.5, 0.7, 20, 200, 0.5, start);
        let hscores = Sprite::new(-0.5, 0.4, 20, 200, 0.2, hscores);
        let quit = Sprite::new(-0.5, 0.1, 20, 200, 0.1, quit);

        let bg = Sprite::new(-1., -1., width, height, std::f32::consts::PI / 6., e.load_texture("bg", "ring.png"));

        let sprites = vec!(e.add_sprite(start), e.add_sprite(hscores), e.add_sprite(quit), e.add_sprite(bg));

        // e.remove_sprite(sprites.get(0).clone());

        Game {
            state: MainMenu(sprites, Start),
            engine: e,
            p1: p1,
            p2: p2,
        }
    }

    /// Handle input
    pub fn handle_event(&mut self, window: &glfw::Window,
                        (_time, event): (f64, glfw::WindowEvent)) {
        match event{
            glfw::KeyEvent(glfw::KeyW, _, glfw::Press, _) | glfw::KeyEvent(glfw::KeyW, _, glfw::Repeat, _) => {
                accel(true, &mut self.p1);
            },
            glfw::SizeEvent(x, y) => {
                self.engine.width = x as GLfloat;
                self.engine.height = y as GLfloat;
                gl::Viewport(0, 0, x, y);
            }
            _ => { }
        }
    }

    /// Update the game simulation
    pub fn tick(&mut self) {
        match self.state {
            MainMenu(ref sprites, ref sel) => {
                sprites.get(0).borrow_mut().rot += 0.03;
            }
            _ => { }
        }
    }

    /// Render current contents
    pub fn render(&self) {
        self.engine.render();
    }
}
