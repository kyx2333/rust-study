
use std::fmt::{self, Display};

use ggez::event::KeyCode;
use specs::World;


#[derive(Default)]
pub struct InputQueue{
    pub keys_pressed: Vec<KeyCode>,
}
pub fn register_resources(world: &mut World) {
    world.insert(InputQueue::default());
    world.insert(Gameplay::default())
}

#[derive(Default)]
pub struct Gameplay {
    pub state: GameplayState,
    pub moves_count: u32
}

pub enum  GameplayState {
    Playing,
    Won    
}

impl  Default for GameplayState {
    fn default() -> Self {
        Self::Playing
    }
}

impl  Display for GameplayState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            GameplayState::Playing => "Playing",
            GameplayState::Won => "Won"
        })?;

        Ok(())
    }
}