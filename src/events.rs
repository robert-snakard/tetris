pub enum Event {
    KeyDownEvent(web_sys::KeyboardEvent),
    KeyUpEvent(web_sys::KeyboardEvent),
}

pub struct EventQueue {
    pub queue: Vec<Event>,
}

impl EventQueue {
    pub fn new() -> EventQueue {
        EventQueue {
            queue: Vec::new(),
        }
    }
}
