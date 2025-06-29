// src/lib.rs

use std::fmt::format;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, window};

#[wasm_bindgen]
pub struct CounterGame {
    context: CanvasRenderingContext2d,
    count: u32,
}

#[wasm_bindgen]
impl CounterGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<CounterGame, JsValue> {
        let document = window().unwrap().document().unwrap();
        let canvas: HtmlCanvasElement = document.get_element_by_id("canvas").unwrap().dyn_into()?;
        let context = canvas.get_context("2d")?.unwrap().dyn_into()?;
        Ok(CounterGame { context, count: 0 })
    }

    #[wasm_bindgen]
    pub fn draw(&self) {
        self.context.clear_rect(0.0, 0.0, 300.0, 200.0);
        self.context.set_font("20px sans-serif");
        self.context.set_fill_style_str("black");
        self.context
            .fill_text(&format!("Count: {}", self.count), 100.0, 100.0)
            .unwrap();
    }

    #[wasm_bindgen]
    pub fn increment(&mut self) {
        self.count += 1;
        self.draw();

        let msg = format!("{{\"type\":\"count\",\"value\":{}}}", self.count);
        let _ = window()
            .unwrap()
            .parent()
            .unwrap()
            .unwrap()
            .post_message(&JsValue::from_str(&msg), "*");
    }
    
    #[wasm_bindgen]
    pub fn decrement(&mut self) {
        if self.count > 0 {
            self.count -= 1;
        }
        self.draw();

        let msg = format!("{{\"type\":\"count\",\"value\":{}}}", self.count);
        let _ = window()
            .unwrap()
            .parent()
            .unwrap()
            .unwrap()
            .post_message(&JsValue::from_str(&msg), "*");
    }
}
