use hashbrown::HashMap;

use crate::{
    client::{Client, ClientId},
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
