use rapier2d::prelude::*;

pub struct DynamicBody{pub handle: RigidBodyHandle}
pub struct GBall
{
	pub _handle: ColliderHandle,
	pub radius: f32,
	pub charge: bool,
}