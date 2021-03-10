//! A popup surface is a short-lived, temporary surface. it can be used to implement
//! for example menus, popovers, tooltips and other similar user interface concepts.
//! a popup can be made to take an explicit grab. see xdg_popup.grab for details.
//! when the popup is dismissed, a popup_done event will be sent out, and at the
//! same time the surface will be unmapped. see the xdg_popup.popup_done event for
//! details. explicitly destroying the xdg_popup object will also dismiss the popup
//! and unmap the surface. clients that want to dismiss the popup when another
//! surface of their own is clicked should dismiss the popup using the destroy
//! request. a newly created xdg_popup will be stacked on top of all previously
//! created xdg_popup surfaces associated with the same xdg_toplevel. the parent of
//! an xdg_popup must be mapped (see the xdg_surface description) before the
//! xdg_popup itself. the client must call wl_surface.commit on the corresponding
//! wl_surface for the xdg_popup state to take effect.

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

use crate::wl::protocols::common::xdg_popup::*;
use crate::wl::protocols::common::xdg_positioner::XdgPositioner;
use crate::wl::protocols::common::xdg_surface::XdgSurface;
use crate::wl::protocols::common::xdg_toplevel::XdgToplevel;
use crate::wl::protocols::common::xdg_wm_base::XdgWmBase;

pub trait XdgPopupExt {
    /// This event asks the popup surface to configure itself given the configuration.
    /// the configured state should not be applied immediately. see
    /// xdg_surface.configure for details. the x and y arguments represent the position
    /// the popup was placed at given the xdg_positioner rule, relative to the upper
    /// left corner of the window geometry of the parent surface. for version 2 or
    /// older, the configure event for an xdg_popup is only ever sent once for the
    /// initial configuration. starting with version 3, it may be sent again if the
    /// popup is setup with an xdg_positioner with set_reactive requested, or in
    /// response to xdg_popup.reposition requests.
    fn configure(&self, x: i32, y: i32, width: i32, height: i32) -> Result<(), SendError>;
    /// The popup_done event is sent out when a popup is dismissed by the compositor.
    /// the client should destroy the xdg_popup object at this point.
    fn popup_done(&self) -> Result<(), SendError>;
    /// The repositioned event is sent as part of a popup configuration sequence,
    /// together with xdg_popup.configure and lastly xdg_surface.configure to notify the
    /// completion of a reposition request. the repositioned event is to notify about
    /// the completion of a xdg_popup.reposition request. the token argument is the
    /// token passed in the xdg_popup.reposition request. immediately after this event
    /// is emitted, xdg_popup.configure and xdg_surface.configure will be sent with the
    /// updated size and position, as well as a new configure serial. the client should
    /// optionally update the content of the popup, but must acknowledge the new popup
    /// configuration for the new position to take effect. see xdg_surface.ack_configure
    /// for details.
    fn repositioned(&self, token: u32) -> Result<(), SendError>;
}

impl XdgPopupExt for XdgPopup {
    /// This event asks the popup surface to configure itself given the configuration.
    /// the configured state should not be applied immediately. see
    /// xdg_surface.configure for details. the x and y arguments represent the position
    /// the popup was placed at given the xdg_positioner rule, relative to the upper
    /// left corner of the window geometry of the parent surface. for version 2 or
    /// older, the configure event for an xdg_popup is only ever sent once for the
    /// initial configuration. starting with version 3, it may be sent again if the
    /// popup is setup with an xdg_positioner with set_reactive requested, or in
    /// response to xdg_popup.reposition requests.
    fn configure(&self, x: i32, y: i32, width: i32, height: i32) -> Result<(), SendError> {
        self.connection().borrow_mut().send(
            Event::Configure {
                x,
                y,
                width,
                height,
            }
            .into_raw(self.id()),
        )
    }
    /// The popup_done event is sent out when a popup is dismissed by the compositor.
    /// the client should destroy the xdg_popup object at this point.
    fn popup_done(&self) -> Result<(), SendError> {
        self.connection()
            .borrow_mut()
            .send(Event::PopupDone {}.into_raw(self.id()))
    }
    /// The repositioned event is sent as part of a popup configuration sequence,
    /// together with xdg_popup.configure and lastly xdg_surface.configure to notify the
    /// completion of a reposition request. the repositioned event is to notify about
    /// the completion of a xdg_popup.reposition request. the token argument is the
    /// token passed in the xdg_popup.reposition request. immediately after this event
    /// is emitted, xdg_popup.configure and xdg_surface.configure will be sent with the
    /// updated size and position, as well as a new configure serial. the client should
    /// optionally update the content of the popup, but must acknowledge the new popup
    /// configuration for the new position to take effect. see xdg_surface.ack_configure
    /// for details.
    fn repositioned(&self, token: u32) -> Result<(), SendError> {
        self.connection()
            .borrow_mut()
            .send(Event::Repositioned { token }.into_raw(self.id()))
    }
}
