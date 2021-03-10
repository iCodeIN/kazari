//! There is one wl_data_device per seat which can be obtained from the global
//! wl_data_device_manager singleton. a wl_data_device provides access to inter-
//! client data transfer mechanisms such as copy-and-paste and drag-and-drop.

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

use crate::wl::protocols::common::wl_data_device::*;
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
use crate::wl::protocols::common::xdg_wm_base::XdgWmBase;

pub trait WlDataDeviceExt {
    /// The data_offer event introduces a new wl_data_offer object, which will
    /// subsequently be used in either the data_device.enter event (for drag-and-drop)
    /// or the data_device.selection event (for selections). immediately following the
    /// data_device_data_offer event, the new data_offer object will send out
    /// data_offer.offer events to describe the mime types it offers.
    fn data_offer(&self, id: NewId) -> Result<(), SendError>;
    /// This event is sent when an active drag-and-drop pointer enters a surface owned
    /// by the client. the position of the pointer at enter time is provided by the x
    /// and y arguments, in surface-local coordinates.
    fn enter(
        &self,
        serial: u32,
        surface: WlSurface,
        x: f32,
        y: f32,
        id: Option<WlDataOffer>,
    ) -> Result<(), SendError>;
    /// This event is sent when the drag-and-drop pointer leaves the surface and the
    /// session ends. the client must destroy the wl_data_offer introduced at enter time
    /// at this point.
    fn leave(&self) -> Result<(), SendError>;
    /// This event is sent when the drag-and-drop pointer moves within the currently
    /// focused surface. the new position of the pointer is provided by the x and y
    /// arguments, in surface-local coordinates.
    fn motion(&self, time: u32, x: f32, y: f32) -> Result<(), SendError>;
    /// The event is sent when a drag-and-drop operation is ended because the implicit
    /// grab is removed. the drag-and-drop destination is expected to honor the last
    /// action received through wl_data_offer.action, if the resulting action is "copy"
    /// or "move", the destination can still perform wl_data_offer.receive requests, and
    /// is expected to end all transfers with a wl_data_offer.finish request. if the
    /// resulting action is "ask", the action will not be considered final. the drag-
    /// and-drop destination is expected to perform one last wl_data_offer.set_actions
    /// request, or wl_data_offer.destroy in order to cancel the operation.
    fn drop(&self) -> Result<(), SendError>;
    /// The selection event is sent out to notify the client of a new wl_data_offer for
    /// the selection for this device. the data_device.data_offer and the
    /// data_offer.offer events are sent out immediately before this event to introduce
    /// the data offer object. the selection event is sent to a client immediately
    /// before receiving keyboard focus and when a new selection is set while the client
    /// has keyboard focus. the data_offer is valid until a new data_offer or null is
    /// received or until the client loses keyboard focus. the client must destroy the
    /// previous selection data_offer, if any, upon receiving this event.
    fn selection(&self, id: Option<WlDataOffer>) -> Result<(), SendError>;
}

impl WlDataDeviceExt for WlDataDevice {
    /// The data_offer event introduces a new wl_data_offer object, which will
    /// subsequently be used in either the data_device.enter event (for drag-and-drop)
    /// or the data_device.selection event (for selections). immediately following the
    /// data_device_data_offer event, the new data_offer object will send out
    /// data_offer.offer events to describe the mime types it offers.
    fn data_offer(&self, id: NewId) -> Result<(), SendError> {
        self.connection()
            .borrow_mut()
            .send(Event::DataOffer { id }.into_raw(self.id()))
    }
    /// This event is sent when an active drag-and-drop pointer enters a surface owned
    /// by the client. the position of the pointer at enter time is provided by the x
    /// and y arguments, in surface-local coordinates.
    fn enter(
        &self,
        serial: u32,
        surface: WlSurface,
        x: f32,
        y: f32,
        id: Option<WlDataOffer>,
    ) -> Result<(), SendError> {
        self.connection().borrow_mut().send(
            Event::Enter {
                serial,
                surface,
                x,
                y,
                id,
            }
            .into_raw(self.id()),
        )
    }
    /// This event is sent when the drag-and-drop pointer leaves the surface and the
    /// session ends. the client must destroy the wl_data_offer introduced at enter time
    /// at this point.
    fn leave(&self) -> Result<(), SendError> {
        self.connection()
            .borrow_mut()
            .send(Event::Leave {}.into_raw(self.id()))
    }
    /// This event is sent when the drag-and-drop pointer moves within the currently
    /// focused surface. the new position of the pointer is provided by the x and y
    /// arguments, in surface-local coordinates.
    fn motion(&self, time: u32, x: f32, y: f32) -> Result<(), SendError> {
        self.connection()
            .borrow_mut()
            .send(Event::Motion { time, x, y }.into_raw(self.id()))
    }
    /// The event is sent when a drag-and-drop operation is ended because the implicit
    /// grab is removed. the drag-and-drop destination is expected to honor the last
    /// action received through wl_data_offer.action, if the resulting action is "copy"
    /// or "move", the destination can still perform wl_data_offer.receive requests, and
    /// is expected to end all transfers with a wl_data_offer.finish request. if the
    /// resulting action is "ask", the action will not be considered final. the drag-
    /// and-drop destination is expected to perform one last wl_data_offer.set_actions
    /// request, or wl_data_offer.destroy in order to cancel the operation.
    fn drop(&self) -> Result<(), SendError> {
        self.connection()
            .borrow_mut()
            .send(Event::Drop {}.into_raw(self.id()))
    }
    /// The selection event is sent out to notify the client of a new wl_data_offer for
    /// the selection for this device. the data_device.data_offer and the
    /// data_offer.offer events are sent out immediately before this event to introduce
    /// the data offer object. the selection event is sent to a client immediately
    /// before receiving keyboard focus and when a new selection is set while the client
    /// has keyboard focus. the data_offer is valid until a new data_offer or null is
    /// received or until the client loses keyboard focus. the client must destroy the
    /// previous selection data_offer, if any, upon receiving this event.
    fn selection(&self, id: Option<WlDataOffer>) -> Result<(), SendError> {
        self.connection()
            .borrow_mut()
            .send(Event::Selection { id }.into_raw(self.id()))
    }
}