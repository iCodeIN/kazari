//! The global interface exposing sub-surface compositing capabilities. a
//! wl_surface, that has sub-surfaces associated, is called the parent surface. sub-
//! surfaces can be arbitrarily nested and create a tree of sub-surfaces. the root
//! surface in a tree of sub-surfaces is the main surface. the main surface cannot
//! be a sub-surface, because sub-surfaces must always have a parent. a main surface
//! with its sub-surfaces forms a (compound) window. for window management purposes,
//! this set of wl_surface objects is to be considered as a single window, and it
//! should also behave as such. the aim of sub-surfaces is to offload some of the
//! compositing work within a window from clients to the compositor. a prime example
//! is a video player with decorations and video in separate wl_surface objects.
//! this should allow the compositor to pass yuv video buffer processing to
//! dedicated overlay hardware when possible.

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

use crate::wl::protocols::common::wl_subcompositor::*;
use crate::wl::protocols::common::wl_subsurface::WlSubsurface;
use crate::wl::protocols::common::wl_surface::WlSurface;
use crate::wl::protocols::common::wl_touch::WlTouch;
use crate::wl::protocols::common::xdg_popup::XdgPopup;
use crate::wl::protocols::common::xdg_positioner::XdgPositioner;
use crate::wl::protocols::common::xdg_surface::XdgSurface;
use crate::wl::protocols::common::xdg_toplevel::XdgToplevel;
use crate::wl::protocols::common::xdg_wm_base::XdgWmBase;

pub trait WlSubcompositorExt {
    /// Informs the server that the client will not be using this protocol object
    /// anymore. this does not affect any other objects, wl_subsurface objects included.
    fn destroy(&self) -> Result<(), SendError>;
    /// Create a sub-surface interface for the given surface, and associate it with the
    /// given parent surface. this turns a plain wl_surface into a sub-surface. the to-
    /// be sub-surface must not already have another role, and it must not have an
    /// existing wl_subsurface object. otherwise a protocol error is raised. adding sub-
    /// surfaces to a parent is a double-buffered operation on the parent (see
    /// wl_surface.commit). the effect of adding a sub-surface becomes visible on the
    /// next time the state of the parent surface is applied. this request modifies the
    /// behaviour of wl_surface.commit request on the sub-surface, see the documentation
    /// on wl_subsurface interface.
    fn get_subsurface(
        &self,
        id: NewId,
        surface: WlSurface,
        parent: WlSurface,
    ) -> Result<(), SendError>;
}

impl WlSubcompositorExt for WlSubcompositor {
    /// Informs the server that the client will not be using this protocol object
    /// anymore. this does not affect any other objects, wl_subsurface objects included.
    fn destroy(&self) -> Result<(), SendError> {
        self.connection()
            .borrow_mut()
            .send(Request::Destroy {}.into_raw(self.id()))
    }
    /// Create a sub-surface interface for the given surface, and associate it with the
    /// given parent surface. this turns a plain wl_surface into a sub-surface. the to-
    /// be sub-surface must not already have another role, and it must not have an
    /// existing wl_subsurface object. otherwise a protocol error is raised. adding sub-
    /// surfaces to a parent is a double-buffered operation on the parent (see
    /// wl_surface.commit). the effect of adding a sub-surface becomes visible on the
    /// next time the state of the parent surface is applied. this request modifies the
    /// behaviour of wl_surface.commit request on the sub-surface, see the documentation
    /// on wl_subsurface interface.
    fn get_subsurface(
        &self,
        id: NewId,
        surface: WlSurface,
        parent: WlSurface,
    ) -> Result<(), SendError> {
        self.connection().borrow_mut().send(
            Request::GetSubsurface {
                id,
                surface,
                parent,
            }
            .into_raw(self.id()),
        )
    }
}
