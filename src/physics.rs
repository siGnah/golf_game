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

impl PhysicsState {
   pub fn new() -> Self
   {
   	Self
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
   	}
   }

   pub fn update(&mut self)
   {
   	self.physics_pipeline.step
   	(
			self.gravity,
			&self.integration_parameters,
			&mut self.island_manager,
			&mut self.broad_phase,
			&mut self.narrow_phase,
			&mut self.rb_set,
			&mut self.collider_set,
			&mut self.impulse_joint_set,
			&mut self.multibody_joint_set,
			&mut self.ccd_solver,
			&(),
			&(),
		)
   }
}