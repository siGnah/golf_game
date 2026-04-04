use rapier2d::prelude::*;

pub fn create_dynamic_body(
	rigid_body_set: &mut RigidBodySet,
	x: f32, y:f32
	) -> RigidBodyHandle
{
	let body = RigidBodyBuilder::dynamic()
			.translation(Vec2{x, y})
			.linear_damping(0.08)
			.build();

	//ハンドルを返す
	rigid_body_set.insert(body)
}

pub fn create_ball_collider(
	rigid_body_set: &mut RigidBodySet,
	rigid_body_handle: RigidBodyHandle,
	collider_set:&mut ColliderSet,
	rad:f32
	) -> ColliderHandle
{
	let collider = ColliderBuilder::ball(rad)
			.friction(0.7)
			.build();

	//
	collider_set.insert_with_parent(collider, rigid_body_handle, rigid_body_set)
}

