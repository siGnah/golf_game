use rapier2d::prelude::*;

pub fn create_dynamic_body(
	rigid_body_set: &mut RigidBodySet,
	x: f32, y:f32
	) -> RigidBodyHandle
{
	let body = RigidBodyBuilder::dynamic()
			.translation(Vec2{x, y})
			.enabled(true)
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
	let collider = ColliderBuilder::ball(rad);

	//
	collider_set.insert_with_parent(collider, rigid_body_handle, rigid_body_set)
}

pub fn get_body(
	rigid_body_set: &mut RigidBodySet,
	rigid_body_handle: RigidBodyHandle
	) -> Option<&mut RigidBody>
{
	if let Some(body) = rigid_body_set.get_mut(rigid_body_handle)
	{
		return Some(body);
	}
	else
	{
		return None;
	}
}
