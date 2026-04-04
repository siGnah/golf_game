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
	bg_color: Color,
}

impl GameScene
{
	pub fn new() -> Self
	{
		let mut game = Self
		{
			world: World::new(),
			physics: PhysicsState
			{
				physics_pipeline: PhysicsPipeline::new(),
				gravity: vector![0.0, 0.0].into(),
				integration_parameters: IntegrationParameters::default(),
				island_manager: IslandManager::new(),
				broad_phase: BroadPhaseBvh::new(),
				narrow_phase: NarrowPhase::new(),
				rb_set: RigidBodySet::new(),
				collider_set: ColliderSet::new(),
				impulse_joint_set: ImpulseJointSet::new(),
				multibody_joint_set: MultibodyJointSet::new(),
				ccd_solver: CCDSolver::new(),
			},
			bg_color: Color::new(0.788, 0.851, 0.706, 1.),
		};

		//ボールを作る
		create_ball(
			&mut game.world,
			&mut game.physics.rb_set,
			&mut game.physics.collider_set
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
		update_physics(&mut self.physics);
		None
	}

	fn draw(&self)
	{
		clear_background(self.bg_color);
		draw_fps();
		draw_ball(&self.world, &self.physics.rb_set);
	}
}
