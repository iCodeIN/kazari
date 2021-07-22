use core::marker::PhantomData;

use alloc::boxed::Box;
use downcast_rs::{impl_downcast, Downcast};
use kazari_common::id_table::IdTable;

use crate::{client::Client, objects::get_message_handler_by_name};

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub struct ObjectId(usize);

pub struct ObjectRef<T: Object> {
    id: ObjectId,
    _pd: PhantomData<T>,
}

impl<T: Object> ObjectRef<T> {
    pub fn id_as_usize(&self) -> usize {
        self.id.0
    }
}

/// A Wayland object.
pub trait Object: Downcast {
    fn name(&self) -> &'static str;
}

impl_downcast!(Object);

pub trait MessageHandler {
    fn wl_buffer(&self, client: &mut Client, object_id: ObjectId) {}
}

pub struct ObjectMap {
    inner: IdTable<Box<dyn Object>>,
}

impl ObjectMap {
    pub fn new() -> ObjectMap {
        ObjectMap {
            inner: IdTable::new(),
        }
    }

    pub fn get_object_by_id(&self, index: ObjectId) -> Option<&dyn Object> {
        self.inner.get(index.0).map(|object| object.as_ref())
    }

    pub fn get_mut_by_id<T: Object>(&mut self, id: ObjectId) -> Option<&mut T> {
        self.get_mut(ObjectRef {
            id,
            _pd: PhantomData::<T>,
        })
    }

    pub fn get_mut<T: Object>(&mut self, index: ObjectRef<T>) -> Option<&mut T> {
        self.inner.get_mut(index.id_as_usize()).map(|object| {
            object
                .as_any_mut()
                .downcast_mut::<T>()
                .expect("failed to downcast into a Wayland object")
        })
    }

    pub fn get_mut2<T1: Object, T2: Object>(
        &mut self,
        index1: ObjectRef<T1>,
        index2: ObjectRef<T2>,
    ) -> (Option<&mut T1>, Option<&mut T2>) {
        let (v1, v2) = self
            .inner
            .get_mut2(index1.id_as_usize(), index2.id_as_usize());

        (
            v1.map(|object| {
                object
                    .as_any_mut()
                    .downcast_mut::<T1>()
                    .expect("failed to downcast into a Wayland object")
            }),
            v2.map(|object| {
                object
                    .as_any_mut()
                    .downcast_mut::<T2>()
                    .expect("failed to downcast into a Wayland object")
            }),
        )
    }
}
