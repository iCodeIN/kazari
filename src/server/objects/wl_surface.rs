use crate::{
    client::Client,
    object::{MessageHandler, Object, ObjectId, ObjectRef},
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
        let this = client.objects_mut().get_mut(this).unwrap();
    }
}
