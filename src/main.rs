
use glam::Vec2;
use ggez::{
    conf, Context, GameResult,
    event::{self, KeyCode, KeyMods}, 
    graphics::{self, DrawParam, Image}};
use specs::{
    join::Join, Builder, Component, ReadStorage, RunNow, 
    System, VecStorage, World, WorldExt,
    Write, WriteStorage, NullStorage, Entities, world::Index
};


use std::{collections::HashMap, path};

const TILE_WIDTH: f32 = 32.0;
const MAP_WIDTH: u8 = 8;
const MAP_HEIGHT: u8 = 9;

#[derive( Debug, Component, Clone, Copy)]
#[storage(VecStorage)]
pub struct Position{
    x: u8,
    y: u8,
    z: u8
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
    path: String
 }

 #[derive(Component)]
 #[storage(VecStorage)]
 pub struct Wall {}

 #[derive(Component)]
 #[storage(VecStorage)]
pub struct Player {}

#[derive(Component)]
#[storage(VecStorage)]
 pub struct Box {}

#[derive(Component)]
#[storage(VecStorage)]
pub struct  BoxSpot{}

#[derive(Default)]
pub struct InputQueue{
    pub keys_pressed: Vec<KeyCode>,
}

pub struct RenderingSystem<'a> {
    context: &'a mut Context,
}

#[derive(Component,Default)]
#[storage(NullStorage)]
pub struct  Movable;

#[derive(Component,Default)]
#[storage(NullStorage)]
pub struct  Immovable;

pub fn register_components(world: &mut World){
    world.register::<Position>();
    world.register::<Renderable>();
    world.register::<Player>();
    world.register::<Wall>();
    world.register::<Box>();
    world.register::<BoxSpot>();
 }

 pub fn register_resources(world: &mut World) {
    world.insert(InputQueue::default())
}

pub fn create_wall(world: &mut World, position: Position){
    world
        .create_entity()
        .with(Position { z: 10, ..position})
        .with(Renderable {
            path: "/images/wall.png".to_string()
        })
        .with(Wall {})
        .with(Immovable)
        .build();
}
pub fn create_floor(world: &mut World,position: Position){
    world
        .create_entity()
        .with(Position { z: 5, ..position})
        .with(Renderable {
            path: "/images/floor.png".to_string()
        })
        .build();
}
pub fn create_box(world: &mut World,position: Position){
    world
        .create_entity()
        .with(Position { z: 10, ..position})
        .with(Renderable {
            path: "/images/box.png".to_string()
        })
        .with(Box {})
        .with(Movable)
        .build();
}
pub fn create_box_spot(world: &mut World,position: Position){
    world
        .create_entity()
        .with(Position { z: 9, ..position})
        .with(Renderable {
            path: "/images/box_spot.png".to_string()
        })
        .with(BoxSpot {})
        .build();
}

pub fn create_player(world: &mut World,position: Position){
    world
        .create_entity()
        .with(Position { z: 10, ..position})
        .with(Renderable {
            path: "/images/player.png".to_string()
        })
        .with(Player {})
        .with(Movable)
        .build();
}
struct Game{
    world: World,
}


impl<'a> System<'a> for RenderingSystem<'a>{
    //data
    type SystemData = (ReadStorage<'a, Position>,ReadStorage<'a,Renderable>);

    fn run(&mut self, data: Self::SystemData){
        let (positions, renderables) = data;
        graphics::clear(self.context, graphics::Color { r: (0.95), g: (0.95), b: (0.95), a: (1.0) });

        //get all the renderables with their postions and sort by the postion z
        //this will allow us to have entities layered visually.
        let mut rendering_data = (&positions, &renderables).join()
        .collect::<Vec<_>>();
        rendering_data.sort_by_key(|&k| k.0.z);

        // iterate through all pairs of positions & renderables, load the image
        //and draw it at the specified position.
        for (postion, renderable) in rendering_data.iter(){
            let image = Image::new(&mut self.context, 
            renderable.path.clone()).expect("expected image");
            let x = postion.x as f32 * TILE_WIDTH;
            let y = postion.y as f32 * TILE_WIDTH;

            let draw_params = DrawParam::new().dest(Vec2::new(x,y));
            graphics::draw(&mut self.context, &image, draw_params)
            .expect("expected render");
        }
        // fianlly, present the context, this will actually display everything 
        // on the screen
        graphics::present(&mut self.context).expect("expected to present");
    }   
}

impl event::EventHandler<ggez::GameError> for Game{
    fn update(&mut self, _context: &mut Context) -> GameResult {
        // Run input system
        {
            let mut is = InputSystem {};
            is.run_now(&self.world);
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


pub fn load_map(world: &mut World, map_string: String){
    //read all lines
    let rows: Vec<&str> = map_string.trim().split('\n').map(|x| x.trim()).collect();

    for (y,row) in rows.iter().enumerate(){
        let columns: Vec<_> = row.split(' ').collect();

        for (x, column) in columns.iter().enumerate(){
            // create the postion at which to create something on the map
            let position =  Position{
                x: x as u8,
                y: y as u8,
                z: 0  // we willl get the z from the factory functions
            };
            // figure out what object we should create
            match *column{
                "." => create_floor(world, position),
                "W" => {
                    create_floor(world, position);
                    create_wall(world, position);
                }
                "P" => {
                    create_floor(world, position);
                    create_player(world, position);
                }
                "B" => {
                    create_floor(world, position);
                    create_box(world, position);
                }
                "S" => {
                    create_floor(world, position);
                    create_box_spot(world, position);
                }
                "N" => (),
                c => panic!("unrecoginzed map item {}",c),
            }
        }
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

pub struct InputSystem {}

impl<'a> System<'a> for InputSystem {

    type SystemData = (
        Write<'a, InputQueue>,
        Entities<'a>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Immovable>,
        ReadStorage<'a, Movable>
    );
    
    fn run(&mut self, data: Self::SystemData){
        
        //println!("{:?}",data);

        let (mut input_queue, entites, mut positions, players,immovables,movables) = data;
        // let mut iterator = (&mut positions, &players).join();
        
        // println!("{}",iterator.count());
        let mut to_move = Vec::new();
        for (position,_player) in (&positions, &players).join(){
            //get the first key pressedlet Some(key) = input_queue.keys_pressed.pop()
            
            if let Some(key) = input_queue.keys_pressed.pop() {
                // apply the key to the position
                //println!("position y{},position x {}",position.y,position.x);

                let mov: HashMap<(u8, u8), Index> = (&entites, &movables, &positions)
                    .join()
                    .map(|t| ((t.2.x, t.2.y), t.0.id()))
                    .collect::<HashMap<_, _>>();

                let immov: HashMap<(u8,u8),Index> = (&entites,&immovables, &positions).join()
                .map(|t| ((t.2.x,t.2.y),t.0.id()))
                .collect::<HashMap<_,_>>();
                
                //now iterate through current position to the end of the map    
                //on the correct axis nad check what needs to move
                let (start, end, is_x) = match key {
                    KeyCode::Up => (position.y, 0, false),
                    KeyCode::Down => (position.y, MAP_HEIGHT, false),
                    KeyCode::Left => (position.x, 0, false),
                    KeyCode::Right => (position.x, MAP_WIDTH, true),    
                    _ => continue
                };
                
                let range = if start < end {
                    (start..=end).collect::<Vec<_>>()
                }else {
                    (end..=start).rev().collect::<Vec<_>>()
                };
                
                for x_or_y in range {
                    let pos = if is_x {
                        (x_or_y,position.y)
                    }else{
                        (position.x,x_or_y)
                    };

                    // find a movable
                    //if it exists, we try to move it and contiune
                    //if it doesn't exist, we continue and try to find an immovable instead

                    match mov.get(&pos) {
                        Some(id) => to_move.push((key, id.clone())),
                        None => {
                            // find an immovable
                            //if it exists. we need to stop and not move anything
                            //if it doesn't exist,we stop because we found a gap
                            match immov.get(&pos) {
                                Some(_id) => to_move.clear(),
                                None => break
                            }
                        }
                    }
                }
               
            }
        }
         // now actually move what needs to be moved
         for(key,id) in to_move {
            let position = positions.get_mut(entites.entity(id));
            if let Some(position) = position{
                match key {
                    KeyCode::Up => position.y -= 1,
                    KeyCode::Down => position.y += 1,
                    KeyCode::Left => position.x -= 1,
                    KeyCode::Right => position.x += 1,
                    _ => (),
                }
            }
        }
    }
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
