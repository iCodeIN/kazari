use alloc::boxed::Box;

use crate::object::MessageHandler;

pub mod wl_buffer;
pub mod wl_surface;

pub fn get_message_handler_by_name(name: &str) -> Box<dyn MessageHandler> {
    match name {
        "wl_surface" => Box::new(wl_surface::Handler),
        _ => unreachable!(),
    }
}
