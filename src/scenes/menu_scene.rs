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

	fn draw(&self)
	{
		clear_background(Color::new(0.933, 0.576, 1., 1.));
		let text = "Klk menol";
		draw_text(&text, 800., 300., 50., WHITE);
	}
}



