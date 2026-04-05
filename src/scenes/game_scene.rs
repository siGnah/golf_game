//hecs
use hecs::World;

//macroquad
use macroquad::window::*;
use macroquad::input::*;
use macroquad::color::Color;
use macroquad::time::*;

//rapier
use rapier2d::prelude::*;

//my stuff
use super::scene::{Scene, SceneKind};
use crate::ecs::entities::ball::*;
use crate::physics::*;



pub struct GameScene
{
	world: World,
	physics: PhysicsState,
	acumulator: f32,
	fixed_dt:f32,
	bg_color: Color,
}

impl GameScene
{
	pub fn new() -> Self
	{
		let mut game = Self
		{
			world: World::new(),
			physics: PhysicsState::new(),
			acumulator: 0.,
			fixed_dt: 1./60.,

			bg_color: Color::new(0.788, 0.851, 0.706, 1.),
		};

		//ボールを作る
		create_ball(
			&mut game.world,
			&mut game.physics.rb_set,
			&mut game.physics.collider_set,
			150., 200.
		);

		create_ball(
		   &mut game.world,
			&mut game.physics.rb_set,
			&mut game.physics.collider_set,
			800., 500.
		);

		//ゲームを返す
		game
	}
}

impl Scene for GameScene
{
	fn update(&mut self) -> Option<SceneKind>
	{
		let _dt:f32 = get_frame_time();

		//一時
		if is_key_pressed(KeyCode::Enter)
		{
			return Some(SceneKind::Menu)
		}

		update_ball(&mut self.world,&mut self.physics.rb_set);

		let frame_time = get_frame_time().min(0.25);
		self.acumulator += frame_time;

		while self.acumulator >= self.fixed_dt
		{
			self.physics.integration_parameters.dt = self.fixed_dt;
			self.physics.update();
			self.acumulator -= self.fixed_dt;
		}

		None
	}

	fn draw(&self)
	{
		clear_background(self.bg_color);
		draw_fps();
		draw_ball(&self.world, &self.physics.rb_set);
	}
}
