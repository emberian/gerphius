//! Game logic

use glfw;
use render;
use std::rc::Rc;
use gl::types::GLint;
use std::cell::RefCell;
use render::{Tex, Sprite, Engine};

pub struct Game {
    state: GameState,
    engine: render::Engine,
}

struct PlayState;

pub enum GameState {
    MainMenu(~[Rc<RefCell<Sprite>>]),
    Playing(PlayState),
    Leaderboard(~[Rc<RefCell<Sprite>>]),
}

impl Game {
    /// Initialize game state
    pub fn new(width: GLint, height: GLint) -> Game {
        let mut e = Engine::new(width, height);

        let start = e.load_texture("menu.start", "menu.start.png");
        let hscores = e.load_texture("menu.highscore", "menu.highscore.png");
        let quit = e.load_texture("menu.quit", "menu.quit.png");

        let start = Sprite::new(100, 200, 20, 200, start);
        let hscores = Sprite::new(100, 160, 20, 200, hscores);
        let quit = Sprite::new(100, 120, 20, 200, quit);

        let sprites = ~[e.add_sprite(start), e.add_sprite(hscores), e.add_sprite(quit)];

        Game {
            state: MainMenu(sprites),
            engine: e
        }
    }

    /// Handle input
    pub fn handle_event(&mut self, window: &glfw::Window,
                        (_time, event): (f64, glfw::WindowEvent)) {
    }

    /// Update the game simulation
    pub fn tick(&mut self) {

    }

    /// Render current contents
    pub fn render(&self) {
        self.engine.render();
    }
}
