//! A surface is a rectangular area that may be displayed on zero or more outputs,
//! and shown any number of times at the compositor's discretion. they can present
//! wl_buffers, receive user input, and define a local coordinate system. the size
//! of a surface (and relative positions on it) is described in surface-local
//! coordinates, which may differ from the buffer coordinates of the pixel content,
//! in case a buffer_transform or a buffer_scale is used. a surface without a "role"
//! is fairly useless: a compositor does not know where, when or how to present it.
//! the role is the purpose of a wl_surface. examples of roles are a cursor for a
//! pointer (as set by wl_pointer.set_cursor), a drag icon
//! (wl_data_device.start_drag), a sub-surface (wl_subcompositor.get_subsurface),
//! and a window as defined by a shell protocol (e.g. wl_shell.get_shell_surface). a
//! surface can have only one role at a time. initially a wl_surface does not have a
//! role. once a wl_surface is given a role, it is set permanently for the whole
//! lifetime of the wl_surface object. giving the current role again is allowed,
//! unless explicitly forbidden by the relevant interface specification. surface
//! roles are given by requests in other interfaces such as wl_pointer.set_cursor.
//! the request should explicitly mention that this request gives a role to a
//! wl_surface. often, this request also creates a new protocol object that
//! represents the role and adds additional functionality to wl_surface. when a
//! client wants to destroy a wl_surface, they must destroy this 'role object'
//! before the wl_surface. destroying the role object does not remove the role from
//! the wl_surface, but it may stop the wl_surface from "playing the role". for
//! instance, if a wl_subsurface object is destroyed, the wl_surface it was created
//! for will be unmapped and forget its position and z-order. it is allowed to
//! create a wl_subsurface for the same wl_surface again, but it is not allowed to
//! use the wl_surface as a cursor (cursor is a different role than sub-surface, and
//! role switching is not allowed).

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
use crate::wl::protocols::common::wl_shm_pool::WlShmPool;
use crate::wl::protocols::common::wl_subcompositor::WlSubcompositor;
use crate::wl::protocols::common::wl_subsurface::WlSubsurface;

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
    /// Deletes the surface and invalidates its object id.
    Destroy {},
    /// Set a buffer as the content of this surface. the new size of the surface is
    /// calculated based on the buffer size transformed by the inverse buffer_transform
    /// and the inverse buffer_scale. this means that at commit time the supplied buffer
    /// size must be an integer multiple of the buffer_scale. if that's not the case, an
    /// invalid_size error is sent. the x and y arguments specify the location of the
    /// new pending buffer's upper left corner, relative to the current buffer's upper
    /// left corner, in surface-local coordinates. in other words, the x and y, combined
    /// with the new surface size define in which directions the surface's size changes.
    /// surface contents are double-buffered state, see wl_surface.commit. the initial
    /// surface contents are void; there is no content. wl_surface.attach assigns the
    /// given wl_buffer as the pending wl_buffer. wl_surface.commit makes the pending
    /// wl_buffer the new surface contents, and the size of the surface becomes the size
    /// calculated from the wl_buffer, as described above. after commit, there is no
    /// pending buffer until the next attach. committing a pending wl_buffer allows the
    /// compositor to read the pixels in the wl_buffer. the compositor may access the
    /// pixels at any time after the wl_surface.commit request. when the compositor will
    /// not access the pixels anymore, it will send the wl_buffer.release event. only
    /// after receiving wl_buffer.release, the client may reuse the wl_buffer. a
    /// wl_buffer that has been attached and then replaced by another attach instead of
    /// committed will not receive a release event, and is not used by the compositor.
    /// if a pending wl_buffer has been committed to more than one wl_surface, the
    /// delivery of wl_buffer.release events becomes undefined. a well behaved client
    /// should not rely on wl_buffer.release events in this case. alternatively, a
    /// client could create multiple wl_buffer objects from the same backing storage or
    /// use wp_linux_buffer_release. destroying the wl_buffer after wl_buffer.release
    /// does not change the surface contents. however, if the client destroys the
    /// wl_buffer before receiving the wl_buffer.release event, the surface contents
    /// become undefined immediately. if wl_surface.attach is sent with a null
    /// wl_buffer, the following wl_surface.commit will remove the surface content.
    Attach {
        /// Buffer of surface contents.
        buffer: Option<WlBuffer>,
        /// Surface-local x coordinate.
        x: i32,
        /// Surface-local y coordinate.
        y: i32,
    },
    /// This request is used to describe the regions where the pending buffer is
    /// different from the current surface contents, and where the surface therefore
    /// needs to be repainted. the compositor ignores the parts of the damage that fall
    /// outside of the surface. damage is double-buffered state, see wl_surface.commit.
    /// the damage rectangle is specified in surface-local coordinates, where x and y
    /// specify the upper left corner of the damage rectangle. the initial value for
    /// pending damage is empty: no damage. wl_surface.damage adds pending damage: the
    /// new pending damage is the union of old pending damage and the given rectangle.
    /// wl_surface.commit assigns pending damage as the current damage, and clears
    /// pending damage. the server will clear the current damage as it repaints the
    /// surface. note! new clients should not use this request. instead damage can be
    /// posted with wl_surface.damage_buffer which uses buffer coordinates instead of
    /// surface coordinates.
    Damage {
        /// Surface-local x coordinate.
        x: i32,
        /// Surface-local y coordinate.
        y: i32,
        /// Width of damage rectangle.
        width: i32,
        /// Height of damage rectangle.
        height: i32,
    },
    /// Request a notification when it is a good time to start drawing a new frame, by
    /// creating a frame callback. this is useful for throttling redrawing operations,
    /// and driving animations. when a client is animating on a wl_surface, it can use
    /// the 'frame' request to get notified when it is a good time to draw and commit
    /// the next frame of animation. if the client commits an update earlier than that,
    /// it is likely that some updates will not make it to the display, and the client
    /// is wasting resources by drawing too often. the frame request will take effect on
    /// the next wl_surface.commit. the notification will only be posted for one frame
    /// unless requested again. for a wl_surface, the notifications are posted in the
    /// order the frame requests were committed. the server must send the notifications
    /// so that a client will not send excessive updates, while still allowing the
    /// highest possible update rate for clients that wait for the reply before drawing
    /// again. the server should give some time for the client to draw and commit after
    /// sending the frame callback events to let it hit the next output refresh. a
    /// server should avoid signaling the frame callbacks if the surface is not visible
    /// in any way, e.g. the surface is off-screen, or completely obscured by other
    /// opaque surfaces. the object returned by this request will be destroyed by the
    /// compositor after the callback is fired and as such the client must not attempt
    /// to use it after that point. the callback_data passed in the callback is the
    /// current time, in milliseconds, with an undefined base.
    Frame {
        /// Callback object for the frame request.
        callback: NewId,
    },
    /// This request sets the region of the surface that contains opaque content. the
    /// opaque region is an optimization hint for the compositor that lets it optimize
    /// the redrawing of content behind opaque regions. setting an opaque region is not
    /// required for correct behaviour, but marking transparent content as opaque will
    /// result in repaint artifacts. the opaque region is specified in surface-local
    /// coordinates. the compositor ignores the parts of the opaque region that fall
    /// outside of the surface. opaque region is double-buffered state, see
    /// wl_surface.commit. wl_surface.set_opaque_region changes the pending opaque
    /// region. wl_surface.commit copies the pending region to the current region.
    /// otherwise, the pending and current regions are never changed. the initial value
    /// for an opaque region is empty. setting the pending opaque region has copy
    /// semantics, and the wl_region object can be destroyed immediately. a null
    /// wl_region causes the pending opaque region to be set to empty.
    SetOpaqueRegion {
        /// Opaque region of the surface.
        region: Option<WlRegion>,
    },
    /// This request sets the region of the surface that can receive pointer and touch
    /// events. input events happening outside of this region will try the next surface
    /// in the server surface stack. the compositor ignores the parts of the input
    /// region that fall outside of the surface. the input region is specified in
    /// surface-local coordinates. input region is double-buffered state, see
    /// wl_surface.commit. wl_surface.set_input_region changes the pending input region.
    /// wl_surface.commit copies the pending region to the current region. otherwise the
    /// pending and current regions are never changed, except cursor and icon surfaces
    /// are special cases, see wl_pointer.set_cursor and wl_data_device.start_drag. the
    /// initial value for an input region is infinite. that means the whole surface will
    /// accept input. setting the pending input region has copy semantics, and the
    /// wl_region object can be destroyed immediately. a null wl_region causes the input
    /// region to be set to infinite.
    SetInputRegion {
        /// Input region of the surface.
        region: Option<WlRegion>,
    },
    /// Surface state (input, opaque, and damage regions, attached buffers, etc.) is
    /// double-buffered. protocol requests modify the pending state, as opposed to the
    /// current state in use by the compositor. a commit request atomically applies all
    /// pending state, replacing the current state. after commit, the new pending state
    /// is as documented for each related request. on commit, a pending wl_buffer is
    /// applied first, and all other state second. this means that all coordinates in
    /// double-buffered state are relative to the new wl_buffer coming into use, except
    /// for wl_surface.attach itself. if there is no pending wl_buffer, the coordinates
    /// are relative to the current surface contents. all requests that need a commit to
    /// become effective are documented to affect double-buffered state. other
    /// interfaces may add further double-buffered surface state.
    Commit {},
    /// This request sets an optional transformation on how the compositor interprets
    /// the contents of the buffer attached to the surface. the accepted values for the
    /// transform parameter are the values for wl_output.transform. buffer transform is
    /// double-buffered state, see wl_surface.commit. a newly created surface has its
    /// buffer transformation set to normal. wl_surface.set_buffer_transform changes the
    /// pending buffer transformation. wl_surface.commit copies the pending buffer
    /// transformation to the current one. otherwise, the pending and current values are
    /// never changed. the purpose of this request is to allow clients to render content
    /// according to the output transform, thus permitting the compositor to use certain
    /// optimizations even if the display is rotated. using hardware overlays and
    /// scanning out a client buffer for fullscreen surfaces are examples of such
    /// optimizations. those optimizations are highly dependent on the compositor
    /// implementation, so the use of this request should be considered on a case-by-
    /// case basis. note that if the transform value includes 90 or 270 degree rotation,
    /// the width of the buffer will become the surface height and the height of the
    /// buffer will become the surface width. if transform is not one of the values from
    /// the wl_output.transform enum the invalid_transform protocol error is raised.
    SetBufferTransform {
        /// Transform for interpreting buffer contents.
        transform: super::super::common::wl_output::Transform,
    },
    /// This request sets an optional scaling factor on how the compositor interprets
    /// the contents of the buffer attached to the window. buffer scale is double-
    /// buffered state, see wl_surface.commit. a newly created surface has its buffer
    /// scale set to 1. wl_surface.set_buffer_scale changes the pending buffer scale.
    /// wl_surface.commit copies the pending buffer scale to the current one. otherwise,
    /// the pending and current values are never changed. the purpose of this request is
    /// to allow clients to supply higher resolution buffer data for use on high
    /// resolution outputs. it is intended that you pick the same buffer scale as the
    /// scale of the output that the surface is displayed on. this means the compositor
    /// can avoid scaling when rendering the surface on that output. note that if the
    /// scale is larger than 1, then you have to attach a buffer that is larger (by a
    /// factor of scale in each dimension) than the desired surface size. if scale is
    /// not positive the invalid_scale protocol error is raised.
    SetBufferScale {
        /// Positive scale for interpreting buffer contents.
        scale: i32,
    },
    /// This request is used to describe the regions where the pending buffer is
    /// different from the current surface contents, and where the surface therefore
    /// needs to be repainted. the compositor ignores the parts of the damage that fall
    /// outside of the surface. damage is double-buffered state, see wl_surface.commit.
    /// the damage rectangle is specified in buffer coordinates, where x and y specify
    /// the upper left corner of the damage rectangle. the initial value for pending
    /// damage is empty: no damage. wl_surface.damage_buffer adds pending damage: the
    /// new pending damage is the union of old pending damage and the given rectangle.
    /// wl_surface.commit assigns pending damage as the current damage, and clears
    /// pending damage. the server will clear the current damage as it repaints the
    /// surface. this request differs from wl_surface.damage in only one way - it takes
    /// damage in buffer coordinates instead of surface-local coordinates. while this
    /// generally is more intuitive than surface coordinates, it is especially desirable
    /// when using wp_viewport or when a drawing library (like egl) is unaware of buffer
    /// scale and buffer transform. note: because buffer transformation changes and
    /// damage requests may be interleaved in the protocol stream, it is impossible to
    /// determine the actual mapping between surface and buffer damage until
    /// wl_surface.commit time. therefore, compositors wishing to take both kinds of
    /// damage into account will have to accumulate damage from the two requests
    /// separately and only transform from one to the other after receiving the
    /// wl_surface.commit.
    DamageBuffer {
        /// Buffer-local x coordinate.
        x: i32,
        /// Buffer-local y coordinate.
        y: i32,
        /// Width of damage rectangle.
        width: i32,
        /// Height of damage rectangle.
        height: i32,
    },
}

impl Message for Request {
    fn into_raw(self, sender: ObjectId) -> RawMessage {
        match self {
            Request::Destroy {} => RawMessage {
                sender,
                opcode: Opcode(1),
                args: smallvec![],
            },
            Request::Attach { buffer, x, y } => RawMessage {
                sender,
                opcode: Opcode(2),
                args: smallvec![buffer.into(), x.into(), y.into()],
            },
            Request::Damage {
                x,
                y,
                width,
                height,
            } => RawMessage {
                sender,
                opcode: Opcode(3),
                args: smallvec![x.into(), y.into(), width.into(), height.into()],
            },
            Request::Frame { callback } => RawMessage {
                sender,
                opcode: Opcode(4),
                args: smallvec![callback.into()],
            },
            Request::SetOpaqueRegion { region } => RawMessage {
                sender,
                opcode: Opcode(5),
                args: smallvec![region.into()],
            },
            Request::SetInputRegion { region } => RawMessage {
                sender,
                opcode: Opcode(6),
                args: smallvec![region.into()],
            },
            Request::Commit {} => RawMessage {
                sender,
                opcode: Opcode(7),
                args: smallvec![],
            },
            Request::SetBufferTransform { transform } => RawMessage {
                sender,
                opcode: Opcode(8),
                args: smallvec![transform.into()],
            },
            Request::SetBufferScale { scale } => RawMessage {
                sender,
                opcode: Opcode(9),
                args: smallvec![scale.into()],
            },
            Request::DamageBuffer {
                x,
                y,
                width,
                height,
            } => RawMessage {
                sender,
                opcode: Opcode(10),
                args: smallvec![x.into(), y.into(), width.into(), height.into()],
            },
        }
    }

    fn from_raw(
        con: Rc<RefCell<dyn Connection>>,
        m: &RawMessage,
    ) -> Result<Request, DeserializeError> {
        match m.opcode {
            Opcode(1) => Ok(Request::Destroy {}),
            Opcode(2) => Ok(Request::Attach {
                buffer: from_optional_object_payload!(WlBuffer, con.clone(), m.args[0]),

                x: from_payload!(Int, m.args[1]),

                y: from_payload!(Int, m.args[2]),
            }),
            Opcode(3) => Ok(Request::Damage {
                x: from_payload!(Int, m.args[0]),

                y: from_payload!(Int, m.args[1]),

                width: from_payload!(Int, m.args[2]),

                height: from_payload!(Int, m.args[3]),
            }),
            Opcode(4) => Ok(Request::Frame {
                callback: from_payload!(NewId, m.args[0]),
            }),
            Opcode(5) => Ok(Request::SetOpaqueRegion {
                region: from_optional_object_payload!(WlRegion, con.clone(), m.args[0]),
            }),
            Opcode(6) => Ok(Request::SetInputRegion {
                region: from_optional_object_payload!(WlRegion, con.clone(), m.args[0]),
            }),
            Opcode(7) => Ok(Request::Commit {}),
            Opcode(8) => Ok(Request::SetBufferTransform {
                transform: from_payload!(Int, m.args[0]),
            }),
            Opcode(9) => Ok(Request::SetBufferScale {
                scale: from_payload!(Int, m.args[0]),
            }),
            Opcode(10) => Ok(Request::DamageBuffer {
                x: from_payload!(Int, m.args[0]),

                y: from_payload!(Int, m.args[1]),

                width: from_payload!(Int, m.args[2]),

                height: from_payload!(Int, m.args[3]),
            }),

            _ => Err(DeserializeError::UnknownOpcode),
        }
    }

    fn into_received_event(self, con: Rc<RefCell<dyn Connection>>, id: ObjectId) -> EventSet {
        panic!("not a event!");
    }

    fn into_received_request(self) -> RequestSet {
        RequestSet::WlSurface(self)
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Error {
    /// Buffer scale value is invalid.
    InvalidScale = 0,
    /// Buffer transform value is invalid.
    InvalidTransform = 1,
    /// Buffer size is invalid.
    InvalidSize = 2,
}

impl Into<Payload> for Error {
    fn into(self) -> Payload {
        Payload::UInt(self as u32)
    }
}

impl From<u32> for Error {
    fn from(value: u32) -> Error {
        match value {
            0 => Error::InvalidScale,
            1 => Error::InvalidTransform,
            2 => Error::InvalidSize,

            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub enum Event {
    /// This is emitted whenever a surface's creation, movement, or resizing results in
    /// some part of it being within the scanout region of an output. note that a
    /// surface may be overlapping with zero or more outputs.
    Enter {
        /// Output entered by the surface.
        output: WlOutput,
    },
    /// This is emitted whenever a surface's creation, movement, or resizing results in
    /// it no longer having any part of it within the scanout region of an output.
    /// clients should not use the number of outputs the surface is on for frame
    /// throttling purposes. the surface might be hidden even if no leave event has been
    /// sent, and the compositor might expect new surface content updates even if no
    /// enter event has been sent. the frame event should be used instead.
    Leave {
        /// Output left by the surface.
        output: WlOutput,
    },
}

impl Message for Event {
    fn into_raw(self, sender: ObjectId) -> RawMessage {
        match self {
            Event::Enter { output } => RawMessage {
                sender,
                opcode: Opcode(11),
                args: smallvec![output.into()],
            },
            Event::Leave { output } => RawMessage {
                sender,
                opcode: Opcode(12),
                args: smallvec![output.into()],
            },
        }
    }

    fn from_raw(
        con: Rc<RefCell<dyn Connection>>,
        m: &RawMessage,
    ) -> Result<Event, DeserializeError> {
        match m.opcode {
            Opcode(11) => Ok(Event::Enter {
                output: from_object_payload!(WlOutput, con.clone(), m.args[0]),
            }),
            Opcode(12) => Ok(Event::Leave {
                output: from_object_payload!(WlOutput, con.clone(), m.args[0]),
            }),

            _ => Err(DeserializeError::UnknownOpcode),
        }
    }

    fn into_received_event(self, con: Rc<RefCell<dyn Connection>>, id: ObjectId) -> EventSet {
        EventSet::WlSurface(WlSurface::new(con, id), self)
    }

    fn into_received_request(self) -> RequestSet {
        panic!("not a request!");
    }
}

/// An onscreen surface.
#[derive(Clone)]
pub struct WlSurface {
    con: Rc<RefCell<dyn Connection>>,
    object_id: ObjectId,
}

impl PartialEq for WlSurface {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl core::fmt::Debug for WlSurface {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "WlSurface@{}", self.object_id.0)
    }
}

impl Into<Payload> for WlSurface {
    fn into(self) -> Payload {
        Payload::ObjectId(self.id())
    }
}

impl Interface for WlSurface {
    type Event = Event;
    type Request = Request;
    const NAME: &'static str = "wl_surface";
    const VERSION: u32 = 4;
    const PAYLOAD_TYPES: &'static [&'static [PayloadType]] = &[
        &[],
        &[PayloadType::ObjectId, PayloadType::Int, PayloadType::Int],
        &[
            PayloadType::Int,
            PayloadType::Int,
            PayloadType::Int,
            PayloadType::Int,
        ],
        &[PayloadType::NewId],
        &[PayloadType::ObjectId],
        &[PayloadType::ObjectId],
        &[],
        &[PayloadType::Int],
        &[PayloadType::Int],
        &[
            PayloadType::Int,
            PayloadType::Int,
            PayloadType::Int,
            PayloadType::Int,
        ],
        &[PayloadType::ObjectId],
        &[PayloadType::ObjectId],
    ];

    fn new(con: Rc<RefCell<dyn Connection>>, object_id: ObjectId) -> WlSurface {
        WlSurface { con, object_id }
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
