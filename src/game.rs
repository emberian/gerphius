//! Game logic

use std;
use libc;
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

impl PlayState {
    fn new() -> PlayState {
        PlayState
    }
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
        let hscores = e.load_texture("menu.highscore", "menu.highscore.png");
        let quit = e.load_texture("menu.quit", "menu.quit.png");

        let start = Sprite::new(100, 200, 20, 200, start);
        let hscores = Sprite::new(100, 160, 20, 200, hscores);
        let quit = Sprite::new(100, 120, 20, 200, quit);

        let sprites = vec!(e.add_sprite(start), e.add_sprite(hscores), e.add_sprite(quit));
        // e.remove_sprite(sprites.get(0).clone());

        Game {
            state: MainMenu(sprites, Start),
            engine: e
        }
    }

    /// Handle input
    pub fn handle_event(&mut self, window: &glfw::Window,
                        (_time, event): (f64, glfw::WindowEvent)) {
        match &mut self.state {
            &MainMenu(ref mut sprites, ref sel) => {
                /*
                let mut next = match *sel {
                    Start => Leaderboard,
                    Leaderboard => Quit,
                    Quit => Start
                };
                let mut prev = match *sel {
                    Start => Quit,
                    Leaderboard => Start,
                    Quit => Leaderboard
                };
                match event {
                    glfw::KeyEvent(glfw::KeyUp, _, glfw::Press, _) => { }
                    glfw::KeyEvent(glfw::KeyDown, _, glfw::Press, _) => { std::mem::swap(&mut next, &mut prev); },
                    glfw::KeyEvent(glfw::KeyEnter, _, glfw::Press, _) => {
                        self.state = match *sel {
                            Start => Playing(PlayState::new()),
                            Leaderboard => LeaderboardState(Vec::new()),
                            Quit => unsafe { libc::exit(1) }
                        };
                        return
                    },
                    _ => { next = *sel; prev = *sel; }
                }
                let (next, prev) = (next as uint, prev as uint);


                // make the selected item a wee bit bigger
                let mut s = sprites.get_mut(prev).borrow_mut();
                s.height -= 10;
                s.width -= 10;
                s.x += 5;
                s.y += 5;

                let mut s = sprites.get_mut(next).borrow_mut();
                s.height += 10;
                s.width += 10;
                s.x -= 5;
                s.y -= 5;
                */
            },
            _ => println!("whatevs")
        }
    }

    /// Update the game simulation
    pub fn tick(&mut self) {

    }

    /// Render current contents
    pub fn render(&self) {
        self.engine.render();
    }
}
