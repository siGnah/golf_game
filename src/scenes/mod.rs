pub mod scene;
pub mod game_scene;
pub mod menu_scene;

use scene::{Scene, SceneKind};
use game_scene::{GameScene};
use menu_scene::{MenuScene};

pub fn make_scene(kind: SceneKind) -> Box<dyn Scene> {
    match kind {
        SceneKind::Menu => Box::new(MenuScene::new()),
        SceneKind::Game => Box::new(GameScene::new()),
    }
}
