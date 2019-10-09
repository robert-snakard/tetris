use crate::events::*;

use std::cell::RefCell;
use std::sync::Arc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;

const WIDTH: u32 = 100;
const HEIGHT: u32 = 200;

const HOOK_ID: &str = "tetris";

pub struct WebApp {
pub    events: Arc<RefCell<EventQueue>>,
pub    ctx: CanvasRenderingContext2d,
}

impl WebApp {
    pub fn new(hook_id: &str) -> WebApp {
        let mut events = Arc::new(RefCell::new(EventQueue::new()));
        let ctx = get_context(hook_id, &mut events).unwrap();

        WebApp { events, ctx }
    }
   
    pub fn request_animation_frame(&self, f: &Closure<dyn FnMut(f64)>) -> Result<(), JsValue> {
        window().unwrap().request_animation_frame(f.as_ref().unchecked_ref())?;
        Ok(())
    }

    pub fn get_next_event(&mut self) -> Option<Event> {
        self.events.borrow_mut().queue.pop()
    }
}

fn get_context(hook_id: &str, eq: &mut Arc<RefCell<EventQueue>>) -> Result<CanvasRenderingContext2d, JsValue> {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let canvas = document.get_element_by_id(hook_id).unwrap()
            .dyn_into::<HtmlCanvasElement>()?;
        canvas.set_width(100);
        canvas.set_height(200);
        let ctx = canvas.get_context("2d")?.unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        register_handler(&canvas, eq.clone());

        Ok(ctx)
}

fn register_handler(elem: &HtmlCanvasElement, eq: Arc<RefCell<EventQueue>>) -> Result<(), JsValue> {
    let deq = eq.clone();
    let ueq = eq.clone();

    let downhandler = move |ev: KeyboardEvent| {
        deq.borrow_mut().queue.push(Event::KeyDownEvent(ev))
    };
    let uphandler = move |ev: KeyboardEvent| {
        ueq.borrow_mut().queue.push(Event::KeyUpEvent(ev))
    };

    let downhandler = Closure::wrap(Box::new(downhandler) as Box<dyn FnMut(_)>);
    let uphandler = Closure::wrap(Box::new(uphandler) as Box<dyn FnMut(_)>);

    // Keyboard has to use win cause elements never get keyboard focus
    if let Some(win) = window() {
        win.add_event_listener_with_callback("keydown", downhandler.as_ref().unchecked_ref())?;
        win.add_event_listener_with_callback("keyup", uphandler.as_ref().unchecked_ref())?;

        downhandler.forget();
        uphandler.forget();
    }

    Ok(())
}
