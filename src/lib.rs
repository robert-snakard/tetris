mod events;

use events::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;

const WIDTH: u32 = 100;
const HEIGHT: u32 = 100;

const HOOK_ID: &str = "tetris";

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let app = WebApp::new(HOOK_ID);
    app.run()?;

    Ok(())
}

struct WebApp {
    events: EventQueue,
    ctx: CanvasRenderingContext2d,
}

impl WebApp {
    fn new(hook_id: &str) -> WebApp {
        let events = EventQueue::new();
        let ctx = get_context(hook_id, &events).unwrap();

        WebApp { events, ctx }
    }

    fn run(&self) -> Result<(), JsValue> {
        let canvas = self.ctx.canvas().unwrap();

        // demo canvas
        canvas.set_width(WIDTH);
        canvas.set_height(HEIGHT);
        self.ctx.fill_rect(0.0, 0.0, WIDTH as f64, HEIGHT as f64);

        Ok(())
    }
}

fn get_context(hook_id: &str, eq: &EventQueue) -> Result<CanvasRenderingContext2d, JsValue> {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let canvas = document.get_element_by_id(hook_id).unwrap()
            .dyn_into::<HtmlCanvasElement>()?;
        let ctx = canvas.get_context("2d")?.unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        register_handler(&canvas, eq);

        Ok(ctx)
}

fn register_handler(elem: &HtmlCanvasElement, eq: &EventQueue) -> Result<(), JsValue> {
    let handler = move |ev: KeyboardEvent| {
        window().unwrap().alert_with_message(&format!("hello {}", ev.key()));
    };

    let handler = Closure::wrap(Box::new(handler) as Box<FnMut(_)>);
    // Keyboard has to use win cause elements never get keyboard focus
    if let Some(win) = window() {
        win.add_event_listener_with_callback("keydown", handler.as_ref().unchecked_ref())?;
        handler.forget();
    }

    Ok(())
}
