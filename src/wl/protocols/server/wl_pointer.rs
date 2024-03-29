//! The wl_pointer interface represents one or more input devices, such as mice,
//! which control the pointer location and pointer_focus of a seat. the wl_pointer
//! interface generates motion, enter and leave events for the surfaces that the
//! pointer is located over, and button and axis events for button presses, button
//! releases and scrolling.

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

use crate::wl::protocols::common::wl_pointer::*;
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

pub trait WlPointerExt {
    /// Notification that this seat's pointer is focused on a certain surface. when a
    /// seat's focus enters a surface, the pointer image is undefined and a client
    /// should respond to this event by setting an appropriate pointer image with the
    /// set_cursor request.
    fn enter(
        &self,
        serial: u32,
        surface: WlSurface,
        surface_x: f32,
        surface_y: f32,
    ) -> Result<(), SendError>;
    /// Notification that this seat's pointer is no longer focused on a certain surface.
    /// the leave notification is sent before the enter notification for the new focus.
    fn leave(&self, serial: u32, surface: WlSurface) -> Result<(), SendError>;
    /// Notification of pointer location change. the arguments surface_x and surface_y
    /// are the location relative to the focused surface.
    fn motion(&self, time: u32, surface_x: f32, surface_y: f32) -> Result<(), SendError>;
    /// Mouse button click and release notifications. the location of the click is given
    /// by the last motion or enter event. the time argument is a timestamp with
    /// millisecond granularity, with an undefined base. the button is a button code as
    /// defined in the linux kernel's linux/input-event-codes.h header file, e.g.
    /// btn_left. any 16-bit button code value is reserved for future additions to the
    /// kernel's event code list. all other button codes above 0xffff are currently
    /// undefined but may be used in future versions of this protocol.
    fn button(
        &self,
        serial: u32,
        time: u32,
        button: u32,
        state: ButtonState,
    ) -> Result<(), SendError>;
    /// Scroll and other axis notifications. for scroll events (vertical and horizontal
    /// scroll axes), the value parameter is the length of a vector along the specified
    /// axis in a coordinate space identical to those of motion events, representing a
    /// relative movement along the specified axis. for devices that support movements
    /// non-parallel to axes multiple axis events will be emitted. when applicable, for
    /// example for touch pads, the server can choose to emit scroll events where the
    /// motion vector is equivalent to a motion event vector. when applicable, a client
    /// can transform its content relative to the scroll distance.
    fn axis(&self, time: u32, axis: Axis, value: f32) -> Result<(), SendError>;
    /// Indicates the end of a set of events that logically belong together. a client is
    /// expected to accumulate the data in all events within the frame before
    /// proceeding. all wl_pointer events before a wl_pointer.frame event belong
    /// logically together. for example, in a diagonal scroll motion the compositor will
    /// send an optional wl_pointer.axis_source event, two wl_pointer.axis events
    /// (horizontal and vertical) and finally a wl_pointer.frame event. the client may
    /// use this information to calculate a diagonal vector for scrolling. when multiple
    /// wl_pointer.axis events occur within the same frame, the motion vector is the
    /// combined motion of all events. when a wl_pointer.axis and a wl_pointer.axis_stop
    /// event occur within the same frame, this indicates that axis movement in one axis
    /// has stopped but continues in the other axis. when multiple wl_pointer.axis_stop
    /// events occur within the same frame, this indicates that these axes stopped in
    /// the same instance. a wl_pointer.frame event is sent for every logical event
    /// group, even if the group only contains a single wl_pointer event. specifically,
    /// a client may get a sequence: motion, frame, button, frame, axis, frame,
    /// axis_stop, frame. the wl_pointer.enter and wl_pointer.leave events are logical
    /// events generated by the compositor and not the hardware. these events are also
    /// grouped by a wl_pointer.frame. when a pointer moves from one surface to another,
    /// a compositor should group the wl_pointer.leave event within the same
    /// wl_pointer.frame. however, a client must not rely on wl_pointer.leave and
    /// wl_pointer.enter being in the same wl_pointer.frame. compositor-specific
    /// policies may require the wl_pointer.leave and wl_pointer.enter event being split
    /// across multiple wl_pointer.frame groups.
    fn frame(&self) -> Result<(), SendError>;
    /// Source information for scroll and other axes. this event does not occur on its
    /// own. it is sent before a wl_pointer.frame event and carries the source
    /// information for all events within that frame. the source specifies how this
    /// event was generated. if the source is wl_pointer.axis_source.finger, a
    /// wl_pointer.axis_stop event will be sent when the user lifts the finger off the
    /// device. if the source is wl_pointer.axis_source.wheel,
    /// wl_pointer.axis_source.wheel_tilt or wl_pointer.axis_source.continuous, a
    /// wl_pointer.axis_stop event may or may not be sent. whether a compositor sends an
    /// axis_stop event for these sources is hardware-specific and implementation-
    /// dependent; clients must not rely on receiving an axis_stop event for these
    /// scroll sources and should treat scroll sequences from these scroll sources as
    /// unterminated by default. this event is optional. if the source is unknown for a
    /// particular axis event sequence, no event is sent. only one
    /// wl_pointer.axis_source event is permitted per frame. the order of
    /// wl_pointer.axis_discrete and wl_pointer.axis_source is not guaranteed.
    fn axis_source(&self, axis_source: AxisSource) -> Result<(), SendError>;
    /// Stop notification for scroll and other axes. for some wl_pointer.axis_source
    /// types, a wl_pointer.axis_stop event is sent to notify a client that the axis
    /// sequence has terminated. this enables the client to implement kinetic scrolling.
    /// see the wl_pointer.axis_source documentation for information on when this event
    /// may be generated. any wl_pointer.axis events with the same axis_source after
    /// this event should be considered as the start of a new axis motion. the timestamp
    /// is to be interpreted identical to the timestamp in the wl_pointer.axis event.
    /// the timestamp value may be the same as a preceding wl_pointer.axis event.
    fn axis_stop(&self, time: u32, axis: Axis) -> Result<(), SendError>;
    /// Discrete step information for scroll and other axes. this event carries the axis
    /// value of the wl_pointer.axis event in discrete steps (e.g. mouse wheel clicks).
    /// this event does not occur on its own, it is coupled with a wl_pointer.axis event
    /// that represents this axis value on a continuous scale. the protocol guarantees
    /// that each axis_discrete event is always followed by exactly one axis event with
    /// the same axis number within the same wl_pointer.frame. note that the protocol
    /// allows for other events to occur between the axis_discrete and its coupled axis
    /// event, including other axis_discrete or axis events. this event is optional;
    /// continuous scrolling devices like two-finger scrolling on touchpads do not have
    /// discrete steps and do not generate this event. the discrete value carries the
    /// directional information. e.g. a value of -2 is two steps towards the negative
    /// direction of this axis. the axis number is identical to the axis number in the
    /// associated axis event. the order of wl_pointer.axis_discrete and
    /// wl_pointer.axis_source is not guaranteed.
    fn axis_discrete(&self, axis: Axis, discrete: i32) -> Result<(), SendError>;
}

impl WlPointerExt for WlPointer {
    /// Notification that this seat's pointer is focused on a certain surface. when a
    /// seat's focus enters a surface, the pointer image is undefined and a client
    /// should respond to this event by setting an appropriate pointer image with the
    /// set_cursor request.
    fn enter(
        &self,
        serial: u32,
        surface: WlSurface,
        surface_x: f32,
        surface_y: f32,
    ) -> Result<(), SendError> {
        self.connection().borrow_mut().send(
            Event::Enter {
                serial,
                surface,
                surface_x,
                surface_y,
            }
            .into_raw(self.id()),
        )
    }
    /// Notification that this seat's pointer is no longer focused on a certain surface.
    /// the leave notification is sent before the enter notification for the new focus.
    fn leave(&self, serial: u32, surface: WlSurface) -> Result<(), SendError> {
        self.connection()
            .borrow_mut()
            .send(Event::Leave { serial, surface }.into_raw(self.id()))
    }
    /// Notification of pointer location change. the arguments surface_x and surface_y
    /// are the location relative to the focused surface.
    fn motion(&self, time: u32, surface_x: f32, surface_y: f32) -> Result<(), SendError> {
        self.connection().borrow_mut().send(
            Event::Motion {
                time,
                surface_x,
                surface_y,
            }
            .into_raw(self.id()),
        )
    }
    /// Mouse button click and release notifications. the location of the click is given
    /// by the last motion or enter event. the time argument is a timestamp with
    /// millisecond granularity, with an undefined base. the button is a button code as
    /// defined in the linux kernel's linux/input-event-codes.h header file, e.g.
    /// btn_left. any 16-bit button code value is reserved for future additions to the
    /// kernel's event code list. all other button codes above 0xffff are currently
    /// undefined but may be used in future versions of this protocol.
    fn button(
        &self,
        serial: u32,
        time: u32,
        button: u32,
        state: ButtonState,
    ) -> Result<(), SendError> {
        self.connection().borrow_mut().send(
            Event::Button {
                serial,
                time,
                button,
                state,
            }
            .into_raw(self.id()),
        )
    }
    /// Scroll and other axis notifications. for scroll events (vertical and horizontal
    /// scroll axes), the value parameter is the length of a vector along the specified
    /// axis in a coordinate space identical to those of motion events, representing a
    /// relative movement along the specified axis. for devices that support movements
    /// non-parallel to axes multiple axis events will be emitted. when applicable, for
    /// example for touch pads, the server can choose to emit scroll events where the
    /// motion vector is equivalent to a motion event vector. when applicable, a client
    /// can transform its content relative to the scroll distance.
    fn axis(&self, time: u32, axis: Axis, value: f32) -> Result<(), SendError> {
        self.connection()
            .borrow_mut()
            .send(Event::Axis { time, axis, value }.into_raw(self.id()))
    }
    /// Indicates the end of a set of events that logically belong together. a client is
    /// expected to accumulate the data in all events within the frame before
    /// proceeding. all wl_pointer events before a wl_pointer.frame event belong
    /// logically together. for example, in a diagonal scroll motion the compositor will
    /// send an optional wl_pointer.axis_source event, two wl_pointer.axis events
    /// (horizontal and vertical) and finally a wl_pointer.frame event. the client may
    /// use this information to calculate a diagonal vector for scrolling. when multiple
    /// wl_pointer.axis events occur within the same frame, the motion vector is the
    /// combined motion of all events. when a wl_pointer.axis and a wl_pointer.axis_stop
    /// event occur within the same frame, this indicates that axis movement in one axis
    /// has stopped but continues in the other axis. when multiple wl_pointer.axis_stop
    /// events occur within the same frame, this indicates that these axes stopped in
    /// the same instance. a wl_pointer.frame event is sent for every logical event
    /// group, even if the group only contains a single wl_pointer event. specifically,
    /// a client may get a sequence: motion, frame, button, frame, axis, frame,
    /// axis_stop, frame. the wl_pointer.enter and wl_pointer.leave events are logical
    /// events generated by the compositor and not the hardware. these events are also
    /// grouped by a wl_pointer.frame. when a pointer moves from one surface to another,
    /// a compositor should group the wl_pointer.leave event within the same
    /// wl_pointer.frame. however, a client must not rely on wl_pointer.leave and
    /// wl_pointer.enter being in the same wl_pointer.frame. compositor-specific
    /// policies may require the wl_pointer.leave and wl_pointer.enter event being split
    /// across multiple wl_pointer.frame groups.
    fn frame(&self) -> Result<(), SendError> {
        self.connection()
            .borrow_mut()
            .send(Event::Frame {}.into_raw(self.id()))
    }
    /// Source information for scroll and other axes. this event does not occur on its
    /// own. it is sent before a wl_pointer.frame event and carries the source
    /// information for all events within that frame. the source specifies how this
    /// event was generated. if the source is wl_pointer.axis_source.finger, a
    /// wl_pointer.axis_stop event will be sent when the user lifts the finger off the
    /// device. if the source is wl_pointer.axis_source.wheel,
    /// wl_pointer.axis_source.wheel_tilt or wl_pointer.axis_source.continuous, a
    /// wl_pointer.axis_stop event may or may not be sent. whether a compositor sends an
    /// axis_stop event for these sources is hardware-specific and implementation-
    /// dependent; clients must not rely on receiving an axis_stop event for these
    /// scroll sources and should treat scroll sequences from these scroll sources as
    /// unterminated by default. this event is optional. if the source is unknown for a
    /// particular axis event sequence, no event is sent. only one
    /// wl_pointer.axis_source event is permitted per frame. the order of
    /// wl_pointer.axis_discrete and wl_pointer.axis_source is not guaranteed.
    fn axis_source(&self, axis_source: AxisSource) -> Result<(), SendError> {
        self.connection()
            .borrow_mut()
            .send(Event::AxisSource { axis_source }.into_raw(self.id()))
    }
    /// Stop notification for scroll and other axes. for some wl_pointer.axis_source
    /// types, a wl_pointer.axis_stop event is sent to notify a client that the axis
    /// sequence has terminated. this enables the client to implement kinetic scrolling.
    /// see the wl_pointer.axis_source documentation for information on when this event
    /// may be generated. any wl_pointer.axis events with the same axis_source after
    /// this event should be considered as the start of a new axis motion. the timestamp
    /// is to be interpreted identical to the timestamp in the wl_pointer.axis event.
    /// the timestamp value may be the same as a preceding wl_pointer.axis event.
    fn axis_stop(&self, time: u32, axis: Axis) -> Result<(), SendError> {
        self.connection()
            .borrow_mut()
            .send(Event::AxisStop { time, axis }.into_raw(self.id()))
    }
    /// Discrete step information for scroll and other axes. this event carries the axis
    /// value of the wl_pointer.axis event in discrete steps (e.g. mouse wheel clicks).
    /// this event does not occur on its own, it is coupled with a wl_pointer.axis event
    /// that represents this axis value on a continuous scale. the protocol guarantees
    /// that each axis_discrete event is always followed by exactly one axis event with
    /// the same axis number within the same wl_pointer.frame. note that the protocol
    /// allows for other events to occur between the axis_discrete and its coupled axis
    /// event, including other axis_discrete or axis events. this event is optional;
    /// continuous scrolling devices like two-finger scrolling on touchpads do not have
    /// discrete steps and do not generate this event. the discrete value carries the
    /// directional information. e.g. a value of -2 is two steps towards the negative
    /// direction of this axis. the axis number is identical to the axis number in the
    /// associated axis event. the order of wl_pointer.axis_discrete and
    /// wl_pointer.axis_source is not guaranteed.
    fn axis_discrete(&self, axis: Axis, discrete: i32) -> Result<(), SendError> {
        self.connection()
            .borrow_mut()
            .send(Event::AxisDiscrete { axis, discrete }.into_raw(self.id()))
    }
}
