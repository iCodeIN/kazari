//! An output describes part of the compositor geometry. the compositor works in the
//! 'compositor coordinate system' and an output corresponds to a rectangular area
//! in that space that is actually visible. this typically corresponds to a monitor
//! that displays part of the compositor space. this object is published as global
//! during start up, or when a monitor is hotplugged.

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

use crate::wl::protocols::common::wl_output::*;
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

pub trait WlOutputExt {
    /// The geometry event describes geometric properties of the output. the event is
    /// sent when binding to the output object and whenever any of the properties
    /// change. the physical size can be set to zero if it doesn't make sense for this
    /// output (e.g. for projectors or virtual outputs). note: wl_output only advertises
    /// partial information about the output position and identification. some
    /// compositors, for instance those not implementing a desktop-style output layout
    /// or those exposing virtual outputs, might fake this information. instead of using
    /// x and y, clients should use xdg_output.logical_position. instead of using make
    /// and model, clients should use xdg_output.name and xdg_output.description.
    fn geometry(
        &self,
        x: i32,
        y: i32,
        physical_width: i32,
        physical_height: i32,
        subpixel: Subpixel,
        make: String,
        model: String,
        transform: Transform,
    ) -> Result<(), SendError>;
    /// The mode event describes an available mode for the output. the event is sent
    /// when binding to the output object and there will always be one mode, the current
    /// mode. the event is sent again if an output changes mode, for the mode that is
    /// now current. in other words, the current mode is always the last mode that was
    /// received with the current flag set. non-current modes are deprecated. a
    /// compositor can decide to only advertise the current mode and never send other
    /// modes. clients should not rely on non-current modes. the size of a mode is given
    /// in physical hardware units of the output device. this is not necessarily the
    /// same as the output size in the global compositor space. for instance, the output
    /// may be scaled, as described in wl_output.scale, or transformed, as described in
    /// wl_output.transform. clients willing to retrieve the output size in the global
    /// compositor space should use xdg_output.logical_size instead. the vertical
    /// refresh rate can be set to zero if it doesn't make sense for this output (e.g.
    /// for virtual outputs). clients should not use the refresh rate to schedule
    /// frames. instead, they should use the wl_surface.frame event or the presentation-
    /// time protocol. note: this information is not always meaningful for all outputs.
    /// some compositors, such as those exposing virtual outputs, might fake the refresh
    /// rate or the size.
    fn mode(&self, flags: Mode, width: i32, height: i32, refresh: i32) -> Result<(), SendError>;
    /// This event is sent after all other properties have been sent after binding to
    /// the output object and after any other property changes done after that. this
    /// allows changes to the output properties to be seen as atomic, even if they
    /// happen via multiple events.
    fn done(&self) -> Result<(), SendError>;
    /// This event contains scaling geometry information that is not in the geometry
    /// event. it may be sent after binding the output object or if the output scale
    /// changes later. if it is not sent, the client should assume a scale of 1. a scale
    /// larger than 1 means that the compositor will automatically scale surface buffers
    /// by this amount when rendering. this is used for very high resolution displays
    /// where applications rendering at the native resolution would be too small to be
    /// legible. it is intended that scaling aware clients track the current output of a
    /// surface, and if it is on a scaled output it should use
    /// wl_surface.set_buffer_scale with the scale of the output. that way the
    /// compositor can avoid scaling the surface, and the client can supply a higher
    /// detail image.
    fn scale(&self, factor: i32) -> Result<(), SendError>;
}

impl WlOutputExt for WlOutput {
    /// The geometry event describes geometric properties of the output. the event is
    /// sent when binding to the output object and whenever any of the properties
    /// change. the physical size can be set to zero if it doesn't make sense for this
    /// output (e.g. for projectors or virtual outputs). note: wl_output only advertises
    /// partial information about the output position and identification. some
    /// compositors, for instance those not implementing a desktop-style output layout
    /// or those exposing virtual outputs, might fake this information. instead of using
    /// x and y, clients should use xdg_output.logical_position. instead of using make
    /// and model, clients should use xdg_output.name and xdg_output.description.
    fn geometry(
        &self,
        x: i32,
        y: i32,
        physical_width: i32,
        physical_height: i32,
        subpixel: Subpixel,
        make: String,
        model: String,
        transform: Transform,
    ) -> Result<(), SendError> {
        self.connection().borrow_mut().send(
            Event::Geometry {
                x,
                y,
                physical_width,
                physical_height,
                subpixel,
                make,
                model,
                transform,
            }
            .into_raw(self.id()),
        )
    }
    /// The mode event describes an available mode for the output. the event is sent
    /// when binding to the output object and there will always be one mode, the current
    /// mode. the event is sent again if an output changes mode, for the mode that is
    /// now current. in other words, the current mode is always the last mode that was
    /// received with the current flag set. non-current modes are deprecated. a
    /// compositor can decide to only advertise the current mode and never send other
    /// modes. clients should not rely on non-current modes. the size of a mode is given
    /// in physical hardware units of the output device. this is not necessarily the
    /// same as the output size in the global compositor space. for instance, the output
    /// may be scaled, as described in wl_output.scale, or transformed, as described in
    /// wl_output.transform. clients willing to retrieve the output size in the global
    /// compositor space should use xdg_output.logical_size instead. the vertical
    /// refresh rate can be set to zero if it doesn't make sense for this output (e.g.
    /// for virtual outputs). clients should not use the refresh rate to schedule
    /// frames. instead, they should use the wl_surface.frame event or the presentation-
    /// time protocol. note: this information is not always meaningful for all outputs.
    /// some compositors, such as those exposing virtual outputs, might fake the refresh
    /// rate or the size.
    fn mode(&self, flags: Mode, width: i32, height: i32, refresh: i32) -> Result<(), SendError> {
        self.connection().borrow_mut().send(
            Event::Mode {
                flags,
                width,
                height,
                refresh,
            }
            .into_raw(self.id()),
        )
    }
    /// This event is sent after all other properties have been sent after binding to
    /// the output object and after any other property changes done after that. this
    /// allows changes to the output properties to be seen as atomic, even if they
    /// happen via multiple events.
    fn done(&self) -> Result<(), SendError> {
        self.connection()
            .borrow_mut()
            .send(Event::Done {}.into_raw(self.id()))
    }
    /// This event contains scaling geometry information that is not in the geometry
    /// event. it may be sent after binding the output object or if the output scale
    /// changes later. if it is not sent, the client should assume a scale of 1. a scale
    /// larger than 1 means that the compositor will automatically scale surface buffers
    /// by this amount when rendering. this is used for very high resolution displays
    /// where applications rendering at the native resolution would be too small to be
    /// legible. it is intended that scaling aware clients track the current output of a
    /// surface, and if it is on a scaled output it should use
    /// wl_surface.set_buffer_scale with the scale of the output. that way the
    /// compositor can avoid scaling the surface, and the client can supply a higher
    /// detail image.
    fn scale(&self, factor: i32) -> Result<(), SendError> {
        self.connection()
            .borrow_mut()
            .send(Event::Scale { factor }.into_raw(self.id()))
    }
}
