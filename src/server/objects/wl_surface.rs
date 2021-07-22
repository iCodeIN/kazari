use crate::object::{MessageHandler, Object, ObjectRef};

use super::wl_buffer::WlBuffer;

pub struct WlSurface {
    buffer: ObjectRef<WlBuffer>,
}

impl Object for WlSurface {
    fn name(&self) -> &'static str {
        "wl_surface"
    }
}

pub struct Handler;

impl MessageHandler for Handler {}
