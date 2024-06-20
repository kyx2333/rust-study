use crate::components::*;
use crate::constants::*;
use crate::resources::{InputQueue, Gameplay};
use ggez::event::KeyCode;
use specs::{world::Index, Entities, Join, ReadStorage, System, Write, WriteStorage};

use std::collections::HashMap;


pub struct InputSystem {}

impl<'a> System<'a> for InputSystem {

    type SystemData = (
        Write<'a, InputQueue>,
        Entities<'a>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Immovable>,
        ReadStorage<'a, Movable>,
        Write<'a ,Gameplay>
    );
    
    fn run(&mut self, data: Self::SystemData){
        
        //println!("{:?}",data);

        let (mut input_queue, entites, mut positions, players,immovables,movables,mut gameplay) = data;
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
                    KeyCode::Left => (position.x, 0, true),
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
        // we've just moved, so let's increase the number of move
        if to_move.len() > 0{
            gameplay.moves_count += 1;
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
   