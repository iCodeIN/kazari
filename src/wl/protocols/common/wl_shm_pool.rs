//! The wl_shm_pool object encapsulates a piece of memory shared between the
//! compositor and client. through the wl_shm_pool object, the client can allocate
//! shared memory wl_buffer objects. all objects created through the same pool share
//! the same underlying mapped memory. reusing the mapped memory avoids the
//! setup/teardown overhead and is useful when interactively resizing a surface or
//! for many small buffers.

//
//
//              GENERATED BY OUR WAYLAND-SCANNER. DO NOT EDIT!
//
//

#![allow(unused)]
#![allow(clippy::from_over_into)]
#![allow(clippy::match_single_binding)]

use crate::wl::protocols::common::{EventSet, RequestSet};
use crate::wl::{
    Array, Connection, DeserializeError, Handle, Interface, Message, NewId, ObjectId, Opcode,
    Payload, PayloadType, RawMessage, SendError,
};
use alloc::rc::Rc;
use alloc::string::String;
use core::cell::RefCell;
use smallvec::smallvec;

use crate::wl::protocols::common::wl_buffer::WlBuffer;
use crate::wl::protocols::common::wl_callback::WlCallback;
use crate::wl::protocols::common::wl_compositor::WlCompositor;
use crate::wl::protocols::common::wl_data_device::WlDataDevice;
use crate::wl::protocols::common::wl_data_device_manager::WlDataDeviceManager;
use crate::wl::protocols::common::wl_data_offer::WlDataOffer;
use crate::wl::protocols::common::wl_data_source::WlDataSource;
use crate::wl::protocols::common::wl_display::WlDisplay;
use crate::wl::protocols::common::wl_keyboard::WlKeyboard;
use crate::wl::protocols::common::wl_output::WlOutput;
use crate::wl::protocols::common::wl_pointer::WlPointer;
use crate::wl::protocols::common::wl_region::WlRegion;
use crate::wl::protocols::common::wl_registry::WlRegistry;
use crate::wl::protocols::common::wl_seat::WlSeat;
use crate::wl::protocols::common::wl_shell::WlShell;
use crate::wl::protocols::common::wl_shell_surface::WlShellSurface;
use crate::wl::protocols::common::wl_shm::WlShm;

use crate::wl::protocols::common::wl_subcompositor::WlSubcompositor;
use crate::wl::protocols::common::wl_subsurface::WlSubsurface;
use crate::wl::protocols::common::wl_surface::WlSurface;
use crate::wl::protocols::common::wl_touch::WlTouch;
use crate::wl::protocols::common::xdg_popup::XdgPopup;
use crate::wl::protocols::common::xdg_positioner::XdgPositioner;
use crate::wl::protocols::common::xdg_surface::XdgSurface;
use crate::wl::protocols::common::xdg_toplevel::XdgToplevel;
use crate::wl::protocols::common::xdg_wm_base::XdgWmBase;

macro_rules! from_optional_object_payload {
    ($ty:ident, $con:expr, $v:expr) => {
        match ($v).clone() {
            Payload::ObjectId(id) if id.is_null() => None,
            Payload::ObjectId(id) => Some($ty::new($con, id)),
            _ => return Err(DeserializeError::UnexpectedType), // Abort deserializing.
        }
    };
}

macro_rules! from_object_payload {
    ($ty:ident, $con:expr, $v:expr) => {
        match ($v).clone() {
            Payload::ObjectId(id) if id.is_null() => return Err(DeserializeError::ObjectIsNull),
            Payload::ObjectId(id) => $ty::new($con, id),
            _ => return Err(DeserializeError::UnexpectedType),
        }
    };
}

macro_rules! from_payload {
    ($ty:ident, $v:expr) => {
        match ($v).clone() {
            Payload::$ty(value) => value.into(),
            _ => return Err(DeserializeError::UnexpectedType),
        }
    };
}

#[derive(Debug)]
pub enum Request {
    /// Create a wl_buffer object from the pool. the buffer is created offset bytes into
    /// the pool and has width and height as specified. the stride argument specifies
    /// the number of bytes from the beginning of one row to the beginning of the next.
    /// the format is the pixel format of the buffer and must be one of those advertised
    /// through the wl_shm.format event. a buffer will keep a reference to the pool it
    /// was created from so it is valid to destroy the pool immediately after creating a
    /// buffer from it.
    CreateBuffer {
        /// Buffer to create.
        id: NewId,
        /// Buffer byte offset within the pool.
        offset: i32,
        /// Buffer width, in pixels.
        width: i32,
        /// Buffer height, in pixels.
        height: i32,
        /// Number of bytes from the beginning of one row to the beginning of the next row.
        stride: i32,
        /// Buffer pixel format.
        format: super::super::common::wl_shm::Format,
    },
    /// Destroy the shared memory pool. the mmapped memory will be released when all
    /// buffers that have been created from this pool are gone.
    Destroy {},
    /// This request will cause the server to remap the backing memory for the pool from
    /// the file descriptor passed when the pool was created, but using the new size.
    /// this request can only be used to make the pool bigger.
    Resize {
        /// New size of the pool, in bytes.
        size: i32,
    },
}

impl Message for Request {
    fn into_raw(self, sender: ObjectId) -> RawMessage {
        match self {
            Request::CreateBuffer {
                id,
                offset,
                width,
                height,
                stride,
                format,
            } => RawMessage {
                sender,
                opcode: Opcode(1),
                args: smallvec![
                    id.into(),
                    offset.into(),
                    width.into(),
                    height.into(),
                    stride.into(),
                    format.into()
                ],
            },
            Request::Destroy {} => RawMessage {
                sender,
                opcode: Opcode(2),
                args: smallvec![],
            },
            Request::Resize { size } => RawMessage {
                sender,
                opcode: Opcode(3),
                args: smallvec![size.into()],
            },
        }
    }

    fn from_raw(
        con: Rc<RefCell<dyn Connection>>,
        m: &RawMessage,
    ) -> Result<Request, DeserializeError> {
        match m.opcode {
            Opcode(1) => Ok(Request::CreateBuffer {
                id: from_payload!(NewId, m.args[0]),

                offset: from_payload!(Int, m.args[1]),

                width: from_payload!(Int, m.args[2]),

                height: from_payload!(Int, m.args[3]),

                stride: from_payload!(Int, m.args[4]),

                format: from_payload!(UInt, m.args[5]),
            }),
            Opcode(2) => Ok(Request::Destroy {}),
            Opcode(3) => Ok(Request::Resize {
                size: from_payload!(Int, m.args[0]),
            }),

            _ => Err(DeserializeError::UnknownOpcode),
        }
    }

    fn into_received_event(self, con: Rc<RefCell<dyn Connection>>, id: ObjectId) -> EventSet {
        panic!("not a event!");
    }

    fn into_received_request(self) -> RequestSet {
        RequestSet::WlShmPool(self)
    }
}

#[derive(Debug)]
pub enum Event {}

impl Message for Event {
    fn into_raw(self, sender: ObjectId) -> RawMessage {
        match self {}
    }

    fn from_raw(
        con: Rc<RefCell<dyn Connection>>,
        m: &RawMessage,
    ) -> Result<Event, DeserializeError> {
        match m.opcode {
            _ => Err(DeserializeError::UnknownOpcode),
        }
    }

    fn into_received_event(self, con: Rc<RefCell<dyn Connection>>, id: ObjectId) -> EventSet {
        EventSet::WlShmPool(WlShmPool::new(con, id), self)
    }

    fn into_received_request(self) -> RequestSet {
        panic!("not a request!");
    }
}

/// A shared memory pool.
#[derive(Clone)]
pub struct WlShmPool {
    con: Rc<RefCell<dyn Connection>>,
    object_id: ObjectId,
}

impl PartialEq for WlShmPool {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl core::fmt::Debug for WlShmPool {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "WlShmPool@{}", self.object_id.0)
    }
}

impl Into<Payload> for WlShmPool {
    fn into(self) -> Payload {
        Payload::ObjectId(self.id())
    }
}

impl Interface for WlShmPool {
    type Event = Event;
    type Request = Request;
    const NAME: &'static str = "wl_shm_pool";
    const VERSION: u32 = 1;
    const PAYLOAD_TYPES: &'static [&'static [PayloadType]] = &[
        &[
            PayloadType::NewId,
            PayloadType::Int,
            PayloadType::Int,
            PayloadType::Int,
            PayloadType::Int,
            PayloadType::UInt,
        ],
        &[],
        &[PayloadType::Int],
    ];

    fn new(con: Rc<RefCell<dyn Connection>>, object_id: ObjectId) -> WlShmPool {
        WlShmPool { con, object_id }
    }

    fn connection(&self) -> &Rc<RefCell<dyn Connection>> {
        &self.con
    }

    fn id(&self) -> ObjectId {
        self.object_id
    }

    fn as_new_id(&self) -> NewId {
        NewId(self.object_id.0)
    }
}
