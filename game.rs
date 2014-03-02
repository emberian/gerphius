//! Game logic

use glfw;

pub struct Game {
    stuff: ()
}

impl Game {
    /// Initialize game state
    pub fn new() -> Game {
        Game { stuff: () }
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

    }
}
