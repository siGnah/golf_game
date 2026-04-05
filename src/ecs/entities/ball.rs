//hecs
use hecs::{World};

//macroquad
use macroquad::shapes::*;
use macroquad::input::*;
use macroquad::color::*;
use macroquad::math::clamp;

//rapier
use rapier2d::prelude::*;

//my stuff
use crate::ecs::components::*;
use crate::ecs::systems::*;

//ボールコンポーネント

pub fn create_ball(
	world: &mut World,
	rigid_body_set: &mut RigidBodySet,
	collider_set: &mut ColliderSet,
	x:f32, y:f32
)
{
	let radius:f32 = 20.;

	let body_handle = create_dynamic_body(rigid_body_set, x, y);
	let collider_handle = create_ball_collider(
		rigid_body_set, body_handle, collider_set, radius
	);

	let _ball = world.spawn((
		DynamicBody{handle: body_handle},
		GBall{_handle:collider_handle, radius, charge:false}
	));
}

//ボール更新処理
pub fn update_ball(
	world: &mut World,
	rigid_body_set: &mut RigidBodySet,
)
{
	ball_physics(world, rigid_body_set);
}

//ボール描画処理
pub fn draw_ball(world: &World, rigid_body_set: &RigidBodySet)
{
	for (dy_b, ball) in world.query::<(&DynamicBody, &GBall)>().iter()
	{


   	if let Some(body) = rigid_body_set.get(dy_b.handle)
		{
			let pos = body.translation();
			draw_circle(pos.x, pos.y, ball.radius, WHITE);

			if ball.charge
			{
				// let (mouse_x, mouse_y) = mouse_position();
				// draw_line(mouse_x + pos.x/2., mouse_y + pos.x/2., pos.x, pos.y, 3., BLUE);
			}
		}
	}
}

//他のこと
fn ball_physics(
	world: &mut World,
	rigid_body_set: &mut RigidBodySet,
)
{
	for (dy_b, ball) in world.query_mut::<(&mut DynamicBody, &mut GBall)>()
	{
		if let Some(body) = rigid_body_set.get_mut(dy_b.handle)
		{
			let pos_x = body.translation().x;
			let pos_y = body.translation().y;


			apply_impulse(body, ball, vector![pos_x, pos_y].into());
			keep_in_bouds(body, ball, vector![pos_x, pos_y].into());
		}
	}
}


fn mouse_hovers_ball(radius:f32, bx:f32, by:f32) -> bool
{
	let (mouse_x, mouse_y) = mouse_position();

	if (mouse_x >= bx - radius) && (mouse_x <= bx + radius)
	&& (mouse_y >= by - radius) && (mouse_y <= by + radius)
	{
		true
	}
	else
	{
		false
	}
}

fn apply_impulse(body: &mut RigidBody, ball: &mut GBall, pos: Vector)
{

	if is_mouse_button_pressed(MouseButton::Left)
	&& mouse_hovers_ball(ball.radius, pos.x, pos.y)
	{
		ball.charge = true;
	}

	if ball.charge
	{
		let (mouse_x, mouse_y) = mouse_position();
		let max_force:f32 = 1000.;

		let fx = clamp((mouse_x - pos.x) * -1., -max_force, max_force);
		let fy = clamp((mouse_y - pos.y) * -1., -max_force, max_force);

		// println!("force: ({}, {})", fx, fy);

		if is_mouse_button_released(MouseButton::Left)
		{
			body.apply_impulse(vector![fx * 6000., fy * 6000.].into(), true);
			ball.charge = false;
		}
	}
}

fn keep_in_bouds(body: &mut RigidBody, ball: &mut GBall, pos: Vector)
{
	let mut vx = body.linvel().x;
	let mut vy = body.linvel().y;

	if vx != 0. && vy != 0.
	{
		if pos.x - ball.radius <= 0. || pos.x + ball.radius >= 1600.
		{
			vx *= -1.;
			body.set_linvel(vector![vx, vy].into(), true);
		}

		if pos.y - ball.radius <= 0. || pos.y + ball.radius >= 900.
		{
			vy *= -1.;
			body.set_linvel(vector![vx, vy].into(), true);
		}
	}
}
