
use ggez::{conf, event::{self, KeyCode, KeyMods}, Context, GameResult};
use specs::{RunNow, World, WorldExt};
use std::path;

mod components;
mod constants;
mod entities;
mod map;
mod resources;
mod systems;

use crate::components::*;
use crate::map::*;
use crate::resources::*;
use crate::systems::*;

struct Game{
    world: World,
}



impl event::EventHandler<ggez::GameError> for Game{
    fn update(&mut self, _context: &mut Context) -> GameResult {
        // Run input system
        {
            let mut is = InputSystem {};
            is.run_now(&self.world);
        }
        // run gameplay state system
        {
            let mut gss = GameplayStateSystem{};

            gss.run_now(&self.world);
        }

        Ok(())
    }
    fn draw(&mut self,context: &mut Context) -> GameResult{
            // Render game entities
        {
            let mut rs = RenderingSystem { context};
            rs.run_now(&self.world);   
        }
        Ok(())
    }
    fn key_down_event(
            &mut self,
            _context: &mut Context,
            keycode: KeyCode,
            _keymods: KeyMods,
            _repeat: bool,
        ) {
        println!("key pressed: {:?}",keycode);
        let mut input_queue = self.world.write_resource::<InputQueue>();
        input_queue.keys_pressed.push(keycode);
        println!("InputQueue: {:?}", input_queue.keys_pressed.len());
    }
}



//resources

pub fn initialize_level(world: &mut World){
    const MAP: &str = "
    N N W W W W W W
    W W W . . . . W
    W . . . B . . W
    W . . . . . . W 
    W . P . . . . W
    W . . . . . . W
    W . . S . . . W
    W . . . . . . W
    W W W W W W W W
    ";
    load_map(world, MAP.to_string());
}
 
pub fn main() -> GameResult{
    let mut world = World::new();
    register_components(&mut world);
    register_resources(&mut world);
    initialize_level(&mut world);

    let context_builder = ggez::ContextBuilder::new("rust_sokoban","sokoban")
        .window_setup(conf::WindowSetup::default().title("Rust Sokoban!"))
        .window_mode(conf::WindowMode::default().dimensions(800.0,600.0))
        .add_resource_path(path::PathBuf::from("./resources"));

    let (context, event_loop) = context_builder.build()?;
    // create the game state
    let game = Game { world };
    // run the main event loop
    event::run(context,event_loop,game)
}
