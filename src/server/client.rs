use crate::object::ObjectMap;

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub struct ClientId(usize);

pub struct Client {
    objects: ObjectMap,
}

impl Client {
    pub fn objects(&self) -> &ObjectMap {
        &self.objects
    }

    pub fn objects_mut(&mut self) -> &mut ObjectMap {
        &mut self.objects
    }

    pub fn register_object(&mut self) {}
}
