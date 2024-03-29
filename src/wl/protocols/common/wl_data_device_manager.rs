//! The wl_data_device_manager is a singleton global object that provides access to
//! inter-client data transfer mechanisms such as copy-and-paste and drag-and-drop.
//! these mechanisms are tied to a wl_seat and this interface lets a client get a
//! wl_data_device corresponding to a wl_seat. depending on the version bound, the
//! objects created from the bound wl_data_device_manager object will have different
//! requirements for functioning properly. see wl_data_source.set_actions,
//! wl_data_offer.accept and wl_data_offer.finish for details.

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
use crate::wl::protocols::common::wl_shm_pool::WlShmPool;
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
    /// Create a new data source.
    CreateDataSource {
        /// Data source to create.
        id: NewId,
    },
    /// Create a new data device for a given seat.
    GetDataDevice {
        /// Data device to create.
        id: NewId,
        /// Seat associated with the data device.
        seat: WlSeat,
    },
}

impl Message for Request {
    fn into_raw(self, sender: ObjectId) -> RawMessage {
        match self {
            Request::CreateDataSource { id } => RawMessage {
                sender,
                opcode: Opcode(1),
                args: smallvec![id.into()],
            },
            Request::GetDataDevice { id, seat } => RawMessage {
                sender,
                opcode: Opcode(2),
                args: smallvec![id.into(), seat.into()],
            },
        }
    }

    fn from_raw(
        con: Rc<RefCell<dyn Connection>>,
        m: &RawMessage,
    ) -> Result<Request, DeserializeError> {
        match m.opcode {
            Opcode(1) => Ok(Request::CreateDataSource {
                id: from_payload!(NewId, m.args[0]),
            }),
            Opcode(2) => Ok(Request::GetDataDevice {
                id: from_payload!(NewId, m.args[0]),

                seat: from_object_payload!(WlSeat, con.clone(), m.args[1]),
            }),

            _ => Err(DeserializeError::UnknownOpcode),
        }
    }

    fn into_received_event(self, con: Rc<RefCell<dyn Connection>>, id: ObjectId) -> EventSet {
        panic!("not a event!");
    }

    fn into_received_request(self) -> RequestSet {
        RequestSet::WlDataDeviceManager(self)
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum DndAction {
    /// No action.
    None = 0,
    /// Copy action.
    Copy = 1,
    /// Move action.
    Move = 2,
    /// Ask action.
    Ask = 4,
}

impl Into<Payload> for DndAction {
    fn into(self) -> Payload {
        Payload::UInt(self as u32)
    }
}

impl From<u32> for DndAction {
    fn from(value: u32) -> DndAction {
        match value {
            0 => DndAction::None,
            1 => DndAction::Copy,
            2 => DndAction::Move,
            4 => DndAction::Ask,

            _ => unreachable!(),
        }
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
        EventSet::WlDataDeviceManager(WlDataDeviceManager::new(con, id), self)
    }

    fn into_received_request(self) -> RequestSet {
        panic!("not a request!");
    }
}

/// Data transfer interface.
#[derive(Clone)]
pub struct WlDataDeviceManager {
    con: Rc<RefCell<dyn Connection>>,
    object_id: ObjectId,
}

impl PartialEq for WlDataDeviceManager {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl core::fmt::Debug for WlDataDeviceManager {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "WlDataDeviceManager@{}", self.object_id.0)
    }
}

impl Into<Payload> for WlDataDeviceManager {
    fn into(self) -> Payload {
        Payload::ObjectId(self.id())
    }
}

impl Interface for WlDataDeviceManager {
    type Event = Event;
    type Request = Request;
    const NAME: &'static str = "wl_data_device_manager";
    const VERSION: u32 = 3;
    const PAYLOAD_TYPES: &'static [&'static [PayloadType]] = &[
        &[PayloadType::NewId],
        &[PayloadType::NewId, PayloadType::ObjectId],
    ];

    fn new(con: Rc<RefCell<dyn Connection>>, object_id: ObjectId) -> WlDataDeviceManager {
        WlDataDeviceManager { con, object_id }
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
