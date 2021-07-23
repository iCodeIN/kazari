use crate::{
    client::Client,
    object::{MessageHandler, Object, ObjectRef},
};

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

impl MessageHandler for Handler {
    fn wl_surface(&self, client: &mut Client, this: ObjectRef<WlSurface>) {
        let buffer = { client.objects().get(&this).unwrap().buffer.clone() };
        let (this, buffer) = client.objects_mut().get_mut2(&this, &buffer);
    }
}
