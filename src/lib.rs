use crate::engine::input::input;
use crate::engine::object::snake_game::SnakeGame;
use crate::engine::object_manager::om;
use crate::engine::timer::Timer;
use crate::engine::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;
use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext;

mod engine;

#[wasm_bindgen(start)]
async fn run() -> Result<(), JsValue> {
    Engine::init().await;

    let snake = SnakeGame::new([1000.0, 1000.0].into()).await;
    object!(snake);

    let mut timer = Timer::new();
    loop {
        if input().key_was_pressed("KeyQ") {
            exit();
        }

        // log!("-----------");
        let delta_time = timer.elapsed_reset() / 1000.0;
        // log!("FPS: {}", 1.0 / delta_time);

        // gl().clear_color(random() as f32, random() as f32, random() as f32, 1.0);
        gl().clear_color(0.1, 0.1, 0.1, 1.0);
        gl().clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        // let mut timer2 = Timer::new();
        om().tick(delta_time);
        // log!("Tick took {} milliseconds", timer2.elapsed_reset());
        om().draw();
        // log!("Draw took {} milliseconds", timer2.elapsed_reset());

        input().flush_pressed_map();

        async_std::task::sleep(Duration::from_micros(10)).await;
    }
}
