pub enum SceneKind
{
	Game,
	Menu
}

pub trait Scene
{
	fn update(&mut self) -> Option<SceneKind>;
	fn draw(&mut self);
}
