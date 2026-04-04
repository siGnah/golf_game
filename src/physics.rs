use rapier2d::prelude::*;

pub struct PhysicsState {
	pub physics_pipeline: PhysicsPipeline,
	pub gravity: Vector,
	pub integration_parameters: IntegrationParameters,
	pub island_manager: IslandManager,
	pub broad_phase: BroadPhaseBvh,
	pub narrow_phase: NarrowPhase,
	pub rb_set: RigidBodySet,
	pub collider_set: ColliderSet,
	pub impulse_joint_set: ImpulseJointSet,
	pub multibody_joint_set: MultibodyJointSet,
	pub ccd_solver: CCDSolver,
}

pub fn update_physics(physics: &mut PhysicsState)
{
	physics.physics_pipeline.step(
		physics.gravity,
		&physics.integration_parameters,
		&mut physics.island_manager,
		&mut physics.broad_phase,
		&mut physics.narrow_phase,
		&mut physics.rb_set,
		&mut physics.collider_set,
		&mut physics.impulse_joint_set,
		&mut physics.multibody_joint_set,
		&mut physics.ccd_solver,
		&(),
		&(),
	);
}

