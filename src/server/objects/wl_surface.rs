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
    fn wl_buffer(&self, client: &mut Client, object_id: ObjectId) {
        let this = client
            .objects_mut()
            .get_mut_by_id::<WlSurface>(object_id)
            .unwrap();
    }
}
