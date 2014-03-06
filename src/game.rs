//! Game logic

use glfw;
use render;
use gl::types::GLint;

pub struct Game {
    state: GameState,
    engine: render::Engine,
}

pub enum GameState {
    MainMenu,
    Playing,
    LeaderBoard
}

impl Game {
    /// Initialize game state
    pub fn new(width: GLint, height: GLint) -> Game {
        Game {
            state: MainMenu,
            engine: render::Engine::new(width, height)
        }
    }

    /// Handle input
    pub fn handle_event(&mut self, window: &glfw::Window,
                        (timestamp, event): (f64, glfw::WindowEvent)) {
    }

    /// Update the game simulation
    pub fn tick(&mut self) {
        debug!("game tick");

    }

    /// Render current contents
    pub fn render(&self) {
        self.engine.render();
    }
}
