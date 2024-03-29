//! The singleton global registry object. the server has a number of global objects
//! that are available to all clients. these objects typically represent an actual
//! object in the server (for example, an input device) or they are singleton
//! objects that provide extension functionality. when a client creates a registry
//! object, the registry object will emit a global event for each global currently
//! in the registry. globals come and go as a result of device or monitor hotplugs,
//! reconfiguration or other events, and the registry will send out global and
//! global_remove events to keep the client up to date with the changes. to mark the
//! end of the initial burst of events, the client can use the wl_display.sync
//! request immediately after calling wl_display.get_registry. a client can bind to
//! a global object by using the bind request. this creates a client-side handle
//! that lets the object emit events to the client and lets the client invoke
//! requests on the object.

//
//
//              GENERATED BY OUR WAYLAND-SCANNER. DO NOT EDIT!
//
//

#![allow(unused)]
#![allow(clippy::from_over_into)]
#![allow(clippy::match_single_binding)]

use crate::wl::{
    Array, Connection, Handle, Interface, Message, NewId, ObjectId, Opcode, Payload, PayloadType,
    RawMessage, SendError,
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

use crate::wl::protocols::common::wl_registry::*;
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

pub trait WlRegistryExt {
    /// Binds a new, client-created object to the server using the specified name as the
    /// identifier.
    fn bind(&self, name: u32, id: NewId) -> Result<(), SendError>;
}

impl WlRegistryExt for WlRegistry {
    /// Binds a new, client-created object to the server using the specified name as the
    /// identifier.
    fn bind(&self, name: u32, id: NewId) -> Result<(), SendError> {
        self.connection()
            .borrow_mut()
            .send(Request::Bind { name, id }.into_raw(self.id()))
    }
}
