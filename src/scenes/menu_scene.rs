use macroquad::prelude::*;
use super::scene::{Scene, SceneKind};

pub struct MenuScene;

impl MenuScene
{
	pub fn new() -> Self
	{
		Self
	}
}

impl Scene for MenuScene
{
	fn update(&mut self) -> Option<SceneKind>
	{
		//一時
		if is_key_pressed(KeyCode::Enter)
		{
			return Some(SceneKind::Game);
		}

		//return none
		None
	}

	fn draw(&mut self)
	{

	}
}



