//! Game logic

use gl;
use std;
use glfw;
use render;
use physics;
use std::rc::Rc;
use gl::types::{GLint, GLfloat};
use std::cell::RefCell;
use render::{Sprite, Engine};
use physics::{accel, rotate, Direction, Forward, Backward, Still, Rotation, Left, Right, Norot};

pub struct Game {
    state: GameState,
    engine: render::Engine,
    p1: Player,
    p2: Player,
    counter: int,
}

struct PlayState;

pub struct Player {
    pub number: int,
    pub positionx: GLfloat,
    pub positiony: GLfloat,
    pub velocity: GLfloat,
    pub accel: GLfloat,
    pub accel_mod: int,
    pub rotation: GLfloat,
    pub points: int,
    pub sprite: Rc<RefCell<Sprite>>,
    pub dir: Direction,
    pub rot: Rotation,
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

        let bg = Sprite::new(-1., -1., 1.0, 1.0, std::f32::consts::PI / 6., e.load_texture("bg", "ring.png"));

        e.add_sprite(bg);

        let p1s = Sprite::new(0.0, 0.0, 0.05, 0.05, 0.0, p1tex);
        let p2s = Sprite::new(0.0, 0.0, 0.05, 0.05, 0.0, p2tex);

        let p1: Player = Player{number:1, positionx:-0.8, positiony:0.0, velocity:0.0, accel: 0.0, accel_mod:0,
            rotation: 0.0, points:0, sprite: e.add_sprite(p1s), dir:Still, rot:Norot};

        let p2: Player = Player{number:2, positionx:0.8, positiony:0.0, velocity:0.0, accel: 0.0, accel_mod:0,
            rotation: 0.0, points:0, sprite: e.add_sprite(p2s), dir:Still, rot:Norot};

        let hscores = e.load_texture("menu.highscore", "menu.highscore.png");
        let quit = e.load_texture("menu.quit", "menu.quit.png");

        let start = Sprite::new(-0.5, 0.7, 0.05, 0.5, 0.5, start);
        let hscores = Sprite::new(-0.5, 0.4, 0.05, 0.5, 0.2, hscores);
        let quit = Sprite::new(-0.5, 0.1, 0.05, 0.5, 0.1, quit);


        let sprites = vec!(e.add_sprite(start), e.add_sprite(hscores), e.add_sprite(quit));

        Game {
            state: MainMenu(sprites, Start),
            engine: e,
            p1: p1,
            p2: p2,
            counter: 0,
        }
    }

    /// Handle input
    pub fn handle_event(&mut self, _window: &glfw::Window,
                        (_time, event): (f64, glfw::WindowEvent)) {
        match event{
            glfw::KeyEvent(glfw::KeyW, _, glfw::Press, _) | glfw::KeyEvent(glfw::KeyW, _, glfw::Repeat, _) => {
                self.p1.dir = Forward;
            },
            glfw::KeyEvent(glfw::KeyS, _, glfw::Press, _) | glfw::KeyEvent(glfw::KeyS, _, glfw::Repeat, _) => {
                self.p1.dir = Backward;
            },
            glfw::KeyEvent(glfw::KeyA, _, glfw::Press, _) | glfw::KeyEvent(glfw::KeyA, _, glfw::Repeat, _) => {
                self.p1.rot = Left;
            },
            glfw::KeyEvent(glfw::KeyD, _, glfw::Press, _) | glfw::KeyEvent(glfw::KeyD, _, glfw::Repeat, _) => {
                self.p1.rot = Right;
            },
            glfw::KeyEvent(glfw::KeyI, _, glfw::Press, _) | glfw::KeyEvent(glfw::KeyI, _, glfw::Repeat, _) => {
                self.p2.dir = Forward;
            },
            glfw::KeyEvent(glfw::KeyK, _, glfw::Press, _) | glfw::KeyEvent(glfw::KeyK, _, glfw::Repeat, _) => {
                self.p2.dir = Backward;
            },
            glfw::KeyEvent(glfw::KeyJ, _, glfw::Press, _) | glfw::KeyEvent(glfw::KeyJ, _, glfw::Repeat, _) => {
                self.p2.rot = Left;
            },
            glfw::KeyEvent(glfw::KeyL, _, glfw::Press, _) | glfw::KeyEvent(glfw::KeyL, _, glfw::Repeat, _) => {
                self.p2.rot = Right;
            }
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
            MainMenu(ref sprites, _) => {
                sprites.get(0).borrow_mut().rot += 0.03;
            }
            _ => { }
        }

        accel(&mut self.p1);
        accel(&mut self.p2);
        rotate(&mut self.p1);
        rotate(&mut self.p2);

        if self.counter % 30 == 0 {
            println!("accel: {} accel_mod: {}, velocity: {}, x: {}, y: {}, rot: {}", self.p1.accel, self.p1.accel_mod, self.p1.velocity, self.p1.positionx, self.p1.positiony, self.p1.rotation);
            println!("accel: {} accel_mod: {}, velocity: {}, x: {}, y: {}, rot: {}", self.p2.accel, self.p2.accel_mod, self.p2.velocity, self.p2.positionx, self.p2.positiony, self.p2.rotation);
        }

        self.p1.dir = Still;
        self.p2.dir = Still;
        self.p1.rot = Norot;
        self.p2.rot = Norot;

        self.p1.sprite.borrow_mut().x = self.p1.positionx;
        self.p2.sprite.borrow_mut().x = self.p2.positionx;
        self.p1.sprite.borrow_mut().y = self.p1.positiony;
        self.p2.sprite.borrow_mut().y = self.p2.positiony;
        self.p1.sprite.borrow_mut().rot = self.p1.rotation;
        self.p2.sprite.borrow_mut().rot = self.p2.rotation;
        if physics::collide(&*self.p1.sprite.borrow(), &*self.p2.sprite.borrow()) {
            println!("Collision!")
        }

        self.counter += 1;
    }

    /// Render current contents
    pub fn render(&self) {
        self.engine.render();
    }
}
