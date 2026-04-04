//hecs
use hecs::{World};

//macroquad
use macroquad::shapes::*;
use macroquad::input::*;
use macroquad::color::*;

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
	let radius:f32 = 10.;

	let body_handle = create_dynamic_body(rigid_body_set, 250., 150.);
	let collider_handle = create_ball_collider(
		rigid_body_set, body_handle, collider_set, radius
	);

	let _ball = world.spawn((
		DynamicBody{handle: body_handle},
		GBall{_handle:collider_handle, radius, charge:false}
	));
}

pub fn update_ball(world: &mut World, rigid_body_set: &mut RigidBodySet)
{
  line(world, rigid_body_set);
}

pub fn draw_ball(world: &World, rigid_body_set: &mut RigidBodySet)
{
	for (dy_b, ball) in world.query::<(&DynamicBody, &GBall)>().iter()
	{
    match get_body(rigid_body_set, dy_b.handle)
		{
	    Some(body) =>
			{
		    let pos = body.translation();
				draw_circle(pos.x, pos.y, ball.radius, WHITE);
			}
			None => {}
		}
	}
}

//他のこと
fn line(world: &mut World, rigid_body_set: &mut RigidBodySet)
{
	for (dy_b, ball) in world.query_mut::<(&mut DynamicBody, &mut GBall)>()
	{
		match get_body(rigid_body_set, dy_b.handle)
		{
			Some(body) =>
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

					let fx = mouse_x - pos_x;
					let fy = mouse_y - pos_y;

					println!("is sleeping: {}", body.is_sleeping());

					println!("fx: {}", fx);
					println!("fy: {}", fy);

					if is_mouse_button_released(MouseButton::Left)
					{	
						body.apply_impulse(vector![fx * 10., fy * 10.].into(), true);
						println!("force added");
						println!("pos: ({}, {})", pos_x, pos_y);
						ball.charge = false;
					}
				}
			}

			None => {}
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
