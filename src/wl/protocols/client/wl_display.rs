//! The core global object. this is a special singleton object. it is used for
//! internal wayland protocol features.

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

use crate::wl::protocols::common::wl_display::*;
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

pub trait WlDisplayExt {
    /// The sync request asks the server to emit the 'done' event on the returned
    /// wl_callback object. since requests are handled in-order and events are delivered
    /// in-order, this can be used as a barrier to ensure all previous requests and the
    /// resulting events have been handled. the object returned by this request will be
    /// destroyed by the compositor after the callback is fired and as such the client
    /// must not attempt to use it after that point. the callback_data passed in the
    /// callback is the event serial.
    fn sync(&self, callback: NewId) -> Result<(), SendError>;
    /// This request creates a registry object that allows the client to list and bind
    /// the global objects available from the compositor. it should be noted that the
    /// server side resources consumed in response to a get_registry request can only be
    /// released when the client disconnects, not when the client side proxy is
    /// destroyed. therefore, clients should invoke get_registry as infrequently as
    /// possible to avoid wasting memory.
    fn get_registry(&self, registry: NewId) -> Result<(), SendError>;
}

impl WlDisplayExt for WlDisplay {
    /// The sync request asks the server to emit the 'done' event on the returned
    /// wl_callback object. since requests are handled in-order and events are delivered
    /// in-order, this can be used as a barrier to ensure all previous requests and the
    /// resulting events have been handled. the object returned by this request will be
    /// destroyed by the compositor after the callback is fired and as such the client
    /// must not attempt to use it after that point. the callback_data passed in the
    /// callback is the event serial.
    fn sync(&self, callback: NewId) -> Result<(), SendError> {
        self.connection()
            .borrow_mut()
            .send(Request::Sync { callback }.into_raw(self.id()))
    }
    /// This request creates a registry object that allows the client to list and bind
    /// the global objects available from the compositor. it should be noted that the
    /// server side resources consumed in response to a get_registry request can only be
    /// released when the client disconnects, not when the client side proxy is
    /// destroyed. therefore, clients should invoke get_registry as infrequently as
    /// possible to avoid wasting memory.
    fn get_registry(&self, registry: NewId) -> Result<(), SendError> {
        self.connection()
            .borrow_mut()
            .send(Request::GetRegistry { registry }.into_raw(self.id()))
    }
}
