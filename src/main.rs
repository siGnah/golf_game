mod scenes;

use macroquad::prelude::*;
use scenes::make_scene;
use scenes::scene::{Scene, SceneKind};

// const BG_COLOR:Color = Color::new(0.788, 0.851, 0.706, 1.);


//画面制定
fn window_conf() -> Conf
{
    Conf
    {
        window_title: "ゴルフゲーム".to_owned(),
        window_width: 800,
        window_height: 450,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() 
{
    let mut current_scene:Box<dyn Scene> = make_scene(SceneKind::Menu);

    loop
    {
        if let Some(next) = current_scene.update()
        {
            current_scene = make_scene(next)
        }

        current_scene.draw();

        next_frame().await
    }
}
