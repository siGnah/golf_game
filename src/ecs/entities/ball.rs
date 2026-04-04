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
	collider_set: &mut ColliderSet
)
{
	let radius:f32 = 20.;

	let body_handle = create_dynamic_body(rigid_body_set, 500., 400.);
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
	impulse_ball(world, rigid_body_set);
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
fn impulse_ball(
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

			if is_mouse_button_pressed(MouseButton::Left)
			&& mouse_hovers_ball(ball.radius, pos_x, pos_y)
			{
				ball.charge = true;
			}
			else

			if ball.charge
			{
				let (mouse_x, mouse_y) = mouse_position();
				let max_force:f32 = 500.;

				let fx = clamp((mouse_x - pos_x) * -1., -max_force, max_force);
				let fy = clamp((mouse_y - pos_y) * -1., -max_force, max_force);

				// println!("force: ({}, {})", fx, fy);

				if is_mouse_button_released(MouseButton::Left)
				{	
					body.apply_impulse(vector![fx * 100., fy * 100.].into(), true);
					ball.charge = false;
				}
			}
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
