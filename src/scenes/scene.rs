pub enum SceneKind
{
	Menu,
	Game	
}

pub trait Scene
{
	fn update(&mut self) -> Option<SceneKind>;
	fn draw(&self);
}