//! The xdg_wm_base interface is exposed as a global object enabling clients to turn
//! their wl_surfaces into windows in a desktop environment. it defines the basic
//! functionality needed for clients and the compositor to create windows that can
//! be dragged, resized, maximized, etc, as well as creating transient windows such
//! as popup menus.

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

use crate::wl::protocols::common::xdg_wm_base::*;

pub trait XdgWmBaseExt {
    /// Destroy this xdg_wm_base object. destroying a bound xdg_wm_base object while
    /// there are surfaces still alive created by this xdg_wm_base object instance is
    /// illegal and will result in a protocol error.
    fn destroy(&self) -> Result<(), SendError>;
    /// Create a positioner object. a positioner object is used to position surfaces
    /// relative to some parent surface. see the interface description and
    /// xdg_surface.get_popup for details.
    fn create_positioner(&self, id: NewId) -> Result<(), SendError>;
    /// This creates an xdg_surface for the given surface. while xdg_surface itself is
    /// not a role, the corresponding surface may only be assigned a role extending
    /// xdg_surface, such as xdg_toplevel or xdg_popup. this creates an xdg_surface for
    /// the given surface. an xdg_surface is used as basis to define a role to a given
    /// surface, such as xdg_toplevel or xdg_popup. it also manages functionality shared
    /// between xdg_surface based surface roles. see the documentation of xdg_surface
    /// for more details about what an xdg_surface is and how it is used.
    fn get_xdg_surface(&self, id: NewId, surface: WlSurface) -> Result<(), SendError>;
    /// A client must respond to a ping event with a pong request or the client may be
    /// deemed unresponsive. see xdg_wm_base.ping.
    fn pong(&self, serial: u32) -> Result<(), SendError>;
}

impl XdgWmBaseExt for XdgWmBase {
    /// Destroy this xdg_wm_base object. destroying a bound xdg_wm_base object while
    /// there are surfaces still alive created by this xdg_wm_base object instance is
    /// illegal and will result in a protocol error.
    fn destroy(&self) -> Result<(), SendError> {
        self.connection()
            .borrow_mut()
            .send(Request::Destroy {}.into_raw(self.id()))
    }
    /// Create a positioner object. a positioner object is used to position surfaces
    /// relative to some parent surface. see the interface description and
    /// xdg_surface.get_popup for details.
    fn create_positioner(&self, id: NewId) -> Result<(), SendError> {
        self.connection()
            .borrow_mut()
            .send(Request::CreatePositioner { id }.into_raw(self.id()))
    }
    /// This creates an xdg_surface for the given surface. while xdg_surface itself is
    /// not a role, the corresponding surface may only be assigned a role extending
    /// xdg_surface, such as xdg_toplevel or xdg_popup. this creates an xdg_surface for
    /// the given surface. an xdg_surface is used as basis to define a role to a given
    /// surface, such as xdg_toplevel or xdg_popup. it also manages functionality shared
    /// between xdg_surface based surface roles. see the documentation of xdg_surface
    /// for more details about what an xdg_surface is and how it is used.
    fn get_xdg_surface(&self, id: NewId, surface: WlSurface) -> Result<(), SendError> {
        self.connection()
            .borrow_mut()
            .send(Request::GetXdgSurface { id, surface }.into_raw(self.id()))
    }
    /// A client must respond to a ping event with a pong request or the client may be
    /// deemed unresponsive. see xdg_wm_base.ping.
    fn pong(&self, serial: u32) -> Result<(), SendError> {
        self.connection()
            .borrow_mut()
            .send(Request::Pong { serial }.into_raw(self.id()))
    }
}