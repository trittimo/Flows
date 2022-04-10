pub mod gui_handler;
pub mod event_handler;

pub fn main() {
    event_handler::init();
    gui_handler::init();
}