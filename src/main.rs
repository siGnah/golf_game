mod ecs;
mod scenes;
mod physics;

use macroquad::window::*;
use scenes::make_scene;
use scenes::scene::{Scene, SceneKind};

//画面制定
fn window_conf() -> Conf
{
	Conf 
	{
	   window_title: "ゴルフゲーム".to_owned(),
	   window_width: 1600,
	   window_height: 900,
	   window_resizable: false,
	   ..Default::default()
	}
}

#[macroquad::main(window_conf)]
async fn main()
{
   let mut current_scene: Box<dyn Scene> = make_scene(SceneKind::Game);

 	loop 
 	{

      if let Some(next) = current_scene.update() 
     	{	
      	current_scene = make_scene(next)
     	}

     current_scene.draw();

     next_frame().await
	}
}
