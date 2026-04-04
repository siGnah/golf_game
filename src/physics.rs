use rapier2d::prelude::*;

pub struct PhysicsState {
	pub physics_pipeline: PhysicsPipeline,
	pub gravity: Vector,
	pub integration_parameters: IntegrationParameters,
	pub island_manager: IslandManager,
	pub broad_phase: BroadPhaseBvh,
	pub narrow_phase: NarrowPhase,
	pub rigid_body_set: RigidBodySet,
	pub collider_set: ColliderSet,
	pub impulse_joint_set: ImpulseJointSet,
	pub multibody_joint_set: MultibodyJointSet,
	pub ccd_solver: CCDSolver,
}

