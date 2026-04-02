use macroquad::prelude::*;

const BG_COLOR:Color = Color::new(0.788, 0.851, 0.706, 1.);

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
    loop
    {
        clear_background(BG_COLOR);

        next_frame().await
    }
}
