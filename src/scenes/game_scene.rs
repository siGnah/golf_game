use hecs::World;
use macroquad::prelude::*;
use super::scene::{Scene, SceneKind};

pub struct GameScene
{
	world: World,
}

impl GameScene
{
	pub fn new() -> Self
	{
		let world = World::new();

		Self {world}
	}
}

impl Scene for GameScene {
	fn update(&mut self) -> Option<SceneKind>
	{
		let _dt:f32 = get_frame_time();

		None
	}	

	fn draw(&self)
	{
		clear_background(Color::new(0.788, 0.851, 0.706, 1.));
	}
}