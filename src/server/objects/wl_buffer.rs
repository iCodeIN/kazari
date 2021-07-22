use crate::object::Object;

pub struct WlBuffer {}

impl Object for WlBuffer {
    fn name(&self) -> &'static str {
        "wl_buffer"
    }
}
