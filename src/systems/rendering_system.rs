use crate::{components::*, Gameplay};
use crate::constants::TILE_WIDTH;

use ggez::{Context, graphics::{self, DrawParam, Image}};
use specs::{Join, Read, ReadStorage, System};
use glam::Vec2;

pub struct RenderingSystem<'a> {
    pub context: &'a mut Context,
}



impl<'a> System<'a> for RenderingSystem<'a>{
    //data
    type SystemData = (Read<'a, Gameplay>, ReadStorage<'a, Position>, ReadStorage<'a, Renderable>);

    fn run(&mut self, data: Self::SystemData){
        let (gameplay,positions, renderables) = data;
        graphics::clear(self.context, graphics::Color { r: (0.95), g: (0.95), b: (0.95), a: (1.0) });

        //get all the renderables with their postions and sort by the postion z
        //this will allow us to have entities layered visually.
        let mut rendering_data = (&positions, &renderables).join()
        .collect::<Vec<_>>();
        rendering_data.sort_by_key(|&k| k.0.z);

        // iterate through all pairs of positions & renderables, load the image
        //and draw it at the specified position.
        for (postion, renderable) in rendering_data.iter(){
            let image = Image::new(self.context, 
            renderable.path.clone()).expect("expected image");
            let x = postion.x as f32 * TILE_WIDTH;
            let y = postion.y as f32 * TILE_WIDTH;

            let draw_params = DrawParam::new().dest(Vec2::new(x,y));
            graphics::draw(&mut self.context, &image, draw_params)
            .expect("expected render");
        }

        // Render any text
        self.draw_text(&gameplay.state.to_string(), 525.0, 80.0);
        self.draw_text(&gameplay.moves_count.to_string(), 525.0, 100.0);


        // fianlly, present the context, this will actually display everything 
        // on the screen
        graphics::present(&mut self.context).expect("expected to present");
    }   
}


impl  RenderingSystem <'_>{
    pub fn draw_text(&mut self,text_string: &str, x: f32, y: f32){
        let text = graphics::Text::new(text_string);
        let destination = Vec2::new(x, y);
        let color = Some(graphics::Color::new(0.0,0.0 ,0.0,1.0));
        let dimensions = Vec2::new(0.0, 20.0);

        graphics::queue_text(self.context, &text, dimensions, color);
        graphics::draw_queued_text(self.context,
             graphics::DrawParam::new().dest(destination), 
             None, graphics::FilterMode::Linear)
             .expect("expected drawing queued text");
    }
}