use hashbrown::HashMap;

use crate::{
    client::{Client, ClientId},
    object::{ObjectId, ObjectRef},
    objects::{get_message_handler_by_name, wl_surface::WlSurface},
    system::System,
};

pub struct Server<S: System> {
    system: S,
    clients: HashMap<ClientId, Client>,
}

impl<S: System> Server<S> {
    pub fn new(system: S) -> Server<S> {
        Server {
            system,
            clients: HashMap::new(),
        }
    }

    fn do_process_request(&mut self, client: &mut Client, object: ObjectId) {
        let client: &mut Client = { todo!() };
        let object_type = match client.objects().get_object_by_id(object) {
            Some(object) => object.name(),
            None => {
                trace!("unknown object ID: {:?}", object);
                return;
            }
        };

        let handler = get_message_handler_by_name(object_type);
        let this = unsafe { ObjectRef::from_id(object) };
        handler.wl_surface(client, this);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        struct DummySystem;
        impl System for DummySystem {}

        let mut server = Server::new(DummySystem);
    }
}
