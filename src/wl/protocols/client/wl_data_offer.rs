//! A wl_data_offer represents a piece of data offered for transfer by another
//! client (the source client). it is used by the copy-and-paste and drag-and-drop
//! mechanisms. the offer describes the different mime types that the data can be
//! converted to and provides the mechanism for transferring the data directly from
//! the source client.

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

use crate::wl::protocols::common::wl_data_offer::*;
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

pub trait WlDataOfferExt {
    /// Indicate that the client can accept the given mime type, or null for not
    /// accepted. for objects of version 2 or older, this request is used by the client
    /// to give feedback whether the client can receive the given mime type, or null if
    /// none is accepted; the feedback does not determine whether the drag-and-drop
    /// operation succeeds or not. for objects of version 3 or newer, this request
    /// determines the final result of the drag-and-drop operation. if the end result is
    /// that no mime types were accepted, the drag-and-drop operation will be cancelled
    /// and the corresponding drag source will receive wl_data_source.cancelled. clients
    /// may still use this event in conjunction with wl_data_source.action for feedback.
    fn accept(&self, serial: u32, mime_type: Option<String>) -> Result<(), SendError>;
    /// To transfer the offered data, the client issues this request and indicates the
    /// mime type it wants to receive. the transfer happens through the passed file
    /// descriptor (typically created with the pipe system call). the source client
    /// writes the data in the mime type representation requested and then closes the
    /// file descriptor. the receiving client reads from the read end of the pipe until
    /// eof and then closes its end, at which point the transfer is complete. this
    /// request may happen multiple times for different mime types, both before and
    /// after wl_data_device.drop. drag-and-drop destination clients may preemptively
    /// fetch data or examine it more closely to determine acceptance.
    fn receive(&self, mime_type: String, fd: Handle) -> Result<(), SendError>;
    /// Destroy the data offer.
    fn destroy(&self) -> Result<(), SendError>;
    /// Notifies the compositor that the drag destination successfully finished the
    /// drag-and-drop operation. upon receiving this request, the compositor will emit
    /// wl_data_source.dnd_finished on the drag source client. it is a client error to
    /// perform other requests than wl_data_offer.destroy after this one. it is also an
    /// error to perform this request after a null mime type has been set in
    /// wl_data_offer.accept or no action was received through wl_data_offer.action. if
    /// wl_data_offer.finish request is received for a non drag and drop operation, the
    /// invalid_finish protocol error is raised.
    fn finish(&self) -> Result<(), SendError>;
    /// Sets the actions that the destination side client supports for this operation.
    /// this request may trigger the emission of wl_data_source.action and
    /// wl_data_offer.action events if the compositor needs to change the selected
    /// action. this request can be called multiple times throughout the drag-and-drop
    /// operation, typically in response to wl_data_device.enter or
    /// wl_data_device.motion events. this request determines the final result of the
    /// drag-and-drop operation. if the end result is that no action is accepted, the
    /// drag source will receive wl_data_source.cancelled. the dnd_actions argument must
    /// contain only values expressed in the wl_data_device_manager.dnd_actions enum,
    /// and the preferred_action argument must only contain one of those values set,
    /// otherwise it will result in a protocol error. while managing an "ask" action,
    /// the destination drag-and-drop client may perform further wl_data_offer.receive
    /// requests, and is expected to perform one last wl_data_offer.set_actions request
    /// with a preferred action other than "ask" (and optionally wl_data_offer.accept)
    /// before requesting wl_data_offer.finish, in order to convey the action selected
    /// by the user. if the preferred action is not in the wl_data_offer.source_actions
    /// mask, an error will be raised. if the "ask" action is dismissed (e.g. user
    /// cancellation), the client is expected to perform wl_data_offer.destroy right
    /// away. this request can only be made on drag-and-drop offers, a protocol error
    /// will be raised otherwise.
    fn set_actions(
        &self,
        dnd_actions: super::super::common::wl_data_device_manager::DndAction,
        preferred_action: super::super::common::wl_data_device_manager::DndAction,
    ) -> Result<(), SendError>;
}

impl WlDataOfferExt for WlDataOffer {
    /// Indicate that the client can accept the given mime type, or null for not
    /// accepted. for objects of version 2 or older, this request is used by the client
    /// to give feedback whether the client can receive the given mime type, or null if
    /// none is accepted; the feedback does not determine whether the drag-and-drop
    /// operation succeeds or not. for objects of version 3 or newer, this request
    /// determines the final result of the drag-and-drop operation. if the end result is
    /// that no mime types were accepted, the drag-and-drop operation will be cancelled
    /// and the corresponding drag source will receive wl_data_source.cancelled. clients
    /// may still use this event in conjunction with wl_data_source.action for feedback.
    fn accept(&self, serial: u32, mime_type: Option<String>) -> Result<(), SendError> {
        self.connection()
            .borrow_mut()
            .send(Request::Accept { serial, mime_type }.into_raw(self.id()))
    }
    /// To transfer the offered data, the client issues this request and indicates the
    /// mime type it wants to receive. the transfer happens through the passed file
    /// descriptor (typically created with the pipe system call). the source client
    /// writes the data in the mime type representation requested and then closes the
    /// file descriptor. the receiving client reads from the read end of the pipe until
    /// eof and then closes its end, at which point the transfer is complete. this
    /// request may happen multiple times for different mime types, both before and
    /// after wl_data_device.drop. drag-and-drop destination clients may preemptively
    /// fetch data or examine it more closely to determine acceptance.
    fn receive(&self, mime_type: String, fd: Handle) -> Result<(), SendError> {
        self.connection()
            .borrow_mut()
            .send(Request::Receive { mime_type, fd }.into_raw(self.id()))
    }
    /// Destroy the data offer.
    fn destroy(&self) -> Result<(), SendError> {
        self.connection()
            .borrow_mut()
            .send(Request::Destroy {}.into_raw(self.id()))
    }
    /// Notifies the compositor that the drag destination successfully finished the
    /// drag-and-drop operation. upon receiving this request, the compositor will emit
    /// wl_data_source.dnd_finished on the drag source client. it is a client error to
    /// perform other requests than wl_data_offer.destroy after this one. it is also an
    /// error to perform this request after a null mime type has been set in
    /// wl_data_offer.accept or no action was received through wl_data_offer.action. if
    /// wl_data_offer.finish request is received for a non drag and drop operation, the
    /// invalid_finish protocol error is raised.
    fn finish(&self) -> Result<(), SendError> {
        self.connection()
            .borrow_mut()
            .send(Request::Finish {}.into_raw(self.id()))
    }
    /// Sets the actions that the destination side client supports for this operation.
    /// this request may trigger the emission of wl_data_source.action and
    /// wl_data_offer.action events if the compositor needs to change the selected
    /// action. this request can be called multiple times throughout the drag-and-drop
    /// operation, typically in response to wl_data_device.enter or
    /// wl_data_device.motion events. this request determines the final result of the
    /// drag-and-drop operation. if the end result is that no action is accepted, the
    /// drag source will receive wl_data_source.cancelled. the dnd_actions argument must
    /// contain only values expressed in the wl_data_device_manager.dnd_actions enum,
    /// and the preferred_action argument must only contain one of those values set,
    /// otherwise it will result in a protocol error. while managing an "ask" action,
    /// the destination drag-and-drop client may perform further wl_data_offer.receive
    /// requests, and is expected to perform one last wl_data_offer.set_actions request
    /// with a preferred action other than "ask" (and optionally wl_data_offer.accept)
    /// before requesting wl_data_offer.finish, in order to convey the action selected
    /// by the user. if the preferred action is not in the wl_data_offer.source_actions
    /// mask, an error will be raised. if the "ask" action is dismissed (e.g. user
    /// cancellation), the client is expected to perform wl_data_offer.destroy right
    /// away. this request can only be made on drag-and-drop offers, a protocol error
    /// will be raised otherwise.
    fn set_actions(
        &self,
        dnd_actions: super::super::common::wl_data_device_manager::DndAction,
        preferred_action: super::super::common::wl_data_device_manager::DndAction,
    ) -> Result<(), SendError> {
        self.connection().borrow_mut().send(
            Request::SetActions {
                dnd_actions,
                preferred_action,
            }
            .into_raw(self.id()),
        )
    }
}
